use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use anyhow::anyhow;
use bytes::Bytes;
use hmac::{Hmac, Mac};
use libaes::{Cipher, AES_256_KEY_LEN};
use p256::ecdh::diffie_hellman;
use p256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use p256::{EncodedPoint, PublicKey};
use prost::Message;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::sync::broadcast::{Receiver, Sender};
use ts_rs::TS;

use super::info::{InternalFileInfo, TransferMetadata};
use super::{InnerState, State};
use crate::channel::{ChannelAction, ChannelDirection, ChannelMessage};
use crate::location_nearby_connections::bandwidth_upgrade_negotiation_frame::upgrade_path_info::Medium;
use crate::location_nearby_connections::connection_response_frame::ResponseStatus;
use crate::location_nearby_connections::payload_transfer_frame::{
    payload_header, PacketType, PayloadChunk, PayloadHeader,
};
use crate::location_nearby_connections::{KeepAliveFrame, OfflineFrame, PayloadTransferFrame};
use crate::protocol::{checked_payload_buffer_size, SANE_FRAME_LENGTH, SANITY_DURATION};
use crate::securegcm::ukey2_alert::AlertType;
use crate::securegcm::ukey2_client_init::CipherCommitment;
use crate::securegcm::{
    ukey2_message, DeviceToDeviceMessage, GcmMetadata, Type, Ukey2Alert, Ukey2ClientFinished,
    Ukey2ClientInit, Ukey2HandshakeCipher, Ukey2Message, Ukey2ServerInit,
};
use crate::securemessage::{
    EcP256PublicKey, EncScheme, GenericPublicKey, Header, HeaderAndBody, PublicKeyType,
    SecureMessage, SigScheme,
};
use crate::sharing_nearby::{
    file_metadata, paired_key_result_frame, FileMetadata, IntroductionFrame,
};
use crate::utils::{
    encode_point, gen_ecdsa_keypair, gen_random, hkdf_extract_expand, normalize_p256_coordinate,
    stream_read_exact, to_four_digit_string, DeviceType, RemoteDeviceInfo,
};
use crate::{location_nearby_connections, sharing_nearby};

type HmacSha256 = Hmac<Sha256>;

fn is_cancel_request(message: &ChannelMessage, transfer_id: &str) -> bool {
    message.direction == ChannelDirection::FrontToLib
        && message.id == transfer_id
        && message.action == Some(ChannelAction::CancelTransfer)
}

struct PreparedOutboundFiles {
    metadata: Vec<FileMetadata>,
    files: HashMap<i64, InternalFileInfo>,
    total_bytes: u64,
}

fn prepare_outbound_files(files: &[String]) -> Result<PreparedOutboundFiles, anyhow::Error> {
    let mut file_metadata = Vec::with_capacity(files.len());
    let mut transferred_files = HashMap::new();
    let mut total_to_send = 0_u64;

    for file_path in files {
        let path = Path::new(file_path);
        if !path.is_file() {
            warn!("Path is not a file: {file_path}");
            continue;
        }

        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                error!("Failed to open file: {file_path}: {error:?}");
                continue;
            }
        };
        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                error!("Failed to get metadata for: {file_path}: {error:?}");
                continue;
            }
        };

        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        let attachment_type = if mime_type.starts_with("image/") {
            file_metadata::Type::Image
        } else if mime_type.starts_with("video/") {
            file_metadata::Type::Video
        } else if mime_type.starts_with("audio/") {
            file_metadata::Type::Audio
        } else if path.extension().is_some_and(|extension| extension == "apk") {
            file_metadata::Type::App
        } else {
            file_metadata::Type::Unknown
        };

        let file_name = path
            .file_name()
            .ok_or_else(|| anyhow!("Failed to get file_name for {file_path}"))?
            .to_str()
            .ok_or_else(|| anyhow!("File name is not valid UTF-8: {file_path}"))?
            .to_owned();

        let payload_id = loop {
            let candidate = rand::rng().random::<i64>();
            if !transferred_files.contains_key(&candidate) {
                break candidate;
            }
        };

        let file_size = i64::try_from(metadata.size())
            .map_err(|_| anyhow!("File is too large to represent in the protocol: {file_path}"))?;

        let protocol_metadata = FileMetadata {
            payload_id: Some(payload_id),
            name: Some(file_name),
            size: Some(file_size),
            mime_type: Some(mime_type),
            r#type: Some(attachment_type.into()),
            ..Default::default()
        };

        transferred_files.insert(
            payload_id,
            InternalFileInfo {
                payload_id,
                file_url: path.to_path_buf(),
                bytes_transferred: 0,
                total_size: file_size,
                file: Some(file),
            },
        );
        file_metadata.push(protocol_metadata);

        total_to_send = total_to_send
            .checked_add(metadata.size())
            .ok_or_else(|| anyhow!("Total outbound transfer size overflow"))?;
    }

    Ok(PreparedOutboundFiles {
        metadata: file_metadata,
        files: transferred_files,
        total_bytes: total_to_send,
    })
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum OutboundPayload {
    Files(Vec<String>),
}

#[derive(Debug)]
pub struct OutboundRequest {
    endpoint_id: [u8; 4],
    socket: TcpStream,
    pub state: InnerState,
    sender: Sender<ChannelMessage>,
    receiver: Receiver<ChannelMessage>,
    payload: OutboundPayload,
}

impl OutboundRequest {
    pub fn new(
        endpoint_id: [u8; 4],
        socket: TcpStream,
        id: String,
        sender: Sender<ChannelMessage>,
        payload: OutboundPayload,
        rdi: RemoteDeviceInfo,
    ) -> Self {
        let receiver = sender.subscribe();
        let OutboundPayload::Files(files) = &payload;

        Self {
            endpoint_id,
            socket,
            state: InnerState {
                id,
                server_seq: 0,
                client_seq: 0,
                state: State::Initial,
                encryption_done: true,
                transfer_metadata: Some(TransferMetadata {
                    id: String::from(""),
                    source: Some(rdi),
                    files: Some(files.to_owned()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            sender,
            receiver,
            payload,
        }
    }

    fn take_pending_cancel_request(&mut self) -> Result<bool, anyhow::Error> {
        loop {
            match self.receiver.try_recv() {
                Ok(message) => {
                    if is_cancel_request(&message, &self.state.id) {
                        debug!("outbound: received cancellation while sending");
                        return Ok(true);
                    }
                }
                Err(TryRecvError::Empty) => return Ok(false),
                Err(TryRecvError::Lagged(skipped)) => {
                    warn!("outbound: control channel lagged by {skipped} messages");
                }
                Err(TryRecvError::Closed) => {
                    return Err(anyhow!("Outbound control channel is closed"));
                }
            }
        }
    }

    pub async fn handle(&mut self) -> Result<(), anyhow::Error> {
        // Buffer for the 4-byte length
        let mut length_buf = [0u8; 4];

        tokio::select! {
            i = self.receiver.recv() => {
                match i {
                    Ok(channel_msg) => {
                        if channel_msg.direction == ChannelDirection::LibToFront {
                            return Ok(());
                        }

                        if channel_msg.id != self.state.id {
                            return Ok(());
                        }

                        debug!("outbound: got: {:?}", channel_msg);
                        match channel_msg.action {
                            Some(ChannelAction::CancelTransfer) => {
                                self.update_state(
                                    |e| {
                                        e.state = State::Cancelled;
                                    },
                                    true,
                                ).await;
                                self.disconnection().await?;
                                return Err(anyhow!(crate::errors::AppError::NotAnError));
                            },
                            None => {
                                trace!("inbound: nothing to do")
                            },
                            _ => {}
                        }
                    }
                    Err(e) => {
                        error!("inbound: channel error: {}", e);
                    }
                }
            },
            h = stream_read_exact(&mut self.socket, &mut length_buf) => {
                h?;

                self._handle(length_buf).await?
            }
        }

        Ok(())
    }

    pub async fn _handle(&mut self, length_buf: [u8; 4]) -> Result<(), anyhow::Error> {
        let msg_length = u32::from_be_bytes(length_buf) as usize;
        // Ensure the message length is not unreasonably big to avoid allocation attacks
        if msg_length > SANE_FRAME_LENGTH as usize {
            error!("Message length too big");
            return Err(anyhow!("value"));
        }

        // Allocate buffer for the actual message and read it
        let mut frame_data = vec![0u8; msg_length];
        stream_read_exact(&mut self.socket, &mut frame_data).await?;

        let current_state = &self.state;
        // Now determine what will be the request type based on current state
        match current_state.state {
            State::SentUkeyClientInit => {
                debug!("Handling State::SentUkeyClientInit frame");
                let msg = Ukey2Message::decode(&*frame_data)?;
                self.update_state(
                    |e| {
                        e.server_init_data = Some(frame_data);
                    },
                    false,
                )
                .await;
                self.process_ukey2_server_init(&msg).await?;

                // Advance current state
                self.update_state(
                    |e: &mut InnerState| {
                        e.state = State::SentUkeyClientFinish;
                        e.encryption_done = true;
                    },
                    false,
                )
                .await;
            }
            State::SentUkeyClientFinish => {
                debug!("Handling State::SentUkeyClientFinish frame");
                let frame = location_nearby_connections::OfflineFrame::decode(&*frame_data)?;
                self.process_connection_response(&frame).await?;

                // Advance current state
                self.update_state(
                    |e: &mut InnerState| {
                        e.state = State::SentPairedKeyEncryption;
                        e.server_init_data = Some(frame_data);
                        e.encryption_done = true;
                    },
                    false,
                )
                .await;
            }
            _ => {
                debug!("Handling SecureMessage frame");
                let smsg = SecureMessage::decode(&*frame_data)?;
                self.decrypt_and_process_secure_message(&smsg).await?;
            }
        }

        Ok(())
    }

    pub async fn send_connection_request(&mut self) -> Result<(), anyhow::Error> {
        let request = location_nearby_connections::OfflineFrame {
            version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(
                    location_nearby_connections::v1_frame::FrameType::ConnectionRequest.into(),
                ),
                connection_request: Some(location_nearby_connections::ConnectionRequestFrame {
                    endpoint_id: Some(String::from_utf8_lossy(&self.endpoint_id).to_string()),
                    endpoint_name: Some(sys_metrics::host::get_hostname()?.into()),
                    endpoint_info: Some(
                        RemoteDeviceInfo {
                            name: sys_metrics::host::get_hostname()?,
                            device_type: DeviceType::Laptop,
                        }
                        .serialize(),
                    ),
                    mediums: vec![Medium::WifiLan.into()],
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };

        self.send_frame(request.encode_to_vec()).await?;

        Ok(())
    }

    pub async fn send_ukey2_client_init(&mut self) -> Result<(), anyhow::Error> {
        let (secret_key, public_key) = gen_ecdsa_keypair();

        let encoded_point = public_key.to_encoded_point(false);
        let x = encoded_point
            .x()
            .ok_or_else(|| anyhow!("Generated P-256 point has no X coordinate"))?;
        let y = encoded_point
            .y()
            .ok_or_else(|| anyhow!("Generated P-256 point has no Y coordinate"))?;

        let pkey = GenericPublicKey {
            r#type: PublicKeyType::EcP256.into(),
            ec_p256_public_key: Some(EcP256PublicKey {
                x: encode_point(Bytes::from(x.to_vec()))?,
                y: encode_point(Bytes::from(y.to_vec()))?,
            }),
            ..Default::default()
        };

        let finish_frame = Ukey2Message {
            message_type: Some(ukey2_message::Type::ClientFinish.into()),
            message_data: Some(
                Ukey2ClientFinished {
                    public_key: Some(pkey.encode_to_vec()),
                }
                .encode_to_vec(),
            ),
        };

        let sha512 = Sha512::digest(finish_frame.encode_to_vec());
        let frame = Ukey2Message {
            message_type: Some(ukey2_message::Type::ClientInit.into()),
            message_data: Some(
                Ukey2ClientInit {
                    version: Some(1),
                    random: Some(gen_random(32)),
                    next_protocol: Some(String::from("AES_256_CBC-HMAC_SHA256")),
                    cipher_commitments: vec![CipherCommitment {
                        handshake_cipher: Some(Ukey2HandshakeCipher::P256Sha512.into()),
                        commitment: Some(sha512.to_vec()),
                    }],
                }
                .encode_to_vec(),
            ),
        };

        self.send_frame(frame.encode_to_vec()).await?;

        self.update_state(
            |e| {
                e.state = State::SentUkeyClientInit;
                e.private_key = Some(secret_key);
                e.public_key = Some(public_key);
                e.client_init_msg_data = Some(frame.encode_to_vec());
                e.ukey_client_finish_msg_data = Some(finish_frame.encode_to_vec());
            },
            false,
        )
        .await;

        Ok(())
    }

    async fn process_ukey2_server_init(&mut self, msg: &Ukey2Message) -> Result<(), anyhow::Error> {
        if msg.message_type() != ukey2_message::Type::ServerInit {
            self.send_ukey2_alert(AlertType::BadMessageType).await?;
            return Err(anyhow!(
                "UKey2: message_type({:?}) != ServerInit",
                msg.message_type
            ));
        }

        let server_init = match Ukey2ServerInit::decode(msg.message_data()) {
            Ok(uk2si) => uk2si,
            Err(e) => {
                return Err(anyhow!("UKey2: Ukey2ClientFinished::decode: {}", e));
            }
        };

        if server_init.version() != 1 {
            self.send_ukey2_alert(AlertType::BadVersion).await?;
            return Err(anyhow!("UKey2: server_init.version != 1"));
        }

        if server_init.random().len() != 32 {
            self.send_ukey2_alert(AlertType::BadRandom).await?;
            return Err(anyhow!("UKey2: server_init.random.len != 32"));
        }

        if server_init.handshake_cipher() != Ukey2HandshakeCipher::P256Sha512 {
            self.send_ukey2_alert(AlertType::BadHandshakeCipher).await?;
            return Err(anyhow!("UKey2: handshake_cipher != P256Sha512"));
        }

        let server_public_key = match GenericPublicKey::decode(server_init.public_key()) {
            Ok(spk) => spk,
            Err(e) => {
                return Err(anyhow!("UKey2: GenericPublicKey::decode: {}", e));
            }
        };

        self.finalize_key_exchange(server_public_key).await?;
        let finish_msg = self
            .state
            .ukey_client_finish_msg_data
            .clone()
            .ok_or_else(|| anyhow!("Missing UKEY2 client finish data"))?;
        self.send_frame(finish_msg).await?;

        let frame = location_nearby_connections::OfflineFrame {
            version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(
                    location_nearby_connections::v1_frame::FrameType::ConnectionResponse.into(),
                ),
                connection_response: Some(location_nearby_connections::ConnectionResponseFrame {
					response: Some(location_nearby_connections::connection_response_frame::ResponseStatus::Accept.into()),
					os_info: Some(location_nearby_connections::OsInfo {
						r#type: Some(location_nearby_connections::os_info::OsType::Linux.into())
					}),
					..Default::default()
				}),
                ..Default::default()
            }),
        };

        self.send_frame(frame.encode_to_vec()).await?;

        Ok(())
    }

    async fn process_connection_response(
        &mut self,
        frame: &location_nearby_connections::OfflineFrame,
    ) -> Result<(), anyhow::Error> {
        let v1_frame = frame
            .v1
            .as_ref()
            .ok_or_else(|| anyhow!("Missing required fields"))?;

        if v1_frame.r#type() != location_nearby_connections::v1_frame::FrameType::ConnectionResponse
        {
            return Err(anyhow!(format!(
                "Unexpected frame type: {:?}",
                v1_frame.r#type()
            )));
        }

        let connection_response = v1_frame
            .connection_response
            .as_ref()
            .ok_or_else(|| anyhow!("Missing connection response"))?;

        if connection_response.response() != ResponseStatus::Accept {
            return Err(anyhow!("Connection rejected by third party"));
        }

        let paired_encryption = sharing_nearby::Frame {
            version: Some(sharing_nearby::frame::Version::V1.into()),
            v1: Some(sharing_nearby::V1Frame {
                r#type: Some(sharing_nearby::v1_frame::FrameType::PairedKeyEncryption.into()),
                paired_key_encryption: Some(sharing_nearby::PairedKeyEncryptionFrame {
                    secret_id_hash: Some(gen_random(6)),
                    signed_data: Some(gen_random(72)),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };

        self.send_encrypted_frame(&paired_encryption).await?;

        Ok(())
    }

    async fn decrypt_and_process_secure_message(
        &mut self,
        smsg: &SecureMessage,
    ) -> Result<(), anyhow::Error> {
        let recv_hmac_key = self
            .state
            .recv_hmac_key
            .as_ref()
            .ok_or_else(|| anyhow!("Missing receive HMAC key"))?;
        let mut hmac = HmacSha256::new_from_slice(recv_hmac_key)?;
        hmac.update(&smsg.header_and_body);
        if !hmac
            .finalize()
            .into_bytes()
            .as_slice()
            .eq(smsg.signature.as_slice())
        {
            return Err(anyhow!("hmac!=signature"));
        }

        let header_and_body = HeaderAndBody::decode(&*smsg.header_and_body)?;

        let msg_data = header_and_body.body;
        let key = self
            .state
            .decrypt_key
            .as_ref()
            .ok_or_else(|| anyhow!("Missing decrypt key"))?;

        let key_bytes: &[u8; AES_256_KEY_LEN] = key
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("Invalid decrypt key length: {}", key.len()))?;
        let mut cipher = Cipher::new_256(key_bytes);
        cipher.set_auto_padding(true);
        let decrypted = cipher.cbc_decrypt(header_and_body.header.iv(), &msg_data);

        let d2d_msg = DeviceToDeviceMessage::decode(&*decrypted)?;

        let seq = self.get_client_seq_inc().await;
        if d2d_msg.sequence_number() != seq {
            return Err(anyhow!(
                "Error d2d_msg.sequence_number invalid ({} vs {})",
                d2d_msg.sequence_number(),
                seq
            ));
        }

        let offline = location_nearby_connections::OfflineFrame::decode(d2d_msg.message())?;
        let v1_frame = offline
            .v1
            .as_ref()
            .ok_or_else(|| anyhow!("Missing required fields"))?;
        match v1_frame.r#type() {
            location_nearby_connections::v1_frame::FrameType::PayloadTransfer => {
                trace!("Received FrameType::PayloadTransfer");
                let payload_transfer = v1_frame
                    .payload_transfer
                    .as_ref()
                    .ok_or_else(|| anyhow!("Missing required fields"))?;

                let header = payload_transfer
                    .payload_header
                    .as_ref()
                    .ok_or_else(|| anyhow!("Missing required fields"))?;
                let chunk = payload_transfer
                    .payload_chunk
                    .as_ref()
                    .ok_or_else(|| anyhow!("Missing required fields"))?;

                match header.r#type() {
                    payload_header::PayloadType::Bytes => {
                        info!("Processing PayloadType::Bytes");
                        let payload_id = header.id();

                        let declared_size = checked_payload_buffer_size(header.total_size())?;

                        self.state
                            .payload_buffers
                            .entry(payload_id)
                            .or_insert_with(|| Vec::with_capacity(declared_size));

                        // Get the current length of the buffer, if it exists, without holding a mutable borrow.
                        let buffer_len = self
                            .state
                            .payload_buffers
                            .get(&payload_id)
                            .ok_or_else(|| anyhow!("Missing payload buffer for {payload_id}"))?
                            .len();
                        if chunk.offset() != buffer_len as i64 {
                            self.state.payload_buffers.remove(&payload_id);
                            return Err(anyhow!(
                                "Unexpected chunk offset: {}, expected: {}",
                                chunk.offset(),
                                buffer_len
                            ));
                        }

                        let body_len = chunk.body().len();
                        let new_len = buffer_len
                            .checked_add(body_len)
                            .ok_or_else(|| anyhow!("Byte payload size overflow"))?;
                        if new_len > declared_size {
                            self.state.payload_buffers.remove(&payload_id);
                            return Err(anyhow!(
                                "Byte payload exceeds declared size: {new_len} > {declared_size}"
                            ));
                        }

                        let buffer = self
                            .state
                            .payload_buffers
                            .get_mut(&payload_id)
                            .ok_or_else(|| anyhow!("Missing payload buffer for {payload_id}"))?;
                        if let Some(body) = &chunk.body {
                            buffer.extend(body);
                        }

                        if (chunk.flags() & 1) == 1 {
                            debug!("Chunk flags & 1 == 1 ?? End of data ??");

                            let innner_frame = sharing_nearby::Frame::decode(buffer.as_slice())?;
                            self.process_transfer_setup(&innner_frame).await?;
                        }
                    }
                    payload_header::PayloadType::File => {
                        error!("Unhandled PayloadType::File: {:?}", header.r#type())
                    }
                    payload_header::PayloadType::Stream => {
                        error!("Unhandled PayloadType::Stream: {:?}", header.r#type())
                    }
                    payload_header::PayloadType::UnknownPayloadType => {
                        error!(
                            "Invalid PayloadType::UnknownPayloadType: {:?}",
                            header.r#type()
                        )
                    }
                }
            }
            location_nearby_connections::v1_frame::FrameType::KeepAlive => {
                trace!("Sending keepalive");
                self.send_keepalive(true).await?;
            }
            _ => {
                error!("Unhandled offline frame encrypted: {:?}", offline);
            }
        }

        Ok(())
    }

    async fn process_transfer_setup(
        &mut self,
        frame: &sharing_nearby::Frame,
    ) -> Result<(), anyhow::Error> {
        let v1_frame = frame
            .v1
            .as_ref()
            .ok_or_else(|| anyhow!("Missing required fields"))?;

        if v1_frame.r#type() == sharing_nearby::v1_frame::FrameType::Cancel {
            info!("Transfer canceled");
            self.update_state(
                |e| {
                    e.state = State::Cancelled;
                },
                true,
            )
            .await;
            self.disconnection().await?;
            return Err(anyhow!(crate::errors::AppError::NotAnError));
        }

        match self.state.state {
            State::SentPairedKeyEncryption => {
                debug!("Processing State::SentPairedKeyEncryption");
                self.process_paired_key_encryption_frame(v1_frame).await?;
                self.update_state(
                    |e| {
                        e.state = State::SentPairedKeyResult;
                    },
                    false,
                )
                .await;
            }
            State::SentPairedKeyResult => {
                debug!("Processing State::SentPairedKeyResult");
                self.process_paired_key_result(v1_frame).await?;
                self.update_state(
                    |e| {
                        e.state = State::SentIntroduction;
                    },
                    true,
                )
                .await;
            }
            State::SentIntroduction => {
                debug!("Processing State::SentIntroduction");
                self.process_consent(v1_frame).await?;
            }
            State::SendingFiles => {}
            _ => {
                info!(
                    "Unhandled connection state in process_transfer_setup: {:?}",
                    self.state.state
                );
            }
        }

        Ok(())
    }

    async fn process_paired_key_encryption_frame(
        &mut self,
        v1_frame: &sharing_nearby::V1Frame,
    ) -> Result<(), anyhow::Error> {
        if v1_frame.paired_key_encryption.is_none() {
            return Err(anyhow!("Missing required fields"));
        }

        let paired_result = sharing_nearby::Frame {
            version: Some(sharing_nearby::frame::Version::V1.into()),
            v1: Some(sharing_nearby::V1Frame {
                r#type: Some(sharing_nearby::v1_frame::FrameType::PairedKeyResult.into()),
                paired_key_result: Some(sharing_nearby::PairedKeyResultFrame {
                    status: Some(paired_key_result_frame::Status::Unable.into()),
                }),
                ..Default::default()
            }),
        };

        self.send_encrypted_frame(&paired_result).await?;

        Ok(())
    }

    async fn process_paired_key_result(
        &mut self,
        v1_frame: &sharing_nearby::V1Frame,
    ) -> Result<(), anyhow::Error> {
        if v1_frame.paired_key_result.is_none() {
            return Err(anyhow!("Missing required fields"));
        }

        // TODO - Handle sending Text
        let PreparedOutboundFiles {
            metadata: file_metadata,
            files: transferred_files,
            total_bytes: total_to_send,
        } = match &self.payload {
            OutboundPayload::Files(files) => prepare_outbound_files(files)?,
        };

        self.update_state(
            |e| {
                if let Some(tmd) = e.transfer_metadata.as_mut() {
                    tmd.total_bytes = total_to_send;
                }
                e.transferred_files = transferred_files;
            },
            false,
        )
        .await;

        let introduction = sharing_nearby::Frame {
            version: Some(sharing_nearby::frame::Version::V1.into()),
            v1: Some(sharing_nearby::V1Frame {
                r#type: Some(sharing_nearby::v1_frame::FrameType::Introduction.into()),
                introduction: Some(IntroductionFrame {
                    file_metadata,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };

        self.send_encrypted_frame(&introduction).await?;

        Ok(())
    }

    async fn process_consent(
        &mut self,
        v1_frame: &sharing_nearby::V1Frame,
    ) -> Result<(), anyhow::Error> {
        if v1_frame.r#type() != sharing_nearby::v1_frame::FrameType::Response {
            return Err(anyhow!("Expected consent response frame"));
        }

        let connection_response = v1_frame
            .connection_response
            .as_ref()
            .ok_or_else(|| anyhow!("Missing consent response"))?;
        let consent_status = connection_response.status();

        match consent_status {
            sharing_nearby::connection_response_frame::Status::Accept => {
                info!("State is now State::SendingFiles");
                self.update_state(
                    |e| {
                        e.state = State::SendingFiles;
                    },
                    true,
                )
                .await;

                // TODO - Handle sending Text
                let ids: Vec<i64> = self.state.transferred_files.keys().cloned().collect();
                info!("We are sending: {:?}", ids);
                let mut ids_iter = ids.into_iter();
                // Loop through all files
                loop {
                    let current = match ids_iter.next() {
                        Some(i) => i,
                        None => {
                            info!("All files have been transferred");
                            self.update_state(
                                |e| {
                                    e.state = State::Finished;
                                },
                                true,
                            )
                            .await;
                            self.disconnection().await?;
                            // Breaking instead of NotAnError to allow peacefull termination
                            break;
                        }
                    };

                    // Loop until we reached end of file
                    loop {
                        if self.take_pending_cancel_request()? {
                            self.update_state(
                                |e| {
                                    e.state = State::Cancelled;
                                },
                                true,
                            )
                            .await;
                            self.disconnection().await?;
                            return Err(anyhow!(crate::errors::AppError::NotAnError));
                        }

                        // Workaround to limit scope of the immutable borrow on self
                        let (curr_state, buffer, bytes_read) = {
                            let curr_state = match self.state.transferred_files.get(&current) {
                                Some(s) => s,
                                None => break,
                            };

                            info!("> Currently sending {:?}", curr_state.file_url);
                            if curr_state.bytes_transferred == curr_state.total_size {
                                debug!("File {current} finished");
                                self.update_state(
                                    |e| {
                                        e.transferred_files.remove(&current);
                                    },
                                    false,
                                )
                                .await;
                                break;
                            }

                            if curr_state.file.is_none() {
                                warn!("File {current} is none");
                                break;
                            }

                            let mut buffer = vec![0u8; 512 * 1024];
                            let mut file = curr_state
                                .file
                                .as_ref()
                                .ok_or_else(|| anyhow!("Outbound file handle is missing"))?;
                            let bytes_read = file.read(&mut buffer)?;

                            (
                                InternalFileInfo {
                                    payload_id: curr_state.payload_id,
                                    file_url: curr_state.file_url.clone(),
                                    bytes_transferred: curr_state.bytes_transferred,
                                    total_size: curr_state.total_size,
                                    file: None,
                                },
                                buffer,
                                bytes_read,
                            )
                        };

                        let sending_buffer = buffer[..bytes_read].to_vec();
                        info!(
                            "> File ready: {bytes_read} bytes && {} && left to send: {} with current offset: {}",
                            sending_buffer.len(),
                            curr_state.total_size - curr_state.bytes_transferred,
							curr_state.bytes_transferred
                        );

                        let payload_header = PayloadHeader {
                            id: Some(current),
                            r#type: Some(payload_header::PayloadType::File.into()),
                            total_size: Some(curr_state.total_size),
                            is_sensitive: Some(false),
                            file_name: curr_state
                                .file_url
                                .file_name()
                                .map(|name| name.to_string_lossy().into_owned()),
                            ..Default::default()
                        };

                        let wrapper = location_nearby_connections::OfflineFrame {
							version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
							v1: Some(location_nearby_connections::V1Frame {
								r#type: Some(
									location_nearby_connections::v1_frame::FrameType::PayloadTransfer.into(),
								),
								payload_transfer: Some(PayloadTransferFrame {
									packet_type: Some(PacketType::Data.into()),
									payload_chunk: Some(PayloadChunk {
										offset: Some(curr_state.bytes_transferred),
										flags: Some(0),
										body: Some(buffer[..bytes_read].to_vec()),
									}),
									payload_header: Some(payload_header.clone()),
									..Default::default()
								}),
								..Default::default()
							}),
						};

                        self.encrypt_and_send(&wrapper).await?;
                        self.update_state(
                            |e| {
                                if let Some(mu) = e.transferred_files.get_mut(&current) {
                                    mu.bytes_transferred += bytes_read as i64;
                                }

                                if let Some(tmd) = e.transfer_metadata.as_mut() {
                                    tmd.ack_bytes += bytes_read as u64;
                                }
                            },
                            true,
                        )
                        .await;

                        // If we just sent the last bytes of the file, mark it as finished
                        if curr_state.bytes_transferred + bytes_read as i64 == curr_state.total_size
                        {
                            debug!(
                                "File {current} finished, curr offset: {} over total: {}",
                                curr_state.bytes_transferred + bytes_read as i64,
                                curr_state.total_size
                            );

                            let wrapper = location_nearby_connections::OfflineFrame {
								version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
								v1: Some(location_nearby_connections::V1Frame {
									r#type: Some(
										location_nearby_connections::v1_frame::FrameType::PayloadTransfer.into(),
									),
									payload_transfer: Some(PayloadTransferFrame {
										packet_type: Some(PacketType::Data.into()),
										payload_chunk: Some(PayloadChunk {
											offset: Some(curr_state.total_size),
											flags: Some(1), // lastChunk
											body: Some(vec![]),
										}),
										payload_header: Some(payload_header),
										..Default::default()
									}),
									..Default::default()
								}),
							};

                            self.encrypt_and_send(&wrapper).await?;
                            break;
                        }
                    }
                }
            }
            sharing_nearby::connection_response_frame::Status::Reject
            | sharing_nearby::connection_response_frame::Status::NotEnoughSpace
            | sharing_nearby::connection_response_frame::Status::UnsupportedAttachmentType
            | sharing_nearby::connection_response_frame::Status::TimedOut => {
                warn!("Cannot process: consent denied: {:?}", consent_status);
                self.update_state(
                    |e| {
                        e.state = State::Disconnected;
                    },
                    true,
                )
                .await;
                self.disconnection().await?;
                return Err(anyhow!(crate::errors::AppError::NotAnError));
            }
            sharing_nearby::connection_response_frame::Status::Unknown => {
                error!("Unknown consent type: aborting");
                self.update_state(
                    |e| {
                        e.state = State::Disconnected;
                    },
                    true,
                )
                .await;
                self.disconnection().await?;
                return Err(anyhow!(crate::errors::AppError::NotAnError));
            }
        }

        Ok(())
    }

    async fn disconnection(&mut self) -> Result<(), anyhow::Error> {
        let frame = location_nearby_connections::OfflineFrame {
            version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(
                    location_nearby_connections::v1_frame::FrameType::Disconnection.into(),
                ),
                disconnection: Some(location_nearby_connections::DisconnectionFrame {
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };

        if self.state.encryption_done {
            self.encrypt_and_send(&frame).await
        } else {
            self.send_frame(frame.encode_to_vec()).await
        }
    }

    async fn finalize_key_exchange(
        &mut self,
        raw_peer_key: GenericPublicKey,
    ) -> Result<(), anyhow::Error> {
        let peer_p256_key = raw_peer_key
            .ec_p256_public_key
            .ok_or_else(|| anyhow!("Missing required fields"))?;

        let x = normalize_p256_coordinate(&peer_p256_key.x)?;
        let y = normalize_p256_coordinate(&peer_p256_key.y)?;

        let mut bytes = Vec::with_capacity(65);
        bytes.push(0x04);
        bytes.extend_from_slice(&x);
        bytes.extend_from_slice(&y);

        let encoded_point = EncodedPoint::from_bytes(bytes)?;
        let peer_key = Option::<PublicKey>::from(PublicKey::from_encoded_point(&encoded_point))
            .ok_or_else(|| anyhow!("Invalid peer P-256 public key"))?;
        let priv_key = self
            .state
            .private_key
            .as_ref()
            .ok_or_else(|| anyhow!("Missing local private key"))?;

        let dhs = diffie_hellman(priv_key.to_nonzero_scalar(), peer_key.as_affine());
        let derived_secret = Sha256::digest(dhs.raw_secret_bytes());

        let client_init_msg_data = self
            .state
            .client_init_msg_data
            .as_ref()
            .ok_or_else(|| anyhow!("Missing UKEY2 client init data"))?;
        let server_init_data = self
            .state
            .server_init_data
            .as_ref()
            .ok_or_else(|| anyhow!("Missing UKEY2 server init data"))?;

        let mut ukey_info: Vec<u8> = vec![];
        ukey_info.extend_from_slice(client_init_msg_data);
        ukey_info.extend_from_slice(server_init_data);

        let auth_label = "UKEY2 v1 auth".as_bytes();
        let next_label = "UKEY2 v1 next".as_bytes();

        let auth_string = hkdf_extract_expand(auth_label, &derived_secret, &ukey_info, 32)?;
        let next_secret = hkdf_extract_expand(next_label, &derived_secret, &ukey_info, 32)?;

        let salt_hex = "82AA55A0D397F88346CA1CEE8D3909B95F13FA7DEB1D4AB38376B8256DA85510";
        let salt =
            hex::decode(salt_hex).map_err(|e| anyhow!("Failed to decode salt_hex: {}", e))?;

        let d2d_client = hkdf_extract_expand(&salt, &next_secret, "client".as_bytes(), 32)?;
        let d2d_server = hkdf_extract_expand(&salt, &next_secret, "server".as_bytes(), 32)?;

        let key_salt_hex = "BF9D2A53C63616D75DB0A7165B91C1EF73E537F2427405FA23610A4BE657642E";
        let key_salt = hex::decode(key_salt_hex)
            .map_err(|e| anyhow!("Failed to decode key_salt_hex: {}", e))?;

        let client_key = hkdf_extract_expand(&key_salt, &d2d_client, "ENC:2".as_bytes(), 32)?;
        let client_hmac_key = hkdf_extract_expand(&key_salt, &d2d_client, "SIG:1".as_bytes(), 32)?;
        let server_key = hkdf_extract_expand(&key_salt, &d2d_server, "ENC:2".as_bytes(), 32)?;
        let server_hmac_key = hkdf_extract_expand(&key_salt, &d2d_server, "SIG:1".as_bytes(), 32)?;

        self.update_state(
            |e| {
                e.decrypt_key = Some(server_key);
                e.recv_hmac_key = Some(server_hmac_key);
                e.encrypt_key = Some(client_key);
                e.send_hmac_key = Some(client_hmac_key);
                e.pin_code = Some(to_four_digit_string(&auth_string));
                e.encryption_done = true;

                if let Some(ref mut tm) = e.transfer_metadata {
                    tm.pin_code = Some(to_four_digit_string(&auth_string));
                }
            },
            true,
        )
        .await;

        info!("Pin code: {:?}", self.state.pin_code);

        Ok(())
    }

    async fn send_ukey2_alert(&mut self, atype: AlertType) -> Result<(), anyhow::Error> {
        let alert = Ukey2Alert {
            r#type: Some(atype.into()),
            error_message: None,
        };

        let data = Ukey2Message {
            message_type: Some(atype.into()),
            message_data: Some(alert.encode_to_vec()),
        };

        self.send_frame(data.encode_to_vec()).await
    }

    async fn send_encrypted_frame(
        &mut self,
        frame: &sharing_nearby::Frame,
    ) -> Result<(), anyhow::Error> {
        let frame_data = frame.encode_to_vec();
        let body_size = frame_data.len();

        let payload_header = PayloadHeader {
            id: Some(rand::rng().random_range(i64::MIN..i64::MAX)),
            r#type: Some(payload_header::PayloadType::Bytes.into()),
            total_size: Some(body_size as i64),
            is_sensitive: Some(false),
            ..Default::default()
        };

        let transfer = PayloadTransferFrame {
            packet_type: Some(PacketType::Data.into()),
            payload_chunk: Some(PayloadChunk {
                offset: Some(0),
                flags: Some(0),
                body: Some(frame_data),
            }),
            payload_header: Some(payload_header.clone()),
            ..Default::default()
        };

        let wrapper = location_nearby_connections::OfflineFrame {
            version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(
                    location_nearby_connections::v1_frame::FrameType::PayloadTransfer.into(),
                ),
                payload_transfer: Some(transfer),
                ..Default::default()
            }),
        };

        // Encrypt and send offline
        self.encrypt_and_send(&wrapper).await?;

        // Send lastChunk
        let transfer = PayloadTransferFrame {
            packet_type: Some(PacketType::Data.into()),
            payload_chunk: Some(PayloadChunk {
                offset: Some(body_size as i64),
                flags: Some(1), // lastChunk
                body: Some(vec![]),
            }),
            payload_header: Some(payload_header),
            ..Default::default()
        };

        let wrapper = location_nearby_connections::OfflineFrame {
            version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(
                    location_nearby_connections::v1_frame::FrameType::PayloadTransfer.into(),
                ),
                payload_transfer: Some(transfer),
                ..Default::default()
            }),
        };

        // Encrypt and send offline
        self.encrypt_and_send(&wrapper).await?;

        Ok(())
    }

    async fn encrypt_and_send(&mut self, frame: &OfflineFrame) -> Result<(), anyhow::Error> {
        let d2d_msg = DeviceToDeviceMessage {
            sequence_number: Some(self.get_server_seq_inc().await),
            message: Some(frame.encode_to_vec()),
        };

        let key = self
            .state
            .encrypt_key
            .as_ref()
            .ok_or_else(|| anyhow!("Missing encrypt key"))?;
        let msg_data = d2d_msg.encode_to_vec();
        let iv = gen_random(16);

        let key_bytes: &[u8; AES_256_KEY_LEN] = key
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("Invalid encrypt key length: {}", key.len()))?;
        let mut cipher = Cipher::new_256(key_bytes);
        cipher.set_auto_padding(true);
        let encrypted = cipher.cbc_encrypt(&iv, &msg_data);

        let hb = HeaderAndBody {
            body: encrypted,
            header: Header {
                encryption_scheme: EncScheme::Aes256Cbc.into(),
                signature_scheme: SigScheme::HmacSha256.into(),
                iv: Some(iv),
                public_metadata: Some(
                    GcmMetadata {
                        r#type: Type::DeviceToDeviceMessage.into(),
                        version: Some(1),
                    }
                    .encode_to_vec(),
                ),
                ..Default::default()
            },
        };

        let send_hmac_key = self
            .state
            .send_hmac_key
            .as_ref()
            .ok_or_else(|| anyhow!("Missing send HMAC key"))?;
        let mut hmac = HmacSha256::new_from_slice(send_hmac_key)?;
        hmac.update(&hb.encode_to_vec());
        let result = hmac.finalize();

        let smsg = SecureMessage {
            header_and_body: hb.encode_to_vec(),
            signature: result.into_bytes().to_vec(),
        };

        self.send_frame(smsg.encode_to_vec()).await?;

        Ok(())
    }

    async fn send_keepalive(&mut self, ack: bool) -> Result<(), anyhow::Error> {
        let ack_frame = location_nearby_connections::OfflineFrame {
            version: Some(location_nearby_connections::offline_frame::Version::V1.into()),
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(location_nearby_connections::v1_frame::FrameType::KeepAlive.into()),
                keep_alive: Some(KeepAliveFrame { ack: Some(ack) }),
                ..Default::default()
            }),
        };

        if self.state.encryption_done {
            self.encrypt_and_send(&ack_frame).await
        } else {
            self.send_frame(ack_frame.encode_to_vec()).await
        }
    }

    async fn send_frame(&mut self, data: Vec<u8>) -> Result<(), anyhow::Error> {
        let length = data.len();

        // Prepare length prefix in big-endian format
        let length_bytes = [
            (length >> 24) as u8,
            (length >> 16) as u8,
            (length >> 8) as u8,
            length as u8,
        ];

        let mut prefixed_length = Vec::with_capacity(length + 4);
        prefixed_length.extend_from_slice(&length_bytes);
        prefixed_length.extend_from_slice(&data);

        self.socket.write_all(&prefixed_length).await?;
        self.socket.flush().await?;

        Ok(())
    }

    async fn get_server_seq_inc(&mut self) -> i32 {
        self.update_state(
            |e| {
                e.server_seq += 1;
            },
            false,
        )
        .await;

        self.state.server_seq
    }

    async fn get_client_seq_inc(&mut self) -> i32 {
        self.update_state(
            |e| {
                e.client_seq += 1;
            },
            false,
        )
        .await;

        self.state.client_seq
    }

    async fn update_state<F>(&mut self, f: F, inform: bool)
    where
        F: FnOnce(&mut InnerState),
    {
        f(&mut self.state);

        if !inform {
            return;
        }

        let _ = self.sender.send(ChannelMessage {
            id: self.state.id.clone(),
            direction: ChannelDirection::LibToFront,
            rtype: Some(crate::channel::TransferType::Outbound),
            state: Some(self.state.state.clone()),
            meta: self.state.transfer_metadata.clone(),
            ..Default::default()
        });
        // Add a small sleep timer to allow the Tokio runtime to have
        // some spare time to process channel's message. Otherwise it
        // get spammed by new requests. Currently set to 10 micro secs.
        tokio::time::sleep(SANITY_DURATION).await;
    }
}

#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn outbound_file_preparation_classifies_files_and_skips_missing_paths() {
        let root = std::env::temp_dir().join(format!(
            "rquickshare-outbound-test-{}",
            rand::random::<u64>()
        ));
        std::fs::create_dir_all(&root).unwrap();

        let image = root.join("photo.png");
        let app = root.join("client.apk");
        let text = root.join("notes.txt");
        let missing = root.join("missing.bin");

        std::fs::write(&image, [1_u8, 2, 3]).unwrap();
        std::fs::write(&app, [4_u8, 5]).unwrap();
        std::fs::write(&text, [6_u8]).unwrap();

        let inputs = vec![
            image.to_string_lossy().into_owned(),
            app.to_string_lossy().into_owned(),
            text.to_string_lossy().into_owned(),
            missing.to_string_lossy().into_owned(),
        ];

        let prepared = prepare_outbound_files(&inputs).unwrap();
        let metadata = prepared.metadata;
        let transferred = prepared.files;

        assert_eq!(metadata.len(), 3);
        assert_eq!(transferred.len(), 3);
        assert_eq!(prepared.total_bytes, 6);

        let by_name = metadata
            .iter()
            .map(|item| (item.name().to_owned(), item))
            .collect::<HashMap<_, _>>();

        assert_eq!(by_name["photo.png"].r#type(), file_metadata::Type::Image);
        assert_eq!(by_name["client.apk"].r#type(), file_metadata::Type::App);
        assert_eq!(by_name["notes.txt"].r#type(), file_metadata::Type::Unknown);

        let payload_ids = metadata
            .iter()
            .map(FileMetadata::payload_id)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(payload_ids.len(), 3);

        for item in metadata {
            let internal = &transferred[&item.payload_id()];
            assert_eq!(internal.total_size, item.size());
            assert!(internal.file.is_some());
            assert_eq!(
                internal.file_url.file_name().unwrap().to_string_lossy(),
                item.name()
            );
        }

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn outbound_file_preparation_handles_empty_input() {
        let prepared = prepare_outbound_files(&[]).unwrap();

        assert!(prepared.metadata.is_empty());
        assert!(prepared.files.is_empty());
        assert_eq!(prepared.total_bytes, 0);
    }

    async fn test_request(files: Vec<String>) -> (OutboundRequest, tokio::net::TcpStream) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let peer = tokio::net::TcpStream::connect(address).await.unwrap();
        let (socket, _) = listener.accept().await.unwrap();
        let (sender, _receiver) = tokio::sync::broadcast::channel(16);

        let mut request = OutboundRequest::new(
            [1, 2, 3, 4],
            socket,
            "test-outbound".to_owned(),
            sender,
            OutboundPayload::Files(files),
            RemoteDeviceInfo {
                name: "Test phone".to_owned(),
                device_type: DeviceType::Phone,
            },
        );
        request.state.encryption_done = false;
        (request, peer)
    }

    fn consent_frame(
        status: sharing_nearby::connection_response_frame::Status,
    ) -> sharing_nearby::V1Frame {
        sharing_nearby::V1Frame {
            r#type: Some(sharing_nearby::v1_frame::FrameType::Response.into()),
            connection_response: Some(sharing_nearby::ConnectionResponseFrame {
                status: Some(status.into()),
            }),
            ..Default::default()
        }
    }

    async fn read_offline_frame(
        socket: &mut tokio::net::TcpStream,
    ) -> location_nearby_connections::OfflineFrame {
        use tokio::io::AsyncReadExt;

        let mut length = [0_u8; 4];
        socket.read_exact(&mut length).await.unwrap();
        let length = u32::from_be_bytes(length) as usize;
        let mut payload = vec![0_u8; length];
        socket.read_exact(&mut payload).await.unwrap();
        location_nearby_connections::OfflineFrame::decode(payload.as_slice()).unwrap()
    }

    #[tokio::test]
    async fn accepted_empty_transfer_finishes_and_disconnects() {
        let (mut request, mut peer) = test_request(vec![]).await;
        let frame = consent_frame(sharing_nearby::connection_response_frame::Status::Accept);

        request.process_consent(&frame).await.unwrap();

        assert_eq!(request.state.state, State::Finished);
        let disconnect = read_offline_frame(&mut peer).await;
        assert_eq!(
            disconnect.v1.unwrap().r#type(),
            location_nearby_connections::v1_frame::FrameType::Disconnection
        );
    }

    #[tokio::test]
    async fn denied_consent_disconnects_cleanly() {
        for status in [
            sharing_nearby::connection_response_frame::Status::Reject,
            sharing_nearby::connection_response_frame::Status::NotEnoughSpace,
            sharing_nearby::connection_response_frame::Status::UnsupportedAttachmentType,
            sharing_nearby::connection_response_frame::Status::TimedOut,
        ] {
            let (mut request, mut peer) = test_request(vec![]).await;
            let result = request.process_consent(&consent_frame(status)).await;

            assert!(result.is_err());
            assert_eq!(request.state.state, State::Disconnected);

            let disconnect = read_offline_frame(&mut peer).await;
            assert_eq!(
                disconnect.v1.unwrap().r#type(),
                location_nearby_connections::v1_frame::FrameType::Disconnection
            );
        }
    }

    #[tokio::test]
    async fn malformed_consent_response_is_rejected() {
        let (mut request, _peer) = test_request(vec![]).await;

        let wrong_type = sharing_nearby::V1Frame {
            r#type: Some(sharing_nearby::v1_frame::FrameType::Introduction.into()),
            ..Default::default()
        };
        assert!(request.process_consent(&wrong_type).await.is_err());
        assert_eq!(request.state.state, State::Initial);

        let missing_response = sharing_nearby::V1Frame {
            r#type: Some(sharing_nearby::v1_frame::FrameType::Response.into()),
            ..Default::default()
        };
        assert!(request.process_consent(&missing_response).await.is_err());
        assert_eq!(request.state.state, State::Initial);
    }

    #[test]
    fn test_is_cancel_request_matches_only_current_frontend_transfer() {
        let cancel = ChannelMessage {
            id: "transfer-1".to_owned(),
            direction: ChannelDirection::FrontToLib,
            action: Some(ChannelAction::CancelTransfer),
            ..Default::default()
        };
        assert!(is_cancel_request(&cancel, "transfer-1"));
        assert!(!is_cancel_request(&cancel, "transfer-2"));

        let lib_message = ChannelMessage {
            direction: ChannelDirection::LibToFront,
            ..cancel.clone()
        };
        assert!(!is_cancel_request(&lib_message, "transfer-1"));

        let accept = ChannelMessage {
            action: Some(ChannelAction::AcceptTransfer),
            ..cancel
        };
        assert!(!is_cancel_request(&accept, "transfer-1"));
    }
}

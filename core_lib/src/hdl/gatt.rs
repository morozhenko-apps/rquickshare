use std::sync::Arc;

use anyhow::anyhow;
use bluer::gatt::local::{
    Application, Characteristic, CharacteristicNotifier, CharacteristicNotify,
    CharacteristicNotifyMethod, CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod,
    Service,
};
use bluer::{Adapter, Uuid, UuidExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::channel::ChannelMessage;
use crate::errors::AppError;
use crate::hdl::{InboundRequest, MigratableStream};

const INNER_NAME: &str = "ReceiverGattServer";

const QS_GATT_SERVICE: u16 = 0xFEF3;
const QS_ADV_SLOT0_UUID: &str = "00000000-0000-3000-8000-000000000000";
const QS_WEAVE_TO_PERIPHERAL: &str = "00000100-0004-1000-8000-001a11000101";
const QS_WEAVE_FROM_PERIPHERAL: &str = "00000100-0004-1000-8000-001a11000102";

const WEAVE_CONTROL: u8 = 0b1000_0000;
const WEAVE_CMD_MASK: u8 = 0b0000_1111;
const WEAVE_FIRST_BIT: u8 = 0b0000_1000;
const WEAVE_LAST_BIT: u8 = 0b0000_0100;
const WEAVE_CMD_CONN_REQUEST: u8 = 0;
const WEAVE_CMD_CONN_CONFIRM: u8 = 1;
const WEAVE_CMD_ERROR: u8 = 2;
const WEAVE_PROTOCOL_VERSION: u16 = 1;

const QS_SVC_HASH: [u8; 3] = [0xfc, 0x9f, 0x5e];
const SOCKET_CTRL_INTRODUCTION: u8 = 1;
const SOCKET_CTRL_DISCONNECTION: u8 = 2;

const MIN_WEAVE_PACKET_SIZE: u16 = 20;
const MAX_WEAVE_PACKET_SIZE: u16 = 509;
const MAX_INBOUND_FRAME_SIZE: usize = 5 * 1024 * 1024;
const MAX_WEAVE_MESSAGE_SIZE: usize = MAX_INBOUND_FRAME_SIZE + 7;

fn parse_connection_request(packet: &[u8]) -> Result<u16, anyhow::Error> {
    if packet.len() < 7 {
        return Err(anyhow!(
            "weave connection request is too short: {} bytes",
            packet.len()
        ));
    }

    let header = packet[0];
    if header & WEAVE_CONTROL == 0 || header & WEAVE_CMD_MASK != WEAVE_CMD_CONN_REQUEST {
        return Err(anyhow!("not a weave connection request"));
    }

    let min_version = u16::from_be_bytes([packet[1], packet[2]]);
    let max_version = u16::from_be_bytes([packet[3], packet[4]]);
    if !(min_version..=max_version).contains(&WEAVE_PROTOCOL_VERSION) {
        return Err(anyhow!(
            "unsupported weave protocol range {min_version}..={max_version}"
        ));
    }

    let requested = u16::from_be_bytes([packet[5], packet[6]]);
    Ok(requested.clamp(MIN_WEAVE_PACKET_SIZE, MAX_WEAVE_PACKET_SIZE))
}

fn connection_confirm(selected_packet_size: u16) -> [u8; 5] {
    [
        WEAVE_CONTROL | WEAVE_CMD_CONN_CONFIRM,
        (WEAVE_PROTOCOL_VERSION >> 8) as u8,
        (WEAVE_PROTOCOL_VERSION & 0xff) as u8,
        (selected_packet_size >> 8) as u8,
        (selected_packet_size & 0xff) as u8,
    ]
}

fn complete_framed_message_len(buffer: &[u8]) -> Result<Option<usize>, anyhow::Error> {
    if buffer.len() < 4 {
        return Ok(None);
    }

    let frame_len = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
    if frame_len == 0 || frame_len > MAX_INBOUND_FRAME_SIZE {
        return Err(anyhow!("invalid framed message length: {frame_len}"));
    }

    let total = 4_usize
        .checked_add(frame_len)
        .ok_or_else(|| anyhow!("framed message length overflow"))?;
    Ok((buffer.len() >= total).then_some(total))
}

pub struct ReceiverGattServer {
    adapter: Arc<Adapter>,
    advertisement: Vec<u8>,
    sender: Sender<ChannelMessage>,
}

impl ReceiverGattServer {
    pub async fn new(
        advertisement: Vec<u8>,
        sender: Sender<ChannelMessage>,
    ) -> Result<Self, anyhow::Error> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;

        Ok(Self {
            adapter: Arc::new(adapter),
            advertisement,
            sender,
        })
    }

    pub async fn run(&self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        let service_uuid = Uuid::from_u16(QS_GATT_SERVICE);
        let slot0: Uuid = QS_ADV_SLOT0_UUID.parse()?;
        let weave_write: Uuid = QS_WEAVE_TO_PERIPHERAL.parse()?;
        let weave_notify: Uuid = QS_WEAVE_FROM_PERIPHERAL.parse()?;
        let advertisement = self.advertisement.clone();

        let (packet_sender, packet_receiver) = unbounded_channel::<Vec<u8>>();
        let packet_receiver: Arc<Mutex<Option<UnboundedReceiver<Vec<u8>>>>> =
            Arc::new(Mutex::new(Some(packet_receiver)));
        let channel_sender = self.sender.clone();

        let app = Application {
            services: vec![Service {
                uuid: service_uuid,
                primary: true,
                characteristics: vec![
                    Characteristic {
                        uuid: slot0,
                        read: Some(CharacteristicRead {
                            read: true,
                            fun: Box::new(move |_request| {
                                let advertisement = advertisement.clone();
                                Box::pin(async move { Ok(advertisement) })
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Characteristic {
                        uuid: weave_write,
                        write: Some(CharacteristicWrite {
                            write: true,
                            write_without_response: false,
                            method: CharacteristicWriteMethod::Fun(Box::new(
                                move |value, _request| {
                                    let packet_sender = packet_sender.clone();
                                    Box::pin(async move {
                                        packet_sender
                                            .send(value)
                                            .map_err(|_| bluer::gatt::local::ReqError::Failed)?;
                                        Ok(())
                                    })
                                },
                            )),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Characteristic {
                        uuid: weave_notify,
                        notify: Some(CharacteristicNotify {
                            notify: true,
                            indicate: true,
                            method: CharacteristicNotifyMethod::Fun(Box::new(move |notifier| {
                                let packet_receiver = packet_receiver.clone();
                                let channel_sender = channel_sender.clone();
                                Box::pin(async move {
                                    if let Err(error) =
                                        weave_session(notifier, packet_receiver, channel_sender)
                                            .await
                                    {
                                        warn!(
                                            "{INNER_NAME}: weave session ended with error: {error}"
                                        );
                                    }
                                })
                            })),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };

        info!(
            "{INNER_NAME}: registering GATT 0x{QS_GATT_SERVICE:04X} (slot0 {} bytes)",
            self.advertisement.len()
        );
        let handle = self.adapter.serve_gatt_application(app).await?;
        ctk.cancelled().await;
        drop(handle);
        info!("{INNER_NAME}: stopped");
        Ok(())
    }
}

async fn weave_session(
    mut notifier: CharacteristicNotifier,
    packet_receiver: Arc<Mutex<Option<UnboundedReceiver<Vec<u8>>>>>,
    sender: Sender<ChannelMessage>,
) -> Result<(), anyhow::Error> {
    let mut receiver_guard = packet_receiver.lock().await;
    let receiver = receiver_guard
        .as_mut()
        .ok_or_else(|| anyhow!("weave packet receiver is unavailable"))?;

    info!("{INNER_NAME}: weave notify session opened");

    let selected_packet_size = loop {
        let packet = receiver
            .recv()
            .await
            .ok_or_else(|| anyhow!("weave packet channel closed during handshake"))?;

        match parse_connection_request(&packet) {
            Ok(selected) => break selected,
            Err(error) => {
                debug!("{INNER_NAME}: ignoring pre-handshake packet: {error}");
            }
        }
    };

    notifier
        .notify(connection_confirm(selected_packet_size).to_vec())
        .await?;
    let max_payload = usize::from(selected_packet_size.saturating_sub(1)).max(19);

    let (inbound_side, weave_side) = tokio::io::duplex(64 * 1024);
    let (mut weave_read, mut weave_write) = tokio::io::split(weave_side);
    let inbound_sender = sender.clone();

    let inbound_task = tokio::spawn(async move {
        let mut request = InboundRequest::new(
            MigratableStream::Ble(inbound_side),
            "ble-weave".to_owned(),
            inbound_sender,
        );

        loop {
            match request.handle().await {
                Ok(()) => {}
                Err(error) => {
                    if !matches!(error.downcast_ref(), Some(AppError::NotAnError)) {
                        debug!("{INNER_NAME}: BLE inbound ended: {error}");
                    }
                    break;
                }
            }
        }
    });

    let mut reassembly = Vec::new();
    let mut outbound_buffer = Vec::new();
    let mut send_counter: u8 = 1;
    let mut read_buffer = [0_u8; 2048];

    let result = loop {
        tokio::select! {
            maybe_packet = receiver.recv() => {
                let Some(packet) = maybe_packet else {
                    break Ok(());
                };
                if packet.is_empty() {
                    continue;
                }

                let header = packet[0];
                if header & WEAVE_CONTROL != 0 {
                    if header & WEAVE_CMD_MASK == WEAVE_CMD_ERROR {
                        break Err(anyhow!("peer sent weave ERROR"));
                    }
                    continue;
                }

                if header & WEAVE_FIRST_BIT != 0 {
                    reassembly.clear();
                }

                let new_len = reassembly
                    .len()
                    .checked_add(packet.len().saturating_sub(1))
                    .ok_or_else(|| anyhow!("weave reassembly length overflow"))?;
                if new_len > MAX_WEAVE_MESSAGE_SIZE {
                    break Err(anyhow!("weave message exceeds safety limit"));
                }
                reassembly.extend_from_slice(&packet[1..]);

                if header & WEAVE_LAST_BIT == 0 {
                    continue;
                }

                if reassembly.len() < 3 {
                    reassembly.clear();
                    continue;
                }

                if reassembly[..3] == [0, 0, 0] {
                    let control_type = if reassembly.len() >= 5 && reassembly[3] == 0x08 {
                        reassembly[4]
                    } else {
                        0
                    };
                    reassembly.clear();

                    match control_type {
                        SOCKET_CTRL_INTRODUCTION => {
                            debug!("{INNER_NAME}: BLE socket introduction");
                            continue;
                        }
                        SOCKET_CTRL_DISCONNECTION => break Ok(()),
                        _ => continue,
                    }
                }

                if reassembly[..3] != QS_SVC_HASH {
                    warn!("{INNER_NAME}: ignoring packet for unknown service hash");
                    reassembly.clear();
                    continue;
                }

                let framed = &reassembly[3..];
                if framed.len() < 4 {
                    break Err(anyhow!("BLE data message is missing frame length"));
                }
                let declared = u32::from_be_bytes([framed[0], framed[1], framed[2], framed[3]]) as usize;
                if declared == 0 || declared > MAX_INBOUND_FRAME_SIZE {
                    break Err(anyhow!("BLE frame length is invalid: {declared}"));
                }
                if framed.len() != 4 + declared {
                    break Err(anyhow!(
                        "BLE frame length mismatch: declared {declared}, got {}",
                        framed.len().saturating_sub(4)
                    ));
                }

                weave_write.write_all(framed).await?;
                reassembly.clear();
            }
            read_result = weave_read.read(&mut read_buffer) => {
                let count = read_result?;
                if count == 0 {
                    break Ok(());
                }

                let new_len = outbound_buffer
                    .len()
                    .checked_add(count)
                    .ok_or_else(|| anyhow!("outbound BLE buffer length overflow"))?;
                if new_len > MAX_WEAVE_MESSAGE_SIZE {
                    break Err(anyhow!("outbound BLE buffer exceeds safety limit"));
                }
                outbound_buffer.extend_from_slice(&read_buffer[..count]);

                while let Some(total) = complete_framed_message_len(&outbound_buffer)? {
                    let framed: Vec<u8> = outbound_buffer.drain(..total).collect();

                    let mut message = Vec::with_capacity(3 + framed.len());
                    message.extend_from_slice(&QS_SVC_HASH);
                    message.extend_from_slice(&framed);

                    let mut offset = 0;
                    while offset < message.len() {
                        let end = (offset + max_payload).min(message.len());
                        let mut header = (send_counter & 0x07) << 4;
                        if offset == 0 {
                            header |= WEAVE_FIRST_BIT;
                        }
                        if end == message.len() {
                            header |= WEAVE_LAST_BIT;
                        }

                        let mut packet = Vec::with_capacity(1 + end - offset);
                        packet.push(header);
                        packet.extend_from_slice(&message[offset..end]);
                        notifier.notify(packet).await?;

                        send_counter = send_counter.wrapping_add(1);
                        offset = end;
                    }
                }
            }
        }
    };

    inbound_task.abort();
    info!("{INNER_NAME}: weave session closed");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_connection_request_and_clamps_packet_size() {
        let packet = [0x80, 0x00, 0x01, 0x00, 0x01, 0x10, 0x00];
        assert_eq!(
            parse_connection_request(&packet).unwrap(),
            MAX_WEAVE_PACKET_SIZE
        );

        let packet = [0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01];
        assert_eq!(
            parse_connection_request(&packet).unwrap(),
            MIN_WEAVE_PACKET_SIZE
        );
    }

    #[test]
    fn rejects_unsupported_or_malformed_connection_requests() {
        assert!(parse_connection_request(&[0x80]).is_err());
        assert!(parse_connection_request(&[0x80, 0x00, 0x02, 0x00, 0x02, 0x01, 0xfd]).is_err());
        assert!(parse_connection_request(&[0x00, 0x00, 0x01, 0x00, 0x01, 0x01, 0xfd]).is_err());
    }

    #[test]
    fn reports_complete_framed_messages_safely() {
        assert_eq!(complete_framed_message_len(&[0, 0, 0]).unwrap(), None);

        let mut frame = 3_u32.to_be_bytes().to_vec();
        frame.extend_from_slice(&[1, 2, 3]);
        assert_eq!(complete_framed_message_len(&frame).unwrap(), Some(7));

        assert!(complete_framed_message_len(&0_u32.to_be_bytes()).is_err());
    }
}

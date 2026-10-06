use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::channel::{ChannelDirection, ChannelMessage};
use crate::errors::AppError;
use crate::hdl::{
    peek_client_introduction, BwuRouter, InboundRequest, OutboundPayload, OutboundRequest, State,
};
use crate::utils::RemoteDeviceInfo;

const INNER_NAME: &str = "TcpServer";
const MAX_UI_ERROR_CHARS: usize = 512;
const BWU_PEEK_LIMIT: usize = 8 * 1024;

async fn route_bandwidth_upgrade_if_pending(
    socket: TcpStream,
    router: &BwuRouter,
) -> Result<Option<TcpStream>, anyhow::Error> {
    if !router.has_pending().await {
        return Ok(Some(socket));
    }

    let mut buffer = vec![0_u8; BWU_PEEK_LIMIT];
    let endpoint_id = match tokio::time::timeout(Duration::from_millis(750), async {
        loop {
            let count = socket.peek(&mut buffer).await?;
            if count == 0 {
                return Ok::<Option<String>, anyhow::Error>(None);
            }

            if count >= 4 {
                let frame_len =
                    u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
                if frame_len == 0 || frame_len > BWU_PEEK_LIMIT.saturating_sub(4) {
                    return Ok(None);
                }

                let total = 4 + frame_len;
                if count >= total {
                    return peek_client_introduction(&buffer[..total]);
                }
            }

            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    {
        Ok(result) => result?,
        Err(_) => None,
    };

    let Some(endpoint_id) = endpoint_id else {
        return Ok(Some(socket));
    };

    match router.route(&endpoint_id, socket).await {
        Ok(()) => Ok(None),
        Err(_socket) => {
            warn!(
                "{INNER_NAME}: received BWU CLIENT_INTRODUCTION for unregistered endpoint {endpoint_id}"
            );
            Ok(None)
        }
    }
}

fn error_for_ui(error: &anyhow::Error) -> String {
    error.to_string().chars().take(MAX_UI_ERROR_CHARS).collect()
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SendInfo {
    pub id: String,
    pub name: String,
    pub addr: String,
    pub ob: OutboundPayload,
}

pub struct TcpServer {
    endpoint_id: [u8; 4],
    tcp_listener: TcpListener,
    sender: Sender<ChannelMessage>,
    connect_receiver: Receiver<SendInfo>,
    bwu_router: BwuRouter,
}

impl TcpServer {
    pub fn new(
        endpoint_id: [u8; 4],
        tcp_listener: TcpListener,
        sender: Sender<ChannelMessage>,
        connect_receiver: Receiver<SendInfo>,
        bwu_router: BwuRouter,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            endpoint_id,
            tcp_listener,
            sender,
            connect_receiver,
            bwu_router,
        })
    }

    pub async fn run(&mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        loop {
            let cctk = ctk.clone();

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                Some(i) = self.connect_receiver.recv() => {
                    info!("{INNER_NAME}: outbound request queued for {}", i.addr);
                    let request_id = i.id.clone();
                    if let Err(e) = self.connect(cctk, i).await {
                        error!("{INNER_NAME}: error sending: {}", e);
                        let _ = self.sender.send(ChannelMessage {
                            id: request_id,
                            direction: ChannelDirection::LibToFront,
                            state: Some(State::Disconnected),
                            error: Some(error_for_ui(&e)),
                            ..Default::default()
                        });
                    }
                }
                r = self.tcp_listener.accept() => {
                    match r {
                        Ok((socket, remote_addr)) => {
                            info!("{INNER_NAME}: accepted inbound client from {remote_addr}");
                            let esender = self.sender.clone();
                            let csender = self.sender.clone();
                            let bwu_router = self.bwu_router.clone();

                            tokio::spawn(async move {
                                let socket = match route_bandwidth_upgrade_if_pending(
                                    socket,
                                    &bwu_router,
                                )
                                .await
                                {
                                    Ok(Some(socket)) => socket,
                                    Ok(None) => {
                                        info!(
                                            "{INNER_NAME}: routed bandwidth-upgrade client from {remote_addr}"
                                        );
                                        return;
                                    }
                                    Err(error) => {
                                        warn!(
                                            "{INNER_NAME}: failed to classify inbound client {remote_addr}: {error}"
                                        );
                                        return;
                                    }
                                };

                                let mut ir = InboundRequest::new(socket, remote_addr.to_string(), csender);

                                loop {
                                    match ir.handle().await {
                                        Ok(_) => {},
                                        Err(e) => match e.downcast_ref() {
                                            Some(AppError::NotAnError) => {
                                                debug!("{INNER_NAME}: inbound session {remote_addr} completed");
                                                break;
                                            },
                                            None => {
                                                if ir.state.state == State::Initial {
                                                    warn!(
                                                        "{INNER_NAME}: inbound client {remote_addr} failed during initial handshake: {e}"
                                                    );
                                                    break;
                                                }

                                                if ir.state.state != State::Finished {
                                                    let _ = esender.send(ChannelMessage {
                                                        id: remote_addr.to_string(),
                                                        direction: ChannelDirection::LibToFront,
                                                        state: Some(State::Disconnected),
                                                        meta: ir.state.transfer_metadata.clone(),
                                                        error: Some(error_for_ui(&e)),
                                                        ..Default::default()
                                                    });
                                                }
                                                error!(
                                                    "{INNER_NAME}: error while handling inbound client {remote_addr}: {e} ({:?})",
                                                    ir.state.state
                                                );
                                                break;
                                            }
                                        },
                                    }
                                }
                            });
                        },
                        Err(err) => {
                            error!("{INNER_NAME}: error accepting: {err}");
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// To be called inside a separate task if we want to handle concurrency
    pub async fn connect(&self, ctk: CancellationToken, si: SendInfo) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: connecting to outbound peer {}", si.addr);
        let socket = TcpStream::connect(si.addr.clone()).await?;
        info!("{INNER_NAME}: TCP connection established to {}", si.addr);

        let mut or = OutboundRequest::new(
            self.endpoint_id,
            socket,
            si.id,
            self.sender.clone(),
            si.ob,
            RemoteDeviceInfo {
                device_type: crate::DeviceType::Unknown,
                name: si.name,
            },
        );

        // Send connection request
        or.send_connection_request().await?;
        debug!("{INNER_NAME}: connection request sent to {}", si.addr);
        // Send UKEY init
        or.send_ukey2_client_init().await?;
        debug!("{INNER_NAME}: UKEY2 client init sent to {}", si.addr);

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                },
                r = or.handle() => {
                    if let Err(e) = r {
                        match e.downcast_ref() {
                            Some(AppError::NotAnError) => break,
                            None => {
                                if or.state.state == State::Initial {
                                    warn!(
                                        "{INNER_NAME}: outbound peer {} failed during initial handshake: {e}",
                                        si.addr
                                    );
                                    break;
                                }

                                if or.state.state != State::Finished && or.state.state != State::Cancelled {
                                    let _ = self.sender.clone().send(ChannelMessage {
                                        id: si.addr.clone(),
                                        direction: ChannelDirection::LibToFront,
                                        state: Some(State::Disconnected),
                                        meta: or.state.transfer_metadata.clone(),
                                        error: Some(error_for_ui(&e)),
                                        ..Default::default()
                                    });
                                }
                                error!(
                                    "{INNER_NAME}: error while handling outbound peer {}: {e} ({:?})",
                                    si.addr,
                                    or.state.state
                                );
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use prost::Message;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    use super::*;
    use crate::location_nearby_connections::bandwidth_upgrade_negotiation_frame::{
        ClientIntroduction, EventType,
    };
    use crate::location_nearby_connections::{
        offline_frame, v1_frame, BandwidthUpgradeNegotiationFrame, OfflineFrame, V1Frame,
    };

    fn client_introduction(endpoint_id: &str) -> Vec<u8> {
        let frame = OfflineFrame {
            version: Some(offline_frame::Version::V1.into()),
            v1: Some(V1Frame {
                r#type: Some(v1_frame::FrameType::BandwidthUpgradeNegotiation.into()),
                bandwidth_upgrade_negotiation: Some(BandwidthUpgradeNegotiationFrame {
                    event_type: Some(EventType::ClientIntroduction.into()),
                    client_introduction: Some(ClientIntroduction {
                        endpoint_id: Some(endpoint_id.to_owned()),
                        supports_disabling_encryption: Some(false),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };

        let encoded = frame.encode_to_vec();
        let mut framed = (encoded.len() as u32).to_be_bytes().to_vec();
        framed.extend_from_slice(&encoded);
        framed
    }

    #[tokio::test]
    async fn routes_bwu_socket_without_consuming_client_introduction() {
        let router = BwuRouter::new();
        let receiver = router.register("peer-1234".to_owned()).await.unwrap();

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let mut client = TcpStream::connect(address).await.unwrap();
        let (server, _) = listener.accept().await.unwrap();

        let framed = client_introduction("peer-1234");
        client.write_all(&framed).await.unwrap();
        client.flush().await.unwrap();

        assert!(
            route_bandwidth_upgrade_if_pending(server, &router)
                .await
                .unwrap()
                .is_none()
        );

        let mut routed = receiver.await.unwrap();
        let mut received = vec![0_u8; framed.len()];
        routed.read_exact(&mut received).await.unwrap();
        assert_eq!(received, framed);
    }
}

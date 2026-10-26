use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn given_nested_control_responses_when_count_or_byte_budget_is_full_then_queue_fails_closed()
{
    // Arrange / Act / Assert
    let (mut request, _peer) = pair().await;
    request.bulk_writes = Some(std::collections::VecDeque::from(vec![vec![0; 4]; 31]));
    request.send_frame(vec![1]).await.unwrap();
    assert_eq!(request.bulk_writes.as_ref().unwrap().len(), 32);
    assert!(request
        .send_frame(vec![2])
        .await
        .unwrap_err()
        .to_string()
        .contains("safety limit"));
    assert_eq!(request.bulk_writes.as_ref().unwrap().len(), 32);
    request.bulk_writes = Some(std::collections::VecDeque::from([vec![
        0;
        SANE_FRAME_LENGTH
            as usize
            - 5
    ]]));
    request.send_frame(vec![1]).await.unwrap();
    assert_eq!(
        request
            .bulk_writes
            .as_ref()
            .unwrap()
            .iter()
            .map(Vec::len)
            .sum::<usize>(),
        SANE_FRAME_LENGTH as usize
    );
    assert!(request.send_frame(vec![2]).await.is_err());
    assert_eq!(request.bulk_writes.as_ref().unwrap().len(), 2);
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn given_local_legacy_cancel_when_payload_is_backpressured_then_control_wakes_writer() {
    // Arrange
    let (mut request, mut peer) = pair().await;
    request.state.state = State::SendingFiles;
    rustix::net::sockopt::set_socket_send_buffer_size(&request.socket, 4096).unwrap();
    let sender = request.sender.clone();
    let id = request.state.id.clone();
    let frame = file_frame();
    let (finished, completion) = tokio::sync::oneshot::channel();
    let remote = async {
        peer.read_exact(&mut [0; 64]).await.unwrap();
        sender
            .send(ChannelMessage {
                id,
                direction: ChannelDirection::FrontToLib,
                action: Some(ChannelAction::CancelTransfer),
                ..Default::default()
            })
            .unwrap();
        let _ = completion.await;
    };
    // Act / Assert
    let (result, ()) = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let send = async {
            let result = request.encrypt_and_send(&frame).await;
            let _ = finished.send(());
            result
        };
        tokio::join!(send, remote)
    })
    .await
    .unwrap();
    assert!(result.is_err());
    assert_eq!(request.state.state, State::Cancelled);
    assert!(request.bulk_writes.is_none());
}

use super::keepalive_tests::{pair, read_secure};
use super::*;

fn file_frame() -> OfflineFrame {
    OfflineFrame {
        v1: Some(location_nearby_connections::V1Frame {
            r#type: Some(location_nearby_connections::v1_frame::FrameType::PayloadTransfer.into()),
            payload_transfer: Some(PayloadTransferFrame {
                packet_type: Some(PacketType::Data.into()),
                payload_header: Some(PayloadHeader {
                    id: Some(50),
                    r#type: Some(payload_header::PayloadType::File.into()),
                    total_size: Some(3 * 1024 * 1024),
                    ..Default::default()
                }),
                payload_chunk: Some(PayloadChunk {
                    offset: Some(0),
                    flags: Some(0),
                    body: Some(vec![0x5a; 3 * 1024 * 1024]),
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

async fn encoded(frames: Vec<OfflineFrame>) -> Vec<u8> {
    let (mut encoder, mut reader) = pair().await;
    let mut bytes = Vec::new();
    for frame in frames {
        encoder.encrypt_and_send(&frame).await.unwrap();
        let data = read_secure(&mut reader).await.encode_to_vec();
        bytes.extend_from_slice(&(data.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&data);
    }
    bytes
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn given_blocked_file_write_when_peer_cancels_disconnects_or_closes_then_sender_stops_without_false_finish(
) {
    for scenario in ["cancel", "disconnect", "eof", "bad-mac"] {
        // Arrange
        let (mut request, mut peer) = pair().await;
        request.state.state = State::SendingFiles;
        rustix::net::sockopt::set_socket_send_buffer_size(&request.socket, 4096).unwrap();
        let mut control = if scenario == "cancel" {
            let cancel = sharing_nearby::Frame {
                v1: Some(sharing_nearby::V1Frame {
                    r#type: Some(sharing_nearby::v1_frame::FrameType::Cancel.into()),
                    ..Default::default()
                }),
                ..Default::default()
            };
            encoded(byte_payload_frames(101, &cancel.encode_to_vec()).unwrap()).await
        } else {
            encoded(vec![OfflineFrame {
                v1: Some(location_nearby_connections::V1Frame {
                    r#type: Some(
                        location_nearby_connections::v1_frame::FrameType::Disconnection.into(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            }])
            .await
        };
        if scenario == "bad-mac" {
            *control.last_mut().unwrap() ^= 1;
        }
        let (finished, completion) = tokio::sync::oneshot::channel::<()>();
        let remote = async {
            // Observe an actual partial payload, then stop draining outbound data.
            peer.read_exact(&mut [0; 64]).await.unwrap();
            if scenario != "eof" {
                peer.write_all(&control).await.unwrap();
            }
            if scenario == "eof" {
                drop(peer);
                return;
            }
            // Keep the socket open until the independent cancellation path ends.
            let _ = completion.await;
        };
        let frame = file_frame();
        // Act: timeout only detects a protocol deadlock; success never waits on it.
        let result = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            let send = async {
                let result = request.encrypt_and_send(&frame).await;
                let _ = finished.send(());
                result
            };
            tokio::join!(send, remote).0
        })
        .await
        .unwrap();
        // Assert
        assert!(result.is_err());
        assert_ne!(request.state.state, State::Finished);
        if scenario == "cancel" {
            assert_eq!(request.state.state, State::Cancelled);
        }
        if scenario == "disconnect" {
            assert_eq!(request.state.state, State::Disconnected);
        }
        assert!(request.bulk_writes.is_none());
    }
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn given_keepalive_during_partial_write_when_peer_drains_then_ack_follows_complete_payload_in_crypto_order(
) {
    // Arrange
    let (mut request, mut peer) = pair().await;
    request.state.state = State::SendingFiles;
    rustix::net::sockopt::set_socket_send_buffer_size(&request.socket, 4096).unwrap();
    let control = encoded(vec![OfflineFrame {
        v1: Some(location_nearby_connections::V1Frame {
            r#type: Some(location_nearby_connections::v1_frame::FrameType::KeepAlive.into()),
            keep_alive: Some(KeepAliveFrame { ack: Some(false) }),
            ..Default::default()
        }),
        ..Default::default()
    }])
    .await;
    let remote = async {
        let length = peer.read_u32().await.unwrap() as usize;
        let mut file = vec![0; length];
        peer.read_exact(&mut file[..64]).await.unwrap();
        peer.write_all(&control).await.unwrap();
        peer.read_exact(&mut file[64..]).await.unwrap();
        let file = decode(SecureMessage::decode(file.as_slice()).unwrap(), 1);
        assert_eq!(
            file.v1
                .unwrap()
                .payload_transfer
                .unwrap()
                .payload_chunk
                .unwrap()
                .body
                .unwrap(),
            vec![0x5a; 3 * 1024 * 1024]
        );
        let ack = decode(read_secure(&mut peer).await, 2).v1.unwrap();
        assert_eq!(
            ack.r#type(),
            location_nearby_connections::v1_frame::FrameType::KeepAlive
        );
        assert_eq!(ack.keep_alive.unwrap().ack, Some(true));
    };
    // Act / Assert
    let frame = file_frame();
    let (result, ()) = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        tokio::join!(request.encrypt_and_send(&frame), remote)
    })
    .await
    .unwrap();
    result.unwrap();
    assert_eq!(request.state.server_seq, 2);
    assert_eq!(request.state.client_seq, 1);
    assert!(request.bulk_writes.is_none());
}

fn decode(secure: SecureMessage, sequence: i32) -> OfflineFrame {
    let mut hmac = HmacSha256::new_from_slice(&[0x22; 32]).unwrap();
    hmac.update(&secure.header_and_body);
    hmac.verify_slice(&secure.signature).unwrap();
    let body = HeaderAndBody::decode(secure.header_and_body.as_slice()).unwrap();
    let mut cipher = Cipher::new_256(&[0x11; 32]);
    cipher.set_auto_padding(true);
    let plaintext = cipher.cbc_decrypt(body.header.iv(), &body.body);
    let message = DeviceToDeviceMessage::decode(plaintext.as_slice()).unwrap();
    assert_eq!(message.sequence_number(), sequence);
    OfflineFrame::decode(message.message()).unwrap()
}

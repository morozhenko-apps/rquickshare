use tokio::io::AsyncReadExt;

#[tokio::test]
async fn given_peer_disconnect_when_outbound_is_active_or_terminal_then_existing_terminal_outcome_is_absorbing(
) {
    for state in [
        State::SendingFiles,
        State::Finished,
        State::Cancelled,
        State::Rejected,
    ] {
        // Arrange
        let (mut target, _peer) = pair().await;
        let (mut encoder, mut reader) = pair().await;
        target.state.state = state.clone();
        encoder
            .encrypt_and_send(&OfflineFrame {
                v1: Some(location_nearby_connections::V1Frame {
                    r#type: Some(
                        location_nearby_connections::v1_frame::FrameType::Disconnection.into(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await
            .unwrap();
        // Act / Assert
        assert!(target
            .decrypt_and_process_secure_message(&read_secure(&mut reader).await)
            .await
            .is_err());
        assert_eq!(
            target.state.state,
            if state == State::SendingFiles {
                State::Disconnected
            } else {
                state
            }
        );
    }
}

use super::*;

pub(super) async fn pair() -> (OutboundRequest, TcpStream) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let peer = TcpStream::connect(listener.local_addr().unwrap())
        .await
        .unwrap();
    let (socket, _) = listener.accept().await.unwrap();
    let (sender, _) = tokio::sync::broadcast::channel(16);
    let mut request = OutboundRequest::new(
        [1, 2, 3, 4],
        socket,
        "keepalive".into(),
        sender,
        OutboundPayload::Files(vec![]),
        RemoteDeviceInfo {
            name: "fixture".into(),
            device_type: DeviceType::Phone,
        },
    )
    .unwrap();
    request.state.encrypt_key = Some(vec![0x11; 32]);
    request.state.decrypt_key = Some(vec![0x11; 32]);
    request.state.send_hmac_key = Some(vec![0x22; 32]);
    request.state.recv_hmac_key = Some(vec![0x22; 32]);
    (request, peer)
}

pub(super) async fn read_secure(peer: &mut TcpStream) -> SecureMessage {
    let size = peer.read_u32().await.unwrap();
    let mut bytes = vec![0; size as usize];
    peer.read_exact(&mut bytes).await.unwrap();
    SecureMessage::decode(bytes.as_slice()).unwrap()
}

#[tokio::test]
async fn given_keepalive_variants_when_outbound_handles_then_requests_reply_and_acks_are_silent() {
    for body in [
        None,
        Some(KeepAliveFrame { ack: None }),
        Some(KeepAliveFrame { ack: Some(false) }),
        Some(KeepAliveFrame { ack: Some(true) }),
    ] {
        // Arrange: production encoder supplies authenticated peer input.
        let (mut encoder, mut encoded) = pair().await;
        let (mut target, mut peer) = pair().await;
        let expects_reply = body.as_ref().is_some_and(|body| !body.ack());
        let malformed = body.is_none();
        if !expects_reply {
            target.state.encrypt_key = None;
            target.state.send_hmac_key = None;
        }
        encoder
            .encrypt_and_send(&OfflineFrame {
                v1: Some(location_nearby_connections::V1Frame {
                    r#type: Some(
                        location_nearby_connections::v1_frame::FrameType::KeepAlive.into(),
                    ),
                    keep_alive: body,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await
            .unwrap();
        // Act
        let result = target
            .decrypt_and_process_secure_message(&read_secure(&mut encoded).await)
            .await;
        // Assert
        assert_eq!(target.state.client_seq, 1);
        if malformed {
            assert_eq!(result.unwrap_err().to_string(), "Missing keepalive body");
        } else {
            result.unwrap();
        }
        assert_eq!(target.state.server_seq, i32::from(expects_reply));
        if expects_reply {
            let secure = read_secure(&mut peer).await;
            let mut hmac = HmacSha256::new_from_slice(&[0x22; 32]).unwrap();
            hmac.update(&secure.header_and_body);
            hmac.verify_slice(&secure.signature).unwrap();
            let body = HeaderAndBody::decode(secure.header_and_body.as_slice()).unwrap();
            let mut cipher = Cipher::new_256(&[0x11; 32]);
            cipher.set_auto_padding(true);
            let plaintext = cipher.cbc_decrypt(body.header.iv(), &body.body);
            let message = DeviceToDeviceMessage::decode(plaintext.as_slice()).unwrap();
            assert_eq!(message.sequence_number(), 1);
            let v1 = OfflineFrame::decode(message.message()).unwrap().v1.unwrap();
            assert_eq!(
                v1.r#type(),
                location_nearby_connections::v1_frame::FrameType::KeepAlive
            );
            assert_eq!(v1.keep_alive.unwrap().ack, Some(true));
        }
        let read = peer.read_u8();
        tokio::pin!(read);
        assert!(futures::poll!(read).is_pending(), "no extra reply");
    }
}

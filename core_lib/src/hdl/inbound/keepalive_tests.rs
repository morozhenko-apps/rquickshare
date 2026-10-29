use tokio::io::{AsyncReadExt, DuplexStream};

use super::*;

fn request(socket: DuplexStream) -> InboundRequest<DuplexStream> {
    let (sender, _) = tokio::sync::broadcast::channel(16);
    let mut request = InboundRequest::new(socket, "keepalive".into(), sender);
    request.state.encrypt_key = Some(vec![0x11; 32]);
    request.state.decrypt_key = Some(vec![0x11; 32]);
    request.state.send_hmac_key = Some(vec![0x22; 32]);
    request.state.recv_hmac_key = Some(vec![0x22; 32]);
    request
}

fn frame(body: Option<KeepAliveFrame>) -> OfflineFrame {
    OfflineFrame {
        v1: Some(location_nearby_connections::V1Frame {
            r#type: Some(location_nearby_connections::v1_frame::FrameType::KeepAlive.into()),
            keep_alive: body,
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[tokio::test]
async fn given_keepalive_ack_variants_when_inbound_handles_then_only_requests_receive_one_ack() {
    for ack in [None, Some(false), Some(true)] {
        // Arrange: real encrypted input and output, external in-memory transport.
        let (socket, peer) = tokio::io::duplex(4096);
        let (mut target, mut peer) = (request(socket), request(peer));
        peer.encrypt_and_send(&frame(Some(KeepAliveFrame { ack })))
            .await
            .unwrap();
        let size = target.socket.read_u32().await.unwrap();
        let mut bytes = vec![0; size as usize];
        target.socket.read_exact(&mut bytes).await.unwrap();
        // Act
        target
            .decrypt_and_process_secure_message(&SecureMessage::decode(bytes.as_slice()).unwrap())
            .await
            .unwrap();
        // Assert
        assert_eq!(target.state.client_seq, 1);
        if ack != Some(true) {
            let size = peer.socket.read_u32().await.unwrap();
            let mut bytes = vec![0; size as usize];
            peer.socket.read_exact(&mut bytes).await.unwrap();
            let response = peer
                .decrypt_secure_message(&SecureMessage::decode(bytes.as_slice()).unwrap())
                .await
                .unwrap();
            let v1 = response.v1.unwrap();
            assert_eq!(
                v1.r#type(),
                location_nearby_connections::v1_frame::FrameType::KeepAlive
            );
            assert_eq!(v1.keep_alive.unwrap().ack, Some(true));
            assert_eq!(target.state.server_seq, 1);
        } else {
            assert_eq!(target.state.server_seq, 0);
        }
        let read = peer.socket.read_u8();
        tokio::pin!(read);
        assert!(futures::poll!(read).is_pending(), "no extra reply");
    }
}

#[tokio::test]
async fn given_ack_or_missing_body_when_inbound_has_no_send_keys_then_ack_is_silent_and_malformed_fails(
) {
    for body in [Some(KeepAliveFrame { ack: Some(true) }), None] {
        // Arrange
        let (socket, mut peer) = tokio::io::duplex(4096);
        let mut target = request(socket);
        target.state.encrypt_key = None;
        target.state.send_hmac_key = None;
        let malformed = body.is_none();
        // Act
        let result = target.process_offline_frame(frame(body)).await;
        // Assert
        if malformed {
            assert_eq!(result.unwrap_err().to_string(), "Missing keepalive body");
        } else {
            result.unwrap();
        }
        assert_eq!(target.state.server_seq, 0);
        let read = peer.read_u8();
        tokio::pin!(read);
        assert!(futures::poll!(read).is_pending());
    }
}

use location_nearby_connections::bandwidth_upgrade_negotiation_frame::EventType;
use location_nearby_connections::v1_frame::FrameType;
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};

use super::*;
use crate::hdl::{BwuRouter, MigratableStream};

#[tokio::test]
async fn given_peer_cancel_during_committed_migration_when_processed_then_no_tcp_swap_or_false_finish(
) {
    // Arrange
    let (mut request, mut peer) = pair();
    let (tcp, _tcp_peer) = tcp_pair().await;
    request.deferred_upgrade_writes = Some(Vec::new());
    peer.send_encrypted_frame(&sharing_nearby::Frame {
        v1: Some(sharing_nearby::V1Frame {
            r#type: Some(sharing_nearby::v1_frame::FrameType::Cancel.into()),
            ..Default::default()
        }),
        ..Default::default()
    })
    .await
    .unwrap();
    // Act / Assert
    assert!(request
        .finish_bandwidth_upgrade(tcp, Instant::now())
        .await
        .is_err());
    assert_eq!(request.state.state, State::Cancelled);
    assert!(matches!(request.socket, MigratableStream::Ble(_)));
    assert_eq!(
        request.state.server_seq, 1,
        "no terminal response interleaved after LAST_WRITE"
    );
}

#[tokio::test]
async fn given_deferred_application_response_when_budget_exceeded_then_no_crypto_sequence_is_consumed(
) {
    // Arrange
    let (mut request, _peer) = pair();
    let oversized = OfflineFrame {
        v1: Some(location_nearby_connections::V1Frame {
            r#type: Some(FrameType::PayloadTransfer.into()),
            payload_transfer: Some(location_nearby_connections::PayloadTransferFrame {
                payload_chunk: Some(
                    location_nearby_connections::payload_transfer_frame::PayloadChunk {
                        body: Some(vec![0; SANE_FRAME_LENGTH as usize]),
                        ..Default::default()
                    },
                ),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    request.deferred_upgrade_writes = Some(Vec::new());
    // Act / Assert
    assert!(request
        .encrypt_and_send(&oversized)
        .await
        .unwrap_err()
        .to_string()
        .contains("safety limit"));
    assert!(request.deferred_upgrade_writes.as_ref().unwrap().is_empty());
    assert_eq!(request.state.server_seq, 0);
}

#[tokio::test(start_paused = true)]
async fn given_missing_or_incomplete_sync_when_committed_then_upgrade_never_swaps_on_deadline_or_frame_limit(
) {
    for event in [
        None,
        Some(EventType::SafeToClosePriorChannel),
        Some(EventType::UpgradePathAvailable),
    ] {
        // Arrange
        let (mut request, mut peer) = pair();
        let (tcp, _tcp_peer) = tcp_pair().await;
        request.deferred_upgrade_writes = Some(Vec::new());
        if let Some(event) = event {
            let count = if event == EventType::UpgradePathAvailable {
                16
            } else {
                1
            };
            for _ in 0..count {
                peer.encrypt_and_send(&negotiation(event)).await.unwrap();
            }
        }
        let started = tokio::time::Instant::now();
        // Act
        let error = request
            .finish_bandwidth_upgrade(tcp, Instant::now())
            .await
            .unwrap_err();
        // Assert
        let limited = event == Some(EventType::UpgradePathAvailable);
        assert!(error
            .to_string()
            .contains(if limited { "incomplete" } else { "timed out" }));
        assert_eq!(
            started.elapsed(),
            if limited {
                Duration::ZERO
            } else {
                Duration::from_secs(5)
            }
        );
        assert!(matches!(request.socket, MigratableStream::Ble(_)));
        assert_eq!(
            peer.read_encrypted_offline_frame().await.unwrap(),
            negotiation(EventType::LastWriteToPriorChannel)
        );
        let read = peer.socket.read_u8();
        tokio::pin!(read);
        assert!(
            futures::poll!(read).is_pending(),
            "no premature disconnect or successful swap"
        );
    }
}

#[tokio::test(start_paused = true)]
async fn given_partial_body_or_bad_mac_when_committed_then_failure_does_not_reuse_partial_old_channel_as_tcp(
) {
    let mut bad_mac = encrypted_bytes(&negotiation(EventType::LastWriteToPriorChannel)).await;
    *bad_mac.last_mut().unwrap() ^= 1;
    for bytes in [vec![0, 0, 0, 16, 1], bad_mac] {
        // Arrange
        let (mut request, mut peer) = pair();
        let (tcp, _tcp_peer) = tcp_pair().await;
        request.deferred_upgrade_writes = Some(Vec::new());
        peer.socket.write_all(&bytes).await.unwrap();
        // Act / Assert
        assert!(request
            .finish_bandwidth_upgrade(tcp, Instant::now())
            .await
            .is_err());
        assert!(matches!(request.socket, MigratableStream::Ble(_)));
        assert!(request.state.client_seq <= 1);
    }
}

#[tokio::test]
async fn given_application_response_during_migration_when_both_peer_transitions_arrive_then_response_uses_tcp_in_sequence(
) {
    for safe_first in [false, true] {
        // Arrange
        let (mut request, mut peer) = pair();
        let (tcp, tcp_peer) = tcp_pair().await;
        request.deferred_upgrade_writes = Some(Vec::new());
        peer.encrypt_and_send(&OfflineFrame {
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(FrameType::KeepAlive.into()),
                keep_alive: Some(KeepAliveFrame { ack: Some(false) }),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await
        .unwrap();
        let remote = async {
            assert_eq!(
                peer.read_encrypted_offline_frame().await.unwrap(),
                negotiation(EventType::LastWriteToPriorChannel)
            );
            if safe_first {
                peer.encrypt_and_send(&negotiation(EventType::SafeToClosePriorChannel))
                    .await
                    .unwrap();
            }
            peer.encrypt_and_send(&negotiation(EventType::LastWriteToPriorChannel))
                .await
                .unwrap();
            // If keepalive were written after LAST_WRITE, this assertion would fail.
            assert_eq!(
                peer.read_encrypted_offline_frame().await.unwrap(),
                negotiation(EventType::SafeToClosePriorChannel)
            );
            if !safe_first {
                peer.encrypt_and_send(&negotiation(EventType::SafeToClosePriorChannel))
                    .await
                    .unwrap();
            }
            let close = read_plain_frame_from(&mut peer.socket).await.unwrap();
            assert_eq!(
                OfflineFrame::decode(close.as_slice())
                    .unwrap()
                    .v1
                    .unwrap()
                    .r#type(),
                FrameType::Disconnection
            );
            peer.socket = MigratableStream::Tcp(tcp_peer);
            let v1 = peer
                .read_encrypted_offline_frame()
                .await
                .unwrap()
                .v1
                .unwrap();
            assert_eq!(v1.r#type(), FrameType::KeepAlive);
            assert_eq!(v1.keep_alive.unwrap().ack, Some(true));
            assert_eq!(peer.state.client_seq, 3);
        };
        // Act / Assert
        let (result, ()) = tokio::join!(
            request.finish_bandwidth_upgrade(tcp, Instant::now()),
            remote
        );
        result.unwrap();
        assert!(matches!(request.socket, MigratableStream::Tcp(_)));
        assert!(request.deferred_upgrade_writes.is_none());
        assert_eq!(request.state.server_seq, 3);
        assert_eq!(request.state.client_seq, 3);
    }
}

#[tokio::test]
async fn given_established_eof_after_last_write_when_full_upgrade_runs_then_worker_receives_session_fatal_error(
) {
    // Arrange
    let (mut request, mut peer) = pair();
    let router = BwuRouter::new();
    let route = router.clone();
    let (socket, mut tcp_peer) = tcp_pair().await;
    let remote = async {
        peer.read_encrypted_offline_frame().await.unwrap();
        let intro = OfflineFrame {
            v1: Some(location_nearby_connections::V1Frame {
                r#type: Some(FrameType::BandwidthUpgradeNegotiation.into()),
                bandwidth_upgrade_negotiation: Some(location_nearby_connections::BandwidthUpgradeNegotiationFrame {
                    event_type: Some(EventType::ClientIntroduction.into()),
                    client_introduction: Some(location_nearby_connections::bandwidth_upgrade_negotiation_frame::ClientIntroduction {
                        endpoint_id: Some("PEER".into()), ..Default::default()
                    }), ..Default::default()
                }), ..Default::default()
            }), ..Default::default()
        };
        send_plain_frame_on(&mut tcp_peer, &intro.encode_to_vec())
            .await
            .unwrap();
        route.route_with_alias("PEER", None, socket).await.unwrap();
        read_plain_frame_from(&mut tcp_peer).await.unwrap();
        assert_eq!(
            peer.read_encrypted_offline_frame().await.unwrap(),
            negotiation(EventType::LastWriteToPriorChannel)
        );
        drop(peer);
    };
    // Act / Assert
    let (result, ()) = tokio::join!(
        request.do_bandwidth_upgrade_at(&router, 4242, [192, 0, 2, 1]),
        remote
    );
    assert!(result
        .unwrap_err()
        .downcast_ref::<BwuSessionError>()
        .is_some());
    assert!(request.deferred_upgrade_writes.is_none());
    assert!(matches!(request.socket, MigratableStream::Ble(_)));
}

// Only the external transport is substituted; encryption, parser and state
// transitions are the production implementations on both ends.
pub(super) fn pair() -> (
    InboundRequest<MigratableStream>,
    InboundRequest<MigratableStream>,
) {
    let (socket, peer) = tokio::io::duplex(65536);
    let (sender, _) = tokio::sync::broadcast::channel(64);
    let make = |socket: DuplexStream| {
        let mut request = InboundRequest::new(
            MigratableStream::Ble(socket),
            "fixture".into(),
            sender.clone(),
        );
        request.state.state = State::SentConnectionResponse;
        request.state.encryption_done = true;
        request.state.encrypt_key = Some(vec![0x11; 32]);
        request.state.decrypt_key = Some(vec![0x11; 32]);
        request.state.send_hmac_key = Some(vec![0x22; 32]);
        request.state.recv_hmac_key = Some(vec![0x22; 32]);
        request.peer_endpoint_id = Some("PEER".into());
        request
    };
    (make(socket), make(peer))
}

pub(super) async fn tcp_pair() -> (TcpStream, TcpStream) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let peer = TcpStream::connect(listener.local_addr().unwrap())
        .await
        .unwrap();
    let (socket, _) = listener.accept().await.unwrap();
    (socket, peer)
}

pub(super) fn negotiation(event: EventType) -> OfflineFrame {
    InboundRequest::<MigratableStream>::bandwidth_upgrade_frame(event, None)
}

async fn encrypted_bytes(frame: &OfflineFrame) -> Vec<u8> {
    let (mut encoder, mut reader) = pair();
    encoder.encrypt_and_send(frame).await.unwrap();
    let length = reader.socket.read_u32().await.unwrap();
    let mut bytes = vec![0; length as usize];
    reader.socket.read_exact(&mut bytes).await.unwrap();
    [length.to_be_bytes().to_vec(), bytes].concat()
}

#[tokio::test(start_paused = true)]
async fn given_authenticated_upgrade_failure_when_waiting_then_ble_continues_without_deadline() {
    // Arrange
    let (mut request, mut peer) = pair();
    let (_sender, receiver) = tokio::sync::oneshot::channel();
    peer.encrypt_and_send(&negotiation(EventType::UpgradeFailure))
        .await
        .unwrap();
    let start = tokio::time::Instant::now();
    // Act
    assert!(request
        .wait_for_bwu_socket(receiver)
        .await
        .unwrap()
        .is_none());
    // Assert: processed authenticated bytes, no timer advance or socket swap.
    assert_eq!(start.elapsed(), Duration::ZERO);
    assert_eq!(request.state.client_seq, 1);
    assert!(matches!(request.socket, MigratableStream::Ble(_)));
    peer.encrypt_and_send(&negotiation(EventType::UpgradePathAvailable))
        .await
        .unwrap();
    request.handle().await.unwrap();
    assert_eq!(request.state.client_seq, 2);
}

#[tokio::test(start_paused = true)]
async fn given_silent_peer_when_no_route_arrives_then_existing_fifteen_second_deadline_remains() {
    // Arrange
    let (mut request, _peer) = pair();
    let (_sender, receiver) = tokio::sync::oneshot::channel();
    let start = tokio::time::Instant::now();
    // Act / Assert
    assert!(request
        .wait_for_bwu_socket(receiver)
        .await
        .unwrap()
        .is_none());
    assert_eq!(start.elapsed(), Duration::from_secs(15));
    assert!(matches!(request.socket, MigratableStream::Ble(_)));
}

#[tokio::test(start_paused = true)]
async fn given_interleaved_frames_when_waiting_then_deadline_is_not_restarted() {
    // Arrange
    let (mut request, mut peer) = pair();
    let (_sender, receiver) = tokio::sync::oneshot::channel();
    let start = tokio::time::Instant::now();
    let waiting = request.wait_for_bwu_socket(receiver);
    tokio::pin!(waiting);
    assert!(futures::poll!(waiting.as_mut()).is_pending());
    // Act: a nonfailure negotiation is processed after fourteen virtual seconds.
    tokio::time::advance(Duration::from_secs(14)).await;
    peer.encrypt_and_send(&negotiation(EventType::UpgradePathAvailable))
        .await
        .unwrap();
    assert!(futures::poll!(waiting.as_mut()).is_pending());
    // Assert
    assert!(waiting.await.unwrap().is_none());
    assert_eq!(start.elapsed(), Duration::from_secs(15));
}

#[tokio::test]
async fn given_closed_route_when_waiting_then_upgrade_error_is_distinct_from_session_failure() {
    // Arrange
    let (mut request, _peer) = pair();
    let (sender, receiver) = tokio::sync::oneshot::channel::<TcpStream>();
    drop(sender);
    // Act / Assert
    let error = request.wait_for_bwu_socket(receiver).await.unwrap_err();
    assert!(error.downcast_ref::<BwuSessionError>().is_none());
    assert!(error.to_string().contains("route closed"));
}

#[tokio::test]
async fn given_tcp_arrival_during_partial_header_when_drain_reads_then_every_prefix_byte_survives()
{
    for split in 0..4 {
        // Arrange
        let (mut request, mut peer) = pair();
        let bytes = encrypted_bytes(&negotiation(EventType::SafeToClosePriorChannel)).await;
        peer.socket.write_all(&bytes[..split]).await.unwrap();
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let (socket, _tcp_peer) = tcp_pair().await;
        let expected = socket.peer_addr().unwrap();
        // Act
        {
            let waiting = request.wait_for_bwu_socket(receiver);
            tokio::pin!(waiting);
            assert!(futures::poll!(waiting.as_mut()).is_pending());
            sender.send(socket).unwrap();
            assert_eq!(
                waiting.await.unwrap().unwrap().peer_addr().unwrap(),
                expected
            );
        }
        peer.socket.write_all(&bytes[split..]).await.unwrap();
        let frame = request.read_encrypted_offline_frame().await.unwrap();
        // Assert: decryption and sequence checks also protect exact bytes.
        assert_eq!(frame, negotiation(EventType::SafeToClosePriorChannel));
        assert_eq!(request.state.client_seq, 1);
    }
}

#[tokio::test(start_paused = true)]
async fn given_deadline_during_partial_header_when_ble_resumes_then_frame_is_not_lost() {
    for split in 1..4 {
        // Arrange
        let (mut request, mut peer) = pair();
        let bytes = encrypted_bytes(&negotiation(EventType::UpgradeFailure)).await;
        peer.socket.write_all(&bytes[..split]).await.unwrap();
        let (_sender, receiver) = tokio::sync::oneshot::channel();
        // Act
        assert!(request
            .wait_for_bwu_socket(receiver)
            .await
            .unwrap()
            .is_none());
        peer.socket.write_all(&bytes[split..]).await.unwrap();
        request.handle().await.unwrap();
        // Assert
        assert_eq!(request.state.client_seq, 1);
        assert!(request.bwu_peer_failed);
    }
}

#[tokio::test]
async fn given_tcp_arrival_during_selected_body_when_waiting_then_body_finishes_before_route() {
    let bytes = encrypted_bytes(&negotiation(EventType::UpgradePathAvailable)).await;
    for split in [4, 5, bytes.len() - 1] {
        // Arrange
        let (mut request, mut peer) = pair();
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let (socket, _tcp_peer) = tcp_pair().await;
        peer.socket.write_all(&bytes[..split]).await.unwrap();
        // Act
        {
            let waiting = request.wait_for_bwu_socket(receiver);
            tokio::pin!(waiting);
            assert!(futures::poll!(waiting.as_mut()).is_pending());
            sender.send(socket).unwrap();
            assert!(
                futures::poll!(waiting.as_mut()).is_pending(),
                "must not discard selected body"
            );
            peer.socket.write_all(&bytes[split..]).await.unwrap();
            assert!(waiting.await.unwrap().is_some());
        }
        // Assert
        assert_eq!(request.state.client_seq, 1);
    }
}

#[tokio::test]
async fn given_invalid_established_frames_when_waiting_then_errors_remain_session_fatal() {
    let mut tampered = encrypted_bytes(&negotiation(EventType::UpgradeFailure)).await;
    *tampered.last_mut().unwrap() ^= 1;
    for bytes in [
        vec![],
        vec![0],
        vec![0, 0, 0, 0],
        (SANE_FRAME_LENGTH + 1).to_be_bytes().to_vec(),
        vec![0, 0, 0, 1, 0xff],
        tampered,
    ] {
        // Arrange: EOF, partial header, zero/oversize, invalid protobuf and MAC.
        let (mut request, mut peer) = pair();
        let (_sender, receiver) = tokio::sync::oneshot::channel();
        peer.socket.write_all(&bytes).await.unwrap();
        drop(peer);
        // Act / Assert
        let error = request.wait_for_bwu_socket(receiver).await.unwrap_err();
        assert!(error.downcast_ref::<BwuSessionError>().is_some());
        assert!(
            !request.bwu_peer_failed,
            "unauthenticated bytes cannot reject an upgrade"
        );
    }
}

#[tokio::test(start_paused = true)]
async fn given_failed_or_terminal_ble_when_upgrade_wait_ends_then_router_releases_registration() {
    for cancel in [false, true] {
        // Arrange
        let (mut request, mut peer) = pair();
        let router = BwuRouter::new();
        if cancel {
            peer.send_encrypted_frame(&sharing_nearby::Frame {
                version: Some(sharing_nearby::frame::Version::V1.into()),
                v1: Some(sharing_nearby::V1Frame {
                    r#type: Some(sharing_nearby::v1_frame::FrameType::Cancel.into()),
                    ..Default::default()
                }),
            })
            .await
            .unwrap();
        } else {
            peer.encrypt_and_send(&negotiation(EventType::UpgradeFailure))
                .await
                .unwrap();
        }
        // Act
        let result = request
            .do_bandwidth_upgrade_at(&router, 1234, [127, 0, 0, 1])
            .await;
        // Assert
        if cancel {
            let error = result.unwrap_err().downcast::<BwuSessionError>().unwrap();
            assert!(error.0.downcast_ref::<crate::errors::AppError>().is_some());
            assert_eq!(request.state.state, State::Cancelled);
        } else {
            result.unwrap();
        }
        assert!(!router.has_pending().await);
        assert!(router.register("PEER".into()).await.is_ok());
    }
}

#[tokio::test]
async fn given_valid_tcp_candidate_when_ble_drain_completes_then_existing_upgrade_still_switches() {
    // Arrange: real framed TCP introduction and authenticated old-channel drain.
    let (mut request, mut peer) = pair();
    let router = BwuRouter::new();
    let route = router.clone();
    let (socket, mut tcp_peer) = tcp_pair().await;
    let upgrade = async {
        request
            .do_bandwidth_upgrade_at(&router, 1234, [127, 0, 0, 1])
            .await
            .unwrap();
        assert!(matches!(request.socket, MigratableStream::Tcp(_)));
        let frame = request.read_encrypted_offline_frame().await.unwrap();
        assert_eq!(frame, negotiation(EventType::UpgradePathAvailable));
        assert_eq!(request.state.client_seq, 3);
        assert!(!router.has_pending().await);
    };
    let remote = async {
        let offer = peer.read_encrypted_offline_frame().await.unwrap();
        assert_eq!(
            offer
                .v1
                .unwrap()
                .bandwidth_upgrade_negotiation
                .unwrap()
                .event_type(),
            EventType::UpgradePathAvailable
        );
        let mut intro = super::security_tests::valid_bwu_client_intro_fixture();
        intro
            .v1
            .as_mut()
            .unwrap()
            .bandwidth_upgrade_negotiation
            .as_mut()
            .unwrap()
            .client_introduction
            .as_mut()
            .unwrap()
            .endpoint_id = Some("PEER".into());
        send_plain_frame_on(&mut tcp_peer, &intro.encode_to_vec())
            .await
            .unwrap();
        route.route_with_alias("PEER", None, socket).await.unwrap();
        let ack = read_plain_frame_from(&mut tcp_peer).await.unwrap();
        let ack = OfflineFrame::decode(ack.as_slice()).unwrap();
        assert_eq!(
            ack.v1
                .unwrap()
                .bandwidth_upgrade_negotiation
                .unwrap()
                .event_type(),
            EventType::ClientIntroductionAck
        );
        assert_eq!(
            peer.read_encrypted_offline_frame().await.unwrap(),
            negotiation(EventType::LastWriteToPriorChannel)
        );
        peer.encrypt_and_send(&negotiation(EventType::LastWriteToPriorChannel))
            .await
            .unwrap();
        assert_eq!(
            peer.read_encrypted_offline_frame().await.unwrap(),
            negotiation(EventType::SafeToClosePriorChannel)
        );
        peer.encrypt_and_send(&negotiation(EventType::SafeToClosePriorChannel))
            .await
            .unwrap();
        let closed = read_plain_frame_from(&mut peer.socket).await.unwrap();
        let closed = OfflineFrame::decode(closed.as_slice()).unwrap();
        assert_eq!(closed.v1.unwrap().r#type(), FrameType::Disconnection);
        peer.socket = MigratableStream::Tcp(tcp_peer);
        peer.encrypt_and_send(&negotiation(EventType::UpgradePathAvailable))
            .await
            .unwrap();
    };
    // Act / Assert: deadline only detects a deadlock, never waits for success.
    tokio::time::timeout(Duration::from_secs(5), async {
        tokio::join!(upgrade, remote);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn given_invalid_drain_frame_lengths_when_reading_then_allocation_is_rejected() {
    for length in [0, SANE_FRAME_LENGTH as u32 + 1, u32::MAX] {
        // Arrange
        let (mut request, mut peer) = pair();
        peer.socket.write_all(&length.to_be_bytes()).await.unwrap();
        // Act / Assert
        assert!(request
            .read_encrypted_offline_frame()
            .await
            .unwrap_err()
            .to_string()
            .contains("Invalid encrypted frame length"));
    }
}

#[tokio::test(start_paused = true)]
async fn given_deferred_bwu_writer_when_local_cancel_runs_then_delivery_fails_without_ble_bytes() {
    // Arrange: reproduce the state left by a dropped committed migration.
    let (mut request, mut peer) = pair();
    request.deferred_upgrade_writes = Some(Vec::new());
    // Act
    let result = request.send_local_cancel().await;
    // Assert: memory enqueue is not transport delivery.
    assert!(
        result.is_err(),
        "buffered Cancel must not report successful delivery"
    );
    assert_eq!(request.state.server_seq, 0);
    assert!(request.deferred_upgrade_writes.as_ref().unwrap().is_empty());
    let read = peer.socket.read_u8();
    tokio::pin!(read);
    assert!(futures::poll!(read).is_pending());
}

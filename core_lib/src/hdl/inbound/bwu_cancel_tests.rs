use location_nearby_connections::bandwidth_upgrade_negotiation_frame::EventType;
use location_nearby_connections::v1_frame::FrameType;
use tokio::io::AsyncReadExt;
use tokio_util::sync::CancellationToken;

use super::bwu_wait_tests::{negotiation, pair, tcp_pair};
use super::*;
use crate::channel::TransferType;
use crate::hdl::{BwuRouter, MigratableStream};
use crate::sessions::{SessionControl, SessionHub};

fn managed_request() -> (
    InboundRequest<MigratableStream>,
    InboundRequest<MigratableStream>,
    SessionControl,
    Receiver<ChannelMessage>,
) {
    let (request, peer) = pair();
    let hub = SessionHub::new(707).unwrap();
    let lease = hub
        .open(TransferType::Inbound, None, CancellationToken::new())
        .unwrap();
    let control = lease.control();
    let mut request = request.attach_session(lease);
    let events = request.sender.subscribe();
    request.bwu_pending = true;
    request.state.state = State::ReceivingFiles;
    request
        .session
        .as_mut()
        .unwrap()
        .observe_state(&request.state);
    (request, peer, control, events)
}

fn introduction(disable: Option<bool>) -> OfflineFrame {
    let mut frame = negotiation(EventType::ClientIntroduction);
    frame
        .v1
        .as_mut()
        .unwrap()
        .bandwidth_upgrade_negotiation
        .as_mut()
        .unwrap()
        .client_introduction = Some(
        location_nearby_connections::bandwidth_upgrade_negotiation_frame::ClientIntroduction {
            endpoint_id: Some("PEER".into()),
            supports_disabling_encryption: disable,
            ..Default::default()
        },
    );
    frame
}

async fn route_candidate(
    peer: &mut InboundRequest<MigratableStream>,
    route: &BwuRouter,
    socket: TcpStream,
    tcp_peer: &mut TcpStream,
    disable: Option<bool>,
) {
    // Wake the real worker's first handle without an application response.
    peer.encrypt_and_send(&OfflineFrame {
        v1: Some(location_nearby_connections::V1Frame {
            r#type: Some(FrameType::KeepAlive.into()),
            keep_alive: Some(KeepAliveFrame { ack: Some(true) }),
            ..Default::default()
        }),
        ..Default::default()
    })
    .await
    .unwrap();
    assert_eq!(
        peer.read_encrypted_offline_frame()
            .await
            .unwrap()
            .v1
            .unwrap()
            .bandwidth_upgrade_negotiation
            .unwrap()
            .event_type(),
        EventType::UpgradePathAvailable
    );
    send_plain_frame_on(tcp_peer, &introduction(disable).encode_to_vec())
        .await
        .unwrap();
    route.route_with_alias("PEER", None, socket).await.unwrap();
    let ack = read_plain_frame_from(tcp_peer).await.unwrap();
    assert_eq!(
        OfflineFrame::decode(ack.as_slice()).unwrap(),
        InboundRequest::<MigratableStream>::client_introduction_ack_frame()
    );
    assert_eq!(
        peer.read_encrypted_offline_frame().await.unwrap(),
        negotiation(EventType::LastWriteToPriorChannel)
    );
}

async fn read_cancel(peer: &mut InboundRequest<MigratableStream>) {
    let first = peer
        .read_encrypted_offline_frame()
        .await
        .unwrap()
        .v1
        .unwrap();
    assert_eq!(first.r#type(), FrameType::PayloadTransfer);
    let first = first.payload_transfer.unwrap();
    let header = first.payload_header.unwrap();
    let chunk = first.payload_chunk.unwrap();
    assert_eq!(chunk.offset(), 0);
    assert_eq!(chunk.flags(), 0);
    let sharing = sharing_nearby::Frame::decode(chunk.body()).unwrap();
    assert_eq!(
        sharing.v1.unwrap().r#type(),
        sharing_nearby::v1_frame::FrameType::Cancel
    );
    assert_eq!(header.total_size(), chunk.body().len() as i64);
    let last = peer
        .read_encrypted_offline_frame()
        .await
        .unwrap()
        .v1
        .unwrap()
        .payload_transfer
        .unwrap();
    assert_eq!(last.payload_header.as_ref().unwrap().id, header.id);
    assert_eq!(
        last.payload_header.as_ref().unwrap().total_size,
        header.total_size
    );
    let end = last.payload_chunk.unwrap();
    assert_eq!(end.offset(), header.total_size());
    assert_eq!(end.flags(), 1);
    assert!(end.body().is_empty());
}

#[tokio::test]
async fn given_local_cancel_at_each_committed_phase_when_worker_finishes_then_only_tcp_receives_complete_cancel(
) {
    for phase in 0..4 {
        // Arrange: real worker, crypto, native token, migration, external loopback TCP.
        let (request, mut peer, control, mut events) = managed_request();
        let sender = request.sender.clone();
        let router = BwuRouter::new();
        let (socket, mut tcp_peer) = tcp_pair().await;
        let lifetime = CancellationToken::new();
        let worker = crate::hdl::gatt::run_inbound_worker_with_resolver(
            request,
            sender,
            router.clone(),
            4242,
            lifetime,
            true,
            || Some([127, 0, 0, 1]),
        );
        let remote = async {
            route_candidate(&mut peer, &router, socket, &mut tcp_peer, Some(false)).await;
            if phase == 0 {
                control.cancel();
            }
            peer.encrypt_and_send(&negotiation(EventType::LastWriteToPriorChannel))
                .await
                .unwrap();
            assert_eq!(
                peer.read_encrypted_offline_frame().await.unwrap(),
                negotiation(EventType::SafeToClosePriorChannel)
            );
            if phase == 1 {
                control.cancel();
            }
            if phase <= 1 {
                // A payload after cancellation must be drained without publishing.
                // Malformed application content would fail if processed normally.
                peer.encrypt_and_send(&OfflineFrame {
                    v1: Some(location_nearby_connections::V1Frame {
                        r#type: Some(FrameType::PayloadTransfer.into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .await
                .unwrap();
            }
            peer.encrypt_and_send(&negotiation(EventType::SafeToClosePriorChannel))
                .await
                .unwrap();
            if phase == 2 {
                control.cancel();
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
            if phase == 3 {
                control.cancel();
            }
            peer.socket = MigratableStream::Tcp(tcp_peer);
            read_cancel(&mut peer).await;
            assert_eq!(
                peer.state.client_seq, 5,
                "offer, LAST_WRITE, SAFE, two Cancel frames"
            );
            drop(peer);
        };
        // Act / Assert: timeout is only a deadlock guard; synchronization uses frames.
        tokio::time::timeout(Duration::from_secs(3), async {
            tokio::join!(worker, remote);
        })
        .await
        .unwrap();
        assert_eq!(control.snapshot().state, State::Cancelled);
        assert!(!router.has_pending().await);
        let mut terminal = Vec::new();
        while let Ok(event) = events.try_recv() {
            if let Some(state) = event.state {
                if crate::sessions::terminal(&state) {
                    terminal.push(state);
                }
            }
        }
        assert_eq!(terminal, [State::Cancelled]);
    }
}

#[tokio::test]
async fn given_encryption_disable_flags_when_full_upgrade_completes_then_cipher_and_sequence_remain_mandatory(
) {
    for disable in [None, Some(false), Some(true)] {
        // Arrange
        let (request, mut peer, control, _events) = managed_request();
        let sender = request.sender.clone();
        let router = BwuRouter::new();
        let (socket, mut tcp_peer) = tcp_pair().await;
        let worker = crate::hdl::gatt::run_inbound_worker_with_resolver(
            request,
            sender,
            router.clone(),
            4242,
            CancellationToken::new(),
            true,
            || Some([127, 0, 0, 1]),
        );
        let remote = async {
            route_candidate(&mut peer, &router, socket, &mut tcp_peer, disable).await;
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
            read_plain_frame_from(&mut peer.socket).await.unwrap();
            peer.socket = MigratableStream::Tcp(tcp_peer);
            // Authenticated keepalive tests both directions after the actual swap.
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
            let response = peer
                .read_encrypted_offline_frame()
                .await
                .unwrap()
                .v1
                .unwrap();
            assert_eq!(response.r#type(), FrameType::KeepAlive);
            assert_eq!(response.keep_alive.unwrap().ack, Some(true));
            assert_eq!(peer.state.client_seq, 4);
            // Plaintext after encrypted upgrade cannot be admitted as a payload.
            let plaintext = negotiation(EventType::UpgradeFailure).encode_to_vec();
            send_plain_frame_on(&mut peer.socket, &plaintext)
                .await
                .unwrap();
            assert!(peer.socket.read_u8().await.is_err());
        };
        // Act / Assert
        tokio::time::timeout(Duration::from_secs(3), async {
            tokio::join!(worker, remote);
        })
        .await
        .unwrap();
        assert_eq!(control.snapshot().state, State::Disconnected);
        assert!(!router.has_pending().await);
    }
}

#[tokio::test]
async fn given_service_stop_during_committed_migration_when_user_also_cancels_then_worker_drops_without_cancel_frames(
) {
    for user_cancel in [false, true] {
        let (request, mut peer, control, _events) = managed_request();
        let sender = request.sender.clone();
        let router = BwuRouter::new();
        let (socket, mut tcp_peer) = tcp_pair().await;
        let lifetime = CancellationToken::new();
        let worker = crate::hdl::gatt::run_inbound_worker_with_resolver(
            request,
            sender,
            router.clone(),
            4242,
            lifetime.clone(),
            true,
            || Some([127, 0, 0, 1]),
        );
        let remote = async {
            route_candidate(&mut peer, &router, socket, &mut tcp_peer, None).await;
            // Act
            if user_cancel {
                control.cancel();
            }
            lifetime.cancel();
            // Assert: no terminal app frame on either channel after LAST_WRITE.
            assert!(peer.socket.read_u8().await.is_err());
            assert!(tcp_peer.read_u8().await.is_err());
        };
        tokio::time::timeout(Duration::from_secs(3), async {
            tokio::join!(worker, remote);
        })
        .await
        .unwrap();
        assert_eq!(control.snapshot().state, State::Cancelled);
        assert!(!router.has_pending().await);
    }
}

#[tokio::test]
async fn given_failed_committed_writer_when_local_cancel_runs_then_no_old_transport_fallback() {
    // Arrange / Act: a failed migration has already cleared its deferred queue.
    let (mut request, mut peer) = pair();
    request.bwu_writer_unavailable = true;
    assert!(request.send_local_cancel().await.is_err());
    // Assert
    assert_eq!(request.state.server_seq, 0);
    let read = peer.socket.read_u8();
    tokio::pin!(read);
    assert!(futures::poll!(read).is_pending());
}

#[tokio::test(start_paused = true)]
async fn given_backpressured_last_write_when_committed_then_combined_phase_budget_bounds_writer() {
    // Arrange: capacity one makes the encrypted LAST_WRITE incomplete.
    let (mut request, _peer) = pair();
    let (socket, mut ble_peer) = tokio::io::duplex(1);
    request.socket = MigratableStream::Ble(socket);
    request.deferred_upgrade_writes = Some(Vec::new());
    let (tcp, mut tcp_peer) = tcp_pair().await;
    let start = tokio::time::Instant::now();
    // Act
    let error = request
        .finish_bandwidth_upgrade(tcp, Instant::now())
        .await
        .unwrap_err();
    // Assert
    assert!(error.to_string().contains("phase budgets"));
    assert_eq!(start.elapsed(), Duration::from_secs(10));
    assert!(matches!(request.socket, MigratableStream::Ble(_)));
    assert_eq!(request.state.server_seq, 1);
    assert_eq!(ble_peer.read_u8().await.unwrap(), 0);
    assert!(tcp_peer.read_u8().await.is_err());
}

#[tokio::test]
async fn given_cancel_while_reading_optional_tcp_intro_when_worker_stops_then_ble_cancel_is_complete(
) {
    // Arrange
    let (request, mut peer, control, _events) = managed_request();
    let sender = request.sender.clone();
    let router = BwuRouter::new();
    let (socket, mut tcp_peer) = tcp_pair().await;
    let worker = crate::hdl::gatt::run_inbound_worker_with_resolver(
        request,
        sender,
        router.clone(),
        4242,
        CancellationToken::new(),
        true,
        || Some([127, 0, 0, 1]),
    );
    let remote = async {
        peer.encrypt_and_send(&negotiation(EventType::UpgradePathAvailable))
            .await
            .unwrap();
        peer.read_encrypted_offline_frame().await.unwrap();
        router.route_with_alias("PEER", None, socket).await.unwrap();
        // Act: TCP has no introduction, hence no LAST_WRITE may be committed.
        control.cancel();
        read_cancel(&mut peer).await;
        assert_eq!(
            peer.state.client_seq, 3,
            "offer and two BLE Cancel frames only"
        );
        assert!(tcp_peer.read_u8().await.is_err());
        drop(peer);
    };
    tokio::time::timeout(Duration::from_secs(3), async {
        tokio::join!(worker, remote);
    })
    .await
    .unwrap();
    assert_eq!(control.snapshot().state, State::Cancelled);
    assert!(!router.has_pending().await);
}

#[tokio::test]
async fn given_local_cancel_and_prior_eof_when_committed_worker_fails_then_neither_channel_gets_false_cancel(
) {
    // Arrange
    let (request, mut peer, control, _events) = managed_request();
    let sender = request.sender.clone();
    let router = BwuRouter::new();
    let (socket, mut tcp_peer) = tcp_pair().await;
    let worker = crate::hdl::gatt::run_inbound_worker_with_resolver(
        request,
        sender,
        router.clone(),
        4242,
        CancellationToken::new(),
        true,
        || Some([127, 0, 0, 1]),
    );
    let remote = async {
        route_candidate(&mut peer, &router, socket, &mut tcp_peer, None).await;
        // Act
        control.cancel();
        drop(peer);
        // Assert: no successful swap, Cancel or plaintext downgrade on TCP.
        assert!(tcp_peer.read_u8().await.is_err());
    };
    tokio::time::timeout(Duration::from_secs(3), async {
        tokio::join!(worker, remote);
    })
    .await
    .unwrap();
    assert_eq!(control.snapshot().state, State::Cancelled);
    assert!(!router.has_pending().await);
}

#[tokio::test(start_paused = true)]
async fn given_local_cancel_and_silent_prior_peer_when_committed_then_deadline_retires_without_ble_application_write(
) {
    // Arrange: prime frames synchronously, use real worker and paused time.
    let (request, mut peer, control, _events) = managed_request();
    let sender = request.sender.clone();
    let router = BwuRouter::new();
    let (socket, mut tcp_peer) = tcp_pair().await;
    let worker = crate::hdl::gatt::run_inbound_worker_with_resolver(
        request,
        sender,
        router.clone(),
        4242,
        CancellationToken::new(),
        true,
        || Some([127, 0, 0, 1]),
    );
    let remote = async {
        route_candidate(&mut peer, &router, socket, &mut tcp_peer, None).await;
        let start = tokio::time::Instant::now();
        control.cancel();
        assert!(peer.socket.read_u8().await.is_err());
        assert_eq!(start.elapsed(), Duration::from_secs(5));
        assert!(tcp_peer.read_u8().await.is_err());
    };
    // Act / Assert: runtime advances only once both peers are waiting on I/O.
    tokio::join!(worker, remote);
    assert_eq!(control.snapshot().state, State::Cancelled);
    assert!(!router.has_pending().await);
}

use tokio::sync::{mpsc, oneshot};

#[tokio::test]
async fn given_pending_sdk_receive_when_probe_completes_then_receive_future_is_not_recreated() {
    // Arrange
    let (events, receiver) = mpsc::unbounded_channel();
    let receiver = std::sync::Arc::new(tokio::sync::Mutex::new(receiver));
    let next_calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count = next_calls.clone();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
    let token = CancellationToken::new();
    let mut cache = HashMap::new();
    let drive = async {
        events
            .send(ServiceEvent::ServiceResolved(service(
                "one",
                "192.0.2.10",
                "One",
            )))
            .unwrap();
        assert_eq!(reader.recv().await.unwrap().name.as_deref(), Some("One"));
        assert_eq!(count.load(std::sync::atomic::Ordering::SeqCst), 2);
        events
            .send(ServiceEvent::ServiceResolved(service(
                "two",
                "192.0.2.11",
                "Two",
            )))
            .unwrap();
        assert_eq!(reader.recv().await.unwrap().name.as_deref(), Some("Two"));
        assert_eq!(count.load(std::sync::atomic::Ordering::SeqCst), 3);
        token.cancel();
    };
    // Act / Assert
    let run = run_event_loop(
        &updates,
        token.clone(),
        &mut cache,
        move || {
            next_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let receiver = receiver.clone();
            async move {
                receiver
                    .lock()
                    .await
                    .recv()
                    .await
                    .ok_or_else(|| anyhow::anyhow!("closed"))
            }
        },
        |_| async { Ok(()) },
    );
    let ((), result) = tokio::join!(drive, run);
    result.unwrap();
    assert_eq!(cache.len(), 2);
}

#[tokio::test]
async fn given_invalid_replacement_when_probe_is_pending_then_old_candidate_is_dropped_without_visibility(
) {
    // Arrange
    let (events, receiver) = mpsc::unbounded_channel();
    let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
    let token = CancellationToken::new();
    let drive = async {
        events
            .send(ServiceEvent::ServiceResolved(service(
                "peer",
                "192.0.2.10",
                "Peer",
            )))
            .unwrap();
        let mut old = calls.recv().await.unwrap();
        let invalid = mdns_sd::ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            "peer",
            "fixture.local.",
            "192.0.2.10",
            4242,
            &[("n", "bad")][..],
        )
        .unwrap();
        events.send(ServiceEvent::ServiceResolved(invalid)).unwrap();
        old.result.closed().await;
        token.cancel();
    };
    // Act / Assert
    let (_, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
    assert!(cache.is_empty());
    assert!(reader.try_recv().is_err());
}

#[tokio::test]
async fn given_failed_address_when_second_succeeds_then_only_reachable_address_is_published() {
    for both_fail in [false, true] {
        // Arrange
        let (events, receiver) = mpsc::unbounded_channel();
        let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
        let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
        let token = CancellationToken::new();
        let drive = async {
            events
                .send(ServiceEvent::ServiceResolved(service(
                    "peer",
                    "192.0.2.10,192.0.2.11",
                    "Peer",
                )))
                .unwrap();
            let first = calls.recv().await.unwrap();
            let second = calls.recv().await.unwrap();
            first
                .result
                .send(Err(std::io::ErrorKind::ConnectionRefused.into()))
                .unwrap();
            if both_fail {
                second
                    .result
                    .send(Err(std::io::ErrorKind::NetworkUnreachable.into()))
                    .unwrap();
                events
                    .send(ServiceEvent::ServiceResolved(service(
                        "barrier",
                        "192.0.2.12",
                        "Barrier",
                    )))
                    .unwrap();
                calls.recv().await.unwrap().result.send(Ok(())).unwrap();
                assert_eq!(
                    reader.recv().await.unwrap().name.as_deref(),
                    Some("Barrier")
                );
            } else {
                second.result.send(Ok(())).unwrap();
                assert_eq!(reader.recv().await.unwrap().id, "192.0.2.11:4242");
            }
            token.cancel();
        };
        // Act / Assert
        let (_, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
        assert_eq!(cache.len(), 1);
        assert!(reader.try_recv().is_err());
    }
}

#[tokio::test]
async fn given_two_addresses_of_one_service_when_later_address_connects_first_then_fastest_reachable_is_selected_without_fanout(
) {
    let (events, receiver) = mpsc::unbounded_channel();
    let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
    let token = CancellationToken::new();
    let drive = async {
        events
            .send(ServiceEvent::ServiceResolved(service(
                "peer",
                "192.0.2.10,192.0.2.11",
                "One advertised device",
            )))
            .unwrap();
        let first = calls.recv().await.unwrap();
        let second = calls.recv().await.unwrap();
        let (mut slower, faster) = if first.address == "192.0.2.10:4242" {
            (first, second)
        } else {
            (second, first)
        };
        assert_eq!(slower.address, "192.0.2.10:4242");
        assert_eq!(faster.address, "192.0.2.11:4242");
        // The lexically first address is still probing. A different address
        // has demonstrated an actual successful TCP connect.
        faster.result.send(Ok(())).unwrap();
        let selected = reader.recv().await.unwrap();
        assert_eq!(selected.id, "192.0.2.11:4242");
        assert_eq!(selected.name.as_deref(), Some("One advertised device"));
        assert!(reader.try_recv().is_err(), "only one address is published");
        slower.result.closed().await;
        token.cancel();
    };
    let (_, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
    assert_eq!(cache.len(), 1);
}

#[tokio::test]
async fn given_pending_probe_when_browser_closes_then_connect_future_is_dropped() {
    // Arrange
    let (events, receiver) = mpsc::unbounded_channel();
    let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
    let drive = async {
        events
            .send(ServiceEvent::ServiceResolved(service(
                "peer",
                "192.0.2.10",
                "Peer",
            )))
            .unwrap();
        let probe = calls.recv().await.unwrap();
        drop(events);
        probe
    };
    // Act / Assert
    let (probe, cache) = tokio::join!(
        drive,
        run(receiver, probes, &updates, CancellationToken::new())
    );
    assert!(probe.result.is_closed());
    assert!(cache.is_empty());
    assert!(reader.try_recv().is_err());
}

#[tokio::test]
async fn given_resolve_then_remove_before_probe_completion_when_driven_then_no_presence_or_phantom_tombstone(
) {
    // Arrange
    let (events, receiver) = mpsc::unbounded_channel();
    let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
    let token = CancellationToken::new();
    let original = service("peer", "192.0.2.10", "Peer");
    let fullname = original.get_fullname().to_owned();
    let drive = async {
        events
            .send(ServiceEvent::ServiceResolved(original))
            .unwrap();
        let mut old = calls.recv().await.unwrap();
        events
            .send(ServiceEvent::ServiceRemoved(
                String::new(),
                fullname.clone(),
            ))
            .unwrap();
        old.result.closed().await;
        assert!(reader.try_recv().is_err());
        token.cancel();
    };
    // Act / Assert
    let (_, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
    assert!(cache.is_empty());
}

use super::*;

struct Probe {
    address: String,
    result: oneshot::Sender<std::io::Result<()>>,
}

fn service(name: &str, addresses: &str, display: &str) -> mdns_sd::ServiceInfo {
    let encoded = crate::utils::gen_mdns_endpoint_info(DeviceType::Phone as u8, display);
    mdns_sd::ServiceInfo::new(
        "_FC9F5ED42C8A._tcp.local.",
        name,
        "fixture.local.",
        addresses,
        4242,
        &[("n", encoded.as_str())][..],
    )
    .unwrap()
}

async fn run(
    events: mpsc::UnboundedReceiver<ServiceEvent>,
    probes: mpsc::UnboundedSender<Probe>,
    updates: &broadcast::Sender<EndpointInfo>,
    token: CancellationToken,
) -> HashMap<String, EndpointInfo> {
    let events = std::sync::Arc::new(tokio::sync::Mutex::new(events));
    let mut cache = HashMap::new();
    let outcome = run_event_loop(
        updates,
        token,
        &mut cache,
        move || {
            let events = events.clone();
            async move {
                events
                    .lock()
                    .await
                    .recv()
                    .await
                    .ok_or_else(|| anyhow::anyhow!("fixture closed"))
            }
        },
        move |address| {
            let (result, receiver) = oneshot::channel();
            probes.send(Probe { address, result }).unwrap();
            async move {
                receiver
                    .await
                    .unwrap_or_else(|_| Err(std::io::ErrorKind::ConnectionReset.into()))
            }
        },
    )
    .await;
    if let Err(error) = outcome {
        assert_eq!(error.to_string(), "fixture closed");
    }
    cache
}

#[tokio::test]
async fn given_slow_service_and_multiple_addresses_when_other_route_is_reachable_then_discovery_progresses(
) {
    // Arrange
    let (events, receiver) = mpsc::unbounded_channel();
    let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
    let token = CancellationToken::new();
    let drive = async {
        events
            .send(ServiceEvent::ServiceResolved(service(
                "slow",
                "192.0.2.10",
                "Slow",
            )))
            .unwrap();
        let slow = calls.recv().await.unwrap();
        events
            .send(ServiceEvent::ServiceResolved(service(
                "fast",
                "192.0.2.21,192.0.2.20",
                "Fast",
            )))
            .unwrap();
        let first = calls.recv().await.unwrap();
        let second = calls.recv().await.unwrap();
        assert_eq!(first.address, "192.0.2.20:4242");
        assert_eq!(second.address, "192.0.2.21:4242");
        second.result.send(Ok(())).unwrap();
        let found = reader.recv().await.unwrap();
        assert_eq!(found.id, "192.0.2.21:4242");
        assert_eq!(found.name.as_deref(), Some("Fast"));
        assert_eq!(found.present, Some(true));
        assert!(first.result.is_closed(), "losing address is canceled");
        assert!(
            !slow.result.is_closed(),
            "independent candidate remains owned"
        );
        token.cancel();
        slow
    };
    // Act / Assert
    let (slow, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
    assert!(slow.result.is_closed());
    assert_eq!(cache.len(), 1);
    assert!(reader.try_recv().is_err());
}

#[tokio::test]
async fn given_pending_resolve_when_removed_or_replaced_then_old_probe_cannot_publish() {
    for replacement in [false, true] {
        // Arrange
        let (events, receiver) = mpsc::unbounded_channel();
        let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
        let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
        let token = CancellationToken::new();
        let original = service("peer", "192.0.2.10", "Old");
        let fullname = original.get_fullname().to_owned();
        let drive = async {
            events
                .send(ServiceEvent::ServiceResolved(original))
                .unwrap();
            let old = calls.recv().await.unwrap();
            if !replacement {
                events
                    .send(ServiceEvent::ServiceRemoved(
                        String::new(),
                        fullname.clone(),
                    ))
                    .unwrap();
            }
            events
                .send(ServiceEvent::ServiceResolved(service(
                    "peer",
                    "192.0.2.11",
                    "New",
                )))
                .unwrap();
            let new = calls.recv().await.unwrap();
            assert!(old.result.is_closed());
            new.result.send(Ok(())).unwrap();
            let found = reader.recv().await.unwrap();
            assert_eq!(found.id, "192.0.2.11:4242");
            assert_eq!(found.name.as_deref(), Some("New"));
            token.cancel();
        };
        // Act / Assert
        let (_, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
        assert_eq!(cache.len(), 1);
        assert_eq!(cache[&fullname].id, "192.0.2.11:4242");
        assert!(reader.try_recv().is_err());
    }
}

#[tokio::test]
async fn given_duplicate_pending_resolve_when_sdk_repeats_then_probe_is_not_restarted() {
    // Arrange
    let (events, receiver) = mpsc::unbounded_channel();
    let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(16);
    let token = CancellationToken::new();
    let peer = service("peer", "192.0.2.10", "Peer");
    let drive = async {
        events
            .send(ServiceEvent::ServiceResolved(peer.clone()))
            .unwrap();
        let old = calls.recv().await.unwrap();
        events.send(ServiceEvent::ServiceResolved(peer)).unwrap();
        events
            .send(ServiceEvent::ServiceResolved(service(
                "barrier",
                "192.0.2.11",
                "Barrier",
            )))
            .unwrap();
        let barrier = calls.recv().await.unwrap();
        assert_eq!(barrier.address, "192.0.2.11:4242");
        assert!(!old.result.is_closed());
        old.result.send(Ok(())).unwrap();
        assert_eq!(reader.recv().await.unwrap().id, "192.0.2.10:4242");
        token.cancel();
    };
    // Act / Assert
    let (_, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
    assert_eq!(cache.len(), 1);
    assert!(calls.try_recv().is_err());
}

#[tokio::test(start_paused = true)]
async fn given_full_probe_capacity_when_budget_expires_then_new_candidate_can_be_admitted() {
    // Arrange
    let (events, receiver) = mpsc::unbounded_channel();
    let (probes, mut calls) = mpsc::unbounded_channel::<Probe>();
    let (updates, mut reader) = broadcast::channel::<EndpointInfo>(64);
    let token = CancellationToken::new();
    let drive = async {
        let mut pending = Vec::new();
        for index in 0..MAX_PENDING_SERVICES {
            events
                .send(ServiceEvent::ServiceResolved(service(
                    &format!("peer-{index}"),
                    "192.0.2.10",
                    "Peer",
                )))
                .unwrap();
            pending.push(calls.recv().await.unwrap());
        }
        events
            .send(ServiceEvent::ServiceResolved(service(
                "over-capacity",
                "192.0.2.12",
                "Overflow",
            )))
            .unwrap();
        tokio::time::advance(PROBE_BUDGET - Duration::from_millis(1)).await;
        assert!(pending.iter().all(|probe| !probe.result.is_closed()));
        tokio::time::advance(Duration::from_millis(1)).await;
        for probe in &mut pending {
            probe.result.closed().await;
        }
        assert!(
            calls.try_recv().is_err(),
            "overflow must not start a connect"
        );
        events
            .send(ServiceEvent::ServiceResolved(service(
                "after-expiry",
                "192.0.2.11",
                "After",
            )))
            .unwrap();
        calls.recv().await.unwrap().result.send(Ok(())).unwrap();
        assert_eq!(reader.recv().await.unwrap().id, "192.0.2.11:4242");
        token.cancel();
    };
    // Act / Assert
    let (_, cache) = tokio::join!(drive, run(receiver, probes, &updates, token.clone()));
    assert_eq!(cache.len(), 1);
}

#[test]
fn given_ipv4_sets_when_candidate_is_built_then_self_is_filtered_and_addresses_are_sorted_and_bounded(
) {
    // Arrange / Act / Assert
    for count in [1, MAX_SERVICE_ADDRESSES, MAX_SERVICE_ADDRESSES + 1] {
        let ips = (1..=count)
            .rev()
            .map(|i| format!("192.0.2.{i}"))
            .collect::<Vec<_>>()
            .join(",");
        let candidate = Candidate::from_service(&service("peer", &ips, "")).unwrap();
        assert_eq!(candidate.addresses.len(), count.min(MAX_SERVICE_ADDRESSES));
        assert!(candidate.addresses.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(candidate.addresses[0], Ipv4Addr::new(192, 0, 2, 1));
        assert_eq!(candidate.name, "fixture");
    }
    assert!(Candidate::from_service(&service("ipv6", "2001:db8::1", "Peer")).is_none());
    assert!(Candidate::from_service(&service("self", "127.0.0.1", "Peer")).is_none());
    assert_eq!(
        Candidate::from_service(&service("mixed", "127.0.0.1,192.0.2.1", "Peer"))
            .unwrap()
            .addresses,
        [Ipv4Addr::new(192, 0, 2, 1)]
    );
}

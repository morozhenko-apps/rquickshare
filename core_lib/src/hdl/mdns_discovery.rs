use std::collections::HashMap;
use std::future::Future;
use std::net::Ipv4Addr;
use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;

use futures::stream::{FuturesUnordered, StreamExt};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::utils::{is_not_self_ip, parse_mdns_endpoint_info};
use crate::DeviceType;

#[derive(Debug, Clone, Default, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct EndpointInfo {
    pub fullname: String,
    pub id: String,
    pub name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub rtype: Option<DeviceType>,
    pub present: Option<bool>,
}

// The runtime indexes discovered routes by fullname, not by the endpoint's
// transport id. Publish the same identity key on removal without exposing
// the cached name, address or port.
fn publish_withdrawal(
    sender: &broadcast::Sender<EndpointInfo>,
    fullname: String,
    endpoint: EndpointInfo,
) {
    let _ = sender.send(EndpointInfo {
        fullname,
        id: endpoint.id,
        present: Some(false),
        ..Default::default()
    });
    crate::receiver_trace::event(crate::receiver_trace::Event::MdnsWithdrawalEmitted);
}

// Unknown and already removed services have no cached route to retire.
fn withdraw_cached_endpoint(
    cache: &mut HashMap<String, EndpointInfo>,
    sender: &broadcast::Sender<EndpointInfo>,
    fullname: &str,
) -> bool {
    let Some(endpoint) = cache.remove(fullname) else {
        return false;
    };
    publish_withdrawal(sender, fullname.to_owned(), endpoint);
    true
}

// Emit one removal per previously advertised peer. Draining the cache is
// essential: a later generation may rediscover the same peer identity.
fn withdraw_cached_endpoints(
    cache: &mut HashMap<String, EndpointInfo>,
    sender: &broadcast::Sender<EndpointInfo>,
) {
    for (fullname, endpoint) in cache.drain() {
        publish_withdrawal(sender, fullname, endpoint);
    }
}

// This is a diagnostic for *untrusted advertised* endpoint IDs, not an
// authenticated device or a route selection policy. Keep every route alive.
fn unverified_hint_collision(cache: &HashMap<String, EndpointInfo>, fullname: &str) -> bool {
    let Some(hint) = crate::utils::parse_mdns_endpoint_id(fullname) else {
        return false;
    };
    cache.keys().any(|previous| {
        previous != fullname && crate::utils::parse_mdns_endpoint_id(previous) == Some(hint)
    })
}

pub struct MDnsDiscovery {
    daemon: ServiceDaemon,
    sender: broadcast::Sender<EndpointInfo>,
}

impl MDnsDiscovery {
    pub fn new(sender: broadcast::Sender<EndpointInfo>) -> Result<Self, anyhow::Error> {
        let daemon = ServiceDaemon::new()?;

        Ok(Self { daemon, sender })
    }

    pub async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        let mut cache = HashMap::new();
        let outcome = self.run_inner(ctk, &mut cache).await;

        // Withdraw endpoints before another generation can advertise them.
        withdraw_cached_endpoints(&mut cache, &self.sender);

        // Joining this worker must also join the mDNS SDK's own daemon.
        let shutdown = async {
            let completion = self.daemon.shutdown()?;
            completion.recv_async().await?;
            Ok::<(), anyhow::Error>(())
        }
        .await;
        outcome.and(shutdown)
    }

    async fn run_inner(
        &self,
        ctk: CancellationToken,
        cache: &mut HashMap<String, EndpointInfo>,
    ) -> Result<(), anyhow::Error> {
        info!("MDnsDiscovery: service starting");

        let service_type = "_FC9F5ED42C8A._tcp.local.";
        let receiver = self.daemon.browse(service_type)?;
        crate::receiver_trace::event(crate::receiver_trace::Event::MdnsBrowseStarted);

        // The SDK owns the browser; the common state machine processes both
        // real SDK events and deterministic external-port test fixtures.
        run_event_loop(
            &self.sender,
            ctk,
            cache,
            move || {
                let receiver = receiver.clone();
                async move { Ok(receiver.recv_async().await?) }
            },
            |ip_port| async move { TcpStream::connect(ip_port).await.map(|_| ()) },
        )
        .await
    }
}

// Discovery probes are speculative, never transfer or authentication deadlines.
const MAX_PENDING_SERVICES: usize = 32;
const MAX_SERVICE_ADDRESSES: usize = 16;
const PROBE_BUDGET: Duration = Duration::from_secs(5);

#[derive(Clone, PartialEq)]
struct Candidate {
    addresses: Vec<Ipv4Addr>,
    port: u16,
    name: String,
    rtype: DeviceType,
}

impl Candidate {
    fn from_service(info: &mdns_sd::ServiceInfo) -> Option<Self> {
        let mut addresses: Vec<_> = info
            .get_addresses_v4()
            .into_iter()
            .copied()
            .filter(is_not_self_ip)
            .collect();
        addresses.sort_unstable();
        addresses.truncate(MAX_SERVICE_ADDRESSES);
        if addresses.is_empty() {
            return None;
        }
        let (rtype, name) = parse_mdns_endpoint_info(info.get_property("n")?.val_str()).ok()?;
        let name = if name.is_empty() {
            info.get_hostname()
                .trim_end_matches(".local.")
                .trim_end_matches(".local")
                .to_owned()
        } else {
            name
        };
        Some(Self {
            addresses,
            port: info.get_port(),
            name,
            rtype,
        })
    }
}

struct PendingCandidate<'a> {
    candidate: Candidate,
    reachability: Pin<Box<dyn Future<Output = Option<EndpointInfo>> + Send + 'a>>,
}

fn poll_candidate(
    pending: &mut HashMap<String, PendingCandidate<'_>>,
    cx: &mut std::task::Context<'_>,
) -> Poll<(String, Option<EndpointInfo>)> {
    for (fullname, entry) in pending.iter_mut() {
        if let Poll::Ready(endpoint) = entry.reachability.as_mut().poll(cx) {
            return Poll::Ready((fullname.clone(), endpoint));
        }
    }
    Poll::Pending
}

// Only SDK events and TCP reachability are injectable. Domain admission,
// fullname cache, cancellation and withdrawal behavior remain production code.
async fn run_event_loop<Next, NextFuture, Probe, ProbeFuture>(
    sender: &broadcast::Sender<EndpointInfo>,
    ctk: CancellationToken,
    cache: &mut HashMap<String, EndpointInfo>,
    mut next: Next,
    mut probe: Probe,
) -> Result<(), anyhow::Error>
where
    Next: FnMut() -> NextFuture,
    NextFuture: std::future::Future<Output = Result<ServiceEvent, anyhow::Error>>,
    Probe: FnMut(String) -> ProbeFuture,
    ProbeFuture: std::future::Future<Output = std::io::Result<()>> + Send,
{
    let mut pending = HashMap::<String, PendingCandidate<'_>>::new();
    // Keep the current SDK receive alive across unrelated probe completions.
    let mut incoming = Box::pin(next());
    loop {
        let event = tokio::select! {
            biased;
            _ = ctk.cancelled() => break,
            result = &mut incoming => {
                incoming = Box::pin(next());
                result
            },
            (fullname, endpoint) = futures::future::poll_fn(|cx| poll_candidate(&mut pending, cx)),
                if !pending.is_empty() => {
                pending.remove(&fullname);
                if let Some(endpoint) = endpoint {
                    cache.insert(fullname, endpoint.clone());
                    let _ = sender.send(endpoint);
                }
                continue;
            },
        };
        let event = match event {
            Ok(event) => event,
            Err(err) => {
                // Closed SDK browse receivers must be terminal, not a hot loop.
                error!("MDnsDiscovery: browser channel closed: {err}");
                return Err(err);
            }
        };

        match event {
            ServiceEvent::ServiceResolved(info) => {
                let fullname = info.get_fullname().to_owned();
                let Some(candidate) = Candidate::from_service(&info) else {
                    // Invalid replacement metadata cannot admit a pending old route.
                    pending.remove(&fullname);
                    continue;
                };
                if unverified_hint_collision(cache, &fullname) {
                    debug!(
                        "MDnsDiscovery: untrusted endpoint hints overlap; preserving independent routes"
                    );
                }
                if pending
                    .get(&fullname)
                    .is_some_and(|entry| entry.candidate == candidate)
                {
                    continue;
                }
                // Drop replaced/removed futures immediately. A late old result has
                // no owner left and cannot resurrect a removed or changed service.
                pending.remove(&fullname);
                if pending.len() >= MAX_PENDING_SERVICES {
                    warn!("MDnsDiscovery: speculative probe capacity reached");
                    continue;
                }
                let attempts = FuturesUnordered::new();
                for ip in &candidate.addresses {
                    let address = format!("{ip}:{}", candidate.port);
                    let connect = probe(address.clone());
                    let ip = *ip;
                    attempts.push(async move {
                        match tokio::time::timeout(PROBE_BUDGET, connect).await {
                            Ok(Ok(())) => Some((ip, address)),
                            _ => None,
                        }
                    });
                }
                let source = candidate.clone();
                let key = fullname.clone();
                let reachability = Box::pin(async move {
                    let mut attempts = attempts;
                    while let Some(result) = attempts.next().await {
                        if let Some((ip, address)) = result {
                            return Some(EndpointInfo {
                                fullname: key,
                                id: address,
                                name: Some(source.name),
                                ip: Some(ip.to_string()),
                                port: Some(source.port.to_string()),
                                rtype: Some(source.rtype),
                                present: Some(true),
                            });
                        }
                    }
                    None
                });
                pending.insert(
                    fullname,
                    PendingCandidate {
                        candidate,
                        reachability,
                    },
                );
            }
            ServiceEvent::ServiceRemoved(_, fullname) => {
                pending.remove(&fullname);
                if withdraw_cached_endpoint(cache, sender, &fullname) {
                    info!("MDnsDiscovery: cached service removed");
                }
            }
            ServiceEvent::SearchStarted(_) | ServiceEvent::SearchStopped(_) => {}
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;

    #[test]
    fn given_same_ephemeral_hint_in_two_services_when_discovered_then_routes_remain_independent() {
        use crate::utils::gen_mdns_name;
        let one = format!("{}._FC9F5ED42C8A._tcp.local.", gen_mdns_name(*b"AB12"));
        let same = one.clone();
        let other = format!("{}._FC9F5ED42C8A._tcp.local.", gen_mdns_name(*b"CD34"));
        let mut cache = HashMap::<String, EndpointInfo>::new();
        assert!(!unverified_hint_collision(&cache, &one));
        cache.insert(one.clone(), EndpointInfo::default());
        assert!(!unverified_hint_collision(&cache, &same));
        assert!(!unverified_hint_collision(&cache, &other));
        // Different fullnames can carry the same untrusted endpoint hint.
        // The diagnostic does not remove either routing key from the cache.
        let alias = format!("{}._fc9f5ed42c8a._TCP.LOCAL.", gen_mdns_name(*b"AB12"));
        assert_ne!(alias, one);
        assert!(unverified_hint_collision(&cache, &alias));
        cache.insert(alias.clone(), EndpointInfo::default());
        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&one));
        assert!(cache.contains_key(&alias));
        assert!(!unverified_hint_collision(&cache, "not-a-nearby-service"));
        cache.remove(&one);
        assert!(!unverified_hint_collision(&cache, &alias));
    }

    fn sample_service(name: &str, encoded: &str) -> mdns_sd::ServiceInfo {
        mdns_sd::ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            name,
            "fixture.local.",
            "192.0.2.20",
            4242,
            &[("n", encoded)][..],
        )
        .unwrap()
    }

    async fn run_fixture(
        sender: &broadcast::Sender<EndpointInfo>,
        ctk: CancellationToken,
        cache: &mut HashMap<String, EndpointInfo>,
        events: Vec<ServiceEvent>,
        probe_ok: bool,
    ) -> Result<(), anyhow::Error> {
        let mut events = std::collections::VecDeque::from(events);
        run_event_loop(
            sender,
            ctk,
            cache,
            move || {
                let event = events.pop_front();
                async move { event.ok_or_else(|| anyhow::anyhow!("fixture browser closed")) }
            },
            move |_addr| async move {
                if probe_ok {
                    Ok(())
                } else {
                    Err(std::io::Error::other("fixture TCP unreachable"))
                }
            },
        )
        .await
    }

    #[tokio::test]
    async fn given_real_sdk_resolve_and_remove_events_when_driven_then_exact_route_and_tombstone_emit(
    ) {
        let (sender, mut reader) = broadcast::channel::<EndpointInfo>(16);
        let mut cache = HashMap::new();
        let info = sample_service(
            "fixture-peer",
            &crate::utils::gen_mdns_endpoint_info(DeviceType::Phone as u8, "Fixture phone"),
        );
        let fullname = info.get_fullname().to_owned();
        let (events, event_reader) = tokio::sync::mpsc::unbounded_channel();
        events.send(ServiceEvent::ServiceResolved(info)).unwrap();
        let ctk = CancellationToken::new();
        let drive = async {
            let added = reader.recv().await.unwrap();
            assert_eq!(added.present, Some(true));
            events
                .send(ServiceEvent::ServiceRemoved(
                    String::new(),
                    fullname.clone(),
                ))
                .unwrap();
            let removed = reader.recv().await.unwrap();
            assert_eq!(removed.present, Some(false));
            events
                .send(ServiceEvent::ServiceRemoved(
                    String::new(),
                    fullname.clone(),
                ))
                .unwrap();
            ctk.cancel();
            (added, removed)
        };
        let event_reader = std::sync::Arc::new(tokio::sync::Mutex::new(event_reader));
        let run = run_event_loop(
            &sender,
            ctk.clone(),
            &mut cache,
            || {
                let event_reader = event_reader.clone();
                async move {
                    event_reader
                        .lock()
                        .await
                        .recv()
                        .await
                        .ok_or_else(|| anyhow::anyhow!("closed"))
                }
            },
            |_| async { Ok(()) },
        );
        let ((added, removed), result) = tokio::join!(drive, run);
        result.unwrap();
        assert!(cache.is_empty());
        assert_eq!(added.fullname, fullname);
        assert_eq!(added.id, "192.0.2.20:4242");
        assert_eq!(added.name.as_deref(), Some("Fixture phone"));
        assert_eq!(added.ip.as_deref(), Some("192.0.2.20"));
        assert_eq!(added.port.as_deref(), Some("4242"));
        assert_eq!(added.present, Some(true));
        assert_eq!(added.rtype, Some(DeviceType::Phone));
        assert_eq!(removed.fullname, fullname);
        assert_eq!(removed.id, added.id);
        assert_eq!(removed.present, Some(false));
        assert!(removed.name.is_none());
        assert!(removed.ip.is_none());
        assert!(removed.port.is_none());
        assert!(removed.rtype.is_none());
        assert!(reader.try_recv().is_err());
    }

    #[tokio::test]
    async fn given_missing_or_bad_txt_and_unreachable_candidate_when_driven_then_no_false_peer() {
        let (sender, mut reader) = broadcast::channel::<EndpointInfo>(16);
        let mut cache = HashMap::new();
        // Build a real ServiceInfo without n=, rather than mocking its getter.
        let missing = mdns_sd::ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            "missing-txt",
            "fixture.local.",
            "192.0.2.20",
            4242,
            &[("other", "x")][..],
        )
        .unwrap();
        let result = run_fixture(
            &sender,
            CancellationToken::new(),
            &mut cache,
            vec![
                ServiceEvent::ServiceResolved(missing),
                ServiceEvent::ServiceResolved(sample_service("invalid-txt", "not-base64!!!")),
                ServiceEvent::ServiceResolved(sample_service(
                    "unreachable",
                    &crate::utils::gen_mdns_endpoint_info(DeviceType::Phone as u8, "Phone"),
                )),
            ],
            false,
        )
        .await;
        assert!(result.is_err());
        assert!(cache.is_empty());
        assert!(reader.try_recv().is_err());
    }

    #[tokio::test]
    async fn given_cancelled_discovery_before_external_event_when_driven_then_no_event_is_admitted()
    {
        let (sender, mut reader) = broadcast::channel(8);
        let mut cache = HashMap::new();
        let ctk = CancellationToken::new();
        ctk.cancel();
        let candidate = ServiceEvent::ServiceResolved(sample_service(
            "late",
            &crate::utils::gen_mdns_endpoint_info(DeviceType::Phone as u8, "Too late"),
        ));
        let mut event = Some(candidate);
        let result = run_event_loop(
            &sender,
            ctk,
            &mut cache,
            move || {
                let candidate = event.take();
                async move { Ok(candidate.expect("cancelled event must not be polled")) }
            },
            |_addr| async move { Ok(()) },
        )
        .await;
        assert!(result.is_ok());
        assert!(cache.is_empty());
        assert!(reader.try_recv().is_err());
    }

    #[tokio::test]
    async fn given_cancellation_during_pending_reachability_when_driven_then_no_route_is_published()
    {
        let (sender, mut reader) = broadcast::channel(8);
        let mut cache = HashMap::new();
        let ctk = CancellationToken::new();
        let cancellation = ctk.clone();
        let candidate = ServiceEvent::ServiceResolved(sample_service(
            "pending",
            &crate::utils::gen_mdns_endpoint_info(DeviceType::Phone as u8, "Waiting"),
        ));
        let mut event = Some(candidate);
        let result = run_event_loop(
            &sender,
            ctk,
            &mut cache,
            move || {
                let candidate = event.take();
                async move { Ok(candidate.expect("one discovery event")) }
            },
            move |_addr| {
                cancellation.cancel();
                async move { std::future::pending::<std::io::Result<()>>().await }
            },
        )
        .await;
        assert!(result.is_ok());
        assert!(cache.is_empty());
        assert!(reader.try_recv().is_err());
    }

    #[test]
    fn given_published_endpoints_when_generation_retires_then_tombstones_precede_fresh_discoveries()
    {
        let (sender, mut reader) = broadcast::channel(8);
        let mut cache = HashMap::from([
            (
                "old-a.local.".to_owned(),
                EndpointInfo {
                    id: "192.0.2.1:1024".to_owned(),
                    name: Some("old-a".to_owned()),
                    present: Some(true),
                    ..Default::default()
                },
            ),
            (
                "old-b.local.".to_owned(),
                EndpointInfo {
                    id: "192.0.2.2:2048".to_owned(),
                    name: Some("old-b".to_owned()),
                    present: Some(true),
                    ..Default::default()
                },
            ),
        ]);

        // Act: complete an old generation before a fresh one can publish.
        withdraw_cached_endpoints(&mut cache, &sender);
        assert!(cache.is_empty());
        sender
            .send(EndpointInfo {
                id: "192.0.2.3:3072".to_owned(),
                name: Some("fresh".to_owned()),
                present: Some(true),
                ..Default::default()
            })
            .unwrap();

        // Assert: first two queued events are pure removal notifications,
        // regardless of randomized HashMap traversal order.
        let mut removed = Vec::new();
        for _ in 0..2 {
            let event = reader.try_recv().unwrap();
            assert_eq!(event.present, Some(false));
            assert_eq!(event.name, None);
            assert_eq!(event.ip, None);
            assert_eq!(event.port, None);
            removed.push((event.fullname, event.id));
        }
        removed.sort();
        assert_eq!(
            removed,
            [
                ("old-a.local.".to_owned(), "192.0.2.1:1024".to_owned()),
                ("old-b.local.".to_owned(), "192.0.2.2:2048".to_owned()),
            ]
        );
        let fresh = reader.try_recv().unwrap();
        assert_eq!(fresh.id, "192.0.2.3:3072");
        assert_eq!(fresh.present, Some(true));
        assert_eq!(fresh.name.as_deref(), Some("fresh"));
        assert!(reader.try_recv().is_err());

        // A second stop must not duplicate removals.
        withdraw_cached_endpoints(&mut cache, &sender);
        assert!(reader.try_recv().is_err());
    }

    #[test]
    fn given_unknown_and_duplicate_mdns_removals_when_processed_then_only_cached_route_is_withdrawn(
    ) {
        let (sender, mut reader) = broadcast::channel(8);
        let mut cache = HashMap::from([(
            "peer.local.".to_owned(),
            EndpointInfo {
                id: "192.0.2.10:3456".to_owned(),
                fullname: "peer.local.".to_owned(),
                name: Some("Private peer name".to_owned()),
                ip: Some("192.0.2.10".to_owned()),
                port: Some("3456".to_owned()),
                present: Some(true),
                ..Default::default()
            },
        )]);

        assert!(!withdraw_cached_endpoint(
            &mut cache,
            &sender,
            "other.local."
        ));
        assert!(withdraw_cached_endpoint(&mut cache, &sender, "peer.local."));
        assert!(!withdraw_cached_endpoint(
            &mut cache,
            &sender,
            "peer.local."
        ));
        assert!(cache.is_empty());
        let event = reader.try_recv().unwrap();
        assert_eq!(event.fullname, "peer.local.");
        assert_eq!(event.id, "192.0.2.10:3456");
        assert_eq!(event.present, Some(false));
        assert!(event.name.is_none());
        assert!(event.ip.is_none());
        assert!(event.port.is_none());
        assert!(event.rtype.is_none());
        assert!(reader.try_recv().is_err());
    }
}

#[cfg(test)]
mod probe_tests;

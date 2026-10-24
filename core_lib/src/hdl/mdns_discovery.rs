use std::collections::HashMap;

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

        loop {
            tokio::select! {
                biased;
                _ = ctk.cancelled() => {
                    info!("MDnsDiscovery: tracker cancelled, breaking");
                    break;
                }
                r = receiver.recv_async() => {
                    match r {
                        Ok(event) => {
                            match event {
                                ServiceEvent::ServiceResolved(info) => {
                                    let port = info.get_port();

                                    let ip_hash = info.get_addresses_v4();
                                    if ip_hash.is_empty() {
                                        debug!(
                                            "MDnsDiscovery: dropping {} because it has no IPv4 address",
                                            info.get_fullname()
                                        );
                                        continue;
                                    }

                                    let ip = match ip_hash.iter().next() {
                                        Some(i) => i,
                                        None => continue,
                                    };

                                    // Check that the IP is not a "self IP"
                                    if !is_not_self_ip(ip) {
                                        trace!(
                                            "MDnsDiscovery: ignoring self service {} at {}",
                                            info.get_fullname(),
                                            ip
                                        );
                                        continue;
                                    }

                                    // Decode the "n" text properties
                                    let n = match info.get_property("n") {
                                        Some(n) => n,
                                        None => {
                                            warn!(
                                                "MDnsDiscovery: dropping {} at {}:{} because TXT n= is missing",
                                                info.get_fullname(),
                                                ip,
                                                port
                                            );
                                            continue;
                                        }
                                    };

                                    info!(
                                        "MDnsDiscovery: candidate fullname={} hostname={} ip={} port={} n_encoded_len={}",
                                        info.get_fullname(),
                                        info.get_hostname(),
                                        ip,
                                        port,
                                        n.val_str().len()
                                    );

                                    // Parse the endpoint info
                                    let (dt, dn) = match parse_mdns_endpoint_info(n.val_str()) {
                                        Ok(r) => r,
                                        Err(err) => {
                                            warn!(
                                                "MDnsDiscovery: dropping fullname={} hostname={} ip={} port={} because endpoint info parsing failed: {}",
                                                info.get_fullname(),
                                                info.get_hostname(),
                                                ip,
                                                port,
                                                err
                                            );
                                            continue;
                                        }
                                    };

                                    let display_name = if dn.is_empty() {
                                        info.get_hostname()
                                            .trim_end_matches(".local.")
                                            .trim_end_matches(".local")
                                            .to_string()
                                    } else {
                                        dn
                                    };

                                    let ip_port = format!("{ip}:{port}");
                                    let fullname = info.get_fullname().to_string();
                                    // An unreachable peer must not hold a stopped generation
                    // alive until the operating system TCP timeout expires.
                    let reachability = tokio::select! {
                        biased;
                        _ = ctk.cancelled() => break,
                        result = TcpStream::connect(&ip_port) => result,
                    };
                    if ctk.is_cancelled() {
                        break;
                    }
                    match reachability {
                                        Ok(_) => {
                                            let ei = EndpointInfo {
                                                fullname: fullname.clone(),
                                                id: ip_port,
                                                name: Some(display_name),
                                                ip: Some(ip.to_string()),
                                                port: Some(port.to_string()),
                                                rtype: Some(dt),
                                                present: Some(true),
                                            };
                                            info!("ServiceResolved: Resolved a new service: {:?}", ei);
                                            cache.insert(fullname.clone(), ei.clone());
                                            let _ = self.sender.send(ei);
                                        }
                                        Err(err) => {
                                            warn!(
                                                "MDnsDiscovery: service {} resolved at {} but TCP reachability check failed: {}",
                                                fullname,
                                                ip_port,
                                                err
                                            );
                                        }
                                    }
                                }
                                ServiceEvent::ServiceRemoved(_, fullname) => {
                                    trace!("ServiceRemoved: checking if should remove {}", fullname);
                                    if withdraw_cached_endpoint(cache, &self.sender, &fullname) {
                                        info!("MDnsDiscovery: cached service removed");
                                    }
                                }
                                ServiceEvent::SearchStarted(_) | ServiceEvent::SearchStopped(_) => {}
                                _ => {}
                            }
                        },
                        Err(err) => {
                            // A closed SDK browser is terminal. Re-reading an
                            // already closed receiver would spin and flood logs.
                            // run() still retires cached peers and awaits SDK shutdown.
                            error!("MDnsDiscovery: browser channel closed: {err}");
                            return Err(err.into());
                        },
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;

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

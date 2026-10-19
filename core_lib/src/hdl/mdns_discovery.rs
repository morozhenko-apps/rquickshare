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
        for endpoint in cache.values() {
            let _ = self.sender.send(EndpointInfo {
                id: endpoint.id.clone(),
                ..Default::default()
            });
        }

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
                                    let should_remove = cache.get(&fullname).map(|ei| ei.id.clone());

                                    if let Some(id) = should_remove {
                                        info!("ServiceRemoved: Remove a previous service: {}", fullname);
                                        cache.remove(&fullname);
                                        let _ = self.sender.send(EndpointInfo {
                                            id,
                                            ..Default::default()
                                        });
                                    }
                                }
                                ServiceEvent::SearchStarted(_) | ServiceEvent::SearchStopped(_) => {}
                                _ => {}
                            }
                        },
                        Err(err) => error!("MDnsDiscovery: error: {}", err),
                    }
                }
            }
        }

        Ok(())
    }
}

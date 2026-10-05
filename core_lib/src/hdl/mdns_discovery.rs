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
        info!("MDnsDiscovery: service starting");

        let service_type = "_FC9F5ED42C8A._tcp.local.";
        let receiver = self.daemon.browse(service_type)?;

        // Map with fullname as key and EndpointInfo as value
        let mut cache: HashMap<String, EndpointInfo> = HashMap::new();

        loop {
            tokio::select! {
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

                                    let ip_port = format!("{ip}:{port}");
                                    let fullname = info.get_fullname().to_string();
                                    match TcpStream::connect(&ip_port).await {
                                        Ok(_) => {
                                            let ei = EndpointInfo {
                                                fullname: fullname.clone(),
                                                id: ip_port,
                                                name: Some(dn),
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

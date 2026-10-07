use anyhow::{anyhow, Context as _};
use bluer::monitor::{Monitor, MonitorEvent, Pattern, RssiSamplingPeriod};
use futures::stream::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

const INNER_NAME: &str = "PassiveBleMonitor";
const AD_TYPE_SERVICE_DATA_16_BIT: u8 = 0x16;
const QUICK_SHARE_UUID_LE: [u8; 2] = [0x2c, 0xfe];

pub(super) struct PassiveBleMonitor {
    sender: Sender<()>,
}

impl PassiveBleMonitor {
    pub(super) fn new(sender: Sender<()>) -> Self {
        Self { sender }
    }

    fn quick_share_pattern() -> Pattern {
        Pattern {
            data_type: AD_TYPE_SERVICE_DATA_16_BIT,
            start_position: 0,
            content: QUICK_SHARE_UUID_LE.to_vec(),
        }
    }

    pub(super) async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        let session = bluer::Session::new()
            .await
            .context("failed to create BlueZ session for passive BLE monitor")?;
        let adapter = session
            .default_adapter()
            .await
            .context("failed to get default adapter for passive BLE monitor")?;
        adapter
            .set_powered(true)
            .await
            .context("failed to power Bluetooth adapter for passive BLE monitor")?;

        let monitor_manager = adapter
            .monitor()
            .await
            .context("BlueZ Advertisement Monitor is unavailable")?;
        let mut monitor_handle = monitor_manager
            .register(Monitor {
                monitor_type: bluer::monitor::Type::OrPatterns,
                rssi_low_threshold: None,
                rssi_high_threshold: None,
                rssi_low_timeout: None,
                rssi_high_timeout: None,
                rssi_sampling_period: Some(RssiSamplingPeriod::First),
                patterns: Some(vec![Self::quick_share_pattern()]),
                ..Default::default()
            })
            .await
            .context("failed to register Quick Share FE2C Advertisement Monitor")?;

        info!(
            "{INNER_NAME}: FE2C advertisement monitor active on {}",
            adapter.name()
        );

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: cancelled");
                    return Ok(());
                }
                event = monitor_handle.next() => {
                    match event {
                        Some(MonitorEvent::DeviceFound(_)) => {
                            debug!("{INNER_NAME}: matched Quick Share FE2C advertisement");
                            let _ = self.sender.send(());
                        }
                        Some(_) => {}
                        None => {
                            return Err(anyhow!(
                                "BlueZ Advertisement Monitor event stream ended unexpectedly"
                            ));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_share_pattern_targets_fe2c_service_data() {
        let pattern = PassiveBleMonitor::quick_share_pattern();

        assert_eq!(pattern.data_type, AD_TYPE_SERVICE_DATA_16_BIT);
        assert_eq!(pattern.start_position, 0);
        assert_eq!(pattern.content, QUICK_SHARE_UUID_LE);
    }
}

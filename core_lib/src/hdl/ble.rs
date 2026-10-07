use std::pin::Pin;
use std::time::Duration;

use anyhow::{anyhow, Context as _};
#[cfg(target_os = "linux")]
use bluer::monitor::{Monitor, MonitorEvent, Pattern, RssiSamplingPeriod};
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use tokio::sync::{broadcast::Sender, watch};
use tokio::time::{sleep, Instant, Sleep};
use tokio_util::sync::CancellationToken;
use uuid::{uuid, Uuid};

const SERVICE_UUID_SHARING: Uuid = uuid!("0000fe2c-0000-1000-8000-00805f9b34fb");

const INNER_NAME: &str = "BleListener";
const DUTY_CYCLE_SCAN: Duration = Duration::from_secs(5);
const DUTY_CYCLE_IDLE: Duration = Duration::from_secs(25);
const ALERT_COOLDOWN: Duration = Duration::from_secs(30);

#[cfg(target_os = "linux")]
const AD_TYPE_SERVICE_DATA_16_BIT: u8 = 0x16;
#[cfg(target_os = "linux")]
const QUICK_SHARE_UUID_LE: [u8; 2] = [0x2c, 0xfe];

pub struct BleListener {
    sender: Sender<()>,
}

impl BleListener {
    pub async fn new(sender: Sender<()>) -> Result<Self, anyhow::Error> {
        Ok(Self { sender })
    }

    fn should_duty_cycle(foreground: bool) -> bool {
        cfg!(target_os = "linux") && !foreground
    }

    #[cfg(target_os = "linux")]
    fn quick_share_monitor_pattern() -> Pattern {
        Pattern {
            data_type: AD_TYPE_SERVICE_DATA_16_BIT,
            start_position: 0,
            content: QUICK_SHARE_UUID_LE.to_vec(),
        }
    }

    #[cfg(target_os = "linux")]
    async fn run_passive_monitor(&self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
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
                patterns: Some(vec![Self::quick_share_monitor_pattern()]),
                ..Default::default()
            })
            .await
            .context("failed to register Quick Share FE2C Advertisement Monitor")?;

        info!(
            "{INNER_NAME}: passive FE2C advertisement monitor active on {}",
            adapter.name()
        );

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: passive monitor cancelled");
                    return Ok(());
                }
                event = monitor_handle.next() => {
                    match event {
                        Some(MonitorEvent::DeviceFound(device)) => {
                            debug!(
                                "{INNER_NAME}: passive monitor matched Quick Share device {:?}",
                                device
                            );
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

    async fn active_adapter() -> Result<Adapter, anyhow::Error> {
        let manager = Manager::new().await?;
        manager
            .adapters()
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("no bluetooth adapter"))
    }

    async fn start_scan(
        adapter: &Adapter,
        scanning: &mut bool,
    ) -> Result<(), anyhow::Error> {
        if !*scanning {
            adapter
                .start_scan(ScanFilter {
                    services: vec![SERVICE_UUID_SHARING],
                })
                .await?;
            *scanning = true;
            debug!("{INNER_NAME}: active fallback scan started");
        }

        Ok(())
    }

    async fn stop_scan(
        adapter: &Adapter,
        scanning: &mut bool,
    ) -> Result<(), anyhow::Error> {
        if *scanning {
            adapter.stop_scan().await?;
            *scanning = false;
            debug!("{INNER_NAME}: active fallback scan stopped");
        }

        Ok(())
    }

    async fn apply_mode(
        adapter: &Adapter,
        scanning: &mut bool,
        foreground: bool,
        mut duty_timer: Pin<&mut Sleep>,
    ) -> Result<(), anyhow::Error> {
        Self::start_scan(adapter, scanning).await?;

        if Self::should_duty_cycle(foreground) {
            duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_SCAN);
        }

        Ok(())
    }

    async fn run_active_scan(
        &self,
        ctk: CancellationToken,
        mut foreground_rx: watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error> {
        let adapter = Self::active_adapter()
            .await
            .context("failed to initialize active BLE discovery fallback")?;
        let mut events = adapter
            .events()
            .await
            .context("failed to subscribe to active BLE discovery events")?;
        let mut scanning = false;
        let mut foreground = *foreground_rx.borrow_and_update();
        let mut last_alert: Option<Instant> = None;

        let duty_timer = sleep(Duration::ZERO);
        tokio::pin!(duty_timer);

        info!("{INNER_NAME}: using active BLE discovery fallback");
        Self::apply_mode(&adapter, &mut scanning, foreground, duty_timer.as_mut()).await?;

        loop {
            let duty = Self::should_duty_cycle(foreground);

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: active fallback cancelled");
                    break;
                }
                changed = foreground_rx.changed() => {
                    if changed.is_err() {
                        debug!("{INNER_NAME}: foreground channel closed, stopping active fallback");
                        break;
                    }

                    foreground = *foreground_rx.borrow_and_update();
                    debug!("{INNER_NAME}: foreground changed: {foreground}");
                    Self::apply_mode(
                        &adapter,
                        &mut scanning,
                        foreground,
                        duty_timer.as_mut(),
                    )
                    .await?;
                }
                _ = &mut duty_timer, if duty => {
                    if scanning {
                        Self::stop_scan(&adapter, &mut scanning).await?;
                        duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_IDLE);
                    } else {
                        Self::start_scan(&adapter, &mut scanning).await?;
                        duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_SCAN);
                    }
                }
                event = events.next() => {
                    let Some(event) = event else {
                        return Err(anyhow!("active BLE discovery event stream ended unexpectedly"));
                    };

                    if !scanning {
                        continue;
                    }

                    if let CentralEvent::ServiceDataAdvertisement { id, service_data } = event {
                        if !service_data.contains_key(&SERVICE_UUID_SHARING) {
                            continue;
                        }

                        let now = Instant::now();
                        if last_alert.is_some_and(|last| now.duration_since(last) <= ALERT_COOLDOWN) {
                            continue;
                        }

                        debug!(
                            "{INNER_NAME}: active fallback found sharing device ({id}) ({service_data:?})"
                        );
                        let _ = self.sender.send(());
                        last_alert = Some(now);
                    }
                }
            }
        }

        if scanning {
            if let Err(error) = adapter.stop_scan().await {
                warn!("{INNER_NAME}: failed to stop active fallback scan during shutdown: {error}");
            }
        }

        Ok(())
    }

    pub async fn run(
        self,
        ctk: CancellationToken,
        foreground_rx: watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        #[cfg(target_os = "linux")]
        match self.run_passive_monitor(ctk.clone()).await {
            Ok(()) => return Ok(()),
            Err(error) => {
                warn!(
                    "{INNER_NAME}: passive FE2C monitor stopped ({error:#}); falling back to active discovery"
                );
            }
        }

        self.run_active_scan(ctk, foreground_rx).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duty_cycles_only_when_backgrounded_on_linux() {
        assert!(!BleListener::should_duty_cycle(true));
        assert_eq!(
            BleListener::should_duty_cycle(false),
            cfg!(target_os = "linux")
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn passive_monitor_targets_fe2c_service_data() {
        let pattern = BleListener::quick_share_monitor_pattern();

        assert_eq!(pattern.data_type, AD_TYPE_SERVICE_DATA_16_BIT);
        assert_eq!(pattern.start_position, 0);
        assert_eq!(pattern.content, QUICK_SHARE_UUID_LE);
    }
}

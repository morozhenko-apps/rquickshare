use std::pin::Pin;
use std::time::Duration;

use anyhow::anyhow;
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

pub struct BleListener {
    adapter: Adapter,
    sender: Sender<()>,
}

impl BleListener {
    pub async fn new(sender: Sender<()>) -> Result<Self, anyhow::Error> {
        let manager = Manager::new().await?;
        let adapter = manager
            .adapters()
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("no bluetooth adapter"))?;

        Ok(Self { adapter, sender })
    }

    fn should_duty_cycle(foreground: bool) -> bool {
        cfg!(target_os = "linux") && !foreground
    }

    async fn start_scan(&self, scanning: &mut bool) -> Result<(), anyhow::Error> {
        if !*scanning {
            self.adapter
                .start_scan(ScanFilter {
                    services: vec![SERVICE_UUID_SHARING],
                })
                .await?;
            *scanning = true;
            debug!("{INNER_NAME}: scan started");
        }

        Ok(())
    }

    async fn stop_scan(&self, scanning: &mut bool) -> Result<(), anyhow::Error> {
        if *scanning {
            self.adapter.stop_scan().await?;
            *scanning = false;
            debug!("{INNER_NAME}: scan stopped");
        }

        Ok(())
    }

    async fn apply_mode(
        &self,
        scanning: &mut bool,
        foreground: bool,
        mut duty_timer: Pin<&mut Sleep>,
    ) -> Result<(), anyhow::Error> {
        self.start_scan(scanning).await?;

        if Self::should_duty_cycle(foreground) {
            duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_SCAN);
        }

        Ok(())
    }

    pub async fn run(
        self,
        ctk: CancellationToken,
        mut foreground_rx: watch::Receiver<bool>,
    ) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        let mut events = self.adapter.events().await?;
        let mut scanning = false;
        let mut foreground = *foreground_rx.borrow_and_update();
        let mut last_alert: Option<Instant> = None;

        let duty_timer = sleep(Duration::ZERO);
        tokio::pin!(duty_timer);

        self.apply_mode(&mut scanning, foreground, duty_timer.as_mut())
            .await?;

        loop {
            let duty = Self::should_duty_cycle(foreground);

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                changed = foreground_rx.changed() => {
                    if changed.is_err() {
                        debug!("{INNER_NAME}: foreground channel closed, stopping listener");
                        break;
                    }

                    foreground = *foreground_rx.borrow_and_update();
                    debug!("{INNER_NAME}: foreground changed: {foreground}");
                    self.apply_mode(&mut scanning, foreground, duty_timer.as_mut()).await?;
                }
                _ = &mut duty_timer, if duty => {
                    if scanning {
                        self.stop_scan(&mut scanning).await?;
                        duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_IDLE);
                    } else {
                        self.start_scan(&mut scanning).await?;
                        duty_timer.as_mut().reset(Instant::now() + DUTY_CYCLE_SCAN);
                    }
                }
                Some(event) = events.next() => {
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

                        debug!("{INNER_NAME}: a device ({id}) is sharing ({service_data:?}) nearby");
                        let _ = self.sender.send(());
                        last_alert = Some(now);
                    }
                }
            }
        }

        if scanning {
            let _ = self.adapter.stop_scan().await;
        }

        Ok(())
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
}

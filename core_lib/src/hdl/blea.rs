use std::sync::Arc;

use bluer::adv::{Advertisement, AdvertisementHandle};
use bluer::UuidExt;
use bytes::Bytes;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::hdl::mdns::Visibility;

const SERVICE_DATA: Bytes = Bytes::from_static(&[
    252, 18, 142, 1, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 45, 91, 160, 225, 216, 117, 36, 202, 0,
]);

const INNER_NAME: &str = "BleAdvertiser";

#[derive(Debug, Clone)]
pub struct BleAdvertiser {
    adapter: Arc<bluer::Adapter>,
    visibility_receiver: watch::Receiver<Visibility>,
}

impl BleAdvertiser {
    pub async fn new(
        visibility_receiver: watch::Receiver<Visibility>,
    ) -> Result<Self, anyhow::Error> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;

        Ok(Self {
            adapter: Arc::new(adapter),
            visibility_receiver,
        })
    }

    fn should_advertise(visibility: Visibility) -> bool {
        matches!(visibility, Visibility::Visible | Visibility::Temporarily)
    }

    pub async fn run(mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!(
            "{INNER_NAME}: using Bluetooth adapter {} with address {}",
            self.adapter.name(),
            self.adapter.address().await?
        );

        let service_uuid = Uuid::from_u16(0xFE2C);
        let mut handle: Option<AdvertisementHandle> = None;
        let mut visibility = *self.visibility_receiver.borrow_and_update();

        if Self::should_advertise(visibility) {
            handle = Some(
                self.adapter
                    .advertise(self.get_advertisement(service_uuid, SERVICE_DATA))
                    .await?,
            );
            info!("{INNER_NAME}: started advertising");
        }

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, returning");
                    break;
                }
                changed = self.visibility_receiver.changed() => {
                    if changed.is_err() {
                        debug!("{INNER_NAME}: visibility channel closed, stopping advertiser");
                        break;
                    }

                    visibility = *self.visibility_receiver.borrow_and_update();
                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");

                    if Self::should_advertise(visibility) {
                        if handle.is_none() {
                            handle = Some(
                                self.adapter
                                    .advertise(self.get_advertisement(service_uuid, SERVICE_DATA))
                                    .await?,
                            );
                            info!("{INNER_NAME}: started advertising");
                        }
                    } else if handle.take().is_some() {
                        info!("{INNER_NAME}: stopped advertising");
                    }
                }
            }
        }

        drop(handle);
        Ok(())
    }

    fn get_advertisement(&self, service_uuid: Uuid, adv_data: Bytes) -> Advertisement {
        Advertisement {
            advertisement_type: bluer::adv::Type::Broadcast,
            service_data: [(service_uuid, adv_data.into())].into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advertises_only_when_discoverable() {
        assert!(BleAdvertiser::should_advertise(Visibility::Visible));
        assert!(BleAdvertiser::should_advertise(Visibility::Temporarily));
        assert!(!BleAdvertiser::should_advertise(Visibility::Invisible));
    }

    #[test]
    fn receiver_advertisement_has_valid_full_layout() {
        let endpoint = [0x11, 0x22, 0x33, 0x44];
        let data = receiver_service_data(endpoint, 3, "Alcotester");

        assert_eq!(data[0], 0x48);
        assert_eq!(&data[1..4], &QS_SVC_HASH);

        let connection_len =
            u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
        let connection_end = 8 + connection_len;
        assert_eq!(data.len(), connection_end + 2);

        let connection = &data[8..connection_end];
        assert_eq!(connection[0], 0x23);
        assert_eq!(&connection[1..4], &QS_SVC_HASH);
        assert_eq!(&connection[4..8], &endpoint);

        let info_len = usize::from(connection[8]);
        let info_end = 9 + info_len;
        let endpoint_info = &connection[9..info_end];

        assert_eq!(endpoint_info[0], (1 << 5) | (3 << 1));
        let name_len = usize::from(endpoint_info[17]);
        assert_eq!(&endpoint_info[18..18 + name_len], b"Alcotester");
        assert_eq!(&connection[info_end..info_end + 8], &[0_u8; 8]);
    }

    #[test]
    fn receiver_advertisement_truncates_long_names() {
        let data = receiver_service_data([1, 2, 3, 4], 3, &"x".repeat(300));

        // The endpoint info name-length byte must remain representable as u8.
        assert!(data.windows(255).any(|window| window == vec![b'x'; 255]));
        assert!(!data.windows(256).any(|window| window == vec![b'x'; 256]));
    }
}

// Quick Share receiver discovery over BLE (service UUID 0xFEF3).
const RX_INNER_NAME: &str = "ReceiverAdvertiser";
const QS_SERVICE_UUID: u16 = 0xFEF3;
const QS_SVC_HASH: [u8; 3] = [0xfc, 0x9f, 0x5e];

/// Build the Nearby Connections receiver advertisement carried as 0xFEF3
/// service data. The endpoint id must match the id used by mDNS.
pub fn receiver_service_data(endpoint_id: [u8; 4], device_type: u8, device_name: &str) -> Vec<u8> {
    let mut endpoint_info = Vec::new();
    endpoint_info.push((1 << 5) | ((device_type & 0x7) << 1));

    let identity: [u8; 16] = rand::random();
    endpoint_info.extend_from_slice(&identity);

    let name = device_name.as_bytes();
    let name_len = name.len().min(255);
    endpoint_info.push(name_len as u8);
    endpoint_info.extend_from_slice(&name[..name_len]);

    let mut connection_advertisement = Vec::new();
    connection_advertisement.push(0x23);
    connection_advertisement.extend_from_slice(&QS_SVC_HASH);
    connection_advertisement.extend_from_slice(&endpoint_id);
    connection_advertisement.push(endpoint_info.len() as u8);
    connection_advertisement.extend_from_slice(&endpoint_info);

    // Reserved Bluetooth MAC, UWB-address length and extra-field byte.
    connection_advertisement.extend_from_slice(&[0_u8; 8]);

    let mut service_data = Vec::new();
    service_data.push(0x48);
    service_data.extend_from_slice(&QS_SVC_HASH);
    service_data.extend_from_slice(&(connection_advertisement.len() as u32).to_be_bytes());
    service_data.extend_from_slice(&connection_advertisement);

    let device_token: [u8; 2] = rand::random();
    service_data.extend_from_slice(&device_token);
    service_data
}

#[derive(Debug, Clone)]
pub struct ReceiverAdvertiser {
    adapter: Arc<bluer::Adapter>,
    service_data: Vec<u8>,
    visibility_receiver: watch::Receiver<Visibility>,
}

impl ReceiverAdvertiser {
    pub async fn new(
        endpoint_id: [u8; 4],
        device_type: u8,
        device_name: &str,
        visibility_receiver: watch::Receiver<Visibility>,
    ) -> Result<Self, anyhow::Error> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;

        Ok(Self {
            adapter: Arc::new(adapter),
            service_data: receiver_service_data(endpoint_id, device_type, device_name),
            visibility_receiver,
        })
    }

    pub async fn run(mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        let service_uuid = Uuid::from_u16(QS_SERVICE_UUID);
        let mut handle: Option<AdvertisementHandle> = None;

        info!(
            "{RX_INNER_NAME}: prepared Quick Share receiver advertisement on {} ({}) ({} bytes served via GATT slot 0)",
            self.adapter.name(),
            self.adapter.address().await?,
            self.service_data.len()
        );

        loop {
            let visibility = *self.visibility_receiver.borrow_and_update();
            if BleAdvertiser::should_advertise(visibility) && handle.is_none() {
                // Keep the on-air packet legacy-sized: advertise only the 0xFEF3
                // service UUID and serve the full Nearby receiver advertisement
                // through GATT slot 0. Putting the full service data on air can
                // force BlueZ into Extended Advertising, which some Android
                // scanners and kernel/adapter combinations do not discover reliably.
                let advertisement = Advertisement {
                    advertisement_type: bluer::adv::Type::Peripheral,
                    service_uuids: [service_uuid].into(),
                    discoverable: Some(true),
                    min_interval: Some(std::time::Duration::from_millis(100)),
                    max_interval: Some(std::time::Duration::from_millis(150)),
                    ..Default::default()
                };
                match self.adapter.advertise(advertisement).await {
                    Ok(advertisement_handle) => {
                        handle = Some(advertisement_handle);
                        info!("{RX_INNER_NAME}: started advertising");
                    }
                    Err(error) => {
                        warn!("{RX_INNER_NAME}: advertise failed ({error}); retrying");
                        tokio::select! {
                            _ = ctk.cancelled() => break,
                            _ = tokio::time::sleep(std::time::Duration::from_secs(3)) => {}
                        }
                        continue;
                    }
                }
            } else if !BleAdvertiser::should_advertise(visibility) && handle.take().is_some() {
                info!("{RX_INNER_NAME}: stopped advertising");
            }

            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{RX_INNER_NAME}: tracker cancelled, returning");
                    break;
                }
                changed = self.visibility_receiver.changed() => {
                    if changed.is_err() {
                        debug!("{RX_INNER_NAME}: visibility channel closed, stopping advertiser");
                        break;
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(30)), if handle.is_some() => {
                    // BlueZ consumes connectable advertising sets on some adapters.
                    // Re-register periodically so repeated transfers stay discoverable.
                    handle.take();
                }
            }
        }

        drop(handle);
        Ok(())
    }
}

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};

/// Routes a Wi-Fi bandwidth-upgrade TCP connection from the application's
/// primary listener to the BLE session that requested it.
#[derive(Clone, Default)]
pub struct BwuRouter {
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<TcpStream>>>>,
}

impl BwuRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register(
        &self,
        endpoint_id: String,
    ) -> Result<oneshot::Receiver<TcpStream>, anyhow::Error> {
        if endpoint_id.is_empty() {
            return Err(anyhow!("cannot register an empty BWU endpoint id"));
        }

        let (sender, receiver) = oneshot::channel();
        let mut pending = self.pending.lock().await;
        if pending.contains_key(&endpoint_id) {
            return Err(anyhow!(
                "bandwidth upgrade is already pending for endpoint {endpoint_id}"
            ));
        }
        pending.insert(endpoint_id, sender);
        Ok(receiver)
    }

    pub async fn route(&self, endpoint_id: &str, socket: TcpStream) -> Result<(), TcpStream> {
        let sender = self.pending.lock().await.remove(endpoint_id);
        match sender {
            Some(sender) => sender.send(socket),
            None => Err(socket),
        }
    }

    pub async fn cancel(&self, endpoint_id: &str) {
        self.pending.lock().await.remove(endpoint_id);
    }

    pub async fn has_pending(&self) -> bool {
        !self.pending.lock().await.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn registration_is_unique_and_cancellable() {
        let router = BwuRouter::new();

        let receiver = router.register("peer-1234".to_owned()).await.unwrap();
        assert!(router.has_pending().await);
        assert!(router.register("peer-1234".to_owned()).await.is_err());

        router.cancel("peer-1234").await;
        assert!(!router.has_pending().await);
        drop(receiver);

        assert!(router.register("peer-1234".to_owned()).await.is_ok());
    }
}

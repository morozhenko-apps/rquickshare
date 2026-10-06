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
    async fn route_delivers_the_registered_socket() {
        let router = BwuRouter::new();
        let receiver = router.register("peer-route".to_owned()).await.unwrap();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let client = tokio::net::TcpStream::connect(address).await.unwrap();
        let (server, _) = listener.accept().await.unwrap();

        router.route("peer-route", server).await.unwrap();
        let pending_after_route = router.has_pending().await;
        assert!(
            !pending_after_route,
            "successful BWU routing must consume the pending endpoint"
        );

        let delivered = tokio::time::timeout(std::time::Duration::from_millis(250), receiver)
            .await
            .expect("BWU route did not deliver the socket")
            .expect("BWU route sender dropped without a socket");

        assert_eq!(delivered.local_addr().unwrap(), client.peer_addr().unwrap());
    }

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

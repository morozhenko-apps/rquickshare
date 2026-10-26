use std::sync::{Arc, Mutex};

use bluer::gatt::local::ReqError;
use bluer::Address;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const CAPACITY: usize = 64;

#[derive(Clone, PartialEq, Eq)]
struct Peer {
    adapter: String,
    address: Address,
}

/// Transport ownership only. This address never authenticates or trusts a peer.
pub(super) struct Ingress {
    owner: Option<Peer>,
    sender: Sender<Vec<u8>>,
    receiver: Option<Receiver<Vec<u8>>>,
}

pub(super) type Shared = Arc<Mutex<Ingress>>;

impl Ingress {
    fn empty() -> Self {
        let (sender, receiver) = channel(CAPACITY);
        Self {
            owner: None,
            sender,
            receiver: Some(receiver),
        }
    }

    pub(super) fn shared() -> Shared {
        Arc::new(Mutex::new(Self::empty()))
    }

    pub(super) fn write(
        shared: &Shared,
        adapter: String,
        address: Address,
        offset: u16,
        value: Vec<u8>,
    ) -> Result<(), ReqError> {
        if offset != 0 {
            return Err(ReqError::InvalidOffset);
        }
        if value.is_empty() || value.len() > usize::from(super::MAX_WEAVE_PACKET_SIZE) {
            return Err(ReqError::InvalidValueLength);
        }
        let mut ingress = shared.lock().map_err(|_| ReqError::Failed)?;
        let peer = Peer { adapter, address };
        match &ingress.owner {
            Some(owner) if *owner != peer => return Err(ReqError::InProgress),
            None if super::parse_connection_request(&value).is_err() => {
                return Err(ReqError::Failed)
            }
            _ => {}
        }
        ingress
            .sender
            .try_send(value)
            .map_err(|_| ReqError::Failed)?;
        ingress.owner = Some(peer);
        Ok(())
    }
}

/// Drop resets both owner and queue, including task cancellation/unwind paths.
pub(super) struct Session {
    shared: Shared,
    pub(super) receiver: Receiver<Vec<u8>>,
}

impl Session {
    pub(super) fn open(shared: Shared) -> Result<Self, ReqError> {
        let receiver = shared
            .lock()
            .map_err(|_| ReqError::Failed)?
            .receiver
            .take()
            .ok_or(ReqError::InProgress)?;
        Ok(Self { shared, receiver })
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if let Ok(mut ingress) = self.shared.lock() {
            *ingress = Ingress::empty();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn connect() -> Vec<u8> {
        vec![0x80, 0, 1, 0, 1, 1, 0xfd]
    }
    fn address(last: u8) -> Address {
        Address::new([1, 2, 3, 4, 5, last])
    }
    fn write(shared: &Shared, adapter: &str, peer: u8, value: Vec<u8>) -> Result<(), ReqError> {
        Ingress::write(shared, adapter.into(), address(peer), 0, value)
    }

    #[test]
    fn given_valid_connect_when_another_peer_or_adapter_writes_then_only_original_transport_reaches_queue(
    ) {
        // Arrange
        let shared = Ingress::shared();
        write(&shared, "hci0", 1, connect()).unwrap();
        let mut session = Session::open(shared.clone()).unwrap();
        // Act / Assert
        for (adapter, peer) in [("hci0", 2), ("hci1", 1)] {
            assert_eq!(
                write(&shared, adapter, peer, connect()),
                Err(ReqError::InProgress)
            );
            assert_eq!(
                write(&shared, adapter, peer, vec![0x0c, 2]),
                Err(ReqError::InProgress)
            );
        }
        write(&shared, "hci0", 1, vec![0x0c, 1]).unwrap();
        assert_eq!(session.receiver.try_recv().unwrap(), connect());
        assert_eq!(session.receiver.try_recv().unwrap(), [0x0c, 1]);
        assert!(session.receiver.try_recv().is_err());
    }

    #[test]
    fn given_notify_before_connect_when_invalid_or_offset_writes_arrive_then_no_peer_is_claimed() {
        // Arrange
        let shared = Ingress::shared();
        let mut session = Session::open(shared.clone()).unwrap();
        // Act / Assert
        for value in [vec![0x0c, 1], vec![0x80], vec![0x80, 0, 2, 0, 2, 1, 0xfd]] {
            assert_eq!(write(&shared, "hci0", 1, value), Err(ReqError::Failed));
        }
        assert_eq!(
            Ingress::write(&shared, "hci0".into(), address(1), 1, connect()),
            Err(ReqError::InvalidOffset)
        );
        for value in [
            Vec::new(),
            vec![0; usize::from(super::super::MAX_WEAVE_PACKET_SIZE) + 1],
        ] {
            assert_eq!(
                write(&shared, "hci0", 1, value),
                Err(ReqError::InvalidValueLength)
            );
        }
        assert!(shared.lock().unwrap().owner.is_none());
        write(&shared, "hci0", 2, connect()).unwrap();
        assert_eq!(session.receiver.try_recv().unwrap(), connect());
        assert!(session.receiver.try_recv().is_err());
    }

    #[test]
    fn given_queued_old_session_when_owner_drops_then_next_session_has_fresh_queue_and_peer() {
        // Arrange
        let shared = Ingress::shared();
        write(&shared, "hci0", 1, connect()).unwrap();
        let session = Session::open(shared.clone()).unwrap();
        write(&shared, "hci0", 1, vec![0x0c, 1]).unwrap();
        assert!(matches!(
            Session::open(shared.clone()),
            Err(ReqError::InProgress)
        ));
        // Act
        drop(session);
        // Assert: old data cannot claim a new connection; a fresh CONNECT can.
        assert_eq!(
            write(&shared, "hci0", 1, vec![0x0c, 1]),
            Err(ReqError::Failed)
        );
        write(&shared, "hci0", 2, connect()).unwrap();
        let mut next = Session::open(shared).unwrap();
        assert_eq!(next.receiver.try_recv().unwrap(), connect());
        assert!(next.receiver.try_recv().is_err());
    }

    #[test]
    fn given_full_queue_when_another_write_arrives_then_bounded_failure_preserves_order_and_reset_frees_capacity(
    ) {
        // Arrange
        let shared = Ingress::shared();
        write(&shared, "hci0", 1, connect()).unwrap();
        let mut session = Session::open(shared.clone()).unwrap();
        for index in 1..CAPACITY {
            write(&shared, "hci0", 1, vec![0x0c, index as u8]).unwrap();
        }
        // Act / Assert
        assert_eq!(
            write(&shared, "hci0", 1, vec![0x0c, 255]),
            Err(ReqError::Failed)
        );
        assert_eq!(session.receiver.try_recv().unwrap(), connect());
        for index in 1..CAPACITY {
            assert_eq!(session.receiver.try_recv().unwrap(), [0x0c, index as u8]);
        }
        assert!(session.receiver.try_recv().is_err());
        drop(session);
        write(&shared, "hci0", 2, connect()).unwrap();
    }

    #[tokio::test]
    async fn given_cancelled_session_future_when_dropped_then_pending_bytes_and_owner_are_released()
    {
        // Arrange
        let shared = Ingress::shared();
        write(&shared, "hci0", 1, connect()).unwrap();
        let session = Session::open(shared.clone()).unwrap();
        let future = async move {
            let _session = session;
            std::future::pending::<()>().await;
        };
        let mut future = Box::pin(future);
        assert!(futures::poll!(future.as_mut()).is_pending());
        // Act
        drop(future);
        // Assert
        write(&shared, "hci0", 2, connect()).unwrap();
        let mut next = Session::open(shared).unwrap();
        assert_eq!(next.receiver.try_recv().unwrap(), connect());
        assert!(next.receiver.try_recv().is_err());
    }
}

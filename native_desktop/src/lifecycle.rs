//! Process-lifetime signal ownership survives GTK reconnect generations.
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub struct Shutdown {
    pub stop: CancellationToken,
    task: JoinHandle<()>,
}
impl Shutdown {
    pub fn new() -> Result<Self, &'static str> {
        let mut signals = rqs_lib::service::process::ShutdownSignals::new()
            .map_err(|_| "Cannot observe process shutdown")?;
        Ok(Self::from_signal(async move {
            let _ = signals.wait().await;
        }))
    }
    fn from_signal(signal: impl std::future::Future<Output = ()> + Send + 'static) -> Self {
        let stop = CancellationToken::new();
        let cancelled = stop.clone();
        let task = tokio::spawn(async move {
            tokio::select! { biased;
                _ = cancelled.cancelled() => (),
                _ = signal => cancelled.cancel(),
            }
        });
        Self { stop, task }
    }
    pub async fn close(self) {
        self.stop.cancel();
        let _ = self.task.await;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn given_owned_signal_wait_when_signal_or_explicit_close_arrives_then_process_stop_is_retained_and_task_joined(
    ) {
        let (signal, observe) = tokio::sync::oneshot::channel();
        let owner = Shutdown::from_signal(async move {
            let _ = observe.await;
        });
        let stop = owner.stop.clone();
        assert!(!stop.is_cancelled());
        signal.send(()).unwrap();
        stop.cancelled().await;
        owner.close().await;
        let owner = Shutdown::from_signal(std::future::pending());
        let stop = owner.stop.clone();
        owner.close().await;
        assert!(stop.is_cancelled());
    }
    #[tokio::test]
    async fn given_immediately_ready_signal_when_shutdown_created_then_stop_is_cancelled_before_close() {
        // Arrange / Act / Assert
        let owner = Shutdown::from_signal(std::future::ready(()));
        let stop = owner.stop.clone();
        stop.cancelled().await;
        assert!(stop.is_cancelled());
        owner.close().await;
        assert!(stop.is_cancelled());
    }

    #[tokio::test]
    async fn given_disconnected_signal_sender_when_wait_finishes_then_stop_is_cancelled() {
        // Arrange: dropping a signal source completes the observed future.
        let (sender, receiver) = tokio::sync::oneshot::channel::<()>();
        drop(sender);
        let owner = Shutdown::from_signal(async move {
            let _ = receiver.await;
        });
        let stop = owner.stop.clone();

        // Act / Assert
        stop.cancelled().await;
        assert!(stop.is_cancelled());
        owner.close().await;
    }

    #[tokio::test]
    async fn given_forever_pending_signal_when_explicit_close_runs_then_worker_joins_and_stop_remains_cancelled() {
        // Arrange
        let owner = Shutdown::from_signal(std::future::pending());
        let stop = owner.stop.clone();
        assert!(!stop.is_cancelled());

        // Act
        owner.close().await;

        // Assert
        assert!(stop.is_cancelled());
        stop.cancelled().await;
    }

    #[tokio::test]
    async fn given_signal_and_close_racing_when_both_complete_then_shutdown_is_idempotently_cancelled() {
        // Arrange: neither outcome depends on which ready branch wins.
        for _ in 0..64 {
            let (sender, receiver) = tokio::sync::oneshot::channel::<()>();
            let owner = Shutdown::from_signal(async move {
                let _ = receiver.await;
            });
            let stop = owner.stop.clone();

            // Act
            let (sent, ()) = tokio::join!(async move { sender.send(()).is_ok() }, owner.close());

            // Assert
            assert!(sent);
            assert!(stop.is_cancelled());
            stop.cancelled().await;
        }
    }

}

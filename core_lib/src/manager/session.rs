//! Protocol workers owned by the TCP manager's join barrier.

#[cfg(test)]
mod identity_tests;

use std::future::Future;
use std::net::SocketAddr;
use std::time::Instant;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio_util::sync::CancellationToken;

use super::{
    cancellation_message, error_for_ui, outbound_failure, route_bandwidth_upgrade_if_pending,
    terminal, ChannelDirection, ChannelMessage, SendInfo, Sender, State, TransferType,
};
use crate::errors::AppError;
use crate::hdl::{BwuRouter, InboundRequest, OutboundRequest};
use crate::sessions::{SessionHub, SessionLease};
use crate::utils::RemoteDeviceInfo;

/// Cancellation encloses the complete operation, including its initial I/O.
pub(crate) async fn until_stopped<T>(
    token: &CancellationToken,
    operation: impl Future<Output = T>,
) -> Option<T> {
    tokio::select! {
        biased;
        _ = token.cancelled() => None,
        result = operation => Some(result),
    }
}

/// A user-requested cancellation may send an authenticated terminal
/// control message; global shutdown, live operations and terminal sessions
/// must not write any additional frames to the sender's socket.
pub(crate) fn should_notify_sender_on_cancel(
    operation_cancelled: bool,
    user_cancelled: bool,
    service_cancelled: bool,
    state: &State,
) -> bool {
    operation_cancelled && user_cancelled && !service_cancelled && !terminal(state)
}

pub(crate) trait Connector: Send + Sync + 'static {
    fn connect<'a>(
        &'a self,
        address: &'a str,
    ) -> impl Future<Output = std::io::Result<TcpStream>> + Send + 'a;
}

pub(crate) struct TcpConnector;

impl Connector for TcpConnector {
    fn connect<'a>(
        &'a self,
        address: &'a str,
    ) -> impl Future<Output = std::io::Result<TcpStream>> + Send + 'a {
        TcpStream::connect(address)
    }
}

pub(super) struct SessionContext<C> {
    endpoint_id: [u8; 4],
    sender: Sender<ChannelMessage>,
    connector: Arc<C>,
    sessions: Option<SessionHub>,
    receive_policy: crate::receive::ReceivePolicy,
    identity: Option<crate::identity::IdentityPolicy>,
}

impl<C> Clone for SessionContext<C> {
    fn clone(&self) -> Self {
        Self {
            endpoint_id: self.endpoint_id,
            sender: self.sender.clone(),
            connector: Arc::clone(&self.connector),
            sessions: self.sessions.clone(),
            receive_policy: self.receive_policy.clone(),
            identity: self.identity.clone(),
        }
    }
}

impl<C: Connector> SessionContext<C> {
    pub(super) fn new(
        endpoint_id: [u8; 4],
        sender: Sender<ChannelMessage>,
        connector: Arc<C>,
        sessions: Option<SessionHub>,
    ) -> Self {
        Self {
            endpoint_id,
            sender,
            connector,
            sessions,
            receive_policy: crate::receive::ReceivePolicy::new(None),
            identity: None,
        }
    }

    pub(super) fn with_identity(
        mut self,
        identity: Option<crate::identity::IdentityPolicy>,
    ) -> Self {
        self.identity = identity;
        self
    }

    pub(super) fn with_receive_policy(mut self, policy: crate::receive::ReceivePolicy) -> Self {
        self.receive_policy = policy;
        self
    }

    pub(super) fn close_sessions(&self) {
        if let Some(sessions) = &self.sessions {
            sessions.close();
        }
    }

    pub(super) fn cancel_queued(&self, id: &str) {
        if let Some(message) =
            cancellation_message(id, TransferType::Outbound, &State::Initial, None)
        {
            let _ = self.sender.send(message);
        }
    }

    #[cfg(test)]
    pub(super) async fn execute_outbound(&self, token: CancellationToken, info: SendInfo) {
        self.execute_session(token, CancellationToken::new(), info, None)
            .await;
    }

    pub(super) async fn execute_session(
        &self,
        token: CancellationToken,
        cancellation: CancellationToken,
        info: SendInfo,
        mut session: Option<SessionLease>,
    ) {
        let id = info.id.clone();
        if let Err(error) = self
            .connect(&token, &cancellation, info, &mut session)
            .await
        {
            if let Some(session) = session.as_mut() {
                session.observe(&State::Disconnected);
            }
            error!("TcpServer: outbound worker failed: {error}");
            if let Some(message) = outbound_failure(&id, &State::Initial, None, &error) {
                let _ = self.sender.send(message);
            }
        }
    }

    async fn connect(
        &self,
        token: &CancellationToken,
        cancellation: &CancellationToken,
        info: SendInfo,
        session: &mut Option<SessionLease>,
    ) -> Result<(), anyhow::Error> {
        let local_name = self
            .identity
            .as_ref()
            .map(|identity| identity.snapshot().name);
        let connect_started = Instant::now();
        let Some(socket) =
            until_either_stops(token, cancellation, self.connector.connect(&info.addr)).await
        else {
            self.cancel_queued(&info.id);
            if let Some(session) = session.as_mut() {
                session.observe(&State::Cancelled);
            }
            return Ok(());
        };
        let connect_latency = connect_started.elapsed();
        let mut request = OutboundRequest::new(
            self.endpoint_id,
            socket?,
            info.id,
            self.sender.clone(),
            info.ob,
            RemoteDeviceInfo {
                device_type: crate::DeviceType::Unknown,
                name: info.name,
            },
        )?;
        request = request.with_connect_latency(connect_latency);
        if let Some(name) = local_name {
            request = request.with_local_name(name);
        }
        if let Some(session) = session.take() {
            request = request.attach_session(session);
        }

        let result: Option<Result<(), anyhow::Error>> =
            until_either_stops(token, cancellation, async {
                request.send_connection_request().await?;
                request.send_ukey2_client_init().await?;
                loop {
                    request.handle().await?;
                }
            })
            .await;

        let message = match result {
            None => cancellation_message(
                &request.state.id,
                TransferType::Outbound,
                &request.state.state,
                request.state.transfer_metadata.clone(),
            ),
            Some(Err(error)) if error.downcast_ref::<AppError>().is_none() => outbound_failure(
                &request.state.id,
                &request.state.state,
                request.state.transfer_metadata.clone(),
                &error,
            ),
            _ => None,
        };
        if let Some(message) = message {
            if let Some(state) = message.state.clone() {
                request.record_termination(state);
            }
            let _ = self.sender.send(message);
        }
        Ok(())
    }

    pub(super) async fn classify_inbound(
        &self,
        token: CancellationToken,
        socket: TcpStream,
        remote_addr: SocketAddr,
        router: BwuRouter,
        sessions: Arc<Semaphore>,
        classification: OwnedSemaphorePermit,
    ) {
        let classified =
            until_stopped(&token, route_bandwidth_upgrade_if_pending(socket, &router)).await;
        let socket = match classified {
            Some(Ok(Some(socket))) => socket,
            Some(Err(error)) => {
                warn!("TcpServer: inbound classification failed: {error}");
                return;
            }
            None | Some(Ok(None)) => return,
        };
        let Ok(permit) = sessions.try_acquire_owned() else {
            // A probe at capacity has no established transfer or user-facing identity.
            return;
        };
        drop(classification);
        let _permit = permit;
        self.execute_inbound(&token, socket, remote_addr).await;
    }

    async fn execute_inbound(
        &self,
        token: &CancellationToken,
        socket: TcpStream,
        remote: SocketAddr,
    ) {
        let mut request = InboundRequest::new(socket, remote.to_string(), self.sender.clone())
            .with_receive_directory(self.receive_policy.snapshot());
        if let Some(sessions) = self.sessions.as_ref() {
            let Ok(session) = sessions.open(TransferType::Inbound, None, CancellationToken::new())
            else {
                return;
            };
            request = request.attach_session(session);
        }
        let cancellation = request.cancellation();
        let result: Option<Result<(), anyhow::Error>> =
            until_either_stops(token, &cancellation, async {
                loop {
                    request.handle().await?;
                }
            })
            .await;
        if should_notify_sender_on_cancel(
            result.is_none(),
            cancellation.is_cancelled(),
            token.is_cancelled(),
            &request.state.state,
        ) {
            if let Err(error) = request.send_local_cancel().await {
                warn!("TcpServer: unable to deliver inbound sender Cancel: {error}");
            }
        }
        let message = match result {
            None => cancellation_message(
                &request.state.id,
                TransferType::Inbound,
                &request.state.state,
                request.state.transfer_metadata.clone(),
            ),
            Some(Err(error))
                if error.downcast_ref::<AppError>().is_none()
                    && request.state.state != State::Initial
                    && !terminal(&request.state.state) =>
            {
                Some(ChannelMessage {
                    id: request.state.id.clone(),
                    direction: ChannelDirection::LibToFront,
                    rtype: Some(TransferType::Inbound),
                    state: Some(State::Disconnected),
                    meta: request.state.transfer_metadata.clone(),
                    error: Some(error_for_ui(&error)),
                    ..Default::default()
                })
            }
            _ => None,
        };
        if let Some(message) = message {
            if let Some(state) = message.state.clone() {
                request.record_termination(state);
            }
            let _ = self.sender.send(message);
        } else if result_is_cancelled(token, &cancellation) {
            request.record_termination(State::Cancelled);
        }
    }
}

pub(crate) async fn until_either_stops<T>(
    service: &CancellationToken,
    request: &CancellationToken,
    operation: impl Future<Output = T>,
) -> Option<T> {
    tokio::select! {
        biased;
        _ = service.cancelled() => None,
        _ = request.cancelled() => None,
        result = operation => Some(result),
    }
}

fn result_is_cancelled(service: &CancellationToken, request: &CancellationToken) -> bool {
    service.is_cancelled() || request.is_cancelled()
}

#[cfg(test)]
mod local_cancel_tests {
    use super::*;

    #[test]
    fn given_user_cancelled_active_inbound_when_worker_returns_then_peer_notification_is_required()
    {
        for state in [
            State::Initial,
            State::WaitingForUserConsent,
            State::ReceivingFiles,
        ] {
            assert!(should_notify_sender_on_cancel(true, true, false, &state));
        }
    }

    #[test]
    fn given_absent_user_cancel_or_shutdown_when_worker_returns_then_peer_notification_is_forbidden(
    ) {
        let negative_inputs = [
            (false, false, false),
            (false, false, true),
            (false, true, false),
            (false, true, true),
            (true, false, false),
            (true, false, true),
            (true, true, true),
        ];
        for state in [
            State::Initial,
            State::WaitingForUserConsent,
            State::ReceivingFiles,
        ] {
            for (operation_cancelled, user_cancelled, service_cancelled) in negative_inputs {
                assert!(
                    !should_notify_sender_on_cancel(
                        operation_cancelled,
                        user_cancelled,
                        service_cancelled,
                        &state
                    ),
                    "must not notify: {state:?} / {operation_cancelled:?} / {user_cancelled:?} / {service_cancelled:?}"
                );
            }
        }
    }

    #[test]
    fn given_terminal_inbound_when_cancel_arrives_then_no_spurious_sender_notification_occurs() {
        for state in [
            State::Finished,
            State::Rejected,
            State::Cancelled,
            State::Disconnected,
        ] {
            for operation_cancelled in [false, true] {
                for user_cancelled in [false, true] {
                    for service_cancelled in [false, true] {
                        assert!(!should_notify_sender_on_cancel(
                            operation_cancelled,
                            user_cancelled,
                            service_cancelled,
                            &state
                        ));
                    }
                }
            }
        }
    }
}

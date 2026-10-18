//! Adversarial, correctly owned Visibility2 service on a private D-Bus.
use std::sync::{Arc, Mutex};

use super::*;
use crate::service::settings_service::SettingsView;
use crate::service::visibility_service::{VisibilityIntent, VisibilityView};

const EPOCH: u128 = 123;

struct ForgedManager;

#[zbus::interface(name = "io.github.morozhenko_apps.RQuickShare.Manager2")]
impl ForgedManager {
    fn begin_snapshot(
        &self,
    ) -> (String, String, u64, String, bool, u32, u32, u64, u32, u32, u32) {
        (
            format!("{EPOCH:032x}-{:016x}", 1),
            format!("{EPOCH:032x}"),
            1,
            "running".to_owned(),
            false,
            0,
            0,
            0,
            0,
            0,
            0,
        )
    }

    fn end_snapshot(&self, _snapshot: String) {}
}

struct ForgedOperations;

#[zbus::interface(name = "io.github.morozhenko_apps.RQuickShare.Operations1")]
impl ForgedOperations {
    fn get_operation_state(&self) -> (u64, u64) {
        (0, 0)
    }
}

struct Replies {
    view: VisibilityView,
    settings: SettingsView,
    set_calls: usize,
}

struct ForgedVisibility(Arc<Mutex<Replies>>);

#[zbus::interface(name = "io.github.morozhenko_apps.RQuickShare.Visibility2")]
impl ForgedVisibility {
    fn read(&self) -> (VisibilityView,) {
        (self.0.lock().unwrap().view.clone(),)
    }

    fn set(&self, _intent: VisibilityIntent) -> (SettingsView,) {
        let mut replies = self.0.lock().unwrap();
        replies.set_calls += 1;
        (replies.settings.clone(),)
    }
}

async fn fixture() -> (PrivateBus, Connection, NativeClient, Arc<Mutex<Replies>>) {
    let private = PrivateBus::new();
    let replies = Arc::new(Mutex::new(Replies {
        view: VisibilityView {
            epoch: format!("{EPOCH:032x}"),
            available: true,
            revision: 1,
            running: true,
            mode: 2,
            remaining_ms: 500,
        },
        settings: SettingsView::unavailable(EPOCH),
        set_calls: 0,
    }));
    // The fake owns the actual service name and is authenticated by the real
    // NativeClient bootstrapping logic. Only server replies are controllable.
    let server = bus::connection_builder(private.socket().await)
        .serve_at(PATH, ForgedManager)
        .unwrap()
        .serve_at(PATH, ForgedOperations)
        .unwrap()
        .serve_at(PATH, ForgedVisibility(replies.clone()))
        .unwrap()
        .build()
        .await
        .unwrap();
    server.request_name(SERVICE).await.unwrap();
    let client = NativeClient::connect(private.socket().await)
        .await
        .unwrap();
    assert_eq!(client.epoch, EPOCH);
    (private, server, client, replies)
}

#[tokio::test]
async fn given_authenticated_visibility_read_with_forged_states_then_client_rejects_each_and_recovers(
) {
    scenario(async {
        let (_private, server, mut client, replies) = fixture().await;
        let original = client.read_visibility().await.unwrap();
        assert!(original.valid(EPOCH));
        let invalid = [
            "wrong-epoch",
            "unavailable-running",
            "unavailable-revision",
            "stopped-visible",
            "invalid-mode",
            "timer-too-large",
            "timer-with-visible",
            "timer-while-stopped",
        ];
        for case in invalid {
            let mut view = original.clone();
            match case {
                "wrong-epoch" => view.epoch = format!("{:032x}", EPOCH + 1),
                "unavailable-running" => view.available = false,
                "unavailable-revision" => {
                    view.available = false;
                    view.running = false;
                    view.mode = 1;
                    view.remaining_ms = 0;
                }
                "stopped-visible" => {
                    view.running = false;
                    view.mode = 0;
                    view.remaining_ms = 0;
                }
                "invalid-mode" => view.mode = 3,
                "timer-too-large" => view.remaining_ms = 60_001,
                "timer-with-visible" => view.mode = 0,
                "timer-while-stopped" => view.running = false,
                _ => unreachable!(),
            }
            assert!(!view.valid(EPOCH), "{case}");
            replies.lock().unwrap().view = view;
            assert_eq!(
                client.read_visibility().await,
                Err(ClientError::Protocol),
                "{case}"
            );
            assert!(!client.has_pending_visibility());
        }
        replies.lock().unwrap().view = original.clone();
        assert_eq!(client.read_visibility().await, Ok(original));
        assert_eq!(replies.lock().unwrap().set_calls, 0);
        client.close().await.unwrap();
        server.close().await.unwrap();
    })
    .await;
}

#[tokio::test]
async fn given_authenticated_success_reply_with_foreign_request_when_retried_then_exact_pending_intent_survives(
) {
    scenario(async {
        let (_private, server, mut client, replies) = fixture().await;
        let intent = VisibilityIntent {
            epoch: format!("{EPOCH:032x}"),
            revision: 0,
            token: "a".repeat(64),
            request: "b".repeat(32),
            mode: 2,
        };
        assert!(intent.valid(EPOCH));
        {
            let mut forged = replies.lock().unwrap();
            forged.settings.last_request = "c".repeat(32);
            assert!(forged.settings.valid(EPOCH));
        }

        // The response is structurally valid and from the exact bus owner;
        // only the acknowledgement request identity is foreign.
        assert_eq!(
            client.set_visibility(intent.clone()).await,
            Err(ClientError::Protocol)
        );
        assert!(client.has_pending_visibility());
        assert_eq!(client.pending_visibility.as_ref(), Some(&intent));
        assert_eq!(
            client.retry_visibility().await,
            Err(ClientError::Protocol)
        );
        assert_eq!(replies.lock().unwrap().set_calls, 2);
        assert_eq!(client.pending_visibility.as_ref(), Some(&intent));

        // Only an explicit retry with an exactly matching receipt may clear it.
        replies.lock().unwrap().settings.last_request = intent.request.clone();
        let accepted = client.retry_visibility().await.unwrap();
        assert_eq!(accepted.last_request, intent.request);
        assert!(!client.has_pending_visibility());
        assert_eq!(replies.lock().unwrap().set_calls, 3);
        assert_eq!(
            client.retry_visibility().await,
            Err(ClientError::InvalidInput)
        );
        client.close().await.unwrap();
        server.close().await.unwrap();
    })
    .await;
}

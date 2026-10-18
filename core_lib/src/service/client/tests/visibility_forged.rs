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
    ) -> (
        String,
        String,
        u64,
        String,
        bool,
        u32,
        u32,
        u64,
        u32,
        u32,
        u32,
    ) {
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

struct ForgedSettings(Arc<Mutex<Replies>>);

#[zbus::interface(name = "io.github.morozhenko_apps.RQuickShare.Settings2")]
impl ForgedSettings {
    fn read(&self) -> (SettingsView,) {
        (self.0.lock().unwrap().settings.clone(),)
    }

    fn reload(&self) -> (SettingsView,) {
        (self.0.lock().unwrap().settings.clone(),)
    }
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
        .serve_at(PATH, ForgedSettings(replies.clone()))
        .unwrap()
        .build()
        .await
        .unwrap();
    server.request_name(SERVICE).await.unwrap();
    let client = NativeClient::connect(private.socket().await).await.unwrap();
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
        assert_eq!(client.retry_visibility().await, Err(ClientError::Protocol));
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

fn corrupted_settings(base: &SettingsView) -> Vec<(&'static str, SettingsView)> {
    let mut cases = Vec::new();
    for case in [
        "wrong-epoch",
        "source-over",
        "visibility-over",
        "port-below",
        "port-above",
        "card-mode-over",
        "device-name-nul",
        "resolved-name-nul",
        "name-status-over",
        "name-status-no-name",
        "directory-over",
        "directory-nul",
        "error-over",
        "applied-revision-over",
        "token-nonhex",
        "token-uppercase",
        "token-missing-available",
        "request-nonhex",
        "request-uppercase",
        "device-name-bidi",
        "resolved-name-over",
    ] {
        let mut view = base.clone();
        match case {
            "wrong-epoch" => view.epoch = format!("{:032x}", EPOCH + 1),
            "source-over" => view.source = 3,
            "visibility-over" => view.visibility = 3,
            "port-below" => view.port = -2,
            "port-above" => view.port = 65_536,
            "card-mode-over" => view.card_mode = 2,
            "device-name-nul" => view.device_name = "a\0b".to_owned(),
            "resolved-name-nul" => view.resolved_name = "a\0b".to_owned(),
            "name-status-over" => view.name_status = 4,
            "name-status-no-name" => view.name_status = 1,
            "directory-over" => view.directory_display = "x".repeat(12_289),
            "directory-nul" => view.directory_display = "folder\0file".to_owned(),
            "error-over" => view.error = 5,
            "applied-revision-over" => view.applied_live_revision = view.revision + 1,
            "token-nonhex" => view.token = "g".repeat(64),
            "token-uppercase" => view.token = "A".repeat(64),
            "token-missing-available" => view.available = true,
            "request-nonhex" => view.last_request = "g".repeat(32),
            "request-uppercase" => view.last_request = "B".repeat(32),
            "device-name-bidi" => view.device_name = "\u{202e}unsafe".to_owned(),
            "resolved-name-over" => {
                view.resolved_name = "x".repeat(crate::identity::MAX_NAME_BYTES + 1);
            }
            _ => unreachable!(),
        }
        assert!(!view.valid(EPOCH), "fixture must be truly invalid: {case}");
        cases.push((case, view));
    }
    assert_eq!(cases.len(), 21);
    cases
}

#[tokio::test]
async fn given_authenticated_settings_read_and_reload_with_corrupted_views_then_each_rejects_and_recovers(
) {
    scenario(async {
        // Arrange: both methods use actual authenticated D-Bus interfaces.
        let (_private, server, mut client, replies) = fixture().await;
        let mut baseline = SettingsView::unavailable(EPOCH);
        baseline.last_request = "b".repeat(32);
        assert!(baseline.valid(EPOCH));
        replies.lock().unwrap().settings = baseline.clone();

        // Act/Assert: every semantic field failure is rejected on both paths.
        for (case, view) in corrupted_settings(&baseline) {
            replies.lock().unwrap().settings = view;
            assert_eq!(
                client.read_settings().await,
                Err(ClientError::Protocol),
                "Read: {case}"
            );
            assert_eq!(
                client.reload_settings().await,
                Err(ClientError::Protocol),
                "Reload: {case}"
            );
            assert!(!client.has_pending_settings(), "{case}");
            assert!(!client.has_pending_visibility(), "{case}");
        }
        // A legal max-boundary directory and port remain accepted.
        let mut repaired = baseline.clone();
        repaired.port = 65_535;
        repaired.directory_display = "x".repeat(12_288);
        repaired.source = 2;
        repaired.visibility = 2;
        repaired.card_mode = 1;
        repaired.error = 4;
        assert!(repaired.valid(EPOCH));
        replies.lock().unwrap().settings = repaired.clone();
        assert_eq!(client.read_settings().await, Ok(repaired.clone()));
        assert_eq!(client.reload_settings().await, Ok(repaired));
        assert_eq!(replies.lock().unwrap().set_calls, 0);
        client.close().await.unwrap();
        server.close().await.unwrap();
    })
    .await;
}

#[tokio::test]
async fn given_pending_visibility_when_corrupted_set_receipts_arrive_then_only_valid_explicit_retry_clears_it(
) {
    scenario(async {
        // Arrange: a real authenticated client sends one canonical request.
        let (_private, server, mut client, replies) = fixture().await;
        let intent = VisibilityIntent {
            epoch: format!("{EPOCH:032x}"),
            revision: 0,
            token: "a".repeat(64),
            request: "b".repeat(32),
            mode: 2,
        };
        let mut baseline = SettingsView::unavailable(EPOCH);
        baseline.last_request = intent.request.clone();
        assert!(intent.valid(EPOCH));
        assert!(baseline.valid(EPOCH));
        let cases = corrupted_settings(&baseline);

        // Act: a wrong reply never acknowledges an otherwise valid request.
        for (index, (case, view)) in cases.into_iter().enumerate() {
            replies.lock().unwrap().settings = view;
            let result = if index == 0 {
                client.set_visibility(intent.clone()).await
            } else {
                client.retry_visibility().await
            };
            assert_eq!(result, Err(ClientError::Protocol), "{case}");
            assert_eq!(client.pending_visibility.as_ref(), Some(&intent));
            assert!(client.has_pending_visibility());
            assert_eq!(replies.lock().unwrap().set_calls, index + 1);
        }

        // Assert: only a valid, matching, explicitly retried receipt completes it.
        replies.lock().unwrap().settings = baseline.clone();
        assert_eq!(client.retry_visibility().await, Ok(baseline));
        assert_eq!(replies.lock().unwrap().set_calls, 22);
        assert!(!client.has_pending_visibility());
        assert_eq!(
            client.retry_visibility().await,
            Err(ClientError::InvalidInput)
        );
        client.close().await.unwrap();
        server.close().await.unwrap();
    })
    .await;
}

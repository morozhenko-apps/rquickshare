#[tokio::test]
async fn given_worker_fault_when_replacing_then_no_new_discovery_is_spawned() {
    scenario(async {
        let mut core = RQS::new(Visibility::Invisible, None, None);
        let (tracker, lifetime, guard) = core.begin_startup().unwrap();
        guard.disarm();
        let (started, ready) = tokio::sync::oneshot::channel();
        core.replace_discovery_with(|_token| {
            Ok(async move {
                started.send(()).unwrap();
                Err(anyhow!("simulated mDNS browser closure"))
            })
        })
        .await
        .unwrap();
        ready.await.unwrap();
        let old_token = core.discovery_ctk.as_ref().unwrap().clone();

        let (created, mut create_called) = tokio::sync::oneshot::channel();
        let result = core
            .replace_discovery_with(|token| {
                created.send(()).unwrap();
                Ok(async move {
                    token.cancelled().await;
                    Ok(())
                })
            })
            .await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "simulated mDNS browser closure"
        );
        assert!(old_token.is_cancelled());
        assert!(core.discovery_ctk.is_none());
        assert!(core.discovery_task.is_none());
        assert!(create_called.try_recv().is_err());
        assert!(!lifetime.is_cancelled());
        assert!(!tracker.is_closed());
        core.stop().await;
        assert!(tracker.is_closed() && tracker.is_empty());
    })
    .await;
}

#[tokio::test]
async fn given_worker_fault_when_stopped_then_discovery_is_joined_and_core_survives() {
    scenario(async {
        let mut core = RQS::new(Visibility::Invisible, None, None);
        let (tracker, lifetime, guard) = core.begin_startup().unwrap();
        guard.disarm();
        let (started, ready) = tokio::sync::oneshot::channel();
        core.replace_discovery_with(|_token| {
            Ok(async move {
                started.send(()).unwrap();
                Err(anyhow!("simulated browser receiver disconnect"))
            })
        })
        .await
        .unwrap();
        ready.await.unwrap();
        let former = core.discovery_ctk.as_ref().unwrap().clone();
        assert!(!former.is_cancelled());

        let result = core.stop_discovery_and_wait().await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "simulated browser receiver disconnect"
        );
        assert!(former.is_cancelled());
        assert!(core.discovery_ctk.is_none());
        assert!(core.discovery_task.is_none());
        assert!(!lifetime.is_cancelled());
        assert_eq!(core.subscribe_visibility().current(), Visibility::Invisible);
        core.stop_discovery_and_wait().await.unwrap();
        core.stop().await;
        assert!(tracker.is_closed() && tracker.is_empty());
    })
    .await;
}

#[tokio::test]
async fn given_active_discovery_when_replaced_then_old_worker_is_joined_before_new_generation_starts(
) {
    scenario(async {
        // Arrange: genuine Core ownership and tracker, with external SDK
        // work represented by an event-controlled future.
        let mut core = RQS::new(Visibility::Invisible, None, None);
        let (tracker, lifetime, guard) = core.begin_startup().unwrap();
        guard.disarm();
        let (old_started, old_ready) = tokio::sync::oneshot::channel();
        let (old_finished, finish_old) = tokio::sync::oneshot::channel();
        core.replace_discovery_with(|token| {
            Ok(async move {
                old_started.send(()).unwrap();
                token.cancelled().await;
                finish_old.await.unwrap();
                Ok(())
            })
        })
        .await
        .unwrap();
        old_ready.await.unwrap();
        let first_token = core.discovery_ctk.as_ref().unwrap().clone();

        let (new_started, mut new_ready) = tokio::sync::oneshot::channel();
        let mut replace = Box::pin(core.replace_discovery_with(|token| {
            Ok(async move {
                new_started.send(()).unwrap();
                token.cancelled().await;
                Ok(())
            })
        }));
        // Act: polling replacement cancels the old worker, then must await
        // its *actual completion*, not merely observe token cancellation.
        assert!(futures::poll!(replace.as_mut()).is_pending());
        assert!(first_token.is_cancelled());
        assert!(matches!(
            new_ready.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        old_finished.send(()).unwrap();
        replace.await.unwrap();
        new_ready.await.unwrap();

        // Assert: exactly the new generation is owned; shutdown joins it.
        assert!(core
            .discovery_ctk
            .as_ref()
            .is_some_and(|ctk| !ctk.is_cancelled()));
        assert!(!lifetime.is_cancelled());
        core.stop_discovery_and_wait().await.unwrap();
        assert!(core.discovery_task.is_none());
        assert!(core.discovery_ctk.is_none());
        core.stop().await;
        assert!(tracker.is_closed() && tracker.is_empty());
    })
    .await;
}

#[tokio::test]
async fn given_discovery_shutdown_failure_when_restarted_then_no_new_worker_is_registered() {
    scenario(async {
        let mut core = RQS::new(Visibility::Invisible, None, None);
        let (tracker, lifetime, guard) = core.begin_startup().unwrap();
        guard.disarm();
        let (old_started, old_ready) = tokio::sync::oneshot::channel();
        core.replace_discovery_with(|token| {
            Ok(async move {
                old_started.send(()).unwrap();
                token.cancelled().await;
                Err(anyhow!("simulated SDK shutdown failure"))
            })
        })
        .await
        .unwrap();
        old_ready.await.unwrap();
        let (attempted_sender, mut attempted_receiver) = tokio::sync::oneshot::channel();
        let result = core
            .replace_discovery_with(|token| {
                attempted_sender.send(()).unwrap();
                Ok(async move {
                    token.cancelled().await;
                    Ok(())
                })
            })
            .await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "simulated SDK shutdown failure"
        );
        assert!(attempted_receiver.try_recv().is_err());
        assert!(core.discovery_task.is_none());
        assert!(core.discovery_ctk.is_none());
        assert!(!lifetime.is_cancelled());
        core.stop().await;
        assert!(tracker.is_closed() && tracker.is_empty());
    })
    .await;
}

#[tokio::test]
async fn given_stopped_discovery_when_restarted_then_only_new_worker_can_be_owned() {
    scenario(async {
        let mut core = RQS::new(Visibility::Invisible, None, None);
        let (tracker, _lifetime, guard) = core.begin_startup().unwrap();
        guard.disarm();
        core.replace_discovery_with(|token| {
            Ok(async move {
                token.cancelled().await;
                Ok(())
            })
        })
        .await
        .unwrap();
        let former = core.discovery_ctk.as_ref().unwrap().clone();
        core.stop_discovery_and_wait().await.unwrap();
        core.stop_discovery_and_wait().await.unwrap();
        assert!(former.is_cancelled());
        assert!(core.discovery_task.is_none());

        let (started, ready) = tokio::sync::oneshot::channel();
        core.replace_discovery_with(|token| {
            Ok(async move {
                started.send(()).unwrap();
                token.cancelled().await;
                Ok(())
            })
        })
        .await
        .unwrap();
        ready.await.unwrap();
        assert!(core
            .discovery_ctk
            .as_ref()
            .is_some_and(|ctk| !ctk.is_cancelled()));
        core.stop().await;
        assert!(tracker.is_empty() && tracker.is_closed());
    })
    .await;
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::*;

async fn scenario(operation: impl Future<Output = ()>) {
    tokio::time::timeout(std::time::Duration::from_secs(5), operation)
        .await
        .expect("lifecycle scenario deadlocked");
}

#[tokio::test]
async fn given_core_not_started_when_discovery_requested_then_no_browser_or_token_is_created() {
    // Arrange: no Core transport lifetime or external mDNS daemon exists.
    let mut core = RQS::new(Visibility::Invisible, None, None);
    let (sender, _receiver) = broadcast::channel(2);
    assert!(core.tracker.is_none());
    assert!(core.discovery_ctk.is_none());

    // Act / Assert: even repeated requests must fail before spawning a
    // discovery daemon or changing the owner's state.
    for _ in 0..2 {
        let error = core.discovery(sender.clone()).await.unwrap_err();
        assert_eq!(error.to_string(), "The service wasn't first started");
        assert!(core.tracker.is_none());
        assert!(core.ctoken.is_none());
        assert!(core.discovery_ctk.is_none());
    }
    core.stop_discovery();
    assert!(core.discovery_ctk.is_none());
}

#[tokio::test]
async fn given_owned_discovery_when_stopped_twice_then_core_lifetime_survives_until_owner_shutdown()
{
    scenario(async {
        // Arrange: the real Core tracker owns an independent discovery-like
        // cancellation-driven worker, without initializing the mDNS SDK.
        // This tests stop_discovery ownership, not repeated discovery starts.
        let mut core = RQS::new(Visibility::Invisible, None, None);
        let (tracker, lifetime, guard) = core.begin_startup().unwrap();
        let discovery = CancellationToken::new();
        core.discovery_ctk = Some(discovery.clone());
        let worker_token = discovery.clone();
        let (started_sender, started_receiver) = tokio::sync::oneshot::channel();
        let (finished_sender, finished_receiver) = tokio::sync::oneshot::channel();
        tracker.spawn(async move {
            started_sender.send(()).unwrap();
            worker_token.cancelled().await;
            finished_sender.send(()).unwrap();
        });
        guard.disarm();

        // Act: discovery stops while the Core lifetime remains running.
        started_receiver.await.unwrap();
        assert!(!discovery.is_cancelled());
        assert!(!lifetime.is_cancelled());
        assert!(!tracker.is_closed());
        core.stop_discovery();
        finished_receiver.await.unwrap();
        core.stop_discovery();

        // Assert: cancellation and removal affect only discovery ownership.
        assert!(discovery.is_cancelled());
        assert!(core.discovery_ctk.is_none());
        assert!(!lifetime.is_cancelled());
        assert!(!tracker.is_closed());
        assert_eq!(core.subscribe_visibility().current(), Visibility::Invisible);

        // The independent Core shutdown still closes/joins the real tracker.
        core.stop().await;
        assert!(lifetime.is_cancelled());
        assert!(tracker.is_closed() && tracker.is_empty());
        assert!(core.ctoken.is_none());
    })
    .await;
}

#[tokio::test(start_paused = true)]
async fn given_started_core_when_owner_is_dropped_then_cancellation_releases_all_tracked_workers() {
    // Arrange
    let mut core = RQS::new(Visibility::Temporarily, None, None);
    let (tracker, token, guard) = core.begin_startup().unwrap();
    let discovery = CancellationToken::new();
    core.discovery_ctk = Some(discovery.clone());
    guard.disarm();
    // Act
    drop(core);
    // Assert
    assert!(token.is_cancelled());
    assert!(discovery.is_cancelled());
    tracker.close();
    tracker.wait().await;
    assert!(tracker.is_empty());
}

#[tokio::test(start_paused = true)]
async fn given_failed_discovery_task_when_core_window_expires_then_policy_survives_and_shutdown_joins(
) {
    // Arrange: actual core lifetime, no radio initialization.
    let mut core = RQS::new(Visibility::Temporarily, None, None);
    let (tracker, token, guard) = core.begin_startup().unwrap();
    let mut reader = core.subscribe_visibility();
    let failed = tracker.spawn(async { Err::<(), _>("controlled discovery failure") });
    assert_eq!(failed.await.unwrap(), Err("controlled discovery failure"));
    assert!(!token.is_cancelled());
    // Act
    tokio::time::advance(std::time::Duration::from_secs(60)).await;
    reader.changed().await.unwrap();
    // Assert
    assert_eq!(reader.current(), Visibility::Invisible);
    assert!(!token.is_cancelled());
    guard.disarm();
    core.stop().await;
    assert!(tracker.is_closed() && tracker.is_empty());
    assert!(token.is_cancelled());
    let (next_tracker, next_token, next_guard) = core.begin_startup().unwrap();
    assert_eq!(core.subscribe_visibility().current(), Visibility::Invisible);
    next_guard.disarm();
    core.stop().await;
    assert!(next_token.is_cancelled());
    assert!(next_tracker.is_empty());
}

#[tokio::test]
async fn given_started_instance_when_both_entrypoints_restart_then_existing_lifetime_is_preserved()
{
    scenario(async {
        // Arrange
        let mut core = RQS::default();
        let (tracker, token, guard) = core.begin_startup().unwrap();
        // Act
        assert!(core.run().await.is_err());
        assert!(core.run_managed().await.is_err());
        // Assert
        assert!(!token.is_cancelled());
        assert!(!tracker.is_closed());
        assert!(core.tracker.is_some());
        guard.disarm();
        core.stop().await;
        assert!(token.is_cancelled());
    })
    .await;
}

#[tokio::test]
async fn given_partial_startup_when_initialization_fails_then_workers_are_joined_before_error_returns(
) {
    scenario(async {
        // Arrange
        let mut core = RQS::default();
        let (tracker, token, guard) = core.begin_startup().unwrap();
        let joined = Arc::new(AtomicBool::new(false));
        let marker = Arc::clone(&joined);
        tracker.spawn(async move {
            token.cancelled().await;
            marker.store(true, Ordering::SeqCst);
        });
        // Act
        let result = core
            .complete_startup::<()>(Err(anyhow!("controlled initialization failure")))
            .await;
        // Assert
        assert_eq!(
            result.unwrap_err().to_string(),
            "controlled initialization failure"
        );
        assert!(joined.load(Ordering::SeqCst));
        assert!(tracker.is_empty() && tracker.is_closed());
        assert!(core.tracker.is_none() && core.ctoken.is_none());
        drop(guard);
    })
    .await;
}

#[tokio::test]
async fn given_abandoned_startup_guard_when_dropped_then_started_workers_cancel_and_stop_can_join()
{
    scenario(async {
        // Arrange
        let mut core = RQS::default();
        let (tracker, token, guard) = core.begin_startup().unwrap();
        let worker_token = token.clone();
        let joined = tracker.spawn(async move {
            worker_token.cancelled().await;
        });
        // Act
        drop(guard);
        core.stop().await;
        // Assert
        assert!(token.is_cancelled());
        joined.await.unwrap();
        assert!(tracker.is_closed() && tracker.is_empty());
        assert!(core.tracker.is_none());
    })
    .await;
}

#[tokio::test]
async fn given_unclosed_tracker_when_stopped_twice_then_stop_closes_and_joins_once() {
    scenario(async {
        // Arrange
        let mut core = RQS::default();
        let (tracker, token, guard) = core.begin_startup().unwrap();
        let worker_token = token.clone();
        tracker.spawn(async move {
            worker_token.cancelled().await;
        });
        guard.disarm();
        // Act
        core.stop().await;
        core.stop().await;
        // Assert
        assert!(token.is_cancelled());
        assert!(tracker.is_closed() && tracker.is_empty());
        assert!(core.ctoken.is_none());
    })
    .await;
}

#[tokio::test]
async fn given_tcp_manager_exit_when_observed_then_error_panic_and_unexpected_success_cancel_transports(
) {
    // Arrange
    for already_stopped in [false, true] {
        for exit in ["ok", "error", "panic"] {
            let token = CancellationToken::new();
            if already_stopped {
                token.cancel();
            }
            // Act
            let failed = supervise_tcp(
                async {
                    match exit {
                        "ok" => Ok(()),
                        "error" => Err(anyhow!("controlled failure")),
                        _ => panic!("controlled supervisor panic"),
                    }
                },
                token.clone(),
            )
            .await;
            // Assert
            assert_eq!(failed, exit != "ok" || !already_stopped);
            assert!(token.is_cancelled());
        }
    }
}

#[tokio::test]
async fn given_occupied_port_when_startup_fails_then_state_is_reset_without_starting_other_transports(
) {
    scenario(async {
        // Arrange
        let occupied = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let mut core = RQS::new(
            Visibility::Invisible,
            Some(occupied.local_addr().unwrap().port().into()),
            None,
        );
        // Act
        let result = core.run().await;
        // Assert
        assert!(result.is_err());
        assert!(core.tracker.is_none() && core.ctoken.is_none());
        core.stop().await;
        assert!(occupied.local_addr().is_ok());
    })
    .await;
}

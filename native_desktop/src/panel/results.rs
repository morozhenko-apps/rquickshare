//! Current-run completion transitions, never a replay of persisted history.
use super::*;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Default)]
pub(super) struct Results {
    epoch: Option<u128>,
    phases: BTreeMap<String, Phase>,
    visible: BTreeSet<String>,
    pending: VecDeque<String>,
}
impl Results {
    pub(super) fn pending_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub(super) fn observe(
        &mut self,
        epoch: u128,
        rows: impl Iterator<Item = (String, Direction, Phase)>,
    ) {
        let baseline = self.epoch != Some(epoch);
        if baseline {
            *self = Self {
                epoch: Some(epoch),
                ..Default::default()
            };
        }
        let rows = rows.collect::<Vec<_>>();
        let present = rows
            .iter()
            .map(|(id, _, _)| id.clone())
            .collect::<BTreeSet<_>>();
        self.phases.retain(|id, _| present.contains(id));
        self.visible.retain(|id| present.contains(id));
        self.pending.retain(|id| present.contains(id));
        // A fresh incoming session retires the prior completion from the
        // compact card, never the History or pending result-reveal queue.
        if !baseline
            && rows.iter().any(|(id, direction, phase)| {
                *direction == Direction::Inbound
                    && !phase.is_terminal()
                    && !self.phases.contains_key(id)
            })
        {
            self.visible.clear();
        }
        // New outgoing work retires only old outgoing Declined feedback; it
        // must not suppress the most recent successfully received result.
        if !baseline
            && rows.iter().any(|(id, direction, phase)| {
                *direction == Direction::Outbound
                    && !phase.is_terminal()
                    && !self.phases.contains_key(id)
            })
        {
            self.visible
                .retain(|id| self.phases.get(id) != Some(&Phase::Rejected));
        }
        for (id, direction, phase) in rows {
            let previous = self.phases.insert(id.clone(), phase);
            if !baseline && previous.is_some_and(|old| !old.is_terminal()) {
                if direction == Direction::Inbound && phase == Phase::Finished {
                    self.visible.insert(id.clone());
                    self.pending.push_back(id);
                } else if direction == Direction::Outbound && phase == Phase::Rejected {
                    // This was a real, live peer decline; show the terminal
                    // status instead of silently erasing the outgoing row.
                    self.visible.insert(id.clone());
                }
            }
        }
    }
    pub(super) fn has_visible(&self) -> bool {
        !self.visible.is_empty()
    }
    pub(super) fn visible(&self, id: &str) -> bool {
        self.visible.contains(id)
    }
    pub(super) fn hide_rows(&mut self) {
        self.visible.clear();
    }
    pub(super) fn dismiss(&mut self) {
        self.visible.clear();
        self.pending.clear();
    }
}
impl Panel {
    pub(super) fn schedule_results(self: &Rc<Self>) {
        if self.results.borrow().pending.is_empty() || self.result_idle.borrow().is_some() {
            return;
        }
        let weak = Rc::downgrade(self);
        let source = glib::idle_add_local_once(move || {
            if let Some(panel) = weak.upgrade() {
                panel.result_idle.borrow_mut().take();
                panel.present_next_result();
            }
        });
        self.result_idle.replace(Some(source));
    }
    fn present_next_result(self: &Rc<Self>) {
        let row = {
            let model = self.model.borrow();
            if model.preview
                || model.closing
                || model.state.disconnected
                || (model.safety_active && !model.safety.manual_allowed())
                || model.waiting
                || model.input_active
                || model.state.busy
                || model.state.pending
                || model.state.text_pending
                || model.state.settings_pending
                || model.dialog.is_some()
                || *self.ingress.gate.subscribe().borrow()
                || self.reveal.borrow().is_some()
                || self
                    .text_review
                    .borrow()
                    .as_ref()
                    .is_some_and(|r| r.visible())
                || self.composer.borrow().as_ref().is_some_and(|r| r.visible())
                || self.image.borrow().as_ref().is_some_and(|r| r.visible())
            {
                return;
            }
            let Some(view) = &model.state.view else {
                return;
            };
            if !view.synchronized || view.lifecycle != Lifecycle::Running {
                return;
            }
            let Some(id) = self.results.borrow_mut().pending.pop_front() else {
                return;
            };
            view.transfers
                .iter()
                .find(|t| {
                    t.transfer.id() == id
                        && t.direction == Direction::Inbound
                        && t.phase == Phase::Finished
                })
                .cloned()
        };
        if let Some(row) = row {
            match row.details.kind {
                rqs_lib::sessions::details::PayloadKind::Files => {
                    if let Some(revision) = row.reveal_revision {
                        self.reveal_result(row.transfer, revision);
                    } else {
                        self.notice("Completed. The saved result is unavailable for automatic folder selection; review History.");
                    }
                }
                _ => {
                    if let (Some(revision), Some(handle)) = (row.content_revision, &self.handle) {
                        self.submit(|| handle.read_text(row.transfer, revision));
                    } else {
                        self.notice("Completed. Text is no longer available; review History.");
                    }
                }
            }
        }
        self.schedule_results();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn observe(results: &mut Results, epoch: u128, rows: &[(&str, Direction, Phase)]) {
        results.observe(
            epoch,
            rows.iter()
                .map(|(id, direction, phase)| ((*id).into(), *direction, *phase)),
        );
    }
    #[test]
    fn given_live_outbound_peer_decline_when_observed_then_compact_card_shows_declined_once_without_replaying_history(
    ) {
        let mut results = Results::default();
        observe(
            &mut results,
            42,
            &[("historical", Direction::Outbound, Phase::Rejected)],
        );
        assert!(!results.visible("historical"));
        observe(
            &mut results,
            42,
            &[
                ("historical", Direction::Outbound, Phase::Rejected),
                ("in-rejected", Direction::Inbound, Phase::Rejected),
                ("out", Direction::Outbound, Phase::Active),
            ],
        );
        assert!(!results.visible("in-rejected"));
        observe(
            &mut results,
            42,
            &[
                ("historical", Direction::Outbound, Phase::Rejected),
                ("out", Direction::Outbound, Phase::Rejected),
            ],
        );
        assert!(results.visible("out"));
        assert!(!results.visible("historical"));
        assert!(results.pending_empty());
        // Repeated terminal snapshots do not create a second message.
        observe(
            &mut results,
            42,
            &[("out", Direction::Outbound, Phase::Rejected)],
        );
        assert!(results.visible("out"));
        assert_eq!(results.visible.len(), 1);
        results.hide_rows();
        observe(
            &mut results,
            42,
            &[("out", Direction::Outbound, Phase::Rejected)],
        );
        assert!(!results.visible("out"));
    }

    #[test]
    fn given_new_outbound_when_previous_declined_and_inbound_completed_then_only_declined_feedback_retires(
    ) {
        let mut results = Results::default();
        observe(&mut results, 4, &[]);
        observe(
            &mut results,
            4,
            &[
                ("in", Direction::Inbound, Phase::Receiving),
                ("out", Direction::Outbound, Phase::Active),
            ],
        );
        observe(
            &mut results,
            4,
            &[
                ("in", Direction::Inbound, Phase::Finished),
                ("out", Direction::Outbound, Phase::Rejected),
            ],
        );
        assert!(results.visible("in"));
        assert!(results.visible("out"));
        assert_eq!(results.pending.len(), 1);
        observe(
            &mut results,
            4,
            &[
                ("in", Direction::Inbound, Phase::Finished),
                ("out", Direction::Outbound, Phase::Rejected),
                ("retry", Direction::Outbound, Phase::Active),
            ],
        );
        assert!(!results.visible("out"));
        assert!(results.visible("in"));
        assert_eq!(results.pending.len(), 1);
        observe(
            &mut results,
            5,
            &[
                ("in", Direction::Inbound, Phase::Finished),
                ("out", Direction::Outbound, Phase::Rejected),
            ],
        );
        assert!(!results.has_visible());
        assert!(results.pending_empty());
    }

    #[test]
    fn given_a_finished_inbound_when_another_consent_arrives_then_old_card_row_retires_but_reveal_remains(
    ) {
        let mut results = Results::default();
        observe(&mut results, 5, &[]);
        observe(
            &mut results,
            5,
            &[("old", Direction::Inbound, Phase::Receiving)],
        );
        observe(
            &mut results,
            5,
            &[("old", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.visible("old"));
        assert_eq!(results.pending.len(), 1);

        // Neither an outbound transfer nor a duplicate completed snapshot
        // suppresses the finished card row.
        observe(
            &mut results,
            5,
            &[
                ("old", Direction::Inbound, Phase::Finished),
                ("out", Direction::Outbound, Phase::Queued),
            ],
        );
        assert!(results.visible("old"));
        assert_eq!(results.pending.len(), 1);

        // A fresh incoming consent is a new transfer, not a name-based
        // replacement of the old daemon session or its saved result.
        observe(
            &mut results,
            5,
            &[
                ("old", Direction::Inbound, Phase::Finished),
                ("out", Direction::Outbound, Phase::Queued),
                ("new", Direction::Inbound, Phase::AwaitingConsent),
            ],
        );
        assert!(!results.visible("old"));
        assert!(results.phases.contains_key("old"));
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["old"],
            "completion reveal must not be lost when a new peer requests consent"
        );

        // Redraws and consent→Receiving are not new IDs.
        for phase in [Phase::AwaitingConsent, Phase::Receiving] {
            observe(
                &mut results,
                5,
                &[
                    ("old", Direction::Inbound, Phase::Finished),
                    ("new", Direction::Inbound, phase),
                ],
            );
            assert!(!results.visible("old"));
            assert_eq!(results.pending.len(), 1);
        }

        observe(
            &mut results,
            5,
            &[
                ("old", Direction::Inbound, Phase::Finished),
                ("new", Direction::Inbound, Phase::Finished),
            ],
        );
        assert!(!results.visible("old"));
        assert!(results.visible("new"));
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["old", "new"],
            "fresh completion is presented once without replaying the old row"
        );
    }

    #[test]
    fn given_duplicate_ids_terminal_arrivals_and_parallel_consents_then_only_new_active_inbound_retires_card_result(
    ) {
        let mut results = Results::default();
        observe(&mut results, 12, &[]);
        observe(
            &mut results,
            12,
            &[("a", Direction::Inbound, Phase::Receiving)],
        );
        observe(
            &mut results,
            12,
            &[("a", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.visible("a"));

        // Newly observed terminal-only records are historical and must not
        // count as live new consent. Fresh outbound sessions also do not clear.
        observe(
            &mut results,
            12,
            &[
                ("a", Direction::Inbound, Phase::Finished),
                ("older", Direction::Inbound, Phase::Rejected),
                ("send", Direction::Outbound, Phase::Active),
            ],
        );
        assert!(results.visible("a"));

        observe(
            &mut results,
            12,
            &[
                ("a", Direction::Inbound, Phase::Finished),
                ("b", Direction::Inbound, Phase::AwaitingConsent),
                ("c", Direction::Inbound, Phase::AwaitingConsent),
            ],
        );
        assert!(!results.visible("a"));
        assert_eq!(results.phases.get("b"), Some(&Phase::AwaitingConsent));
        assert_eq!(results.phases.get("c"), Some(&Phase::AwaitingConsent));
        assert_eq!(results.pending.len(), 1);

        // Two independent requests remain tracked even across repeated
        // snapshots and one rejection, without reviving a stale completed row.
        observe(
            &mut results,
            12,
            &[
                ("a", Direction::Inbound, Phase::Finished),
                ("b", Direction::Inbound, Phase::Rejected),
                ("c", Direction::Inbound, Phase::AwaitingConsent),
            ],
        );
        assert!(!results.visible("a"));
        assert_eq!(results.phases.get("b"), Some(&Phase::Rejected));
        assert_eq!(results.phases.get("c"), Some(&Phase::AwaitingConsent));

        // A different generation cannot replay the old completion or reveal.
        observe(
            &mut results,
            13,
            &[("a", Direction::Inbound, Phase::Finished)],
        );
        assert!(!results.visible("a"));
        assert!(results.pending_empty());
    }

    #[test]
    fn given_baseline_or_new_epoch_when_completed_records_are_read_then_old_results_never_reopen() {
        let mut results = Results::default();
        observe(
            &mut results,
            1,
            &[("old", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.pending.is_empty());
        observe(
            &mut results,
            1,
            &[("old", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.pending.is_empty());
        observe(
            &mut results,
            1,
            &[("new", Direction::Inbound, Phase::Receiving)],
        );
        observe(
            &mut results,
            2,
            &[("new", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.pending.is_empty());
        assert!(!results.visible("new"));
    }
    #[test]
    fn given_repeated_terminal_and_cross_direction_snapshots_when_observed_then_only_fresh_inbound_success_is_presented(
    ) {
        let mut results = Results::default();

        // A persisted Finished baseline is historical, not a new delivery.
        observe(
            &mut results,
            41,
            &[("historical", Direction::Inbound, Phase::Finished)],
        );
        assert!(!results.has_visible());
        assert!(results.pending_empty());

        // A same-epoch live transition is the only route to a new result.
        observe(
            &mut results,
            41,
            &[
                ("in", Direction::Inbound, Phase::Receiving),
                ("out", Direction::Outbound, Phase::Sending),
            ],
        );
        observe(
            &mut results,
            41,
            &[
                ("in", Direction::Inbound, Phase::Finished),
                ("out", Direction::Outbound, Phase::Finished),
            ],
        );
        assert!(results.visible("in"));
        assert!(!results.visible("out"));
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["in"]
        );

        // Duplicate terminal observations do not add a second reveal.
        observe(
            &mut results,
            41,
            &[
                ("in", Direction::Inbound, Phase::Finished),
                ("out", Direction::Outbound, Phase::Finished),
            ],
        );
        assert_eq!(results.pending.len(), 1);
        results.dismiss();
        assert!(!results.visible("in") && results.pending_empty());
        observe(
            &mut results,
            41,
            &[("in", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.pending_empty());

        // A later epoch's already-finished record cannot replay either.
        observe(
            &mut results,
            42,
            &[("in", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.pending_empty());
        assert!(!results.has_visible());
    }

    #[test]
    fn given_multiple_live_successes_and_removed_rows_when_observed_then_queue_remains_fifo_and_prunes_stale_results(
    ) {
        let mut results = Results::default();
        observe(&mut results, 7, &[]);

        // Arrival order is the authoritative order for notifications.
        observe(
            &mut results,
            7,
            &[
                ("b", Direction::Inbound, Phase::Receiving),
                ("a", Direction::Inbound, Phase::Receiving),
                ("z", Direction::Inbound, Phase::Failed),
            ],
        );
        observe(
            &mut results,
            7,
            &[
                ("b", Direction::Inbound, Phase::Finished),
                ("a", Direction::Inbound, Phase::Receiving),
                ("z", Direction::Inbound, Phase::Failed),
            ],
        );
        observe(
            &mut results,
            7,
            &[
                ("b", Direction::Inbound, Phase::Finished),
                ("a", Direction::Inbound, Phase::Finished),
            ],
        );
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
        results.hide_rows();
        assert!(!results.has_visible());
        assert_eq!(results.pending.len(), 2);

        // Removing B from the current daemon snapshot must immediately
        // revoke its pending reveal, but A remains.
        observe(
            &mut results,
            7,
            &[("a", Direction::Inbound, Phase::Finished)],
        );
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["a"]
        );
        assert!(!results.visible("b"));
        results.dismiss();
        assert!(results.pending_empty());

        // An explicit new in-progress→Finished transition can re-arm A,
        // whereas a repeated Finished snapshot remains silent.
        observe(
            &mut results,
            7,
            &[("a", Direction::Inbound, Phase::Finished)],
        );
        assert!(results.pending_empty());
        observe(
            &mut results,
            7,
            &[("a", Direction::Inbound, Phase::Receiving)],
        );
        assert!(results.pending_empty());
        observe(
            &mut results,
            7,
            &[("a", Direction::Inbound, Phase::Finished)],
        );
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["a"]
        );
        observe(&mut results, 7, &[]);
        assert!(results.pending_empty());
        assert!(!results.has_visible());
        assert!(results.phases.is_empty());
    }

    #[test]
    fn given_live_transitions_when_finished_then_inbound_success_is_queued_once_in_observed_order_and_hide_preserves_active_work(
    ) {
        let mut results = Results::default();
        observe(&mut results, 1, &[]);
        for (id, direction, end) in [
            ("a", Direction::Inbound, Phase::Finished),
            ("b", Direction::Inbound, Phase::Failed),
            ("c", Direction::Outbound, Phase::Finished),
            ("d", Direction::Inbound, Phase::Cancelled),
            ("e", Direction::Inbound, Phase::Rejected),
            ("f", Direction::Inbound, Phase::Disconnected),
        ] {
            observe(&mut results, 1, &[(id, direction, Phase::Receiving)]);
            observe(&mut results, 1, &[(id, direction, end)]);
            assert_eq!(results.pending.len(), usize::from(id == "a"));
            observe(&mut results, 1, &[(id, direction, end)]);
            assert_eq!(results.pending.len(), usize::from(id == "a"));
        }
        observe(
            &mut results,
            1,
            &[
                ("x", Direction::Inbound, Phase::Receiving),
                ("y", Direction::Inbound, Phase::AwaitingConsent),
            ],
        );
        results.dismiss();
        observe(
            &mut results,
            1,
            &[
                ("x", Direction::Inbound, Phase::Finished),
                ("y", Direction::Inbound, Phase::AwaitingConsent),
            ],
        );
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["x"]
        );
        results.hide_rows();
        assert!(!results.visible("x"));
        observe(
            &mut results,
            1,
            &[
                ("x", Direction::Inbound, Phase::Finished),
                ("y", Direction::Inbound, Phase::Finished),
            ],
        );
        assert_eq!(
            results
                .pending
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["x", "y"]
        );
        observe(&mut results, 1, &[]);
        assert!(results.pending.is_empty());
        assert!(results.visible.is_empty());
    }
}

//! Pure presentation choices; consent and transfer ownership stay in the controller.
use rqs_lib::service::{Direction, Phase};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Drawer {
    #[default]
    Closed,
    Open,
}
impl Drawer {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "compact" => Some(Self::Closed),
            "full" => Some(Self::Open),
            _ => None,
        }
    }
}

pub fn compact_index(transfers: impl IntoIterator<Item = (Direction, Phase)>) -> Option<usize> {
    let mut active = None;
    for (index, (direction, phase)) in transfers.into_iter().enumerate() {
        if direction == Direction::Inbound && phase == Phase::AwaitingConsent {
            return Some(index);
        }
        if active.is_none() && !phase.is_terminal() {
            active = Some(index);
        }
    }
    active
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_bounded_current_transfers_when_projected_then_pending_inbound_precedes_active_and_terminal_is_not_pending(
    ) {
        // Arrange / Act / Assert
        assert_eq!(compact_index([]), None);
        for phase in [
            Phase::Queued,
            Phase::Active,
            Phase::AwaitingConsent,
            Phase::Receiving,
            Phase::Sending,
            Phase::CancelRequested,
            Phase::Finished,
            Phase::Rejected,
            Phase::Cancelled,
            Phase::Failed,
            Phase::Disconnected,
        ] {
            for direction in [Direction::Inbound, Direction::Outbound] {
                assert_eq!(
                    compact_index([(direction, phase)]),
                    if phase.is_terminal() { None } else { Some(0) }
                );
                assert_eq!(
                    compact_index([
                        (direction, phase),
                        (Direction::Inbound, Phase::AwaitingConsent)
                    ]),
                    if direction == Direction::Inbound && phase == Phase::AwaitingConsent {
                        Some(0)
                    } else {
                        Some(1)
                    }
                );
            }
        }
        assert_eq!(
            compact_index([
                (Direction::Inbound, Phase::Finished),
                (Direction::Outbound, Phase::Sending),
                (Direction::Inbound, Phase::Receiving)
            ]),
            Some(1)
        );
        assert_eq!(
            compact_index([(Direction::Inbound, Phase::AwaitingConsent); 64]),
            Some(0)
        );
        assert_eq!(Drawer::parse("full"), Some(Drawer::Open));
        assert_eq!(Drawer::parse("compact"), Some(Drawer::default()));
        for value in ["", "Full", "full\0", "unknown"] {
            assert_eq!(Drawer::parse(value), None);
        }
    }
    #[test]
    fn given_all_direction_phase_triples_when_compact_card_selected_then_consent_has_absolute_priority() {
        // Arrange: exercise every ordered triple of the public direction/phase states.
        let phases = [
            Phase::Queued, Phase::Active, Phase::AwaitingConsent,
            Phase::Receiving, Phase::Sending, Phase::CancelRequested,
            Phase::Finished, Phase::Rejected, Phase::Cancelled,
            Phase::Failed, Phase::Disconnected,
        ];
        let choices: Vec<_> = phases
            .into_iter()
            .flat_map(|phase| [Direction::Inbound, Direction::Outbound].map(|direction| (direction, phase)))
            .collect();
        assert_eq!(choices.len(), 22);

        // Act / Assert: build the oracle from two independent priority searches.
        for first in &choices {
            for second in &choices {
                for third in &choices {
                    let snapshot = [*first, *second, *third];
                    let pending = snapshot.iter().position(|(direction, phase)| {
                        *direction == Direction::Inbound && *phase == Phase::AwaitingConsent
                    });
                    let active = snapshot.iter().position(|(_, phase)| !phase.is_terminal());
                    let expected = pending.or(active);
                    assert_eq!(
                        compact_index(snapshot),
                        expected,
                        "unexpected compact card for {snapshot:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn given_terminal_prefix_and_duplicate_active_transfers_when_later_consent_arrives_then_earliest_consent_wins() {
        // Arrange / Act / Assert: priority must not be lost to early terminal
        // states, repeated active items, or a later duplicate consent entry.
        let finished = (Direction::Inbound, Phase::Finished);
        let sending = (Direction::Outbound, Phase::Sending);
        let consent = (Direction::Inbound, Phase::AwaitingConsent);
        assert_eq!(compact_index([]), None);
        assert_eq!(compact_index([finished; 128]), None);
        assert_eq!(compact_index([finished, sending, sending, finished]), Some(1));
        assert_eq!(compact_index([finished, sending, consent, consent, sending]), Some(2));
        assert_eq!(compact_index([consent, finished, consent, sending]), Some(0));
        assert_eq!(compact_index([finished, finished, consent]), Some(2));
        assert_eq!(
            compact_index(std::iter::repeat_n(finished, 128).chain([sending, consent])),
            Some(129)
        );
    }

    #[test]
    fn given_drawer_mode_strings_when_parsed_then_only_exact_ascii_contract_is_accepted() {
        // Arrange / Act / Assert: the option is a strict protocol literal, not
        // a case-folded, trimmed, prefix, or Unicode-normalized string.
        assert_eq!(Drawer::default(), Drawer::Closed);
        for (value, expected) in [("compact", Drawer::Closed), ("full", Drawer::Open)] {
            assert_eq!(Drawer::parse(value), Some(expected));
        }
        for invalid in [
            "", "Compact", "FULL", " compact", "full ", "full\\n",
            "compact\\0", "full\\0", "fuller", "compactness",
            "fúll", "ｆｕｌｌ", "full/compact",
        ] {
            assert_eq!(Drawer::parse(invalid), None, "accepted {invalid:?}");
        }
        assert_eq!(Drawer::parse(&"x".repeat(4096)), None);
    }

}

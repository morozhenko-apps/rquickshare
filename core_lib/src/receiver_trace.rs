//! Explicit local discovery diagnostics: static stage names and monotonic time only.
use std::{ffi::OsStr, sync::OnceLock, time::Instant};
#[derive(Clone, Copy)]
pub enum Event {
    Start,
    RuntimeStartRequested,
    RuntimeCoreReady,
    RuntimeDiscoveryWorkerReady,
    MdnsBrowseStarted,
    FirstMdnsRouteAdmitted,
    Visible,
    Hidden,
    AdvertisementStarted,
    AdvertisementFailed,
    AdvertisementStopped,
    AdvertisementImmediate,
    AdvertisementDeferred,
    AdvertisementPeriodic,
    GattRead,
    GattHeaderReady,
    MdnsRegister,
    MdnsResend,
    BwuOfferSent,
    BwuTcpClassified,
    BwuTcpUnmatched,
    BwuRouteConsumed,
    BwuTcpReceived,
    BwuTcpTimeout,
    BwuIntroValidated,
    BwuAckSent,
    BwuPriorDrainStarted,
    BwuPriorSafeToClose,
    BwuTcpSwitched,
}
impl Event {
    fn name(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::RuntimeStartRequested => "runtime_start_requested",
            Self::RuntimeCoreReady => "runtime_core_ready",
            Self::RuntimeDiscoveryWorkerReady => "runtime_discovery_worker_ready",
            Self::MdnsBrowseStarted => "mdns_browse_started",
            Self::FirstMdnsRouteAdmitted => "first_mdns_route_admitted",
            Self::Visible => "visible",
            Self::Hidden => "hidden",
            Self::AdvertisementStarted => "advertisement_started",
            Self::AdvertisementFailed => "advertisement_failed",
            Self::AdvertisementStopped => "advertisement_stopped",
            Self::AdvertisementImmediate => "advertisement_immediate",
            Self::AdvertisementDeferred => "advertisement_deferred",
            Self::AdvertisementPeriodic => "advertisement_periodic",
            Self::GattRead => "gatt_slot0_read",
            Self::GattHeaderReady => "gatt_header_ready",
            Self::MdnsRegister => "mdns_register",
            Self::MdnsResend => "mdns_resend",
            Self::BwuOfferSent => "bwu_offer_sent",
            Self::BwuTcpClassified => "bwu_tcp_classified",
            Self::BwuTcpUnmatched => "bwu_tcp_unmatched",
            Self::BwuRouteConsumed => "bwu_route_consumed",
            Self::BwuTcpReceived => "bwu_tcp_received",
            Self::BwuTcpTimeout => "bwu_tcp_timeout",
            Self::BwuIntroValidated => "bwu_intro_validated",
            Self::BwuAckSent => "bwu_ack_sent",
            Self::BwuPriorDrainStarted => "bwu_prior_drain_started",
            Self::BwuPriorSafeToClose => "bwu_prior_safe_to_close",
            Self::BwuTcpSwitched => "bwu_tcp_switched",
        }
    }
}
fn enabled(value: Option<&OsStr>) -> bool {
    value == Some(OsStr::new("1"))
}
fn line(event: Event, elapsed: u128) -> String {
    format!("rqs-receiver event={} elapsed_ms={elapsed}", event.name())
}
pub fn event(event: Event) {
    static CLOCK: OnceLock<(bool, Instant)> = OnceLock::new();
    let (on, start) = CLOCK.get_or_init(|| {
        (
            enabled(std::env::var_os("RQUICKSHARE_RECEIVER_TRACE").as_deref()),
            Instant::now(),
        )
    });
    if *on {
        eprintln!("{}", line(event, start.elapsed().as_millis()));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_diagnostic_flag_and_stages_when_formatted_then_only_explicit_opt_in_and_allowlisted_numeric_output_exist(
    ) {
        for value in [
            None,
            Some(""),
            Some("0"),
            Some("true"),
            Some(" 1"),
            Some("1 "),
        ] {
            assert!(!enabled(value.map(OsStr::new)));
        }
        assert!(enabled(Some(OsStr::new("1"))));
        for stage in [
            Event::Start,
            Event::RuntimeStartRequested,
            Event::RuntimeCoreReady,
            Event::RuntimeDiscoveryWorkerReady,
            Event::MdnsBrowseStarted,
            Event::FirstMdnsRouteAdmitted,
            Event::Visible,
            Event::Hidden,
            Event::AdvertisementStarted,
            Event::AdvertisementFailed,
            Event::AdvertisementStopped,
            Event::AdvertisementImmediate,
            Event::AdvertisementDeferred,
            Event::AdvertisementPeriodic,
            Event::GattRead,
            Event::GattHeaderReady,
            Event::MdnsRegister,
            Event::MdnsResend,
            Event::BwuOfferSent,
            Event::BwuTcpClassified,
            Event::BwuTcpUnmatched,
            Event::BwuRouteConsumed,
            Event::BwuTcpReceived,
            Event::BwuTcpTimeout,
            Event::BwuIntroValidated,
            Event::BwuAckSent,
            Event::BwuPriorDrainStarted,
            Event::BwuPriorSafeToClose,
            Event::BwuTcpSwitched,
        ] {
            for elapsed in [0, 1, u128::MAX] {
                let message = line(stage, elapsed);
                assert_eq!(message.split_whitespace().count(), 3);
                assert!(message.ends_with(&format!("elapsed_ms={elapsed}")));
                assert!(!message.contains('/'));
                assert!(!message.contains('"'));
                assert!(!message.contains('@'));
            }
        }
    }
}

#[cfg(test)]
mod startup_timeline_contract_tests {
    use super::{line, Event};

    #[test]
    fn given_cold_discovery_markers_when_rendered_then_names_are_distinct_and_no_peer_data_is_logged(
    ) {
        let markers = [
            (Event::RuntimeStartRequested, "runtime_start_requested"),
            (Event::RuntimeCoreReady, "runtime_core_ready"),
            (
                Event::RuntimeDiscoveryWorkerReady,
                "runtime_discovery_worker_ready",
            ),
            (Event::MdnsBrowseStarted, "mdns_browse_started"),
            (Event::FirstMdnsRouteAdmitted, "first_mdns_route_admitted"),
        ];
        let mut unique = std::collections::BTreeSet::new();
        for (event, name) in markers {
            assert!(unique.insert(name));
            assert_eq!(
                line(event, 17),
                format!("rqs-receiver event={name} elapsed_ms=17")
            );
        }
        assert_eq!(unique.len(), 5);
    }
}

//! Native outbound route ownership. A discovery hint is never peer proof.
//! The only rankable aliases are exact route generations that independently
//! completed authenticated peer binding (no provider currently wires this in).
use std::collections::BTreeMap;
use std::ops::Deref;
use std::time::{Duration, Instant};

use super::Route;
use crate::peer_authentication::VerifiedPeerCredential;

struct AuthenticatedRoute {
    peer: VerifiedPeerCredential,
    connect_latency: Duration,
    expires_at: Instant,
}

/// One owner for independently selectable exact route capabilities.
/// A later authenticated peer observation can supply bounded alias evidence;
/// an mDNS name, address or four-byte advertised ID cannot.
#[derive(Default)]
pub(super) struct RouteRegistry {
    routes: BTreeMap<String, Route>,
    authenticated: BTreeMap<String, AuthenticatedRoute>,
}

impl Deref for RouteRegistry {
    type Target = BTreeMap<String, Route>;

    fn deref(&self) -> &Self::Target {
        &self.routes
    }
}

impl RouteRegistry {
    /// After losing any proof-event observation, no cached association is
    /// trustworthy. Keep exact independent routes, revoke all shared links.
    pub(super) fn invalidate_proofs(&mut self) {
        self.authenticated.clear();
    }

    pub(super) fn insert(&mut self, id: String, route: Route) -> Option<Route> {
        // An address or fullname replacement must not inherit past proof.
        self.authenticated.remove(&id);
        self.routes.insert(id, route)
    }

    pub(super) fn remove(&mut self, id: &str) -> Option<Route> {
        self.authenticated.remove(id);
        self.routes.remove(id)
    }

    /// Returns one exact route. No speculative fanout or fallback after
    /// consent: the caller pins the result in Pending before any network job.
    ///
    /// With anonymous discovery, return the selected route unchanged.
    /// Only *both* independently authenticated, live route generations for
    /// the *same verified credential* are eligible for measured ranking.
    pub(super) fn preferred<'a>(
        &'a self,
        selected: &'a str,
        now: Instant,
    ) -> Option<(&'a str, &'a Route)> {
        let original = self.routes.get_key_value(selected)?;
        let Some(proof) = self
            .authenticated
            .get(selected)
            .filter(|proof| proof.expires_at > now)
        else {
            return Some((original.0.as_str(), original.1));
        };
        let chosen = self
            .authenticated
            .iter()
            .filter(|(id, candidate)| {
                candidate.expires_at > now
                    && candidate.peer == proof.peer
                    && self.routes.contains_key(*id)
            })
            .min_by_key(|(id, candidate)| (candidate.connect_latency, *id))?;
        let route = self.routes.get_key_value(chosen.0.as_str())?;
        Some((route.0.as_str(), route.1))
    }

    /// The runtime owner invokes this only on a live managed-session proof,
    /// for the same exact route generation and connected socket it dispatched.
    /// It is not callable from mDNS/IPC, and cannot elevate an advertised hint.
    pub(super) fn bind_authenticated_route(
        &mut self,
        id: &str,
        connected_addr: std::net::SocketAddr,
        peer: VerifiedPeerCredential,
        connect_latency: Duration,
        now: Instant,
    ) -> bool {
        const PROOF_TTL: Duration = Duration::from_secs(60);
        if !self
            .routes
            .get(id)
            .is_some_and(|route| route.addr == connected_addr)
        {
            return false;
        }
        let Some(expires_at) = now.checked_add(PROOF_TTL) else {
            return false;
        };
        self.authenticated.insert(
            id.to_owned(),
            AuthenticatedRoute {
                peer,
                connect_latency,
                expires_at,
            },
        );
        true
    }

    #[cfg(test)]
    pub(super) fn bind_test_authenticated_route(
        &mut self,
        id: &str,
        peer: VerifiedPeerCredential,
        connect_latency: Duration,
        expires_at: Instant,
    ) {
        assert!(self.routes.contains_key(id));
        self.authenticated.insert(
            id.into(),
            AuthenticatedRoute {
                peer,
                connect_latency,
                expires_at,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use super::*;

    fn route(host: u8) -> Route {
        Route {
            fullname: format!("route-{host}"),
            name: "Same label".into(),
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, host)), 4242),
        }
    }

    #[test]
    fn anonymous_collisions_always_preserve_the_exact_selected_recipient() {
        let mut registry = RouteRegistry::default();
        registry.insert("a".into(), route(10));
        registry.insert("b".into(), route(11));
        let now = Instant::now();
        assert_eq!(registry.preferred("a", now).unwrap().0, "a");
        assert_eq!(registry.preferred("b", now).unwrap().0, "b");
        assert!(registry.preferred("gone", now).is_none());
    }

    #[test]
    fn only_two_independently_authenticated_live_routes_can_be_ranked() {
        let now = Instant::now();
        let after = now + Duration::from_secs(60);
        let one = VerifiedPeerCredential::test_fixture([1; 32]);
        let another = VerifiedPeerCredential::test_fixture([2; 32]);
        let mut registry = RouteRegistry::default();
        for (id, host) in [("a", 10), ("b", 11), ("c", 12), ("d", 13)] {
            registry.insert(id.into(), route(host));
        }
        registry.bind_test_authenticated_route("a", one.clone(), Duration::from_millis(95), after);
        registry.bind_test_authenticated_route("b", one, Duration::from_millis(25), after);
        registry.bind_test_authenticated_route("c", another, Duration::from_millis(1), after);
        // Matching labels and a faster connection cannot steal an
        // unverified route or redirect a separately authenticated peer.
        assert_eq!(registry.preferred("a", now).unwrap().0, "b");
        assert_eq!(registry.preferred("b", now).unwrap().0, "b");
        assert_eq!(registry.preferred("c", now).unwrap().0, "c");
        assert_eq!(registry.preferred("d", now).unwrap().0, "d");
        assert_eq!(registry.preferred("a", after).unwrap().0, "a");

        // A route withdrawn and later readmitted loses its old proof.
        registry.remove("b");
        registry.insert("b".into(), route(14));
        assert_eq!(registry.preferred("a", now).unwrap().0, "a");
    }

    #[test]
    fn equal_measured_quality_chooses_one_stable_route_without_fanout() {
        let now = Instant::now();
        let expires = now + Duration::from_secs(30);
        let peer = VerifiedPeerCredential::test_fixture([7; 32]);
        let mut registry = RouteRegistry::default();
        for (id, host) in [("z", 10), ("a", 11)] {
            registry.insert(id.into(), route(host));
            registry.bind_test_authenticated_route(
                id,
                peer.clone(),
                Duration::from_millis(50),
                expires,
            );
        }
        assert_eq!(registry.preferred("z", now).unwrap().0, "a");
        assert_eq!(registry.preferred("a", now).unwrap().0, "a");
    }
}

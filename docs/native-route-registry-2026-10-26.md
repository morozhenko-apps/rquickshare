# Native #16 — exact-route registry, authenticated route preference and proof gap

Date: 2026-10-26. Branch `dev`. Implementation SHA `84c1b32fc6e34c0b8f5b9f379b5ae4b80c5cc28d`. Owner's FINAL UX decision: no visual changes whatsoever, no manual route picker, one selected device → at most one contacted peer, automatic best usable route **only after genuinely corroborated same-device evidence**. Incomplete or contradictory evidence remains separate. This document is an implementation checkpoint, not a claim that real Android certificates or outbound BLE support exist.

## Source changes

- `core_lib/src/service/runtime/route_registry.rs` (new): the first actual **owned Native route registry**, replacing `NativeRuntime.routes: BTreeMap<String,Route>` with a wrapper that owns the map. Incoming mDNS services still create distinct exact generation-specific route capabilities and survive unchanged in public endpoint snapshots. No GTK, Windows visual, IPC schema, settings persistence or Google wire changes.
- `RouteRegistry::insert/remove` revoke any previous authenticated association on replacement or tombstone. No trusted peer linkage can survive a changed socket or a retired route ID. The previous mDNS address-generation safety fix still enforces new IDs and fail-closed already-queued transfers.
- `RouteRegistry::preferred(selected, now)` **returns the original exact route for every anonymous or expired credential**, independently of matching names, mDNS temporary ID, addresses, device type or measured TCP reachability. For routes that have each independently obtained a valid certificate-backed association to the *same* credential, it selects the smallest observed connection latency; equal costs use deterministic exact route ID order. A faster unrelated verified peer is never considered. The winner is a single route, not a list/multicast.
- `NativeRuntime::dispatch()` uses the selector when admitting a Send. The selected actual exact endpoint ID is pinned in `Pending` **before** the first managed outbound job. Backpressure cannot trigger fresh alias search; a revoked pending route fails instead of targeting a newly advertised socket. An active connection retains its original peer.
- `core_lib/src/peer_authentication.rs::VerifiedPeerCredential` still originates only from mutual certificate verification in production. A separate `#[cfg(test)]` fingerprint constructor permits deterministic policy fixtures without exposing a way for discovery to forge verified credentials. `RouteRegistry::bind_test_authenticated_route` is **test-only**, not a production identity source. No real Android/Google session currently supplies a verified route claim; production registry therefore keeps the same exact-route behavior, which is correct and safe until the proof source exists.

## Implemented-focused test matrix

| ID | Case | Oracle |
| --- | --- | --- |
| P1 | Two independently proven routes, slower selected first | Actual NativeRuntime dispatch chooses faster proven socket, exactly one managed Send job |
| P2 | Equal quality, same verified credential | Stable deterministic one route |
| N1 | Two same-name/4-byte-ID discovered routes with no proof | Original exact selected destination preserved, zero fanout |
| N2 | Very fast independently proven different device | Cannot borrow another device's route |
| N3 | One verified and one anonymous route | No alias redirection from anonymous |
| N4 | Verifier expiration boundary | Exact original route only; no stale merging |
| N5 | mDNS tombstone/withdrawal followed by new route with same label | All former associations revoked |
| N6 | Replacement IP/port while current verified send is active | Already dispatched job retains initial socket; replacement gets fresh identity and unverified state |
| N7 | Canceled/slowed/backpressured older route | Stale queued work fails rather than following new socket (previous source tests retained) |

Actual new focused code tests:
- `route_registry::tests::anonymous_collisions_always_preserve_the_exact_selected_recipient`
- `route_registry::tests::only_two_independently_authenticated_live_routes_can_be_ranked`
- `route_registry::tests::equal_measured_quality_chooses_one_stable_route_without_fanout`
- `runtime::tests::given_two_independently_verified_routes_for_one_peer_when_sending_then_core_chooses_one_measured_best`

The integration fixture uses **synthetic verified credentials**, explicitly not Google production trust. Other tests from earlier changes cover real ECDSA certificate verification and actual mDNS/Cancel/queued-send behavior. No performance, proof validity or real Bluetooth result is implied by these synthetic route-ranking fixtures.

## Blockers for complete #16

1. **Certificate material:** real Google-compatible trusted peer certificates and refresh/revocation (not anonymous UKEY2). Google Quick Share account-backed trust cannot be inferred from advertisements.
2. **Exact-route authenticated binding:** after full mutual PairedKey `Success`, tie the credential to the actual selected live route generation and session, with a bounded expiry; propagate proof to NativeRuntime without exposing a trackable fingerprint in IPC/UI or leaking it in logs. A verified *credential* without session-to-route binding is insufficient. Ensure no stale proof inherited when mDNS data or service generation changes.
3. **BLE outbound discovery:** the existing passive BlueZ FE2C monitor sends `Sender<()>` wakeup notifications, not outbound verified endpoints; the GATT module handles *incoming* transfers. Build an addressable outbound BLE scanner/session admission adapter with distinct capabilities and cancellation ownership; integrate it with the same route registry. Do not call an FE2C sighting an established BLE route.
4. **Connection quality data:** current mDNS picks first actually reachable TCP candidate among a service's addresses. Ranking policy requires measured candidate connect latency **from genuine established routes** and independent authentication. RSSI, medium name and “Wi-Fi always faster” are not unconditional proofs. Add bounded health samples and terminal reset on disconnect. No invented values.
5. **Verified-only duplicate list:** once actual proof and route source exist, publish **one normal unchanged device row** per verified group, with one stable logical ID; exact transport selection stays backend-only. Preserve separate rows on conflicts/uncertainty and every existing widget, label, focus, size and icon. Must synchronously update coordinator endpoints, tombstone alias bindings, avoid stale consent and support two concurrent devices with identical names.
6. **Mode B / physical:** complete per-file inventory, comprehensive negative/mutation/concurrency suites, Bluetooth/Wi-Fi/WBW real Pixel and Windows acceptance, install/provenance gate and no phantom notification. `#8` A2.2 plaintext TCP BWU admission risk remains separately open.

## CI / deployment disposition

As of this documentation commit, source-exact `84c1b32f` CI had been queued but not yet read as terminal. **Do not claim 6/6 PASS** unless each required job succeeds. No local system install, package rebuild, physical Pixel/Windows test, master promotion, tag, R2 or release. This is the safe **first production-compiled registry and selector**, not finished #16.

[Owner's FINAL device/route UX and comparison](nearby-endpoint-identity-comparison-2026-10-26.md) · [#16](https://github.com/morozhenko-apps/rquickshare/issues/16) · [release gates](release-gates-live-2026-10-26.md)

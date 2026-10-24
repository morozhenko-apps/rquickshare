# P0 mDNS removal tombstones: producer / Native runtime contract

Date: 2026-10-24. MODE A corrective slice. Owner: [issue #19](https://github.com/morozhenko-apps/rquickshare/issues/19). Development branch `dev`, `master` release-only. No installed package, publication or permanent peer trust change.

## Repository scan, root cause and hypotheses

Physical symptom: 2026-10-23 Native 0.9.0-1 Devices drawer showed repeated Android-like advertised labels. This symptom alone cannot establish which advertisements correspond to the same physical peer; no anonymous route may be merged by name, mDNS fullname, IP or MAC.

Candidate causes reviewed:
1. **mDNS tombstone fields are incompatible with runtime** (verified from actual code): producer emits only `id`, leaving `fullname` empty and `present: None`; runtime rejects empty fullname and only accepts `Some(false)` for removal.
2. mDNS service refresh may advertise a different fullname/endpoint for one physical device (not yet attributed).
3. Separate BLE/mDNS/inbound route cards cannot be safely joined absent authenticated identity (#16, open).
4. mDNS SDK removal events may be delayed or missing on the real LAN (#9, unverified).
5. Restart-generation cache and subscription ordering may produce stale events (#12; race-sensitive hardware gate pending).
6. Frontend snapshots or GTK list focus/scroll may retain a row after Core retirement (#13, physical gate pending).
7. Physical Android device/Google service may advertise multiple genuine route keys (unverified).

The direct production bug is confirmed in `core_lib/src/hdl/mdns_discovery.rs::withdraw_cached_endpoints`, its `ServiceRemoved` branch, and `core_lib/src/service/runtime.rs::discovered`. The mDNS lifecycle test currently asserts `present == None` for removal, while Native runtime tests fabricate a separate `present == Some(false)` event. Thus individual tests conceal the contradictory contract. **No real Pixel reproduction or timing is claimed.**

## Invariants / implementation decision

- Removal is a **route-lifecycle notification**, not a peer-authentication proof. The exact fullname was previously accepted as the cache key, and `id` is preserved for legacy consumers.
- Every tombstone produced by both mDNS paths MUST contain the cached `fullname`, existing endpoint `id`, `present: Some(false)` and no other peer metadata. Unknown/noncached removals never publish. Stop drains once before a fresh worker publishes, regardless of map order.
- Native runtime may retire only the route whose current fullname matches. Never group by label, substitute a recipient, auto-consent or grant trust; no material changes to network/auth or IPC schema.
- Only after `coordinator.set_endpoints` succeeds, `routes.remove` can retire the visible route; a failed revision must preserve the old entry. An opt-in fixed-name stage in the existing `RQUICKSHARE_RECEIVER_TRACE=1` logger can confirm both source emission and Core retirement **without names, IP, ids or private files**. No new daemon or always-on diagnostics.
- A stale delayed tombstone for a now-reused fullname would require true SDK generation/ownership evidence (#9/#16); this scoped fix does not claim cross-generation authenticated route identity.
- Rollback all source changes together (mDNS producer/Native runtime/trace/tests); no persistent migration required.

## Branch/negative/interaction test inventory (scoped)

| Area | Positive and edge branches | Negative/recovery | Evidence target |
| --- | --- | --- | --- |
| `hdl/mdns_discovery.rs` | one/multiple cached withdrawals, unordered keys, stop twice, publish fresh after stop | unknown key, no duplicate notices, no metadata leakage, empty cache, cache entry with old blank fullname field | exact emitted event and broadcast ordering |
| `service/runtime.rs` | one/two distinct fullnames, duplicate names / identical socket destination, revocation, rediscovery new ephemeral route ID | wrong fullname / empty fullname, double removal, terminal safety, endpoint revision exhaust | real `NativeRuntime::discovered` and coordinator snapshots |
| `receiver_trace.rs` | existing flag and fixed-name source/consumer diagnostics | no peer address/name/PIN/content; exact format with 0/max elapsed, disabled flag inert | deterministic output-shape tests |
| Existing outbox/runtime lifecycle | send queued/pending while route removed, stale transfer progress, backpressure | no unwanted redirect/cross-peer or phantom send | existing runtime fixtures, no mock bypass |

Applicability: Positive plus N1 malformed/empty, N2 0/1/128 endpoints, N3 stale/missing SDK, N4 repeated stop/duplicate tombstone, N5 concurrent generation/send, N6 outdated events, N7 spoof/cross-peer selection, N8 Unicode display labels, N9 restart/revoke, N10 privacy, N11 not applicable (no monetization), N12 no local storage in the route tombstone (transfer file guards separately tested). Interactions: 2 peers × equal names/sockets × removal of one; stop old generation × new discovery; already queued selection × removed route; empty/unknown tombstone × current sender; current epoch × reissued route key. Do not advertise full coverage from one pure test.

## Execution and completion gate

- [x] Source scan, multiple-cause analysis, fail-closed route contract and rollback.
- [x] GitHub issue #19 and documentation-first plan.
- [ ] Production changes and comprehensive focused tests.
- [ ] One final source-exact Preflight (Core+Tauri+frontend/format/Clippy) and Native GTK; failed/cancelled predecessors tracked separately, not counted as tests.
- [ ] Hardware verification: Pixel disappears from long Devices list after peer goes offline; no unintended deletion of another device; reconnect yields current route. Only then consider closing #19.
- [ ] #9, #12, #16 and release 1.0 full matrix independently remain OPEN where their own acceptance is not met.

The doc commit is intentionally independent of executable/test commits. Source and tests will not alter settings, installed KDE services, user files, R2, signing, tags or protected master.

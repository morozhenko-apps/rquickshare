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
- [x] Production changes and three focused test methods committed (source plus formatting-only test correction).
- [ ] One final source-exact Preflight (Core+Tauri+frontend/format/Clippy) and Native GTK; failed/cancelled predecessors tracked separately, not counted as tests.
- [ ] Hardware verification: Pixel disappears from long Devices list after peer goes offline; no unintended deletion of another device; reconnect yields current route. Only then consider closing #19.
- [ ] #9, #12, #16 and release 1.0 full matrix independently remain OPEN where their own acceptance is not met.

The doc commit is intentionally independent of executable/test commits. Source and tests will not alter settings, installed KDE services, user files, R2, signing, tags or protected master.

## 2026-10-24 implementation and first quality-gate evidence

- Production / test source `470b1f5245714e0bcb660a3845de7e915f973db3` modifies four Core files: `hdl/mdns_discovery.rs`, `service/runtime.rs`, `service/runtime/tests.rs` and `receiver_trace.rs`. Both SDK `ServiceRemoved` and SDK-worker shutdown call a single `publish_withdrawal` contract preserving fullname/id and setting `present: Some(false)`. It omits name, IP, port and type. Unknown or duplicate events publish nothing. A successful Core retirement emits a fixed opt-in `mdns_route_retired` event; a produced tombstone emits `mdns_withdrawal_emitted` (not proof a listener consumed it).
- Three separate test methods: mDNS cached named/unknown/duplicate withdrawal, actual NativeRuntime two identically labelled/socket-routed peers with exact tombstone and re-admission, and rejected empty-key legacy tombstone. The original stop-order test now asserts exact fullname + explicit `false` on both removed routes and blank metadata. Existing pending/queued transfer withdrawal tests remain untouched.
- Initial Preflight [40080720223](https://github.com/morozhenko-apps/rquickshare/actions/runs/40080720223) exhibited a real **Rust format** failure with two multiline layouts in the new mDNS test, established by exact job `120833515337` log. No security behavior or assertion changes were justified. Corrected exactly as nightly rustfmt requested in test-only `b63a69f487c436abf166627946dc3cbb9477e1ab`.
- Final-source [Preflight 40080796430](https://github.com/morozhenko-apps/rquickshare/actions/runs/40080796430) and [Native GTK 40080796275](https://github.com/morozhenko-apps/rquickshare/actions/runs/40080796275) were **IN PROGRESS** at the source checkpoint. No success claim until an exact-source completed job and relevant test names can be verified.
- No installed-package change and no hardware Pixel/KDE acceptance. #19 remains OPEN, as do #9/#16; one route removal is not proof of authenticated identity equivalence.


## Exact compiler diagnostic and repair

Source `b63a69f4` passed nightly `Rust format`, but Core Clippy job [120833802225](https://github.com/morozhenko-apps/rquickshare/actions/runs/40080796430/job/120833802225) **FAILED** in compilation with two precise errors:

1. `E0596`, `mdns_discovery.rs`: `cache` inside `run_inner` is already an `&mut HashMap`; passing `&mut cache` unnecessarily attempts to mutably borrow the binding. Pass `cache` directly to the helper, retaining the original mutable owner.
2. `E0609`, newly added `service/runtime/tests.rs`: a `service::Snapshot` exposes coordinator revision, not an `endpoints` member. Assert the actual public `NativeRuntime::endpoints()` count (already present) and assert the authenticated client snapshot revision advances on retirement, instead of inventing a field.

Both corrections are isolated in `c384e8e2123e6c1d0eceb14f3d6b2af277f34eb7`. The mDNS/fullname + explicit `Some(false)` protocol repair, focused negative assertions and opt-in privacy trace semantics are unchanged. Source-exact CI for `c384e8e2` must still complete; do not inherit a green result from any predecessor, including the formatter-only pass or previous diagnostics.

# MODE B P0: Forged authenticated D-Bus visibility responses — 2026-10-18

## 1. Repository scan
Studied `core_lib/src/service/client/{visibility.rs,settings.rs}`, `core_lib/src/service/client.rs` bootstrap/authenticated wire, `core_lib/src/service/client/tests.rs` private D-Bus fixture, `core_lib/src/service/bus/tests/settings.rs` (real successful/timeout/uncertain visibility integration), `core_lib/src/service/visibility_service.rs` validated wire DTOs, `core_lib/src/service/settings_service.rs`, `core_lib/src/service/bus.rs` and `core_lib/Cargo.toml` (`zbus=5.19.0`). The real daemon fixture returns honest protocol bodies; these adversarial response branches remain a distinct P0 risk.

## 2. Risks and invariants
Even the correct D-Bus unique owner can reply with malformed semantics. The client must reject `Read` responses with wrong epoch, impossible available/running/mode combinations or invalid countdown. It must reject a validly typed `Set` reply when its `last_request` is not the exact submitted request, **retain** the original pending command, and reconcile only after an explicit retry yields a correctly matched receipt. A forged response never grants device discovery by default.

## 3. Test inventory (production-file map)
| Production method | P0 branch | Planned method |
| --- | --- | --- |
| `client/visibility.rs::read_visibility` | reject malformed authenticated D-Bus reply and recover on next valid read | forged view test |
| `client/visibility.rs::{set_visibility,retry_visibility,has_pending_visibility}` | valid envelope + mismatched successful receipt retained through retry | forged receipt test |
| `client/settings.rs::checked_settings` | verify bounded SettingsView and no FDs from D-Bus | shared path in forged receipt test |
| `client.rs::{connect,wire}` | same-UID broker, unique owner, private bus bootstrap | real test-only Zbus service adapter |
| `bus.rs` real daemon implementation | already tested successful live visibility, durable and uncertain write, expiry | no duplicate |

Exactly **two** new asynchronous tests; a test-only fake manager/operations/visibility service implements just the normal bootstrap and correct D-Bus methods, but can supply adversarial reply bodies. This is **transport-faithful protocol injection**, not a fake client/domain validation.

## 4. Branch map
`read_visibility`: valid tuple decode -> `VisibilityView::valid(epoch)`; false -> `Protocol`; later valid -> success without pending commands. `set_visibility`: validate current intent -> pending -> `retry_visibility`; matched typed settings -> clear pending; mismatched `last_request` -> `Protocol` with unchanged pending. `checked_settings`: valid typed SettingsView -> accepted, invalid epoch / out-of-range fields -> `Protocol`; `wire`: connection and unique sender authenticity enforced.

## 5–6. Positive / N1–N12 / interactions
| Test | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Forged Read | follow-up valid response | wrong epoch/mode | timer 60_001 | actual private bus | repeated forged responses | serialized client | N/A persistent storage | reject unavailable-running | no external clock | valid read after invalid | do not trust malicious discovery state | N/A | N/A |
| Forged Set | corrected receipt succeeds | wrong 32-byte request identity | valid envelope, wrong association | actual private bus | two retries, unchanged command | no concurrent client mutation | N/A persisted state | client-owner-only command | N/A | explicit reconciliation only | refuse foreign receipt | N/A | N/A |

Meaningful interactions: `wrong epoch / impossible visibility / countdown bounds` x `same authenticated sender`, plus `valid receipt envelope` x `wrong or correct request` x `pending lifecycle`. No sleeps or physical adapters.

## 7–8. Coverage estimation / file map
Two new test functions, several invalid response cases, real `zbus` transport. Code changes: `core_lib/src/service/client/tests/visibility_forged.rs` (test-only), `core_lib/src/service/client/tests.rs` (test module declaration), this document. Production semantic source files not modified. No measured per-file line or branch percentage yet.

## 9–10. Implementation and mutation review
Implement the specified service in test-only code through `#[zbus::interface]` and a private `dbus-daemon`; insist on full authenticated client bootstrap via `NativeClient::connect` rather than invoking validators directly. Mutation targets: bypass view check, accept wrong receipt, clear pending on protocol ambiguity, carry stale reply across explicit retry, accept invalid epoch. Manual mutation hypotheses only; no mutation engine execution is claimed.

## 11–12. Execution and completion gate
Exact-source Core Rust, rustfmt, Clippy, Native GTK, Tauri and frontend jobs required before scoped PASS. Full MODE B `mode-b.yml` strict 10x, current-source instrumented coverage, additional hardware BLE/Android/mDNS and global Stage 12 remain OPEN. No `master` merge, release or production behavior change. Rollback: remove these test-only additions.

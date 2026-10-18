# MODE B P0: mDNS/GATT identity generation acknowledgements — 2026-10-18

## Stages 1–2 — source scan, risk and invariants
Files scanned: `core_lib/src/identity.rs`, `core_lib/src/identity/tests.rs`, `core_lib/src/hdl/{mdns.rs,gatt.rs}`, `docs/mode-b-completion-audit-2026-10-18.md`. Test-only additions to `identity/tests.rs`. The owner's display name must not be reported applied until *both* required consumers have acknowledged the exact latest revision. Rapid renamed generations, stale acknowledgements and dropped services cannot falsely complete the latest generation. Existing tests cover single rename, duplicate subscription, basic two-service acknowledgement, one superseded pending waiter, and single-service drops; these cases extend meaningful interactions instead of duplicating them.

## Stage 3 — inventory and estimation
| Production functions and branch | Risk | Level | New tests |
| --- | --- | --- | ---: |
| `IdentityPolicy::{set_override, applied, wait_applied}` + `IdentityConsumer::{changed, snapshot, acknowledge}`: skipped generations and stale acknowledgements from mDNS/GATT | P0 | Deterministic async unit | 1, two consumer acknowledgement orders |
| `IdentityConsumer::drop` + `IdentityPolicy::{applied, wait_applied}`: service loss after previous success and after revision increment | P0 | Deterministic async unit | 1 |
| Actual BlueZ/GATT advertising, mDNS registered service, Android discovery | P0 | hardware/integration | OPEN (not simulated or claimed) |

## Stage 4 — branch map
- Current name unchanged -> no revision, changed -> checked increment, multiple rapid writes supersede intervening revisions.
- Subscribe required mDNS and GATT; each consumer starts at `Ack::Running(0)`.
- `acknowledge(revision)` accepts nonregressing revisions only, may lag latest; `applied(latest)` remains false until both equal latest; `applied(old)` must return `Superseded`.
- `wait_applied` first polls Pending then resolves only after second latest acknowledgement, independent of mDNS/GATT completion order.
- Already-applied current generation becomes `Unavailable` when a required consumer drops; subsequent rename cannot return success from the surviving peer alone.

## Stages 5–6 — Positive/N1–N12 and interactions
| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Rapid revisions | planned exact latest | N/A typed `DeviceName` | planned rev 1,2,3,4 | N/A local watch | planned stale/repeat | planned consumer order | planned skipped revisions | planned two required peers | N/A no clock | planned supersede | planned no false visible status | N/A no billing | N/A no FS |
| Service drops | planned prior success | N/A typed | planned before/after generation | planned service disappearance | planned failure persists | planned surviving peer ack | N/A no persisted data | planned fail closed | N/A no clock | planned dropped consumer | planned no false success | N/A | N/A |

Meaningful cross-product: consumer order {mDNS then GATT, GATT then mDNS} × latest generation {1,4} × stale ack {2,3} × service presence {both, one dropped}. Two new named test methods, no real network, mocks, timers, sleeps or OS integration.

## Stages 7–10 — coverage and mutation design
Coverage map: `core_lib/src/identity.rs` generation state and watcher confirmation; no new coverage claimed for `hdl/mdns.rs`, `hdl/gatt.rs` external event loops. Mutation targets: accepting the first consumer only, treating stale acknowledgement as latest, changing `all()` to `any()`, auto-recovering a dropped service, reporting success for superseded generations. Assert independently against the exact `IdentityError` and pending/success states. This is manual mutation analysis, not an executed mutant score.

## Stages 11–12 — execution and completion gates
Add planned tests in `identity/tests.rs` only. Verify exact-source Core Rust, Clippy, rustfmt and Native GTK jobs; do not declare globally complete. Full 10× strict runs, hardware integration and repository-wide per-production-method coverage remain OPEN. Rollback removes only two tests and this document.

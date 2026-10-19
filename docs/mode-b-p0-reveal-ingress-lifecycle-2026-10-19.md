# MODE B P0: Native GTK reveal ingress ownership lifecycle — 2026-10-19

## Stage 1. Repository scan
Baseline test code `291b7733`: [Preflight 39252238333](https://github.com/morozhenko-apps/rquickshare/actions/runs/39252238333) and [Native GTK 39252238328](https://github.com/morozhenko-apps/rquickshare/actions/runs/39252238328) both successful. Read `native_desktop/src/panel/reveal.rs` and `panel/reveal/tests.rs` (current reveal identity matcher and GTK widget smoke), `native_desktop/src/ingress.rs` (owned per-operation worker and `reveal_job`), `native_desktop/src/portal.rs` (strict FD-only desktop portal), `core_lib/src/service/client/controller.rs` (real Transfer handles and permissions), and [same-SHA measured coverage](mode-b-coverage-baseline-2026-10-19.md). `panel/reveal.rs` was 10.87% lines and `ingress.rs` 74.73% lines at coverage SHA `99c26335`.

## Stage 2. P0 risk and invariants
An explicitly initiated file reveal must not lose its permit, start another concurrent reveal, or outlive an owner shutdown. Dropping the GUI reply receiver must NOT terminate the background operation or release its permit early. An operation cancelled by its owner must return the precise portal `Cancelled` error and release the gate only after the operation completes; shutdown must join it before returning. A portal rejection must be returned unchanged, and gate re-admission must occur only after completion. Real portal interface and client-side result/FD authority are separately validated elsewhere; these tests target actual `Ingress::reveal_job` and `PreparationGate`, with the **external portal future controlled only at the dependency boundary**.

## Stage 3. Scoped exhaustive inventory (P0/P1)
| Production method / branch | Risk | Level | Planned tests |
| --- | --- | --- | --- |
| `Ingress::reveal_job` success after external completion | release permit before responding, restore admission, keep accurate `IngressState.busy` | P0 integration, real runtime and gate | 1 named test |
| `Ingress::reveal_job` error return `portal::Error::Protocol` | preserve exact rejection, never report success | P0 integration | included in second named test |
| `Ingress::reveal_job` abandoned GUI receiver + explicit cancellation | worker ownership retained, permit held, shutdown joins, closes gate permanently | P0 integration | 1 named test |
| `Ingress::start_reveal` / `start_directory_reveal` real desktop portal operations | correct FDs, owner, user-prompt response, no accidental file handler | P0 live D-Bus/portal integration | OPEN: not covered by fake portal success |
| `Panel::reveal_target` state/transfer/current-key and final GUI guard | stale async completion after disconnection or lock cannot open file | P0 actual GUI integration | OPEN; genuine `Transfer` handle and controller + portal needed |
| `portal::show_directory_contents` descriptor mode protections | FILE/directory/FD flags, no handler launch | P0 unit/integration | existing guards; separate direct real-FD test feasible later |

## Stage 4. Branch map
`reveal_job`: previous task absent -> execute work -> `state.busy=false` -> drop permit -> send result; receiver present -> result delivered; receiver dropped -> result send fails but worker still completes; shutdown with pending future -> waits until completion and returns with gate closed; after normal result -> gate can be acquired. Explicit external future error must remain `Err(Error::Protocol)` (not converted to `Cancelled`/Ok). Gate's single-owner contract is asserted while external work is pending. The existing real worker join/cancellation and directory FD tests provide sibling infrastructure branches; don't duplicate them.

## Stages 5–6. Positive/N1–N12 + interaction map
| New method | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Reveal result/permit | Ok and exact Error::Protocol | Err propagation | one active permit | real Tokio runtime, no portal network | after completion reacquire | sequential worker, no time race | state busy true -> false | cannot acquire before finish | no time-based expectations | GUI reply still connected | no file body inspected | N/A | no real filesystem |
| Dropped GUI + shutdown | owner close after worker done | cancelled external dependency | receiver dropped | real tracked task | no early completion | concurrent close + pending work | busy and permanent gate close | prevents new work | deterministic event barrier | abandoned GUI | no body/file leak | N/A | no actual file writes |

Independent interaction factors: receiver {present,dropped} × dependency result {Ok,Protocol,Cancelled} × owner {open,closing} × permit {held,released} (planned relevant combinations only; never require unreproducible sleeps). Exact new named tests: **2**; existing image/directory ingress tests remain.

## Stages 7–8. Per-production-file coverage map
Test-only additions: `native_desktop/src/ingress.rs` `#[cfg(test)]` module, targeting production `Ingress::reveal_job` and `Ingress::close`. Adjacent target `panel/reveal.rs` uses `Ingress::start_reveal` but **its line coverage does not increase directly** by testing the ingress adapter. No percentage claims without new llvm-cov on exact code SHA. Hard-to-test `Panel::reveal_target` retains a distinct P0 outstanding item requiring a real panel/controller fixture or MODE A seam decision; do not invent fake Transfer handles.

## Stages 9–10. Implementation / mutation review
Create two `#[tokio::test]` with `tokio::sync::oneshot` barrier signals and cancellation token. Use the production `Ingress::new`, `reveal_job`, `PreparationGate::try_acquire`, `watch` state, `close` and Tokio current runtime. Only the **external portal future** is substituted, as permitted for dependency contract tests. No manual sleeps or OS connection; use signal barriers to demonstrate the join boundary deterministically. Mutation targets: premature permit drop; skip previous task wait; close not joining; dropped reply aborting worker; incorrectly mapping portal rejection; marking busy=false before work finishes. Exact mutation score not run.

## Stages 11–12. Gate
Source-exact Native GTK strict lint/isolated unit tests required, plus Preflight if triggered. Ten full runs, source-exact coverage re-measure, actual KDE portal, BLE/Android/mDNS hardware and global Stage 12 **OPEN**. Remain in `dev`, no `master`/release/production changes. Rollback only test additions and this plan.

# MODE B P0: Native visibility client preflight and daemon replacement (2026-10-18)

## Stage 1: repository scan
Mode B, **tests only**, on `dev`. Sources: `core_lib/src/service/client/visibility.rs` (read/set/retry and exact pending intent), `core_lib/src/service/client.rs` (authenticated D-Bus `wire`, test-only reply hook and lifecycle), `core_lib/src/service/client/tests.rs` (actual private-bus harness), `core_lib/src/service/bus/tests/settings.rs` (existing real visibility integration and lost-reply retries), `core_lib/src/service/visibility_service.rs` (wire intent/view validation). No new transport abstraction or domain mocks.

## Stage 2: risk and invariants
P0: malformed visibility intent must be rejected **locally, before any outbound mutation or pending intent is stored**. If daemon identity changes while a valid visibility command is retained at a deliberate pre-send cancellation barrier, the client must fail closed with `DaemonChanged`, keep exact pending intent for explicit reconciliation, and must never deliver it to a replacement daemon. While pending, a second visibility command must fail with `PendingOperation`. This is a client-owned control transaction; no implicit replays or timed discovery extension.

## Stage 3: exhaustive scoped artifact inventory
| Real method / branch | Risk | Level | New tests |
| --- | --- | --- | ---: |
| `NativeClient::retry_visibility` with no pending | P0 | real private-bus client | 1, combined with local reject matrix |
| `NativeClient::set_visibility` invalid current-epoch token/request/mode and no pending | P0 | real private-bus client | 1, 17 table-driven variants including boundary lengths and case |
| `NativeClient::{set_visibility,retry_visibility,has_pending_visibility}` valid intent interrupted **before** `wire` and daemon owner replaced | P0 | real private-bus client + deterministic owned reply hook | 1 |
| `NativeClient::read_visibility` actual unavailable-visibility reply | P0 | existing real private-bus fixture | Assert in both methods |
| `NativeClient::retry_visibility` lost completed reply, indeterminate disk write, expiry, renewed timer, rejected not running | P0 | existing `bus/tests/settings.rs` | 0, source-confirmed; avoid duplication |
| Malformed-but-authenticated `Read` reply and wrong `Set` receipt | P0 | controlled external daemon | OPEN: native fixture supplies honest production reply; protocol-forgery requires explicit controlled external D-Bus service fixture and separate inventory |

## Stage 4: branch map
`set_visibility`: pending visibility/settings => `PendingOperation`; invalid epoch/mode/token/request => `InvalidInput` before retaining; valid => retain then retry. `retry_visibility`: no pending => `InvalidInput`; pending + owner mismatch => `DaemonChanged`, retained; success/rejection/indeterminate/other failures handled in existing integration. `read_visibility`: honest daemon unavailable returns bounded view, and no mutation occurs on local validation errors. `wire`: hook before outbound send; authenticated owner change checked before send. Already-tested other client operations are out of scope.

## Stages 5–6: Positive/N1–N12 and interaction matrix
| Flow | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Local admission | Valid intent admits pending (second method) | Planned wrong epoch, modes, bad ID chars/case | Planned token 0/63/64/65, request 0/31/32/33 | N/A local validation before network | Planned repeated rejects, empty retry | N/A single client borrowed | N/A no persisted records | Planned canonical ID refusal | N/A no clock | N/A no UI | Planned no private intent in debug | N/A | N/A no persistence |
| Owner change | Planned authenticated private bus | N/A validated before send | N/A one intent | Planned old bus service replaced | Planned unchanged retained command, never auto-replayed | Planned explicit pre-wire cancellation barrier | N/A no stored protocol versions | Planned epoch and owner isolation | N/A no timed status | Planned interrupted future | Planned no foreign daemon mutation | N/A | N/A no disk |

Interaction: `bad field` × `no pending` (17 variants); pending valid × second command × host stop × new daemon epoch × explicit retry; baseline unavailable view before/after. Single client mutable borrow prevents ambiguous interleaving. No sleeps, random timestamps or production mocks.

## Stages 7–8: expected count and per-production-file coverage
**Exactly two** named async test methods under `core_lib/src/service/client/tests.rs`. Covered: `core_lib/src/service/client/visibility.rs` methods `read_visibility`, `set_visibility`, `retry_visibility`, `has_pending_visibility` scoped admission and owner-change branches. Indirect: `core_lib/src/service/client.rs::wire` owner authentication and test-only pre-wire barrier. Existing real D-Bus workflows cover successful and uncertain saved visibility. Uncovered: deliberately forged daemon replies and wrong-response receipt; explicit task remains after this suite.

## Stages 9–10 implementation and mutation review
Implement every planned variant with `Given_When_Then` names, real private bus and existing `hook/interrupted` barrier. Assert error enums, pending state, unchanged visibility snapshot and no replacement-owner side effects. Mutation targets: accept malformed intent, clear the pending command on replacement, allow a second command while pending, silently send to new owner, automatically retry on daemon change. Mutation review is reasoning, **not** a measured mutant score.

## Stages 11–12 execution and completion
Source-exact Preflight Core Rust + core Clippy + rustfmt + frontend/Tauri, plus native GTK. Ten **complete** consecutive `mode-b.yml` runs, current-source instrumented file coverage, all public branch categories, and physical BLE/mDNS acceptance remain OPEN regardless of scoped success. Rollback = remove these two test methods and this document; no production changes, no merge to `master`.


## Stage 9 implementation checkpoint (source SHA `e45b89ee`)
- Plan committed before tests: `c37075be`.
- Two named real-private-bus methods in `core_lib/src/service/client/tests.rs` at `7ac3c8db`, followed by a **formatting-only** correction at `e45b89ee`. First method covers 17 local invalid-admission variants, no pending state, unchanged daemon-visible state, empty retry, and redacted intent debug. Second retains one valid intent with the existing pre-wire hook, verifies second-action blocking, replaces the actual D-Bus service owner with epoch 456, checks two refused retries and no mutation at replacement owner.
- First Preflight's Rust format step flagged one multiline assertion. No product logic was changed; no assertions were relaxed. CI re-run for exact source: [Preflight 39168715549](https://github.com/morozhenko-apps/rquickshare/actions/runs/39168715549) and [Native GTK 39168715424](https://github.com/morozhenko-apps/rquickshare/actions/runs/39168715424). Neither is declared successful at this checkpoint.
- Mutation candidates: bypass local `valid()`, replace `PendingOperation` guard with last-write-wins, clear `pending_visibility` on `DaemonChanged`, silently send to new owner, and expose private token/request in `Debug`.
- Coverage note: authenticated but intentionally forged daemon `Read` responses or a forged successful `Set` receipt are **not** tested by this existing honest-bus fixture. They remain separate P0 testability/injection scenarios; this suite must not be labeled exhaustive for that branch. Full MODE B 10x and Stage 12 remain OPEN.


## Stage 11 exact-source evidence (source SHA `e45b89ee`)

- [Core Rust tests, Preflight 39168715549](https://github.com/morozhenko-apps/rquickshare/actions/runs/39168715549): **SUCCESS**, 763 passed / 0 failed / 0 ignored / 8 separately filtered race tests, plus two integration binaries with 4 passes each. Both new `service::client::tests` names explicitly printed `ok`.
- Same source: **Rust format, frontend and core_lib Clippy SUCCESS**.
- [Native GTK 39168715424](https://github.com/morozhenko-apps/rquickshare/actions/runs/39168715424): **SUCCESS**, 80 passed / 0 failed.
- Tauri Clippy and desktop tests had not yet finished at the evidence checkpoint. Preflight overall conclusion must be verified separately before claiming green.
- Strict 10× complete-suite test, instrumented coverage per current production file, forged daemon-response injection, mutation-engine scores and hardware acceptance are still OPEN, so global Stage 12 is NO.

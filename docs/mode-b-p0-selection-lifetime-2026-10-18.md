# MODE B P0 — real broker selection lifetime, foreign owner and replay protection

Date: 2026-10-18. Primary MODE B (Testing), branch `dev`.
Strictly test-only in `core_lib/src/service/client/controller/tests.rs`.
No change to production authority or security, protocol, services,
application, `master`, releases or installed packages.
Global Mode B Stage 12 remains NO.

## Stage 1 — scan and baseline

The user-supplied `agent.md` 378-line testing SSOT is binding.
Read `docs/mode-b-inventory.md`, `docs/current-state.md`,
`docs/mode-b-p0-native-async-clipboard-2026-10-18.md`,
`core_lib/src/service/client.rs`,
`core_lib/src/service/client/controller.rs`,
`core_lib/src/service/client/controller/tests.rs`,
`core_lib/src/service/client/tests.rs`,
`core_lib/src/service/client/tests/batch.rs` and
`native_desktop/src/panel/outbound.rs`.

The controller tests already create an actual `NativeClient`,
`PanelController` and `SelectionBus` on a private D-Bus broker.
The source file has 11 test methods, including explicit
`choose` then `send`, close/release, receipt recovery,
interrupted batch/text acquisition, anonymous memfd content,
and extension. The client tests already exercise a foreign
`Upload` being rejected and invalid recipient names.
Missing focused *cross-controller tests* prove only an
actual owner may use a prepared `Selection` for
`send_selection`/`queue_recipient`/`send_many`,
and a stale selection handle from a closed GUI cannot
be replayed by a newly connected client while a
new genuine selection still works.

## Stage 2 — P0 risks/invariants

- `Selection` is a secure, client-connection-bound
  `{id, connection}` handle with nonpublic constructor.
  Never invent, forge or mock a Selection. Create it
  only by real `PanelHandle::choose` and the
  actual daemon SDK/SCM_RIGHTS path.
- The remote broker must not enqueue any transfer
  when an unrelated client attempts to use another
  connection's Selection (single, batch or queue).
  The SDK returns exactly `ClientError::ForeignHandle`
  without creating a pending operation or
  mutating any owner state. The correct owner
  must remain able to send once.
- After controller close, its unsent selection
  must release its handle ownership. A
  *new client* cannot reuse the old Selection.
  A newly prepared Selection under the new
  client may send to a legitimate discovered
  endpoint. Preserve ownership, queue order,
  byte totals and original file integrity.
- This integration uses a private actual broker
  but does **not** prove end-to-end GTK
  `finish_prepared(Some(Selection))`, actual
  Android delivery, BlueZ or authenticated
  BLE-to-TCP alias continuity; those remain P0 OPEN.
- No sleeps, no mocked app-domain methods,
  no relaxed assertion, no production changes.

## Stage 3 — scoped testable artifact inventory

| Production branch / artifact | Risk | New method |
|---|---|---|
| `NativeClient::mutation`: Send, QueueMany and SendMany foreign `Selection` owners, early validation and no pending op; real private broker ledger | P0 connection authority | 1 test with 3 API variants |
| `PanelController::choose`/close, `NativeClient::send_selection` after client identity replacement; new genuine selection can still transfer | P0 replay + lifecycle | 1 test with old/new owner |
| `Panel::finish_prepared(Some(Selection))` GTK/UI arrival vs closing/rebind and actual platform clipboard owner loss mid-read | P0 GTK + real OS | OPEN; a live connected Panel/Wayland fixture is not present |

**Exactly 2 original Given_When_Then tests**, not
an additional mock/broker implementation.

## Stage 4 — test branch map

1. `given_foreign_prepared_selection_when_other_client_sends_queues_or_batches_then_no_transfer_is_admitted_and_owner_can_send`.
   Arrange: one real controller, real `OpenedSelection`,
   capture original Selection, second NativeClient on
   same PrivateBus, try single, queued and batch
   operations with original handle. Each must
   return `ForeignHandle` and leave `has_pending=false`.
   Assert no queued transfer on original view
   and exact one new queued send after owner sends.
2. `given_closed_preparation_owner_when_new_gui_receives_old_selection_then_replay_is_denied_and_new_selection_sends`.
   Arrange: real controller prepares a Selection,
   then closes before sending; new NativeClient
   connects to same private broker. Old Selection
   yields `ForeignHandle` and no daemon mutation.
   New controller chooses a new legitimate source;
   explicit send succeeds exactly once and survives
   GUI close. Verify old file still exists, new
   queue contains only new source bytes.

## Stage 5 — Positive and N1–N12 matrix

P planned new, C prior baseline, X not applicable to
this bounded source path, OPEN separate integration.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Foreign Selection APIs | C+P legitimate send | P foreign token | P 1 vs batch | X no link loss | P repeated reject | P 2 clients | X real immutable token | P exact connection owner | X no locale | P failed send | P no cross-client bytes | X no monetization | P no leaked handles |
| Controller close/reconnect | C+P new send | P stale token | X no length bound | P owner disconnect | P old not replayed | P old/new clients | X token created by daemon | P change of owner | X no clock | P close/return | P no reuse | X | P retain original file |
| Physical GTK/Wayland + Google transfer | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | X | OPEN |

## Stage 6 — interaction matrix

Owner identity {current owner, unrelated connected
owner, closed former owner, new accepted owner}
× action {single send, queue-recipient, batch send}
× file source {original pre-close, fresh after close}
× terminal {rejected as foreign, successful single
queued transfer}. No forged `Selection`. All
meaningful testable combinations represented in
the 2 proposed methods.

## Stage 7 — count estimates

2 new Rust integration test methods.
Positive 2, N1 2, N2 1, N3 1, N4 2,
N5 2, N6 0 X, N7 2, N8 0 X,
N9 2, N10 2, N11 0 X, N12 2.
Overlapping method attribution, not independent
methods/percentage. Real broker I/O, not a
copy of the domain model.

## Stage 8 — production-file map

`core_lib/src/service/client.rs`:
`owned`, `mutation(Command::Send)`,
`Command::QueueMany`, `Command::SendMany`
and `send_selection` with real decoded handle.
`core_lib/src/service/client/controller.rs`:
`PanelHandle::choose`/send, `PanelController::close`,
state projections and SDK client lifetime.
`core_lib/src/service/bus/`: real private
bus host and actual daemon coordinator
state transitions (only bounded cases).
`native_desktop/src/panel/outbound.rs`:
no false GTK coverage claimed; still requires
a live connected panel + controlled asynchronous
clipboard loss.

## Stage 10 — mutation plan

Accept foreign connection identity, validate
only canonical Selection ID, enqueue on failed
authorization, mark unexpected pending operation,
retain stale selection after close, reject
fresh valid owner after a foreign attempt,
or replay old bytes: direct assertions must fail.
Manual mutation review only; no mutation engine.

## Milestones and rollback

1. [x] Stage 1–8 repository scan, risk, inventory,
   branch/category/interaction/count and
   per-production-file map written first.
2. [x] Add both original controller/broker tests.
3. [x] Run exact-source Core Rust, Core Clippy,
   formatter, Preflight and native GTK and
   inspect actual test-by-name logs.
4. [x] Update D2 actual and red-state triage,
   mutation review, Current State and global inventory.
5. [ ] Global ten consecutive *complete* Mode B
   and Stage 12 all-production-branch proof OPEN.

Rollback only test source and docs. No
production security policy changed.

## First test implementation / red CI triage

Added the two original controller/broker methods,
test-only SHA `11ae9bd7`. Initial
[Preflight 39129305604](https://github.com/morozhenko-apps/rquickshare/actions/runs/39129305604)
reports **Rust formatter failure** on three
format-only differences (one extra blank line,
two multiline assertion layouts and one short
`let ... else` construction) in the new
`controller/tests.rs` test region. No test
assertions or product logic have been changed.
The formatter job could not prove the new
test results; do not count this run as a
clean same-source Preflight. Apply only
the exact rustfmt-recommended line layouts
then rerun. The security invariant remains
unchanged: three wrong-owner admission
methods must reject before pending or queue,
and a new owner must work after old owner
close. No architecture/security decision.

## Scoped D1–D5 results and verified CI

**D1 — execution.** The two test-only methods
were added to `core_lib/src/service/client/controller/tests.rs`
at SHA `11ae9bd7`; its first CI formatter job
reported four layout differences in the
new methods and is historical RED evidence.
A test-only rustfmt correction `e5731932`
applied the exact expected layouts, with
all business assertions unchanged.

The actual Core job `117681317085` in
[Preflight 39129396907](https://github.com/morozhenko-apps/rquickshare/actions/runs/39129396907)
at SHA `e573193261863da6c65a6a38983f539c6ae78f16`
completed **SUCCESS**. Full job log inspected:
stable Core **742/742 passed, 0 failed,
0 ignored** (8 race-sensitive tests filtered by
the existing quick profile); existing
Core integration bins **4/4 + 4/4**.
Each of the two new unique method names appears
**exactly once** as `... ok`.
Rust format, frontend and Core Clippy
jobs finished SUCCESS. The desktop Tauri
Clippy/unit job also completed **SUCCESS**;
its actual job `117681317266` log has
**10 passed, 0 failed, 0 ignored**.
The complete same-source quick Preflight
finished **5/5 jobs SUCCESS**.

Independent
[Native GTK 39129396869](https://github.com/morozhenko-apps/rquickshare/actions/runs/39129396869)
**SUCCESS** on identical test SHA. Its full
job log `117681299538` shows native
**57/57 passed, 0 failed/ignored**, including
previously covered GDK/GTK fixtures unchanged.

**D2 — executed category assignment** (two
new original Rust test methods, overlapping
category assignments; not coverage percentages):

| Category | New methods | Actual scoped result |
|---|---:|---|
| Positive | 2 | PASS: intact original and replacement owner's explicit send |
| N1 invalid token | 2 | PASS: foreign/closed-owner Selection denied |
| N2 batch boundary | 1 | PASS: single/queue/batch entry points deny foreign |
| N3 external failure | 1 | PASS: owner connection closes while daemon remains |
| N4 duplicate/replay | 2 | PASS: no pending operation/transfer after rejected attempt |
| N5 concurrency | 2 | PASS: two real clients and successive owner identities |
| N6 corrupt data | 0 | N/A: Selection is daemon-minted immutable typed handle |
| N7 authority/security | 2 | PASS: exact per-connection authorization |
| N8 locale/time | 0 | N/A: no time/locale |
| N9 interruption | 2 | PASS: sender/controller exit before transfer |
| N10 privacy/cross-client | 2 | PASS: old/foreign source not enqueued |
| N11 monetization | 0 | N/A: no billing |
| N12 storage/FD | 2 | PASS: original local paths remain, no leaked foreign queue |

**D3 — files modified.**
Only `#[cfg(test)]` `controller/tests.rs`
and this documentation ledger/global indexes.
No production behavior, API, protocol or
installed software changes.

**D4 — testability.** Existing real private
D-Bus broker and real SDK/client/controller
already support strict lifecycle tests
without a new injected production seam.
Actual GTK connected `Panel`, Wayland
clipboard ownership loss, D-Bus to
external Quick Share peer, BLE→TCP
identity authentication and Windows
hardware remain separate P0 gaps.

**D5 — mutation review and residual risk.**
Replacing `owned` identity rejection with
canonical-id-only acceptance, marking
foreign rejects pending, accidentally
enqueuing a rejected foreign send, retaining
an abandoned Selection after controller
close, or blocking fresh correct sending
would contradict explicit assertions.
This is manual sensitivity review
only. Not every daemon authorization branch,
race-sensitive case, negative N1–N12
interaction or complete final-source 10×
has been proven. Full Stage 11 and
global Stage 12 remain **NO**.

## Final same-source quick CI checkpoint

Both [Preflight 39129396907](https://github.com/morozhenko-apps/rquickshare/actions/runs/39129396907)
and [Native GTK 39129396869](https://github.com/morozhenko-apps/rquickshare/actions/runs/39129396869)
completed **SUCCESS** on
`e573193261863da6c65a6a38983f539c6ae78f16`.
Preflight jobs **5/5 SUCCESS**;
Core **742/742 stable** + two **4/4**
integrations, desktop **10/10**;
GTK **57/57**. Both new original
Core methods independently `... ok`.
The initially red rustfmt run was
corrected test-only and is retained
in the audit trail. Later doc commits
do not modify tested source.

Still **NO** for full final-source
10 consecutive complete MODE B
runs, live Linux/Google Android/
Windows system acceptance,
BWU alias authentication Issue #8,
live mDNS lifecycle Issue #9 and
global Stage 12 completeness.

# MODE B P0: Native GTK outbound clipboard URI and recipient authorization

Date: 2026-10-18. Selected mode: **MODE B / Testing**.
Target branch: `dev`. Test-only `native_desktop/src/panel/outbound.rs`;
no production contract, D-Bus/Wayland implementation, network,
`master`, release or installed package change.

## Stage 1 — source scan and guide

Read user-supplied `agent.md` universal testing SSOT,
`docs/mode-b-inventory.md`, `docs/current-state.md`,
`docs/native-gtk-coverage-baseline-2026-10-17.md`,
`native_desktop/src/panel/outbound.rs`,
`native_desktop/src/panel.rs`,
`native_desktop/src/panel/tests.rs`,
`native_desktop/src/desktop_guard.rs`,
`core_lib/src/service/client/controller.rs`, and
`core_lib/src/service/client/state.rs`.
Existing `given_overlapping_formats...` checks all
eight clipboard MIME-precedence combinations.
Existing `given_copied_file_lists...` probes a
small happy/negative set and count 256/257.
`outbound::verify(panel)` is invoked within the real
native GTK widget smoke; it checks stale target,
disconnected and pending but not the full fail-closed
`input_allowed` matrix. The source uses GLib/GIO to
parse local clipboard URIs and a runtime controller
to validate recipient/selection after acquisition.

## Stage 2 — invariants and P0 risks

The input parser `paths(body, gnome)` must reject
oversized or malformed bytes atomically, never
return a partial set after a later bad URI,
and admit **local filesystem paths only**. GNOME
`copy` and `cut` first lines are MIME-specific
commands rather than real paths; ordinary URI-list
has no such header. Preserve exact selected item
order, percent-decoded characters and localhost
case-insensitive authority.

`Panel::input_allowed` is the final GTK gate
for user-initiated outbound selection. It must
fail closed when the panel is closing, preview,
locked while safety enforcement is enabled,
busy/pending, stale or unsynchronized daemon
epoch, non-running lifecycle, absent endpoint
or wrong recipient ID. The existing live
`outbound::verify(panel)` smoke has a real
panel and a synchronized (preview-only test)
`StateView`; extend it without mocking
`input_allowed` or changing the model's
production method.

Tests will not claim that the GTK clipboard
OS service, actual portal file chooser,
real cross-process capability grant, or
Android/Windows delivery are exercised. These
remain hardware/OS acceptance. Do not construct
false `Selection` tokens; the real controller
already validates current selection ownership.

## Stage 3 — exhaustive bounded test artifact inventory

| Production function/branch | Level | Planned change |
|---|---|---|
| `outbound::paths`: GNOME action header, comments, CRLF, whitespace, localhost, percent decode, list order | P0 parser | 1 new table-driven method |
| `outbound::paths`: no partial admission on late remote URI, invalid percent, NUL/UTF-8, wrong GNOME command, 256 KiB cap | P0 parser | 1 new table-driven method |
| `Panel::input_allowed`: busy/text/settings/pending, closing/preview, safety, stale epoch, missing device, invalid lifecycle/synchronization | P0 GTK action gating | Extend existing real `outbound::verify` smoke (1 existing method, no fake new test count) |
| `Panel::finish_prepared`, `capture_clipboard`, runtime network delivery and KDE/Wayland clipboard acquisition | P0 actual asynchronous SDK/e2e | OPEN; no unauthorized refactor |

Total: **two** original new Rust test methods plus one
substantial extension to an existing GTK smoke. No
new fake domain/production helper or dependency.

## Stage 4 — branch map and expected test assertions

1. `given_local_gnome_and_uri_clipboard_lists_when_decoded_then_paths_keep_order_and_utf8`:
   GNOME `copy` / `cut`, header line removed,
   newline+CRLF, comments, empty whitespace,
   localhost case insensitive, percent-decoded
   escaped space and UTF-8, exact `Vec<PathBuf>`
   order/contents.
2. `given_clipboard_uri_list_with_late_invalid_inputs_when_parsed_then_entire_import_is_rejected`:
   invalid GNOME verb, wrong MIME header,
   remote host/userinfo/port, partial or malformed
   hex escape, inserted NUL, invalid UTF-8, a
   second invalid URI after a legitimate first,
   empty/comment-only, over-budget bytes. Assert
   `Err` for the **whole** body, not partial results.
3. `outbound::verify` existing widget smoke:
   start valid target at epoch 123, mutate each
   `PanelState` disqualifier independently,
   mutate `StateView` epoch/synchronized/lifecycle/
   endpoints, exercise `Model` preview/closing and
   safety-active locked/unknown/closed, restore
   all state at end; all must return false,
   and baseline must still return true.

## Stage 5 — Positive / N1–N12

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Valid copied URI lists | P exact local order | P UTF-8 | P multis | X no network | P repeated decode | X pure | P escaped names | P local-only | P Unicode | X no UI | P no remote disclosure | X no billing | P resolved path |
| Hostile clipboard lists | P valid+invalid rejects | P malformed | P 256KiB+1 | X no I/O | P whole import atomic | X pure | P corrupted | P authority | P invalid encoding | X | P no partial output | X | P local only |
| GTK recipient action gate | Existing PASS | P missing peer | P stale epoch | P lost connectivity state | P prevent stale callback | P state interleavings | P stale view | P wrong recipient/lock | X no date | P closing | P no silent send | X | X |
| Actual OS clipboard & real device transfer | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | X | OPEN |

X = not applicable **for this pure/smoke sub-path**.
`OPEN` physical or async handoff must not be
reported as covered by this code-only matrix.

## Stage 6 — interactions

MIME kind {URI-list, GNOME copy/cut} × list
{one/two/invalid-second/comment-only} × authority
{empty/localhost/remote/userinfo/port} × encoding
{plain/escaped UTF-8/malformed} × budget {small, over}.
Recipient {current/stale/missing} × panel flags
{healthy, closing, preview, locked, busy, pending,
text pending, settings pending, disconnected} ×
view {epoch, synchronization, lifecycle, endpoints}.
Each applicable branch is assigned a concrete
new or existing smoke assertion.

## Stage 7 — estimates

2 new methods + 1 expanded existing live GTK smoke.
New method category attribution (overlap allowed):
Positive 2, N1 2, N2 2, N3 0 X,
N4 2, N5 0 X, N6 2, N7 2, N8 2,
N9 0 X, N10 2, N11 0 X, N12 2.
Existing smoke adds N3/N5/N7/N9 security-negative
branches but does **not** increase test-method count.
These are planned categories, not coverage percentages.

## Stage 8 — per production file

`native_desktop/src/panel/outbound.rs`:
`format` existing fully covered; add table cases
for `paths`; exercise existing `input_allowed`
via GTK smoke. `clipboard_paths`, asynchronous
`finish_prepared`/controller, `capture_clipboard`
and real file selection still require actual
adapter/hardware coverage.
`native_desktop/src/panel.rs` rendering and
other handlers unchanged. Core `NativeClient`
selection validation is independently tested
but not claimed fully covered by this slice.
Global Stage 8 full map and Stage 12 remain OPEN.

## Stage 10 — mutation review plan

Accidentally drop the GNOME first-line gate,
accept remote host, return partial list before
validating all items, ignore malformed percent
or size cap, reorder paths, or permit stale
epoch/locked/closing/missing recipient:
exact new assertions should fail.
Manual mutation sensitivity, **not** a
mutant-engine execution.

## Milestones / rollback

1. [x] Stage 1–8 scan, scoped plan, branch/N/
   interaction/count map committed before tests.
2. [x] Add 2 table-driven parser test methods.
3. [x] Expand actual `outbound::verify` GUI
   security gate smoke and restore fixture state.
4. [x] Run exact-source Native GTK and Preflight;
   inspect 2 new method names and non-ignored
   native totals, document any red CI.
5. [x] Record D2–D5 actual, mutation review,
   update Current State and global Mode B Inventory.
6. [ ] Strict full 10× after latest source,
   actual installed desktop and all-public-branch
   Completion Gate remain OPEN.

Rollback: revert only test changes and this
documentation. Production behavior, protocol,
installed package and `master` unaffected.

## Stage 9 test implementation checkpoint — CI not yet validated

- Source-only test change `44c9fec8`: two original
  methods for real GIO clipboard URI path order and
  hostile/late-invalid local-URI batches; exact
  URI_BYTES boundary and Unicode coverage.
- Source-only test change `3186526e`: an expansion
  of existing real GTK `outbound::verify` smoke
  checking five independent controller flags,
  five stale/invalid live-view variants, closing,
  preview and three lock/unknown/closed safety states
  with a positive unlocked manual-send control.
  The test restores the prior Model safety and
  controller snapshot before returning.
- Exact test source for both changes:
  [Native GTK 39126097688](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126097688)
  and [Preflight 39126097726](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126097726).
  Until actual job results and per-test logs are
  inspected do not claim these pass.
- No production changes, no real clipboard/OS
  user interaction, no Mode A decision.

## Red native formatting CI and exact-source retry

[Native GTK 39126097688](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126097688)
at `3186526e` failed the native rustfmt gate **before
executing the new tests**. The log showed
two vec! layouts, one assert_eq! wrap and two
extra empty lines near the test module end,
all inside the new `outbound.rs` test section.
Pure test formatting commits `11f4e875`
and `c911fc02` addressed these exact differences,
with assertion contents and production code
unchanged. The intermediate CI run on
`11f4e875` was cancelled by the second
format correction; it cannot count as PASS.
Latest same-source CI:
[Native GTK 39126241182](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126241182),
[Preflight 39126241201](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126241201).
Report actual test results only when the
completed job logs prove all new methods pass.

## Actual scoped D1–D5 and verified native run

**D1 — source and runner.** All actual production
functions under test are unmodified:
`paths` uses real GIO URI decoding and
`input_allowed` is called by the existing
full GTK panel smoke. No new production seam or
GUI fake was added. The final test-only code SHA
`c911fc02816b5f1052f5f24d72f68a4397081483`
was compiled/run in
[Native GTK 39126241182](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126241182)
(job `117671867546`). The complete native log
states **57 passed / 0 failed / 0 ignored**.
Each of the **two new original method names**
appeared independently **exactly once as `... ok`**.
The previous GTK total was 55; +2 is reconciled.
The existing GTK panel-smoke method also passed,
now executing the larger fail-closed recipient
and safety matrix.

**D2 — actual category evidence**, counted by
unique new methods only (2), with the expanded
existing GTK smoke listed separately:

| Category | New unique methods | Actual evidence |
|---|---:|---|
| Positive | 2 | PASS: ordered local files, valid GIO Unicode and exact budget |
| N1 bad input | 2 | PASS: malformed GNOME header, invalid late list item |
| N2 bounds | 2 | PASS: long input / 256 KiB + 1 and zero/empty list |
| N3 external failures | 0 | N/A: no external clipboard/device transfer in parser |
| N4 repeat/idempotency | 2 | PASS: no partial batch, duplicate local URIs retain order |
| N5 concurrency | 0 | N/A: pure path parser; existing GTK smoke validates state gate |
| N6 corrupt data | 2 | PASS: truncated/invalid % hex, null byte, UTF-8 corruption |
| N7 authority | 2 | PASS: rejects remote host, userinfo and nonlocal port |
| N8 locale/unicode | 2 | PASS: encoded UTF-8 and CRLF/list whitespace |
| N9 interruption | 0 | N/A for parser; closing/stale states exercised by smoke |
| N10 privacy | 2 | PASS: remote/partial URI batches never admitted |
| N11 monetization | 0 | N/A: no entitlement in path parser |
| N12 local FS | 2 | PASS: only absolute local GIO paths returned |

Existing `outbound::verify` GTK smoke now
covers 5 independent controller-disqualifying
flags, 5 stale/invalid view branches, closing,
preview, 3 unsafe desktop lock/provider states,
and an unlocked/inhibited manual-action
positive control. These **do not count as new
methods**. They do not prove a real pending
daemon send was attempted.

**D3 — test-only source commits.**
`44c9fec8` adds two new parser tests,
`3186526e` expands real GTK smoke,
`11f4e875` and `c911fc02` apply exact
native rustfmt corrections without weakening
assertions. [Native GTK 39126097688](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126097688)
failed at formatting before running the
new tests; it is historical red CI,
not counted toward pass evidence. No
runtime code changed.

**D4 — refactor/audit.** No refactor needed
to test these pure URI and real GTK guard
branches. `clipboard_paths` asynchronous
stream/close, controller action scheduling,
real Wayland clipboard and cross-device
sending remain OPEN and require separate
integration/hardware acceptance.

**D5 — manual mutation review.** Mutations
that admit a remote host, process GNOME
`copy` as a file, accept malformed percent
escapes, drop/shift/reorder a decoded path,
accept an invalid late entry, ignore the
URI_BYTES cap, allow a stale epoch,
unsynchronized view, locked desktop, busy
controller, closing window or missing target
will contradict explicit assertions.
This is NOT an executed mutation framework
or a final full 10× Mode B suite.

**Same-code general CI checkpoint.**
[Preflight 39126241201](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126241201)
Core job `117671870007` **SUCCESS**
with **740/740 stable Core**, both existing
integration binaries **4/4 + 4/4**, no
failed/ignored cases. Rust formatter,
frontend and Core Clippy jobs also completed
SUCCESS. Desktop Tauri **10/10** Rust tests and
Clippy also completed SUCCESS, verified from
job `117671869993`. The same-source quick
Preflight **5/5 jobs completed SUCCESS**,
and separate Native GTK completed SUCCESS.
This is one quick run, NOT full 10x Mode B.
`
## Exact-source final checkpoint

[Preflight 39126241201](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126241201)
and [Native GTK 39126241182](https://github.com/morozhenko-apps/rquickshare/actions/runs/39126241182)
both completed **SUCCESS** on
`c911fc02816b5f1052f5f24d72f68a4397081483`.
All five Preflight jobs green:
Core **740/740 stable + 4/4 + 4/4**,
desktop Tauri **10/10**, frontend,
Core and desktop Clippy, formatter.
GTK **57/57**, with the two new methods
individually `... ok` and the existing
real-GTK panel smoke passing. The old
red native format run is retained in
this ledger as historical, not used as
pass evidence.

Global Stage 11 final-source ten-pass and
Stage 12 completion gate remain **OPEN/NO**.

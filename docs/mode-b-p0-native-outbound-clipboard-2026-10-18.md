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
4. [ ] Run exact-source Native GTK and Preflight;
   inspect 2 new method names and non-ignored
   native totals, document any red CI.
5. [ ] Record D2–D5 actual, mutation review,
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

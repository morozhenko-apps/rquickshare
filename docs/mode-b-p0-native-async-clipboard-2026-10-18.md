# MODE B P0 — real GTK clipboard asynchronous stream and preparation cancellation

Date: 2026-10-18. Primary mode: **MODE B / Testing**.
Scoped to test code in `native_desktop/src/panel/outbound.rs`,
branch `dev`. Production GTK behavior, authorization,
network, protocol, native package, user clipboard, `master`
and releases remain unchanged. Global Stage 12 = NO.

## Stage 1 — scan and baseline

Read the user's `agent.md` testing SSOT and
`docs/mode-b-p0-native-outbound-clipboard-2026-10-18.md`,
`docs/mode-b-inventory.md`,
`docs/current-state.md`,
`native_desktop/src/panel/outbound.rs`,
`native_desktop/src/panel.rs`,
`native_desktop/src/panel/tests.rs`,
`native_desktop/src/panel/compose/tests.rs`,
`native_desktop/src/panel/text/tests.rs`,
`core_lib/src/service/client.rs`,
`core_lib/src/service/client/controller.rs`,
`core_lib/src/service/client/state.rs`,
`native_desktop/Cargo.toml`.
Native GTK CI runs a single actual widget smoke on an
isolated Xvfb/DBus display with GLib's main context;
`outbound::verify(panel)` is already invoked in that
smoke. GTK4 0.11 exposes
`gdk::ContentProvider::for_bytes` and
`gdk::Clipboard::set_content`; they can provide
genuine GDK clipboard MIME bytes without
injecting a production adapter.

The earlier focused checks cover `paths` direct
URI parsing and `input_allowed` preflight
but not `clipboard_paths` asynchronous
stream or `close_input`/terminal `wait_selection`.
`Selection` has private secure connection fields.
Inventing a selection token to exercise a fake
successful network send is prohibited. Existing
real core controller tests handle authenticated
selection and send; these native tests do not
claim real cross-device sending.

## Stage 2 — risks and invariants

1. The real `clipboard_paths` reads only preferred
   GNOME `x-special/gnome-copied-files` or
   freedesktop `text/uri-list`, bounded to 256 KiB.
   A complete local list yields **exactly**
   the original ordered decoded paths.
2. A late malformed or remote URI, oversized
   clipboard body, or unsupported MIME
   yields `Err`, never a partially accepted
   selection or fallback transfer. Stream
   acquisition/read/close actually happen under
   an isolated GTK display, no application send.
3. Pending `wait_selection` must mark
   input active. `close_input` cancels the
   actual pending cancellation token, aborts
   the GLib task, invalidates the generation,
   clears input_active and strip_target.
   The stale generation must not cause
   a notice or a target send on later arrival.
4. If current `wait_selection` receives
   `None`, the real callback must clear
   the input state and show only its
   preparation failure notice, without send.
5. Tests must not claim that OS clipboard
   ownership loss, stalled remote Wayland
   compositor, GTK file chooser or user
   device transfer has been executed.
   No new production hooks, mock use
   cases, sleeps or wall-clock timing.

## Stage 3 — testable artifact inventory

| Production path | Risk and level | Planned scenarios |
|---|---|---|
| `clipboard_paths` through actual GDK `Clipboard` / `ContentProvider`: valid URI-list, GNOME copy/cut, correct order and decoded UTF-8 | P0 GTK/SDK integration | 1 expanded real-widget method, parameter rows |
| `clipboard_paths` invalid second URI, malicious authority, unsupported content-type, over-limit byte stream | P0 async negative | Same widget method, all independent failure variants |
| `Panel::wait_selection` + `close_input`: pending task/token and stale generation | P0 async UI lifecycle | 1 expanded real-widget method, real oneshot and task |
| `Panel::finish_prepared` receiving `None` on current generation | P0 negative terminal | Same widget method, real GLib task completion |
| Real `Some(Selection)` send to device and stalled compositor, lost remote clipboard owner | P0 E2E/hardware | OPEN; needs real authenticated controller and installed GUI |

Use two new **test-only fixture functions** invoked
from the existing actual GTK panel smoke
`outbound::verify`; this adds **zero independently
counted Rust test methods**, but significantly
extends execution of a critical existing
integration method. The code contains
assertions and no new production function.

## Stage 4 — branch-to-test map

- GDK valid file MIME: `read_future`,
  per-chunk `read_bytes_future`, EOF,
  stream `close_future` and actual
  `paths`: exact list equality for
  two MIME types and Unicode.
- Malformed second URI, over-budget
  bytes and unsupported MIME:
  `Err` and no selection.
- `wait_selection` pending oneshot:
  verify input_active, GLib task
  existence, `close_input` token
  cancelled and new Rc generation,
  state cleared, old callback must
  not update notices.
- `wait_selection` completed `None`:
  block the **real GLib JoinHandle**
  on a prepared oneshot (no sleeps),
  assert explicit failure notice,
  input inactive and absence of send.

## Stage 5 — Positive / N1–N12

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Real GDK valid clipboard | planned exact output | planned MIME | planned ordered list | N/A no network | planned repeat | N/A one user clipboard | planned UTF-8 | planned only local | planned Unicode | N/A no cancel | planned no other data | N/A no billing | planned local URI |
| Real GDK invalid clipboard | planned no selection | planned malformed/unsupported | planned 256KiB + 1 | planned no usable provider | planned no partial retry | N/A independent input | planned invalid second URI | planned remote host reject | planned UTF-8 | planned rejection | planned no partial data | N/A | planned invalid data |
| Cancel/late result | planned current valid idle | planned None reply | N/A no numeric limit | N/A no socket | planned repeated close | planned token/task/gen interleave | planned stale result | planned no send | N/A no date | planned cancel | planned notice/selection isolation | N/A | N/A |

The external GTK clipboard is contract-level
SDK input, **not** a mocked application parser
or controller. Real-world Wayland/Android
remains OPEN separately.

## Stage 6 — interaction matrix

MIME {freedesktop, GNOME, unsupported} ×
payload {valid two paths, Unicode, remote
second URI, size 256KiB+1} ×
worker {pending, finished with None,
cancelled} × generation {current, stale}
and target {unspecified, existing}.
Only meaningful existing GTK smoke
combinations are asserted; a real
`Some(Selection)` and app send are
not forged, and remain an E2E gap.

## Stage 7 — planned counts

0 additional test runner methods,
2 private test-only fixture functions
called by 1 existing GTK smoke.
Cover category groups:
Positive 2 fixture functions, N1 2,
N2 1, N3 1, N4 2, N5 1,
N6 1, N7 2, N8 1, N9 1,
N10 2, N11 0 N/A, N12 1.
Category counts are overlapping
functions, not independently
executed Rust methods.

## Stage 8 — production file coverage map

`native_desktop/src/panel/outbound.rs`:
`clipboard_paths` SDK asynchronous
read, close and validation; `wait_selection`,
`finish_prepared(None)` and `close_input`
cancellation ownership. Other
`capture_clipboard` image/text/real file
acquisition and send-to-target branches
OPEN because the real controller/daemon
is not connected in the GTK widget fixture.
`native_desktop/src/panel.rs`: real GTK
panel instance used but remaining
production methods not claimed covered.
Every other production file remains
subject to global Stage 8 and Stage 12.

## Stage 10 — mutation sensitivity

Ignoring the MIME preference, permitting
a remote second URI, accepting over-limit
bytes, skipping stream closure, failing
to abort pending task, keeping a stale
generation, not cancelling token,
or delivering a false successful selection
should violate explicit checked
outputs/states. The absence of a
stream spy means `close_future` is
covered as real execution but a
no-close mutant is not automatically
guaranteed to be caught: record that
limitation honestly. Manual review,
not executed mutation engine.

## Milestones and rollback

1. [x] Stage 1–8 scanned and documented BEFORE code.
2. [x] Implement both test-only fixture helpers
   in `outbound.rs` and invoke from actual GTK smoke.
3. [x] Exact-source Native GTK and Preflight:
   check real failures and assertion names.
4. [x] Update D2 actual, mutation review,
   current-state and MODE B inventory.
5. [ ] Final complete 10x Stage 11 and
   global all-method Stage 12 remain
   OPEN (no release side effects).

Rollback only test module additions
and documentation; no product change.

## Stage 9 source and CI checkpoint

- Tests-only source commit `f9551a28` embeds two
  fixture helpers in the existing real GTK panel
  smoke. They exercise GDK MIME ownership with
  actual `clipboard_paths`, and actual pending
  GLib `wait_selection` / `close_input`
  cancellation, generation and terminal `None`.
- [Native GTK 39128231753](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128231753)
  failed its rustfmt gate on one new `for_bytes`
  multiline call **before executing tests**.
  That is not evidence of a product failure.
- Test-only format correction `1501fa1e`
  changes only the argument layout, not assertions.
  [Native GTK retry 39128370942](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128370942)
  and [Preflight retry 39128370901](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128370901)
  are the exact-code checks to inspect. The
  older same-code Preflight might be cancelled
  by workflow concurrency; never count cancelled
  attempts as green.
- Execution D2 and CI completion remain pending
  until actual logs show the integrated widget
  method PASS. Global Stage 11 full 10x and
  Stage 12 remain NO.

## D1–D5 actual scoped evidence and limitations

**D1 — exact-source tests.** Code SHA
`1501fa1eed09bfb349ec48fcab038d15bba7a83f`
ran in [Native GTK 39128370942](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128370942),
job `117678215874`, **SUCCESS**. Read the
actual full job log: **57 passed / 0 failed /
0 ignored / 0 filtered**; the existing
`given_actual_builder_and_untrusted_details...`
real GTK smoke executes both new helper fixture
groups. This test count remains **57**, not 59:
the two private fixtures are not independent
Rust test methods. No fake selection or
domain dispatch was injected. Native lint and
source formatting also passed.

**D2 — actual overlapping fixture-category coverage.**
The two test-only fixture functions were run
inside that real-panel smoke. Counts are fixture
groups with evidence of their internal rows,
NOT independent test method counts.

| Category | Fixture groups | Scoped verified behavior |
|---|---:|---|
| Positive | 2 | PASS: GDK URI-list and GNOME copy/cut, terminal None handled |
| N1 validation | 2 | PASS: invalid second URI, unsupported MIME, unsuccessful selection |
| N2 bounds | 1 | PASS: URI_BYTES+1 delivered by real GDK provider rejected |
| N3 external input error | 1 | PASS: unsupported clipboard provider |
| N4 idempotency/retry | 2 | PASS: no partial result; close resets old generation |
| N5 concurrency | 1 | PASS: async pending task cancelled before stale completion |
| N6 corrupted data | 1 | PASS: malformed percent escape and late remote URI |
| N7 security/authority | 2 | PASS: local-only files and stale-generation guard |
| N8 locale/Unicode | 1 | PASS: UTF-8 filenames through GDK byte stream |
| N9 interruption | 1 | PASS: actual token cancellation/task abort |
| N10 privacy | 2 | PASS: no foreign URI or fake prepared content accepted |
| N11 entitlements | 0 | N/A: no purchase/advertising logic |
| N12 storage/input | 1 | PASS: bounded local URI bytes, no FS writes |

**D3 — exact code files.** Only
`native_desktop/src/panel/outbound.rs` within
the preexisting `#[cfg(test)]` section
and its `verify` smoke-call site changed;
`f9551a28` added both test fixture methods
and `1501fa1e` applied rustfmt without
weakening assertions. No production
branch or installed package was modified.
[Native GTK 39128231753](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128231753)
was RED at the formatter before executing
tests and remains historical evidence, not
counted toward completed runs.

**D4 — testability.** No production seam
needed for these specific GDK MIME bytes,
`clipboard_paths`, or GTK completion/task
abort. Actual loss of Wayland clipboard
ownership while reading and the
daemon-side `Some(Selection)` send path
require additional external event/control
boundaries and physical platform testing;
not claimed here. Actual file-descriptor
close of the `GInputStream` is performed
by the production method in this suite,
but not independently observed by a
stream spy; this remains a mutation gap.

**D5 — assertion-sensitive mutations.**
Removing the async read, accepting malformed
URI after valid first item, admitting
unsupported MIME, lifting the byte limit,
ignoring task cancellation/generation,
retaining `input_active`/strip_target,
or reporting a successful `None` reply
would fail specific assertions. No executed
mutation engine or 10 consecutive full
runs. Global Stage 11 and Stage 12 **NO**.

**General quick CI checkpoint.**
[Preflight 39128370901](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128370901)
at exact source `1501fa1e`: stable Core,
Core Clippy, frontend and Rust formatter
jobs SUCCESS at first inspection.
The desktop Tauri Clippy/unit job
subsequently completed SUCCESS and its
job log `117678271520` proves
**10/10 desktop Rust tests**, zero failures
or ignored. The complete quick Preflight
**finished SUCCESS, 5/5 jobs**, on the
same test source.

## Final same-source quick CI gate (not full Mode B)

Both workflows completed **SUCCESS** at
`1501fa1eed09bfb349ec48fcab038d15bba7a83f`:

- [Native GTK 39128370942](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128370942),
  job `117678215874`, real actual-widget suite
  **57/57 passed**. This remains 57 because
  two test-only fixture functions were called
  inside the already-existing smoke method.
- [Preflight 39128370901](https://github.com/morozhenko-apps/rquickshare/actions/runs/39128370901),
  **all 5/5 jobs SUCCESS**. Core job
  `117678271834` actual log: stable
  **740/740**, two integration binaries
  **4/4 + 4/4**; desktop Tauri job
  `117678271520` **10/10**. Formatter,
  frontend, Core Clippy and desktop
  Clippy passed.
- Original rustfmt-only red CI is retained
  in the ledger and not counted toward
  success. No assertions were weakened.

**Limitations:** no standalone executable
mutation run; no ten consecutive full
post-change repository passes; no real
Wayland/Google peer acceptance; no proof
of independently observed GInputStream
closure. Stage 11 global / Stage 12 global
remain **NO**.

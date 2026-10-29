# MODE B P0/P1 batch: native CLI input, consent-card, result queue and idle safety

Date: 2026-10-18. Current branch dev. The authoritative user
agent.md testing SSOT is followed. **MODE B testing only**:
write documented test cases and assertions, no production
behavior/security changes, installed desktop/master/release untouched.

## Stage 1: repository scan and scope

Sources inspected before edits: docs/mode-b-inventory.md,
docs/mode-b-completion-audit-2026-10-18.md,
docs/current-state.md, native_desktop/src/cli.rs,
native_desktop/src/card_policy.rs,
native_desktop/src/panel/results.rs,
native_desktop/src/panel/idle.rs and native GTK CI.
Existing baseline at dev `024aac6f`:
Core 742 stable, GTK 57, Tauri 10; full 10x and
global all-method Stage12 still NO.
All proposed tests use **real production functions**,
not mocked domain logic.

## Stage 2: risks/invariants

1. **P0 CLI local authority**: command-line local file URIs
   must percent-decode bytes exactly on Unix, reject
   remote owner/userinfo/port, foreign URI scheme,
   malformed escapes and encoded NUL. Unicode and
   non-UTF8 filename bytes must be preserved, never
   replaced or normalized; no partial admission after
   an invalid later argument. Count and total-byte cap
   must apply to whole argv, not per-item only.
2. **P0 consent-card**: automatic display is only a
   presentation effect; an incoming consent request
   outranks terminal outcome notices, deterministic
   order independent of snapshot input order. Dismissal
   must suppress unchanged revision and not persist
   across new consent round/epoch, cannot implicitly
   accept user consent. A busy visible automatic
   stale notice must hide rather than leak stale
   information from removed peers.
3. **P1 success results**: only a LIVE nonterminal→
   inbound Finished transition in the same daemon epoch
   may become a reveal pending result; duplicate
   Finished, initial historical Finished, outbound
   Finished, old epoch, missing or restarted source
   cannot surface unowned capabilities. Queue insertion
   respects observation order and teardown retains
   only current rows.
4. **P0 idle**: a native card never auto-dismisses
   while daemon is draining, security lock/unknown,
   a local source/selection is prepared or actual
   nonterminal transfer exists. Existing time deadlines
   are synthetic but must not authorize accidental
   dismissal with outstanding work.

No new refactor or mock permits, and no true GTK
client/hardware delivery is implied.

## Stage 3: artifact inventory & planned methods

| Production file and testable branch | Level/risk | New unique methods |
|---|---|---:|
| cli.rs paths local percent decode; escaped non-UTF8 and NUL/query/fragment | P0 security parser | 1 |
| cli.rs paths batch 255/256/257 cap, total 64 KiB, late invalid remote/malformed | P0 security/atomicity | 1 |
| card_policy.rs CardPolicy::reconcile priority/order/dismiss/new consent revision | P0 consent presentation | 1 |
| card_policy.rs CardPolicy::reconcile visible+busy old notice removed, epoch replacement | P0 stale UI/privacy | 1 |
| panel/results.rs Results::observe live-transition duplicate, directions, reconnect/epoch | P1 result capability | 1 |
| panel/results.rs Results::observe pending FIFO/snapshot prune/dismiss/replay | P1 result state | 1 |
| panel/idle.rs free real view with lifecycle draining, active source/selection/transfer | P0 user work safety | 1 |

**Seven** original independent Rust test methods; table-driven
cases each count as one method. No API test-only adapter needed.

## Stage 4: branch-to-test map

- CLI 1: literal relative path, percent-escaped bytes
  [0x20, 0xFF, 0x2F], non-UTF8 raw OsString,
  uppercase/lowercase hex, escaped NUL, query/hash
  rejected, remote/userinfo/port/encoded authority
  rejected. Validate exact OsStr bytes.
- CLI 2: mixed valid + invalid second/last => whole Err,
  multi-args 255 and 256 accepted, 257 rejected,
  total raw argv byte sum 65535/65536/65537
  and empty/missing --; preserve order/repeats.
- Card 1: three pending notices across two priorities;
  incoming round 9 prioritized over outcome, deterministic
  BTreeMap ID even if rows permute. Dismiss once => exact
  emitted set, repeated same snapshot => None; incoming
  round 10 reopens, new epoch resets stale dismissal.
- Card 2: visible automatic showing A; busy rerender
  with A removed and B ready => Hide, no unsolicited
  Show while busy, subsequent not busy => Show B;
  old epoch new ID resets all records.
- Results 1: baseline historical finished is inert;
  receiving -> inbound finished once; repeated finish
  inert, outbound finished inert; epoch reset never
  replays old Finished.
- Results 2: queue order A/B, removal prunes, dismissal
  suppresses old rows, re-arm only fresh nonterminal->
  finished; missing rows empty pending/visible.
- Idle 1: current healthy model plus each independent
  source/selection/active transfer and draining vs
  stopped lifecycle. Existing flags and timer baseline
  stay unchanged.

## Stage 5: Positive and N1–N12 map

Legend P = new test, C = existing test, X = N/A for
this pure state/path, OPEN = external hardware.
Per-method categories overlap.

| Artifact | Pos | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| CLI parse | P | P invalid escapes/remote | P cap±1 | X no network | P batch atomic | X pure | P invalid bytes | P local-only | P UTF8/nonUTF8 | X no async | P no other host | X | P local paths |
| Consent card | P | P missing round | P multi pending | X UI event only | P dismissal/new revision | P busy transitions | P stale epoch | P notice ownership | X no locale/time | P dismiss | P no stale reveal | X | X |
| Result queue | P | P invalid initial | P multi queue | X no network | P duplicate/epoch | P transitions | P old phase/epoch | P inbound only | X no locale | P dismiss/reset | P no false result | X | X |
| Idle safety | P | P absent model | P states | X no socket | P repeated | P parallel work | P stale view | P lock/selection | P fake clock already C | P prevent hide | P no accidental dismissal | X | P active sources |

N11 X: no billing features; do not generalize to repo.
Real X11/Wayland/MIME and peer flows remain OPEN elsewhere.

## Stage 6: interaction map

CLI URI [local, remote, malformed] × encoding
[ASCII, UTF8, nonUTF8, NUL] × position [first, late]
× count [1/255/256/257] × total bytes
[65535/65536/65537].
Consent [AwaitingConsent, Finished, Failed] ×
[2 peers, reversed order] × revision [old/new] ×
epoch [same/new] × [busy, visible, automatic].
Results [Inbound/Outbound] ×
[Queued/Receiving/Finished/Failed] ×
[old/new epoch] × [duplicate/new/snapshot removed].
Idle [healthy, draining, locked, active source,
selection, active transfer, absent view].
Meaningful testable conditions assigned explicitly.

## Stage 7: estimates

Seven new methods distributed 2/2/2/1 across
four production files; no fake coverage percentages.
Scoped category attribution: Positive 7; N1 5;
N2 6; N3 0 X; N4 7; N5 5;
N6 6; N7 7; N8 2; N9 5;
N10 7; N11 0 X; N12 3.
These are planned overlapping method labels, not
independent unique test totals or measured branches.

## Stage 8: per-production-file coverage map

- native_desktop/src/cli.rs: `paths` percent
  decoder/error classes and whole-argv authority
  contract. Real DesktopClient import and device
  delivery remain covered separately or OPEN.
- native_desktop/src/card_policy.rs: `reconcile`,
  `dismiss` and `manual` bounded state replay.
  Real desktop rendering/daemon consent via app
  callbacks is OPEN.
- native_desktop/src/panel/results.rs:
  `Results::observe`, `dismiss`, `hide_rows`
  state machine. Real reveal/portal file authority
  and app crash/restart remain OPEN.
- native_desktop/src/panel/idle.rs:
  `free` state branches, existing `deadline`
  tests. Real GLib timer/Wayland focus remains OPEN.
Other production-file branch maps continue under
global Stage 8 and Stage 12 = NO.

## Stage 10: mutation review plan

Permit foreign authority/scheme, lossy filename
conversion, percent NUL or late-invalid partial
result; relax total raw byte/count limit; change
consent priority/order, replay dismissed notices,
keep stale `shown` on busy target loss; reveal
initial historical Finished or outbound completion,
double queue Finished, leak removed results; hide
a UI when work is still pending. Each proposed
mutation has exact assertions. Manual review,
not claim of executed mutation engine.

## Execution and rollback

1. [x] Stages 1–8 and mutation plan documented BEFORE code.
2. [x] Add 7 independent methods; no production changes.
3. [x] Run same-source native GTK, Preflight Core/format/frontend/
   Clippy/Tauri; check **actual logs** and method names.
4. [x] Record D1–D5 and observed category matrix; update
   global inventory/current state.
5. [ ] Strict final-source full suite 10x and physical
   OS/device acceptance remain OPEN; all Stage12 NO.
Rollback only test/docs; do not touch release or master.

## Stage 9 implementation checkpoint (not execution proof)

The seven planned test-only methods were implemented
across four production-file test modules:

- `native_desktop/src/cli.rs`, commit `5939949e`:
  2 new independently named Rust tests. Malformed,
  remote, non-UTF8 and percent-decoded Unix filenames;
  mixed late-invalid argv, exact 256-entry and
  combined 64-KiB byte boundary.
- `native_desktop/src/card_policy.rs`, commit
  `85075542`: 2 new independent tests for
  deterministic consent priority over terminal
  outcomes, dismissal/new revision/epoch behavior,
  and stale automatic presentation during busy
  peer replacement.
- `native_desktop/src/panel/results.rs`, commit
  `da387197`: 2 new independent tests for
  live Finished-vs-historical/current direction,
  one-shot no-replay, FIFO and snapshot pruning.
- `native_desktop/src/panel/idle.rs`, commit
  `30a220f3`: 1 independently named test for
  active safety-provider observations (18
  combinations) and Running/Draining/Stopped.
  The test-only fixture's independently constructed
  UI model was corrected in `3de97ce0`
  because Model is `Default` but not `Clone`.
  No runtime code/production fixture changed.

The first CI attempt on exact source `3de97ce0`:
[Native GTK 39131803149](https://github.com/morozhenko-apps/rquickshare/actions/runs/39131803149)
and [Preflight 39131803168](https://github.com/morozhenko-apps/rquickshare/actions/runs/39131803168).
Earlier in-flight runs on intermediate test commits
were cancelled by the repository's per-branch
concurrency; they are not passing receipts.
Do not count any new tests as passed until
exact-source job logs confirm the method names.
Full 10× and physical system gate stay NO.

## Formatter-only red trial during concurrent test additions

Intermediate [Native GTK 39131803149](https://github.com/morozhenko-apps/rquickshare/actions/runs/39131803149),
source `3de97ce0`, reported **rustfmt diff only**
in newly written `card_policy.rs`, `cli.rs`
and `panel/results.rs`; the full native
test executable had not run in that attempt.
Its exact diff was retrieved and scoped to
assertion wrapping, function layout and
iterator formatting. No production code,
test assertion or scenario is implicated.
Later separate preparation gate tests
at `4015ad70` were added while that
intermediate run was still executing,
therefore intermediate cancelled CI cannot
be counted as final. Correct the strictly
formatting diffs on final source before
claiming any PASS. No weakening allowed.

## First combined-code test fixture compile diagnosis

At test source `0e227a9e`,
[Native GTK 39132196774](https://github.com/morozhenko-apps/rquickshare/actions/runs/39132196774)
native formatting passed but native Clippy/test
compilation failed at `card_policy.rs:523` with
Rust `E0382`: newly written test reused a
`Surface` after it had been moved into
`CardPolicy::reconcile`, which takes
the Surface by value. The production API is
intentionally unchanged. Replace the local
test fixture with a zero-argument constructor
closure producing an independent busy Surface
for each invocation. All negative assertions
are preserved. The test did **not** execute
and the run is NOT a passing receipt.

## Second combined Native GTK CI and exact formatting correction

At source `4a29bc02`,
[Native GTK 39132422435](https://github.com/morozhenko-apps/rquickshare/actions/runs/39132422435)
failed only the native rustfmt gate before running
the new test executable: two wrapped
`policy.reconcile(..., busy())` calls in
the new card-policy test. The local
`Surface`-factory correction had removed
the earlier E0382 move error.
Applied exactly the CI-requested
formatting at source commit `913fac10`,
without modifying any assertion or
production implementation. This red
run is retained, not counted as success.
Final source jobs must be verified afresh.

## First executed card-policy tests: two invalid presentation-fixture assumptions

At exact code SHA `913fac10`,
[Native GTK run 39132639231](https://github.com/morozhenko-apps/rquickshare/actions/runs/39132639231)
reached the complete test executable for the first time:
**62 passed, 2 failed**, both newly written CardPolicy
methods, no ignored/filtered cases. Full job
`117690759565` pinpoints failures at
`card_policy.rs:452` and `:529`.

Source analysis identifies **fixture contract mismatches**
rather than an established production regression:
- `CardPolicy::reconcile` intentionally sends
  `Effect::Show(notice)` on an otherwise unchanged
  notice when `surface.visible=false`, because
  the surface must reopen to present it. The new
  idempotency tests incorrectly passed `safe()`
  (invisible), then required `Effect::None`.
  For the intended unchanged visible automatic
  surface, use `visible=true; automatic=true`;
  require exactly `Effect::None` and that the
  shown notice identity remains unchanged.
- While `surface.busy=true` and
  `automatic=true; visible=true`, a stale
  currently shown notice must be hidden even
  if the same new unrelated pending peer is
  still eligible; `Effect::Hide` may repeat
  on redraw, rather than `Effect::None`.
  The new fixture incorrectly asserted None.
  Preserve the strong safety assertion that
  there is never a displayed old or new
  notice while busy and the old row has
  been pruned.

Only correct test preconditions and expected
presentation effects to match the actual
documented production state machine;
**do not** modify `CardPolicy::reconcile`,
suppress failures, remove negative branches
or relax no-stale-notice ownership checks.
Both failed tests must be rerun and pass
on exact source before a success claim.

## Final assertion layout correction after contract fixture repair

At corrected semantic test SHA `96957ca1`,
[Native GTK 39132953233](https://github.com/morozhenko-apps/rquickshare/actions/runs/39132953233)
failed only native rustfmt on the one new
`assert!(policy.shown.is_none(), ...)`
message layout, before test execution.
Exact format-only commit `66ddf1fc`
adopts the formatter's multiline assertion;
the checked boolean, message and
strong no-stale-UI invariants are identical.
[Native GTK 39133124223](https://github.com/morozhenko-apps/rquickshare/actions/runs/39133124223)
and [Preflight 39133124486](https://github.com/morozhenko-apps/rquickshare/actions/runs/39133124486)
are the final-code follow-up runs.
Neither stale failed CI nor an intermediate
cancelled run counts as Stage 11 evidence.

## D1–D5 verified native test source (after full red-state triage)

**D1 — source/runner and actual logs.**
Final test code SHA `66ddf1fc24f906e74c9246c31ebc1ad967b0e8d2`.
[Native GTK 39133124223](https://github.com/morozhenko-apps/rquickshare/actions/runs/39133124223)
**completed SUCCESS**, actual job `117692170743`
verified end-to-end: **64 passed, 0 failed,
0 ignored, 0 filtered**. All SEVEN original
new method names appear exactly **once
each as `... ok`**, together with
57 earlier native methods. Native formatter,
Clippy and widget suite passed.
The earlier native `62 passed, 2 failed`
at `913fac10` resulted from incorrect
new CardPolicy fixture surface assumptions,
not an established production defect.
The contract correction maintains
strict priority/no-stale owner assertions:
unchanged visible+automatic notice =>
None (no redraw), invisible may Show,
and repeated busy/automatic visible
with no owned shown notice => Hide.
No suppression/skip or domain code change.
Intermediate rustfmt-only failures
and compile-time consumed Surface
errors remain recorded above.

**D2 — actual overlapping method category
assignment** (SEVEN unique native Rust
methods, one-to-many; NOT seven tests
in each category and not a coverage
percentage):

| Category | New native methods | Verified bounded outcome |
|---|---:|---|
| Positive | 7 | PASS: legal local byte arguments, prioritized consent, live reveal, allowed idle |
| N1 validation | 5 | PASS: foreign/malformed CLI, absent/stale notice and phase |
| N2 limits | 6 | PASS: 255/256/257 argv, 64 KiB−1/exact/+1, multiple rows |
| N3 network error | 0 | N/A these direct helpers do not perform network I/O |
| N4 replay/idempotency | 7 | PASS: repeated card, dismissal, Finished and idle states |
| N5 state interactions | 5 | PASS: busy card, epoch/owner transitions and draining |
| N6 stale/corrupted | 6 | PASS: Unicode/nonUTF8 byte bounds, stale owner/snapshot state |
| N7 authority/security | 7 | PASS: strictly local URI, live consent priority, inbound results |
| N8 Unicode/time | 2 | PASS: Unix byte-exact URI, safety/time guard baseline |
| N9 interruption | 5 | PASS: dismiss, reset, removed peer, interrupted result or drain |
| N10 privacy/data isolation | 7 | PASS: no foreign URI/old peer/false revealed result |
| N11 entitlement | 0 | N/A no subscriptions or billing |
| N12 local path/storage | 3 | PASS: byte-preserving local paths and no background disappearance |

**D3 — implementation.** All seven
`#[test]` methods added in existing
production-file *test modules only*:
`native_desktop/src/cli.rs`,
`native_desktop/src/card_policy.rs`,
`native_desktop/src/panel/results.rs`,
`native_desktop/src/panel/idle.rs`.
No runtime source, public method,
protocol, installed UI, master or
release modification. Exact-format
fixes were made without weakening
meaningful assertions.

**D4 — refactor/testability.** No refactor
needed to test these pure rules.
Live backend consent decisions,
physical KDE/Wayland/Android/Windows
and delayed GUI/daemon callbacks
are not covered by these table-driven
methods. Do not infer release-ready
status from the strong native fast
suite.

**D5 — reviewed mutation sensitivity.**
Remote file:// authority, loss of
raw nonUTF8 filename, percent-NUL
admission, per-item instead of
batch raw byte caps, reversed
consent priority, dismissed-notice
replay, busy stale notice show,
initial/outbound completion
reveal, repeated Finished enqueue,
queue not pruned after removal,
or idle dismissal under lock/drain:
each breaks a specified assertion.
Only manual mutation sensitivity
is claimed; no mutation engine run.
The complete current-source 10
consecutive *full* test suites,
instrumented new-source branch map
and global Stage 12 remain **OPEN/NO**.

**Same-code general Preflight.**
[Preflight 39133124486](https://github.com/morozhenko-apps/rquickshare/actions/runs/39133124486)
at source `66ddf1fc` has Core Rust
**744/744 stable** (plus **4/4+4/4**
integration binaries), Rust format,
frontend and Core Clippy **SUCCESS**
at this evidence checkpoint. Desktop
Tauri job subsequently completed
**SUCCESS**, actual desktop job log
`117692233854` confirms **10/10 Rust
unit tests, 0 failed/ignored**.
The full exact-source quick Preflight
**completed SUCCESS, 5/5 jobs**.
The separate native GTK workflow
also completed SUCCESS, 64/64.
This is not full Mode B 10×.

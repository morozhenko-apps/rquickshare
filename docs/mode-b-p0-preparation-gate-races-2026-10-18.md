# MODE B P0: Native Desktop preparation gate simultaneous contenders and permanent close

Date: 2026-10-18. Primary mode MODE B/Testing on dev, no
production modification. User-supplied agent.md SSOT
requires documentation before tests, branch/negative
inventory and exact-source CI evidence.

## Scan and P0 invariants (Stages 1–2)

Inspected core_lib/src/service/client/desktop.rs,
core_lib/src/service/client/desktop/tests.rs,
core_lib/src/service/client/controller/tests.rs,
docs/mode-b-inventory.md and complete-mode-b audit.
The real PreparationGate has a Mutex-protected
(held, closed) state and a watch::Sender<bool>.
The existing real broker test `given_shared_gate_and_sender_loss...`
covers staging, abort, disconnect and closed gate
during a single sequence, **not** 16 simultaneous
try_acquire contenders or close vs held permit.
This can be tested directly without modifying the
production gate or inventing daemon/selection mocks.

P0 invariants: a single preparation permit at a time;
all unsuccessful attempts must remain false until the
winner is released. Closing the gate while a permit
is held is permanent; eventual Drop clears busy
watch state and must not reopen the gate. Neither
concurrent access nor repeated close can authorize
a new transfer. No sleeps or timing assumptions.

## Scoped inventory (Stage 3) / branch map (Stage 4)

| Production artifact | Risk and level | New original methods |
|---|---|---:|
| `PreparationGate::try_acquire`: 16 simultaneous contenders, only one permit while held, release permits future acquisition | P0 lock/ownership | 1 |
| `PreparationGate::close` + `PreparationPermit::Drop`: closing during outstanding permit; repeated close; published busy state after drop; no reacquire | P0 shutdown/race | 1 |

Two independently named unit methods in existing
`core_lib/src/service/client/desktop/tests.rs`.
Use `std::sync::Barrier` to coordinate contenders,
hold winner until all attempts have completed,
then join and check exact one winner.
Use real `std::thread` for close while permit
held and inspect actual watch receiver; no domain mock.

## N1–N12 and positive matrix (Stage 5)

| Artifact | Pos | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Concurrent permits | P exact winner | X type safe | P 16 threads | X no network | P reacquire only after drop | P barrier | X no persisted | P one owner | X no clock | P release | P no cross-prep | X no billing | X no disk |
| Close vs permit | P stable watch | X no untrusted | P held/released | X no socket | P repeated close | P thread join | P closed stale state | P permanent lockout | X no time | P shutdown | P cannot start after close | X | X |

External OS D-Bus/client disconnect and physical
device delivery are covered separately, not by
this pure ownership test.

## Interaction matrix (Stage 6) and counts (Stage 7)

Gate {open, held, closed} × contender
{winner, loser} × event {attempt, synchronized
recheck, drop, close, repeated close} × state
{published busy true/false}. Two independent
new test methods: Pos 2, N1 0, N2 2,
N3 0, N4 2, N5 2, N6 1,
N7 2, N8 0, N9 2, N10 2,
N11 0, N12 0. Overlapping tags only.

## Per-production-file map (Stage 8)

`core_lib/src/service/client/desktop.rs`:
`PreparationGate::new`, `try_acquire`,
`subscribe`, `close`, `PreparationPermit::Drop`,
including true/false lock and permanently
closed states. No other DesktopServer,
wire protocol, or client import error branch
is marked complete by this slice.

## Mutation review and completion (Stages 9–12)

Remove atomic Mutex ownership, allow
second permit before first Drop, unclose
on Drop, skip watch reset, allow new permit
after close: each has an assertion expected
to fail. This is manual mutation sensitivity,
not an executed mutator.

1. [x] Source/risk/inventory/branch/negative/
   interactions/counts/production map written first.
2. [x] Add two tests, no production code.
3. [x] Verify exact CI test names, Core Rust,
   Native GTK and all quick Preflight jobs.
4. [x] Update D1–D5, inventory/current-state.
5. [ ] Full complete 10× and global Stage12 still NO.
Rollback only tests/doc. No master/package/release.

## Implemented code and first exact-source CI checkpoint

Two independently named P0 tests were added in
test-only commit `4015ad70` to
`core_lib/src/service/client/desktop/tests.rs`.

1. 16 real `std::thread` contender calls start at one
   `Barrier`, and hold any winning permit until a second
   barrier proves all 16 attempts have happened.
   Asserts exactly one winner, 15 blocked, published
   busy cleared on Drop and reacquire possible only
   after release.
2. A separate thread closes the real gate while the
   original permit is held; subsequent Drop publishes
   idle but must never reopen authorization; repeated
   close/try_acquire remain denied.

No sleeps, domain mock, test-only production adapter,
or runtime source changes. After the concurrent
batch of 7 separate native methods was reformatted,
the combined newest same-code CI source is
`0e227a9e`, [Preflight 39132196768](https://github.com/morozhenko-apps/rquickshare/actions/runs/39132196768)
and [Native GTK 39132196774](https://github.com/morozhenko-apps/rquickshare/actions/runs/39132196774).
Only actual job logs may certify D2 pass and
overall Stage 11. The strict ten-consecutive
full-source and global Stage 12 remain OPEN.

## Actual scoped D1–D5 final source checkpoint

**D1 — test execution.** Final combined
test code SHA `66ddf1fc24f906e74c9246c31ebc1ad967b0e8d2`
runs both original `core_lib/src/service/client/desktop/tests.rs`
methods. [Preflight 39133124486](https://github.com/morozhenko-apps/rquickshare/actions/runs/39133124486)
Core job `117692233730` reports
**744 stable Rust PASS, 0 failed, 0 ignored**,
with two independent integration binaries
**4/4 and 4/4**; eight predefined
race-sensitive methods are outside
this fast profile. Both new method
names are verified independently
`... ok` in complete Core CI logs
(the same two had previously passed
the intermediate `913fac10` Core
source as well). This is one final
same-source quick validation, not
the release-only full ten-pass gate.

**D2 — category evidence** (two original
Rust methods, overlapping attributions):

| Category | Methods | Execution/result |
|---|---:|---|
| Positive | 2 | PASS: gate releases after sole owner drops and watch is idle |
| N1 malformed | 0 | N/A typed gate interface |
| N2 cardinality | 2 | PASS: sixteen contenders, one permit, repeated attempts |
| N3 network | 0 | N/A no network |
| N4 retry/duplicates | 2 | PASS: reacquire after Drop, always reject after close |
| N5 concurrency | 2 | PASS: actual threads and synchronization barriers |
| N6 stale state | 1 | PASS: closing while prior permit held never reopens |
| N7 authority | 2 | PASS: exclusive grant and permanent shutdown |
| N8 clock/locale | 0 | N/A no timing/locale logic |
| N9 interruption | 2 | PASS: owner release and shutdown during active permit |
| N10 cross-actor | 2 | PASS: no overlapping source-owner permits |
| N11 monetization | 0 | N/A |
| N12 filesystem | 0 | N/A pure capability ownership |

**D3 — test-only files.** Only
`core_lib/src/service/client/desktop/tests.rs`
and docs modified; no gate runtime, network
wire, SDK or installed application changed.

**D4 — refactor requirement.**
The real `Arc<Mutex<...>>` gate can
be tested with deterministic barriers,
so no production test seam/refactor
is required. Actual multi-process
D-Bus ownership and Linux app
shutdown remain separate system
tests; existing integration suite
already covers portions.

**D5 — mutation review.**
Remove exclusive ownership,
allow two permits, reset closed
flag in destructor, retain busy
after drop or allow repeated
acquire after close: each breaks
strong exact assertions. Manual
review only, no executed mutation
engine. Final full 10x Stage 11
and repository-wide Stage 12
remain NO. No production changes.

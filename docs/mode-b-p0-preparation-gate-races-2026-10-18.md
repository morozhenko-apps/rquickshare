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
2. [ ] Add two tests, no production code.
3. [ ] Verify exact CI test names, Core Rust,
   Native GTK and all quick Preflight jobs.
4. [ ] Update D1–D5, inventory/current-state.
5. [ ] Full complete 10× and global Stage12 still NO.
Rollback only tests/doc. No master/package/release.

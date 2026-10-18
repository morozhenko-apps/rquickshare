# MODE B: Native shutdown signal lifecycle (2026-10-18)

## Stage 1-2: repository scan and invariants
`native_desktop/src/lifecycle.rs` owns a process-scoped cancellation token and a Tokio join handle. One existing test covers the signal and explicit-close paths. This scoped Mode B extension protects cancellation races without changing signal registration or production behavior. Invariants: close always returns with the worker joined; explicit close cancels stop; a completed signal cancels stop; dropping a signal source must also terminate its await; duplicate close/signal races are idempotent and not deadlocked.

## Stage 3: complete scoped inventory
`Shutdown::from_signal`: two select branches (explicit cancellation, signal future resolves), source future completes normally/with disconnected channel, and simultaneous readiness. `Shutdown::close`: cancellation and awaited task joining, regardless of previous signal resolution. `Shutdown::new` is OS signal installation and requires integration evidence rather than a unit mock.

Planned test methods: (1) pre-resolved signal future; (2) closed oneshot sender; (3) explicit close preceding a forever-pending signal; (4) concurrent signal-and-close race across independently constructed owners. All are deterministic Tokio unit tests. No sleeps or system signals.

## Stage 4-5: branches and negative categories
| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| from_signal/close | planned signal | NA typed input | planned empty/ready | NA no network | planned repeated cancellation | planned simultaneous readiness | planned dropped sender | NA local token | NA no clock | planned explicit close | NA no PII | NA no billing | NA no storage |
| new | OS integration open | NA | NA | NA | NA | OS process signal integration | NA | process-signal ownership | NA | interruption via real signals open | NA | NA | NA |

## Stage 6-8: interaction and coverage map
Dimensions: signal future {pending, immediately ready, sender dropped, completed by sender} × close {before, after, racing} × token {fresh, cancelled}. Expected new methods: four. Each source branch has an explicit assertion on the observable stop token or successfully joined close; full OS signal semantics remain unverified. File `native_desktop/src/lifecycle.rs`, production functions `new/from_signal/close`.

## Stage 9-12
Implement only test methods. Mutation review: changing signal-resolution cancellation, removing explicit token cancellation or failing to join the worker must break assertions where observable. No production refactor. CI must verify quick GTK/Preflight at final test SHA; strict 10x and repo-wide Stage 12 stay NO until the separate complete gate. Rollback is test-only changes plus this doc.

Status: scoped plan recorded before tests.

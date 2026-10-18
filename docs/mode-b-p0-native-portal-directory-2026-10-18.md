# MODE B P0 Native GTK directory-reveal portal hardening

Date: 2026-10-18. Primary **MODE B (Testing)**. Test-only expansion
`native_desktop/src/portal/tests.rs` on `dev`. No production
portal/security contract changes, no real file-manager launch,
user desktop/session change, installation, `master`, release tag
or hardware/network action. Global MODE B Stage 12 = **NO**.

## Stage 1 — scan and applicable guides

Authoritative user `agent.md`; read
`docs/mode-b-completion-audit-2026-10-18.md`,
`docs/native-essential-priorities.md`,
`docs/native-gtk-coverage-baseline-2026-10-17.md`,
`native_desktop/src/portal.rs`,
`native_desktop/src/portal/tests.rs`,
`native_desktop/src/ingress.rs`, `native_desktop/src/panel/reveal.rs`,
`native_desktop/Cargo.toml` and current native GTK CI.
Production `show_directory_contents` verifies an actual
directory descriptor, excludes `O_PATH` and non-read-only
access *before* connecting to a portal.
`show_on_mode(..., contents=true)` validates the
external `OpenFile` contract through a private D-Bus
broker, request owner, expected returned path, response
sender and code, and no-reply `Close` on pending requests.
The preexisting test fake implements **only the external
portal SDK**; the actual app logic is unmocked.
Preexisting tests cover regular-file rejection,
successful directory request, `OpenDirectory` response
codes (0/1/2/3/92/93) and version 2, portal lost-owner,
and request close on dropped GUI owner.

## Stage 2 — P0 risks and invariants

1. `OpenFile` must never receive a regular file, writable
   directory or `O_PATH` descriptor through the
   `show_directory_contents` public entry. Reject
   unsafe descriptors **before** bus connection.
2. A valid read-only directory must use external
   `OpenFile` (never `OpenDirectory` for contents),
   with a private broker verifying the opened FD,
   token, sender and path. Only response 0 succeeds;
   1=Cancelled, 2=Rejected, >2/malformed/foreign path=Protocol,
   incompatible portal version=Unsupported.
3. A cancellation after portal admission but before
   response must complete as Cancelled, send exactly
   one owned `Request.Close`, free FD, and never
   falsely acknowledge success. Use an event from the
   real external D-Bus mock, not a sleep.
4. These headless tests are contract-level only:
   a real KDE/Wayland file-manager open, unexpected
   portal crash, lock/DND focus and installed IPC are
   separate hardware/desktop gates.
5. No weakening of state/security assertions or
   production code behavior is authorized.

## Stage 3 — scoped exhaustive artifact inventory

| Production artifact and branch | Risk / level | Planned unique tests |
|---|---|---:|
| `show_directory_contents`: O_PATH directory must reject synchronously; regular-file rejection already covered | P0 FS privilege unit/integration | 1 table-driven method: O_PATH, read-only precondition comparison |
| `show_on_mode(contents=true)` + real `interaction`: approved read-only directory, exact OpenFile; response [0/1/2/3/92/93], portal [v2/v3] | P0 D-Bus contract | 1 method with seven rows |
| `show_on_mode`: cancellation after admitted request, before response; owned Close and weak Arc release | P0 async lifecycle | 1 event-controlled method |
| Actual KDE/Wayland portal, OS app installation and file-manager UX | P0 physical acceptance | OPEN separately |

Exactly **3 new original methods**. Actual portal test broker
and filesystem directory handles, no mock of the application
or synthetic timers. Tests remain independent and clean up
their private broker connections.

## Stage 4 — branch map

- `given_path_only_directory_descriptor_when_contents_requested_then_rejected_without_portal_connection`:
  actual Linux `O_PATH` directory with type(dir)=true
  (contrasts valid read-only directory). Expect Protocol
  at FD guard before invoking system bus. Do not open a
  real external portal.
- `given_directory_contents_portal_response_variants_when_received_then_only_valid_openfile_is_acknowledged`:
  actual `show_on_mode(contents=true)`, private
  D-Bus portal, supported/version-old path and six
  response codes, exact returned-path/signal origin.
  Missing path/malformed response returns Protocol,
  code 1 Cancelled, code 2 Rejected; all failing
  pending requests require Close exactly once.
- `given_inflight_directory_contents_portal_when_cancelled_then_request_closes_and_no_success_leaks`:
  response withheld after admitted OpenFile, cancel
  token, resulting Cancelled, one owned Close, underlying
  directory FD released. No sleeps.

## Stage 5 — positive, N1–N12 matrix

C=preexisting coverage; P=new planned; X=not applicable
with reason; OPEN=desktop physical behavior not proven.

| Artifact | Pos | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Descriptor guard | C+P | P non-readable mode | P O_PATH | X no socket | P reject before IPC | X pure guard | P special FD | P no capability escalation | X no locale | X no GUI | P no document launch | X no billing | P descriptor/permissions |
| Contents OpenFile D-Bus | C+P | P malformed response | P version 2/3, codes 0/1/2/3 | P portal rejection | P one response | P early signal before reply | P incorrect path/body | P sender/path token | X no locale | P user cancel code | P FD authority to correct external owner | X | X |
| Mid-request cancellation | C+P | X valid request | X no size | P pending portal | P single Close | P in-flight cancellation | X response not received | P owned request Close | X no clock | P interruption | P no file disclosure/replay | X | X |
| Physical KDE/Wayland and real portal lifecycle | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | X | OPEN |

## Stage 6 — interaction matrix

FD kind {regular preexisting, valid directory, O_PATH
directory} × operation {OpenDirectory preexisting,
OpenFile contents} × portal protocol {v2, v3}
× status {0,1,2,3, malformed signal, wrong object path,
no signal with cancellation} × ownership
{pending admitted, finished, lost/dropped existing}.
All newly planned meaningful combinations within these
three source paths are represented; actual installed
file-manager launch is intentionally excluded.

## Stage 7 — planned counts

3 new methods. Overlapping method-level category attribution:
Pos 2; N1 2; N2 2; N3 2; N4 3; N5 2; N6 2;
N7 3; N8 0 (X); N9 2; N10 3; N11 0 (X); N12 1.
No claim that overlap counts equal independent methods.

## Stage 8 — per-production-file coverage map

`native_desktop/src/portal.rs`: new coverage of
`show_directory_contents` O_PATH fast-fail and
`show_on_mode` + `interaction` OpenFile success,
cancel/reject/protocol/version and pending cleanup.
Existing `checked`, `show_on` and lost-owner tests
unchanged. Remaining: actual system bus/installed
desktop, process crash, portal activation, unusual
real FD privilege modes, Wayland and file manager UX.

`native_desktop/src/ingress.rs`: existing owner
`reveal_job` and shutdown tests; this slice validates
actual request result, not every ingress branch.
`native_desktop/src/panel/reveal.rs`: UI stale
revision guards already have a scoped pure test;
actual UI-to-portal activation requires a live
installed desktop acceptance. Not claimed here.
All other production files remain under global
Mode B Stage 3–8 gap.

## Stage 10 — mutation plan

Replacing the `O_PATH` guard with allow, using
`OpenDirectory` for contents, accepting v2,
wrong reply path, malformed response, allowing
nonzero success codes, failing to issue owned Close
after cancellation, or leaking a descriptor must
fail specific exact assertions. This is a planned
manual mutation sensitivity audit, not an
executed mutation framework.

## Execution and rollback

1. [x] Record scan, risks, full scoped inventory,
   branches, N1–N12, interaction/count/production map
   **before editing any test**.
2. [x] Implement all 3 planned methods in the
   existing private D-Bus contract tests.
3. [x] Check Rust format, native GTK strict job,
   CI exact SHA and actual 3 new method names.
4. [x] Record actual category matrix, failures,
   manual mutation review; update Current State and
   global inventory D1–D5.
5. [ ] Full 10× latest-source Stage 11 and all-branch
   Stage 12 remain OPEN without exact evidence.

Rollback only this doc and the test source commit.
No public APIs, runtime behavior or installation change.

## Stage 9 implementation checkpoint

Three new original Rust test methods committed test-only
in `ceeefe21e4f2f33e70b4a1fbfe11cdc703fb2bc4`:
`O_PATH` directory safety vs a true read-only directory,
actual `OpenFile` through seven v2/v3 status/path/body
branches, and cancel-after-admission of a private
D-Bus request, including exact terminal event, single
Request.Close and FD-owner release. Production source,
portal code, real system bus and panel UI untouched.

Corresponding CI:
[Native GTK 39123998214](https://github.com/morozhenko-apps/rquickshare/actions/runs/39123998214)
and [Preflight 39123998176](https://github.com/morozhenko-apps/rquickshare/actions/runs/39123998176).
Status must be verified against actual jobs/logs; no
PASS or D2 actual counts inferred from a pending run.

## Red formatting triage and retry

At `ceeefe21`,
[Native GTK 39123998214](https://github.com/morozhenko-apps/rquickshare/actions/runs/39123998214)
failed its native rustfmt step on three line-wrapping
differences in the **new tests**. It did not execute the
new Rust methods; there was no observed production
failure. Exact formatter suggestions were applied in
the test-only commit `f63c3df1`, preserving all checks.
[Native GTK 39124127819](https://github.com/morozhenko-apps/rquickshare/actions/runs/39124127819)
and [Preflight 39124127809](https://github.com/morozhenko-apps/rquickshare/actions/runs/39124127809)
are the follow-up exact-code runs. Until they complete
and the per-method logs are inspected, no claim of
passed methods, counts or scoped closure is authorized.

## D1–D5 scoped verified outcome (2026-10-18)

**D1 — execution environment.** The native GTK container
uses the repository's GTK4/Xvfb/private D-Bus harness.
[Native GTK run 39124127819](https://github.com/morozhenko-apps/rquickshare/actions/runs/39124127819)
ran on exact test SHA
`f63c3df1cece64de08de57cadf52683875acd088`.
Actual job log `117665583696`: **55 passed,
0 failed, 0 ignored**, versus 52 in the previous
confirmed native suite. All **three** newly authored
method names appeared exactly **once** with `... ok`.
Native formatter/lint and isolated suite both succeeded.
This is one native quick run, not ten consecutive runs.

**D2 — category attribution (overlapping, not independent test totals).**

| Category | New unique methods | Verified scoped evidence |
|---|---:|---|
| Positive | 2 | PASS: supported OpenFile result and valid FD baseline |
| N1 malformed response/descriptor | 2 | PASS: unsupported descriptor and malformed portal message |
| N2 numeric bounds | 2 | PASS: portal 2/3, response 0/1/2/3/92/93, O_PATH |
| N3 network/portal failure | 2 | PASS: rejected/no response and cancellation |
| N4 retry/idempotency | 3 | PASS: exact single request lifecycle and safe rejection |
| N5 concurrency | 2 | PASS: early portal signal, cancel while admitted |
| N6 corrupt reply | 2 | PASS: wrong returned path and response body |
| N7 security/permissions | 3 | PASS: FD kind/flags, owner and request Close |
| N8 locale/time | 0 | N/A: no local clock/locale data in portal contract |
| N9 interruption | 2 | PASS: user cancellation and portal cancellation code |
| N10 privacy/file authority | 3 | PASS: directory-only FD, no false success or leak |
| N11 monetization | 0 | N/A: no payment/entitlement flow |
| N12 storage/FD failure | 1 | PASS: unsafe O_PATH descriptor blocked before system bus |

Three methods with seven portal status rows plus
the FD and cancellation cases. These counts
do not imply complete coverage across all native
GTK application functions.

**D3 — commits.** `ceeefe21` added tests in
`native_desktop/src/portal/tests.rs` only;
`f63c3df1` applied three exact native rustfmt
line wraps without modifying assertions.
The earlier [Native GTK 39123998214](https://github.com/morozhenko-apps/rquickshare/actions/runs/39123998214)
failed the formatter before test execution and
is retained as a red historical run, not counted.

**D4 — testability.** No production refactor required
for these specific portal calls: the existing
private D-Bus external contract fake represents
the SDK boundary while actual show/reply/cancel
logic runs unmocked. Live KDE/Wayland,
portal activation and file-manager rendering
are out-of-process physical acceptance gates.

**D5 — assertion mutation review.** Mutating the
O_PATH reject, bypassing OpenFile for contents,
admitting portal version 2, treating nonzero codes
as success, accepting wrong object path/malformed
reply, suppressing Request.Close on cancellation,
or retaining the FD would fail explicit
new assertions. This was a manual review;
no mutation engine or global 10× was executed.
Complete Stage 11 on final source and full
Stage 12 still **NO**.

# MODE B P0: Reject unexpected Unix FD in authenticated portal Response (2026-10-19)

## Stages 1–2: repository scan / invariant / scope
Read SSOT `agent.md`, source `native_desktop/src/portal.rs`, existing full `native_desktop/src/portal/tests.rs`, `native_desktop/src/ingress.rs` and the linked [reveal ingress lifecycle suite](mode-b-p0-reveal-ingress-lifecycle-2026-10-19.md). Exact source-tested baseline `b912108f`: [Preflight 39268764131](https://github.com/morozhenko-apps/rquickshare/actions/runs/39268764131) SUCCESS, [Native GTK 39268764105](https://github.com/morozhenko-apps/rquickshare/actions/runs/39268764105) SUCCESS, 84 GTK / 769 Core / 10 Tauri. Code updates restricted to the **test-only** server and test methods in `native_desktop/src/portal/tests.rs`, in `dev` only. `portal.rs`, KDE integration, application behavior, installed packages, and `master` unchanged.

P0: Even a correct, same-UID, uniquely authenticated portal owner may send a `Response` with a Unix file descriptor that the client did not request. `portal::checked` forbids any reply FD, including an otherwise successful response. An invalid signal must **not** report success, and must cause the application to issue exactly one `Close` to the request owned by that same portal. Returning the FD or using it to open/execute a file is forbidden. OpenFile must remain limited to a read-only directory descriptor; OpenDirectory uses a real read-only file descriptor.

## Stage 3: exhaustive scoped test inventory
| Production method | P0 branch | Existing coverage | New test |
| --- | --- | --- | --- |
| `portal::interaction` Response signal `checked` | authenticated signal with an unexpected FD -> Protocol | `checked` pure handcrafted-message test only | yes, real private-broker signal |
| `portal::show_on_mode` `contents=false` | OpenDirectory invalid signal -> owned request Close exactly once | earlier valid/invalid body and cancellation | same test, branch 1 |
| `portal::show_on_mode` `contents=true` | OpenFile directory FD and invalid signal -> owned Close exactly once | earlier valid/invalid body and cancellation | same test, branch 2 |
| Real KDE portal and `Panel::reveal_target` | native desktop lifecycle and actual approval | no physical desktop run | OPEN; not a claimed integration pass |

Exactly **one** new named async method using the existing `Broker` and real D-Bus clients, with one test-only `Portal` response mode. No new mock client, production DI or architecture changes.

## Stage 4: actual branch map
`interaction`: owner authenticated, portal version >= 3, `OpenDirectory/OpenFile` request authenticated with received FD and expected handle_token, response path accepted; response signal arrives with nonempty `message.data().fds()` -> `checked` returns `Error::Protocol`, not `Ok` and not `Cancelled`; `pending` remains owned and `show_on_mode` builds one `Close` with `NoReplyExpected`. Unknown owner, no FD, wrong path, wrong Response type and code 0/1/2/3 are already tested; do not duplicate.

## Stages 5–6: Positive/N1–N12 and interaction matrix
| Flow | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Authenticated unexpected FD Response | existing valid `code=0` | invalid extra body `h` | exactly one FD | real private D-Bus | pending Close on rejection | no races invented, single actor | no persisted state | reject unrequested descriptor authority | no timing dependency | wrong response after accepted request | no descriptor opened by client | no entitlements | no target file writes |
| OpenFile vs OpenDirectory | existing approved OpenFile/Directory | reject on both modes | 2 modes | exact D-Bus path | request Close once | signal emitted before call reply | no schema migration | directory FD type checked | no wall-clock | no UI fixture | no accidental payload execute | N/A | temp read-only file or directory |

Negative scenario interactions: `Response` {good code but extra FD} × `contents` {false, true} × `OpenDirectory`/ `OpenFile` exact method × `Close` count {0 vs 1 vs duplicate}; assert both errors, admissions and exactly one Close. No sleeps, real device or network access. Important limitation: appended `Fd` changes the body signature as well as contributing an FD to the D-Bus header. This establishes rejection of the actual FD-bearing response, but cannot isolate the FD guard from signature mismatches under a mutation-only criterion.

## Stages 7–10: coverage map, implementation, mutation review
Test-only `portal/tests.rs`: extend existing `Portal::open_directory` fake-server code with one reserved response code (94), emitting an otherwise-success code with an additional borrowed `Fd`; add one async method that exercises both portal modes using real test broker and distinct file/directory descriptors. Production target methods: `native_desktop/src/portal.rs::{checked,interaction,show_on_mode}`. Mutation targets: accept unexpected FDs, discard pending without Close, select OpenDirectory even for contents, open a regular file in OpenFile, return Ok for an invalid Response. This is manual mutation design, no reported kill percentage.

## Stages 11–12: verification gate
Run exact-source Native GTK strict format/Clippy/isolated tests; Preflight as triggered. Scope is successful only with named test `ok` and all required jobs green. Recompute llvm-cov only on a new manual instrumented run, not from test counts. Global Stage 11 full-suite 10× and Stage 12 on-device, current-source branch instrumentation, KDE/Android/BlueZ/mDNS acceptance remain OPEN. Rollback solely the test-only additions and this document.


## Stage 9 implementation checkpoint
- Documentation-first plan committed at `b4d7ff33`.
- Code SHA `ea47d49d`: one new async test `given_authenticated_portal_response_with_unrequested_fd_then_both_reveal_modes_reject_and_close_once` and one reserved `code == 94` behavior in **test-only** `Portal` interface; source limited to `native_desktop/src/portal/tests.rs`. Test uses broker-authenticated same-owner Response with success code `0` and one borrowed Unix FD, and checks exact `Error::Protocol`, request admission and single owned `Close` for both OpenDirectory and OpenFile.
- Exact-source CI: [Native GTK 39282270300](https://github.com/morozhenko-apps/rquickshare/actions/runs/39282270300), [Preflight 39282270240](https://github.com/morozhenko-apps/rquickshare/actions/runs/39282270240). At this checkpoint queued; do not claim passed. If rustfmt/Clippy/runtime finds fixture issues, diagnose the exact logs and repair test-only code with unchanged production semantics.
- Completion gate: source-matched named test must pass on isolated GTK; Core/Frontend/Tauri may also execute but cannot imply actual KDE portal acceptance. 10× entire-suite and on-hardware checks remain OPEN.

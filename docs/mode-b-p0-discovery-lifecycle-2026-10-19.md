# MODE B P0 — Discovery command ownership and cancellation (2026-10-19)

## 1. Environment, source baseline and repository scan
Environment: remote GitHub connector with write access to `dev`, no attached user Ubuntu GUI/BlueZ, local repository shell or live Android network. Baseline `e734477e63a3296b29711e64aae76eb8cdf3a6d7`. [Last Preflight 39302655899](https://github.com/morozhenko-apps/rquickshare/actions/runs/39302655899): all five jobs SUCCESS; Core Rust 769/769 and Tauri Desktop 12/12. Read `app/main/src-tauri/src/{main.rs,cmds/discovery.rs}`, `core_lib/src/{lib.rs,lifecycle_tests.rs,hdl/mdns_discovery.rs}`, recent MODE B audit and uploaded updated universal AGENT SSOT.

`start_discovery` and `stop_discovery` are thin Tauri `State<AppState>` wrappers around `RQS::discovery` and `RQS::stop_discovery`. Core discovery creates real `mdns_sd::ServiceDaemon` and reaches real sockets; event-loop and reachability dependency boundaries are already tracked under [issue #9](https://github.com/morozhenko-apps/rquickshare/issues/9). MODE B **cannot** add a fake `RQS` or alter production code to make this command testable.

## 2. Risk analysis and invariant, architectural STOP
Source-level P0 concern in `core_lib/src/lib.rs::RQS::discovery`: after checking `tracker.is_some()`, it creates a new `CancellationToken`, unconditionally sets `self.discovery_ctk=Some(token.clone())`, creates a new `MDnsDiscovery`, and tracks a new worker. It does **not** cancel or return an error for a previous discovery generation. Consequently, a second invocation can orphan the first generation's cancellation token; `stop_discovery` only cancels the most recently stored token, allowing multiple tracked workers to continue until global shutdown. The code-path risk is established; a real multi-device reproduction is **not** claimed.

The desired repeated-`start_discovery` contract is **an architectural STOP**: choose between rejecting duplicates/maintaining the first generation, or explicitly replacing it with a cancel-and-join lifecycle; neither should be silently chosen under MODE B. Both alternatives must keep one active discovery owner, preserve sender/subscriber semantics and prevent stale generation events. No MODE A fix or failing test weakened to claim success in this slice. A linked issue will track this.

## 3. Exhaustive IN-SCOPE test inventory and production-file map
| Source / method branch | Priority | Level | Test plan |
| --- | --- | --- | --- |
| `RQS::discovery`: `tracker=None` => `The service wasn't first started`, no mDNS daemon or token installed | P0 | isolated core unit; no radio | 1 Given/When/Then method |
| `RQS::stop_discovery`: active token cancels, reference removed, second stop idempotent, core service + independent workers remain active | P0 | Tokio lifecycle integration; real tracker+tokens | 1 Given/When/Then method |
| `RQS::stop`: afterwards full owner shutdown joins surviving core tasks | P0 | same test, exact owner gate | 1 existing test strengthened in new method |
| `RQS::discovery`: repeated start, stop/restart, stale workers and sender replacement | P0 | mDNS SDK test seam or real network | **BLOCKED** pending architecture choice and issue #9, no CI coverage claimed |
| Tauri `cmds::start_discovery/stop_discovery` `tauri::State<AppState>` injection and UI listener cardinality | P0 | real AppHandle E2E | OPEN, requires fixture/contract and choice above |

Production files `core_lib/src/lib.rs` and `app/main/src-tauri/src/cmds/discovery.rs` remain unchanged. Test-only addition: `core_lib/src/lifecycle_tests.rs`. Exactly **two** independently named tests planned, no new test-only production DI.

## 4. Branch map
- pre-start request -> error before constructing `MDnsDiscovery`, no `tracker`/`discovery_ctk`/lifetime mutation; repeated attempt remains an error.
- active synthetic discovery lifetime *representing token-owning state, not SDK service* -> `stop_discovery` cancels exactly this token, clears ownership; duplicate stop has no effect; does not cancel `self.ctoken`; does not close tracker or visibility policy.
- independent worker (real tracked Tokio task) observes cancellation; global `stop()` cancels general lifetime, closes/joins work and permits clean independent start.
- repeated real `discovery()` while active **unresolved**; never assert existing defective behavior as desired.

## 5–6. Positive/N1–N12 and interaction inventory
| Flow | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Prestart reject | valid result is rejection before start | missing tracker | repeated 0/1/2 calls | N/A: no SDK reached | repeated rejection | N/A: no task | N/A | no synthetic start / token | N/A | no UI | no endpoint events | N/A | N/A |
| Stop lifecycle | owned token cancellation, core still alive | absent token | one/two stops | real Tokio channel, no network | idempotent stop / full shutdown | cancellation worker and caller independently | N/A | no stray stopped service | no wall-time | no GUI | no broadcast endpoint leaked by test | N/A | N/A |

No clocks, sleeps, brute-force CI loops or fabricated device handles. Interaction: prior startup {none, real core tracker} × discovery ownership {none, owned token} × stop calls {0,1,2} × global lifetime {active, stopped}. N categories marked not applicable have no business meaning for these methods.

## 7–10. Coverage, execution and mutation review
Exactly two deterministic test methods, explicit source-level error, `CancellationToken::is_cancelled`, `tracker.is_closed`, `tracker.is_empty`, internal option clearing, `watch` visibility unchanged. Use `tokio::sync::oneshot` to acknowledge actual tracked worker execution, no sleeps. Mutation hypotheses: skip tracker precondition (would initialize a network daemon), fail to clear `discovery_ctk`, cancel general `ctoken` instead of discovery, stop tracking workers, or treat subsequent `stop_discovery` as an error. Real repeated start mutation **cannot** be killed by these limited scenarios and remains blocked.

## 11–12. Final execution & completion, updated SSOT
Current uploaded AGENT SSOT Stage 11: final **STABLE suite once**, known **FLAKY once separately**, targeted bounded reruns on failure. It explicitly states **do not require ten consecutive complete-suite successes** except separately required CI/release checks. Historical completion-audit references to 10x are old requirements, not a current generic gate. Once code is ready, use GitHub Preflight exact-source, confirm stable Core and other jobs and any separately configured flaky jobs; **no repeated polling**, if the CI is not completed record exact next check. Docs and test code must be separate commits. Scope is PARTIAL unless the two tests pass; even after pass, repeat-start contract, mDNS integration and full repo-wide Stage 12 remain OPEN. No production changes or main merge.

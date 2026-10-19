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

## Stage 9 — implementation and source-exact validation entry

- Docs-first plan commit: `62aee485`.
- Architectural repeated-start problem recorded as [P0 issue #12](https://github.com/morozhenko-apps/rquickshare/issues/12). Two viable design choices and recommended safe contract are captured there. **STOP on the affected production correction** until a user design decision. No claim that the duplicate worker is reproduced on hardware, or that these tests cover its resolution.
- Test-only commit `b8545375053bb132bff4d30db744df819df125fc` adds **two** named async tests to existing `core_lib/src/lifecycle_tests.rs`:
  1. `given_core_not_started_when_discovery_requested_then_no_browser_or_token_is_created`: two successive calls to real `RQS::discovery` reject before the mDNS constructor, preserve `tracker`, global lifetime and discovery ownership, then stopping remains harmless.
  2. `given_owned_discovery_when_stopped_twice_then_core_lifetime_survives_until_owner_shutdown`: real `RQS` lifecycle, task tracker and cancellation tokens, synchronized Tokio worker barriers, exact token cancellation and idempotent second stop, survival of core visibility/tracker and proper final Core shutdown/join. This does **not** instantiate a real mDNS worker and is not a substitute for duplicate-start/SDK event tests.
- As of the first source check, [Preflight 39309762791](https://github.com/morozhenko-apps/rquickshare/actions/runs/39309762791) and [Native GTK 39309762487](https://github.com/morozhenko-apps/rquickshare/actions/runs/39309762487) were **in progress** on exact test-code SHA `b8545375`. Results not inferred. Do not poll: use completion event or a single later status check. Production, installed binary, `master` and release unchanged.
- Stage 10 manual mutations: bypass `tracker` precondition, drop cancellation instead of invoking it, leave `discovery_ctk` populated, cancel global token from `stop_discovery`, prematurely close tracker. Duplicate-generation mutations remain untested and **blocked by issue #12/#9**.
- Stage 11 updated SSOT validation: one stable gate, separate existing known-flaky group, bounded targeted reruns only on actual defects. Historic 10× wording in previous audits does not override the uploaded AGENT Stage 11 or independently mandatory release workflows. Stage 12 is globally OPEN.


## Read-only frontend interaction review

`app/main/src/composables/ContentStatus.vue::ensureDiscovery` and `app/main/src/components/HomePage.vue` both conditionally invoke `start_discovery` when the frontend-local `discoveryRunning` flag is false. `ensureDiscovery` updates that flag **after awaiting** the IPC invocation; simultaneous UI triggers can therefore both observe false and invoke the command. This is a source-level duplicate-start interleaving, not a measured end-user reproduction. The Tauri `AppState::dch_sender` is shared and each call clones the **same broadcast sender**, supporting the idempotent-single-worker option but not resolving how endpoint cache replay after UI remount should work. Do not rely on disabling a button as the sole concurrency boundary. The CORE owner must enforce single-flight semantics once the architecture decision is made.

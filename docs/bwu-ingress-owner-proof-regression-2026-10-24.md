# MODE B: BWU unauthenticated ingress — fail-closed regression oracle

Date: 2026-10-24. Source: `dev`, no release or installed-package changes. Related P0: [#8](https://github.com/morozhenko-apps/rquickshare/issues/8). This is a bounded **MODE B testing** slice. Production security/pending-route policy is a STOP FACTOR and **must not be changed** before the user chooses a protocol-compatible Mode A design.

## D1 / repository scan, source and reproducible diagnostic

Live `core_lib/src/manager.rs::route_bandwidth_upgrade_if_pending` classifies a TCP socket using only a plaintext `CLIENT_INTRODUCTION` frame `endpoint_id` and `last_endpoint_id`, then forwards these untrusted strings to `core_lib/src/hdl/bwu.rs::BwuRouter::route_with_alias`. `route_with_alias` removes a matching pending one-shot sender from its map before the BLE owner can authenticate the claiming TCP peer. `core_lib/src/hdl/inbound.rs::do_bandwidth_upgrade` repeats plaintext ID matching after socket delivery; the established BLE secure-session cryptography does not bind the candidate TCP socket *before consumption*. It can subsequently fail and retain BLE, but the original pending route and upgrade opportunity have already been consumed.

Prior unit tests `routes_bwu_socket_without_consuming_client_introduction` and `routes_dynamic_bwu_endpoint_by_last_endpoint_id` use real TCP loopback, prove syntactic routing and byte preservation, **not ownership authentication**. `hdl/bwu.rs` covers duplicate, cancel, alias priority, concurrency and replay-after-consumption; its ordinary unknown endpoint tests do not cover **guessed exact or previous alias** theft. Source-level bug is established; a real Google Android attack was **not performed**, and encrypted payload access is not established.

Plausible observed-path contributors: (1) plaintext exact ID used as admission capability; (2) unverified previous alias accepted; (3) one-shot channel removed on assignment; (4) no authenticated proof in pre-consumption TCP intro; (5) downstream validation checks the same attacker-controlled fields; (6) route cancellation on failed setup does not restore prior waiter; (7) race with legitimate connection, where first classified socket wins. Root cause is (1)–(4), not TCP packet fragmentation or local SDK timing. No test may normalize this as correct.

## D2 / invariants, authority and architecture STOP

- Required **future** behavior: unauthenticated TCP introductions that claim a pending exact/previous ID must not irreversibly consume another authenticated BLE session's BWU route; the real owner must still be eligible.
- A valid Google Quick Share dynamic endpoint migration must continue working. No breaking on-wire challenge may be added without compatibility evidence or an explicit choice. Direct IP/MAC/name equality is **not proof**, and routing a socket early but allowing the session to keep old BLE is **not sufficient** to satisfy the availability invariant.
- This test slice cannot implement any gate. It adds explicit `#[ignore = "... #8 ..."]` fail-closed oracle tests that are **expected red on current source if executed**; they are not silently counted as passing stable tests. Exact required behavior is not relaxed. No sensitive peer data is logged (test IDs are synthetic), no real BLE/phone involved.
- Decisions requiring approval: authenticated admission proof from existing protocol vs staged candidate reservation/handshake with verifiable rollback/rearm; a new wire challenge that breaks Google interoperability is not an acceptable fallback. The owner must authorize a concrete protocol-compatible architecture; issue #8 still OPEN.

## Stage 3/4: exhaustive in-scope inventory / branch map

| Production artifact / branch | Risk | Level | Plan |
| --- | --- | --- | --- |
| `manager.rs::route_bandwidth_upgrade_if_pending`, unknown intro | P0 | real loopback contract | existing green `unregistered_bwu_introduction...` |
| Same, parse invalid/truncated/oversized | P0 | real TCP and parser | existing green `malformed_frame...`, `given_non_bwu_tcp_ingress...` and segmented test |
| Same, matching current claimed ID | P0 | real TCP fail-closed diagnostic | ignored new exact-ID oracle, target false consumption |
| Same, matching previous claimed ID | P0 | real TCP fail-closed diagnostic | ignored new alias-ID oracle, target false consumption |
| `BwuRouter::route_with_alias`, consumed/replay/cancellation | P0 | real socket owner contract | existing green replay, cancel and eight-way competition tests |
| `InboundRequest::do_bandwidth_upgrade`, subsequent secure BLE fallback | P0 | full secure-session integration | OPEN: new secure owner gate is not designed/approved; no falsified coverage |
| External real Quick Share interoperability | P0 | physical | OPEN, independent acceptance |

Planned **two independently named ignored tests** sharing one loopback fixture, with exact-ID and previous-ID adversarial inputs. They assert the original sender was not fulfilled and the registration remains pending. The ability of a legitimate authenticated TCP session to complete will require a separate fixture after the Mode A authenticated admission contract is implemented. No field-value heuristics from claimed endpoint strings can decide legitimate vs attacker; this test exposes the missing authenticated proof, rather than offering a matching-string bypass.

## Stage 5: Positive / N1–N12 (each cell explicit)

| Flow | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Spoofed exact/previous claim | existing valid route tests | existing parser malformed | IDs of two kinds planned | loopback only; no external network | replay existing; further physical open | first-arrival race scenario planned | unavailable stale epoch requires secure binding | **planned** unauthorized-consumption oracle | NA no locale/time semantics | cancellation/rearm expected, currently unsatisfied | fixed synthetic IDs, no PII | NA no entitlements | NA no storage |
| Current classifier non-BWU | existing non-BWU socket-preserve | existing invalid frame tests | existing 0/oversize/fragment | loopback | existing repeated unknown | existing concurrency | NA no versioned data | existing rejected unknown | NA | existing cancellation | no secrets in test | NA | NA |

Question-mark cells above are explicitly `planned` and are not counted as achieved; no other flows in this narrow slice. `N5` is the first-arrival competition between a malicious and a legitimate candidate; without a security proof this is **not** a correct green test. Existing negative checks are still part of source-exact stable Core validation.

## Stage 6/7/8 interactions, estimation and coverage map

- Interactions: current-ID vs previous-ID spoof × pending single BLE owner; forged claim arrives before legitimate connection; malicious socket vs byte-preserved candidate; cancelled/late claimed route; replay/new generation. Only one pair of direct spoof scenarios is implemented as a disabled, expected-red **oracle**, not claimed as active P0 protection.
- Two independently named ignored negative cases and one shared test fixture, no production refactor, zero stable behavior changes. Coverage improvement is **diagnostic**, not measured line/branch coverage. `manager.rs` current/alias unsafe branch has a fail-closed expectation in source; actual passing security branch coverage is **0** until Mode A. `hdl/bwu.rs` registration/replay/concurrency existing tests remain; `hdl/inbound.rs` authenticated BWU ingress needs contract testability after decision.
- Required risk delta: without actual preconsumption proof, the test should fail if `#[ignore]` is removed; **do not** rewrite it to assert a stolen route is correct or hide the P0.

## Stages 9–12 execution / mutation / completion

- [x] Source scan and 5–7 causal hypotheses, user-approved limits/STOP
- [x] Doc-first test inventory, interaction, N-matrix and per-production-file coverage
- [x] Add two ignored, deterministic real-loopback fail-closed oracles in one test-only commit
- [ ] Source-exact final Preflight and Native GTK; verify test compiles and remains visible as ignored, not passed
- [ ] Mode A approved and real authenticated admission implemented
- [ ] Re-enable and actually pass both spoof cases; real Android/Windows physical gate
- [ ] Repo-wide Mode B Stage 12 full branch/interaction/mutation evidence and 1.0 release acceptance

Mutation analysis: removing any future credential check or allowing early route removal must make the oracle fail; an unknown ID test does **not** prove this. The current unapproved security behavior means Stage 12 **NO**. Revert the test-only commit independently; no new protocol fields, APIs or installed process changes.

## 2026-10-24 implementation checkpoint

- Test-only change on `dev`: `f6e36cf491e7b4d41c5a7eeda119631a5b61cef0`, `core_lib/src/manager.rs` only. Two independently named `#[ignore]` Tokio tests are `given_forged_current_bwu_id_when_classified_then_established_owner_is_not_consumed` and `given_forged_previous_bwu_id_when_classified_then_established_owner_is_not_consumed`. They share an actual TCP loopback socket+real `BwuRouter`, not a mocked classifier.
- Inputs are synthetic `established-ble-session` and `untrusted-dynamic-id`. The malicious frame contains valid protobuf but no credential-bound ownership proof. Each oracle expects the pending registration and original one-shot receiver to remain intact; because current classifier consumes by plain string matching, the oracle is **deliberately ignored/expected red** pending the MODE A architecture decision. The test comments explicitly state the unresolved issue. No source function, Rust public API, protocol message, certificate source or timeouts changed.
- Automatic final-source [Preflight 40086478024](https://github.com/morozhenko-apps/rquickshare/actions/runs/40086478024) and [Native GTK 40086478029](https://github.com/morozhenko-apps/rquickshare/actions/runs/40086478029) were **IN PROGRESS** at commit checkpoint. Verify terminal statuses and exact ignored test names/count in Core output; a build passing does not mean both ignored oracles are passing.
- No injected real Android/Windows adversarial test, no exploit claim against payload confidentiality or privacy, no additional production remediation. Source-level availability risk and the #8 architectural STOP remain OPEN.


## 2026-10-24 MODE A decision and CI inventory correction (supersedes ignored-oracle staging)

The owner explicitly approved Option A: **provisionally stage a TCP candidate and commit it only after verified binding to its BLE session**. The approved security invariant stands. Source audit plus public Google Nearby `ForBwuIntroduction` shows that the current CLIENT_INTRODUCTION/ACK contains no such verified TCP proof; current BLE encrypted prior-channel traffic cannot alone authenticate a separate TCP socket. The protocol-compatible proof source is an additional **material STOP**. See [MODE A feasibility and milestones](https://github.com/morozhenko-apps/rquickshare/blob/dev/docs/bwu-option-a-admission-design-2026-10-24.md). No speculative challenge, IP heuristic, or protocol format change has been implemented.

The first two `#[ignore]` fail-closed oracle tests at `f6e36cf4` made [Preflight 40086572309](https://github.com/morozhenko-apps/rquickshare/actions/runs/40086572309) **FAIL** its deliberate stable inventory: **781 passed, 2 ignored, 8 filtered** versus exactly **783 passed, 0 ignored** required by `scripts/ci-core-test-groups.sh`. This is a genuine CI policy failure even though 781 independent test methods succeeded. The correct resolution is not to modify the test-group script or exclude vulnerabilities silently. Test-only `c4dc1f3ed8bd5ee87d511e7fb31508e03ee6d135` changed the two metadata annotations to `#[should_panic(expected = ...)]`, renamed them to explicitly signify detection of **known** unsafe routing, and preserved the actual fail-closed assertions byte-for-byte. This makes the risk reproducible rather than ignored: the expected panic confirms the security gap persists, and when the actual authenticated admission fix lands those two methods must be converted into normal non-panicking safety tests. **Passing a should-panic test here is not security acceptance**.

Initial `c4dc1f3e` source CI caught exactly two nightly rustfmt attribute-layout changes; those were applied as test-only `6974da5e9f0453a86395acd7c1cce088c01f1313`. Pending source-exact [Preflight](https://github.com/morozhenko-apps/rquickshare/actions) and Native GTK terminal results for `6974da5e`. The security admission failure remains OPEN under issue #8; Stage 12 is NOT complete.

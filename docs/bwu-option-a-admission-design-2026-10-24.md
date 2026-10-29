# MODE A — authenticated BWU candidate admission: protocol feasibility gate

Date: 2026-10-24. Explicit owner selection: **Option A**, stage the TCP candidate and require verifiable binding to the already established encrypted BLE session before the route is irrevocably assigned or the old medium is abandoned. Issue #8. This memo preserves that choice without inventing a proof unsupported by the real protocol. Source baseline `33169356702295e66e8a98d76de51d6e6d5ce222` on `dev`. Never modify `master`, installed 0.9.0-1, wire frames or trust state from this design-only phase.

## 1. Protocol/code audit: WHY a two-phase local reservation alone is insufficient

1. `core_lib/src/manager.rs::route_bandwidth_upgrade_if_pending` **peeks an unencrypted TCP CLIENT_INTRODUCTION** and passes `endpoint_id` and optional `last_endpoint_id` to `BwuRouter::route_with_alias`.
2. `core_lib/src/hdl/bwu.rs::BwuRouter` matches these strings and removes its one-shot owner *before* `InboundRequest` sees any TCP bytes. Matching exact or previous IDs is deterministic, but the TCP sender is not authenticated.
3. `core_lib/src/hdl/inbound.rs::do_bandwidth_upgrade` waits for that one TCP socket, reads the same plaintext CLIENT_INTRODUCTION, sends an unencrypted CLIENT_INTRODUCTION_ACK, then sends encrypted LAST_WRITE_TO_PRIOR_CHANNEL on **BLE**. It processes encrypted BLE drain responses and only after sending a prior-channel DISCONNECTION assigns `MigratableStream::Tcp(tcp)` to the encrypted session. Existing TCP crypto keys/counters are carried to the new medium **after** this point.
4. `PeerAuthentication` proves a certificate signature on the established secure session when configured, but production uses `AnonymousSource`, and even an authenticated BLE session alone **does not authenticate a distinct TCP socket**.
5. Inspected public Google Nearby `offline_frames.cc::ForBwuIntroduction/ForBwuIntroductionAck` and the `bwu_manager.cc` prior-channel shutdown flow. The public introduction fields are endpoint strings and `supports_disabling_encryption`, not a session-bound signature/MAC. A new mandatory challenge sent before CLIENT_INTRODUCTION_ACK has **no demonstrated Google Quick Share response contract** and could break phone compatibility. This is source evidence of a current proof gap, not a claim to have audited all Android proprietary implementations.

Reference: https://github.com/google/nearby/blob/main/connections/implementation/offline_frames.cc and https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/02354e29683393549b588a3ce2599a717f6da98e/connections/implementation/bwu_manager.cc .

## 2. Threat model and invariants

A LAN adversary may learn/guess the four-byte Nearby endpoint label and race the genuine device's TCP connect, claiming `current` or `last_endpoint_id`. The consequence source-proved today is *early consumption and potential denial of bandwidth upgrade*, not AES-GCM secret compromise. Our implementation must never confuse successful BLE identity verification with TCP channel ownership. Exact vs alias winner choice is not authorization. Retry/backoff, private IP matching, TCP source address, fixed sleep, UI prompts, a random local routing ID or logging do not constitute cryptographic proof.

The approved end-state:

- Candidate admission is provisional, bounded in space and lifetime, cancellable, never closes another peer's authenticated BLE session or consumes another owner's pending route on an unproven string.
- The candidate must demonstrate key possession or a transcript-bound MAC **on that same TCP stream**, tied to current BLE transcript/session/epoch and expected role, with replay defense and no raw keys exposed.
- Only validated evidence authorizes commit; failed and timed-out candidates release reservations while the original BLE session and pending route remain owned by the same real peer. At most one accepted candidate per generation; double/foreign owners cannot steal pending route.
- Successful Google Android/Windows Quick Share without an additional proprietary client is a required compatibility invariant. The bytes or negotiation feature flags supplying the proof must be verified from actual peer behavior before making an on-wire change.
- The existing encrypted channel/sequence ownership, consents, file capabilities and no-automatic-trust controls must not regress.

## 3. Mode A milestone plan / STOP boundary

**A0 — source audit (completed):** trace Manager → BwuRouter → Inbound BLE-to-TCP migration, verify protocol data from publicly accessible Nearby source and distinguish *secure prior medium* from *secure new TCP medium*.

**A1 — deterministic security baseline and CI integrity (planned):** retain two real-loopback, exact/previous-id *fail-closed* oracle assertions. In the current buggy implementation these inevitably panic. Convert their `#[ignore]` metadata into `#[should_panic(expected = "P0 #8: unauthenticated BWU CLIENT_INTRODUCTION stole the pending owner")]` **without touching or relaxing the fail-closed assertions**, and rename the methods to make their meaning explicit: these are **known-vulnerability sentinels**, not a security-accepted passing gate. The stable CI inventory computes all test entries and rightfully requires **0 ignored**. Two ignored tests created a genuine CI failure `Core stable result does not match the validated unit-test inventory (783)` despite 781 other tests passing. Expected-panic sentinels preserve executable evidence without hiding the vulnerability as `ignored`; they must be replaced by ordinary **no-panic** fail-closed tests as part of the eventual security fix. Do not modify the CI script/count, weaken tests, or count them as implementation success.

**A2 — proof-binding architecture (MATERIAL STOP):** identify **an actual compatible transcript or Google-supported capability** by which the legitimate Android peer sends an unforgeable proof on the candidate TCP socket before irreversible BWU route consumption/old-medium close. The currently inspected TCP introduction and BLE drain frames do not provide such evidence. Implementing a staged map without this proof only shifts the DoS to a different window. A mandatory new challenge/MAC would be a wire-contract modification; permitting anonymous legacy connections would retain the current P0 failure. Do not implement either silently. This is a *distinct protocol-compatibility choice* beyond the owner-approved Option A, not a repetition of A-vs-B.

**A3 — post-proof implementation (BLOCKED by A2):** only after compatible proof identified and authorized: route candidates via a bounded authenticated-provisional owner API; preserve a one-shot commit and exactly-once rollback; keep BLE session running until candidate proof; implement exact/previous alias, replay, two owners, malicious first + legitimate second, expiry, cancellation, and encrypted sequence continuity. Security oracle tests must then be changed from expected-panic to strict passing safety assertions.

**A4 — validation (BLOCKED by A2/A3):** unit/integration tests at native Core and real TCP/crypto fixture; strict fmt and Clippy; source-exact Preflight/GTK, mandatory final Mode B stable and separate race-sensitive suite, and real Pixel/Windows acceptance.

## 4. Coverage and branch mapping, concrete A1 inventory

| Source and branch | Prior test evidence | A1 improvement | Remains blocked |
| --- | --- | --- | --- |
| `manager.rs` non-BWU malformed/fragmented | existing green loopback | no change | protocol proof, malicious same ID |
| `manager.rs` exact and previous-ID claim | green routing tests prove no bytes lost | two **expected-panic** known-vulnerability sentinels with real TCP + full protobuf | fail-closed authentication |
| `hdl/bwu.rs` pending remove/alias/unique | existing concurrency, cancellation and replay tests | no production refactor | scoped candidate reservation and cryptographic commit |
| `hdl/inbound.rs` BLE drain, LAST_WRITE, SAFE_TO_CLOSE, DISCONNECTION | existing migration/framing fixtures | protocol feasibility analysis, no production change | verify proof **before** BLE closes |
| `peer_authentication.rs` ECDSA and anonymous outcome | existing crypto test vectors and tests | no change | new medium binding proof |
| `scripts/ci-core-test-groups.sh` stable inventory | expected `(test_count - 8)` **passed, 0 ignored** | preserve unchanged contract | never suppress/skip security tests to greenwash |

N1 empty/wrong IDs and malformed frame: existing parser checks; N2 0/1/2/8 independent routes: existing tests; N3 disconnected TCP: existing; N4 duplicate/replay: existing; N5 simultaneous candidate: existing one-shot test does not confer security; N6 old alias: security sentinel; N7 exact/previous-id spoof: security sentinels; N8 names/locale N/A for ID-auth; N9 cancel/late: existing; N10 no real IP/name/keys in sentinels; N11 entitlements N/A; N12 file authority remains untouched. Coverage *of correct authenticated admission* stays zero until A3. Source-level P0 is not claimed solved.

## 5. Rollback / evidence gate

A1 is test-only and reversible; CI script unchanged; feature gates and installed binary untouched. If source-exact Core/GTK passes, document the terminal outcomes and exact test names **as expected-panic diagnostics, not proof the security bug is fixed**. If A2 requires breaking wire compatibility or disabling unverified BWU, apply an explicit new STOP decision with trade-offs; there is no architecturally sound way to infer an untransmitted proof. Do not spin/poll for CI.

## A2 protocol compatibility: concrete decision required

The approved Option A is retained. The remaining question is what to do with **peers whose legacy TCP introduction has no authenticated binding proof**, notably current Google Quick Share Android builds. At least two defensible product contracts remain, with distinct consequences:

**A2.1 (preferred for a security-blocking release): authenticated-capable peers use two-phase BWU; legacy peers keep the already encrypted BLE connection without Wi-Fi upgrade**. Pros: no unverified TCP ownership and no invented wire frames; contents remain on the established crypto channel; retry/failure semantics explicit. Cons: physical Pixel transfers would be slower until interoperable TCP proof is identified; needs hardware confirmation that the legacy peer tolerates declining an offered upgrade without dropping the transfer; this is a material fallback change.

**A2.2: keep shipping the existing legacy upgrade path while researching authenticated on-TCP proof, then switch all supported peers at once**. Pros: existing Android BWU throughput and protocol behavior unchanged in the meantime; no compatibility regression while measuring. Cons: the demonstrated unauthenticated route-consumption availability risk remains in production; unsuitable for closing #8 or claiming security-ready 1.0.0.

A speculative mandatory challenge/proprietary packet before CLIENT_INTRODUCTION_ACK is **not** presented as a sound third option: available upstream frames contain no such commitment, so claiming Google compatibility without a cooperating peer would be technically false. Likewise, choosing source-IP equality or waiting for a BLE-only encrypted message is not proof the candidate TCP socket belongs to that session.

The preferred decision is **A2.1** if secure staged BWU must ship before there is an interoperable proof mechanism. If full-speed Google Quick Share BWU is a non-negotiable 1.0.0 invariant, choose **A2.2** and leave the security gate formally OPEN while a genuine protocol-level solution is researched. Explicit owner decision is required because this changes the throughput/backward-compatibility contract independently from selecting Option A.


## 6. A1 source-exact CI completion

Final executable source SHA: `6974da5e9f0453a86395acd7c1cce088c01f1313`.

- [Preflight 40087230904](https://github.com/morozhenko-apps/rquickshare/actions/runs/40087230904): **completed/success**. Strict Rust format, Core Rust stable tests, Core Clippy, Tauri Clippy and frontend all succeeded.
- [Native GTK 40087230963](https://github.com/morozhenko-apps/rquickshare/actions/runs/40087230963): **completed/success**, independently.
- Core job `120852660106` actual log: **783 passed, 0 failed, 0 ignored, 8 filtered** plus 4/4 modern-protocol and 4/4 native-daemon integration suites.
- Both `manager::tests::given_forged_*_then_known_pre_auth_theft_is_detected` methods ran and logged `should panic ... ok`. Their `Ok` outcome means **expected reproduction of the current unverified TCP route-consumption defect**, **NOT** authenticated admission success. They must become ordinary no-panic security assertions during A3.
- Neither authenticated TCP candidate staging nor route-owner proof was added to the executable. A2 **STOP** and the user-facing compatibility decision remain pending. Tests alone cannot close P0 #8.

This closes only the A1 reproducibility/CI repair. Global Mode B Stage 12 and hardware Google Android/Windows acceptance remain unresolved.


## Owner decision — 2026-10-24: A2.2 selected, BWU remediation deferred to backlog

**Decision: A2.2.** Preserve the existing Google Quick Share-compatible BLE→TCP bandwidth upgrade and its current fast-transfer behavior while a genuinely compatible cryptographic proof on the new TCP socket is researched. Do **not** switch Google peers to BLE-only fallback, invent a proprietary mandatory challenge, or substitute IP/name/endpoint-ID matching for authentication. The previously approved high-level **Option A (two-phase authenticated candidate admission)** remains the **future target**, not a current implementation.

**Status: P0 #8 OPEN / DEFERRED / NOT FIXED.** This is a prioritization and compatibility decision, **not** evidence that the observed unauthenticated route-consumption risk is resolved, not approval to mark the release fully secure, and not a waiver of a future security/release review. Existing two expected-panic tests reproduce the known failure; they must not be reported as passing security controls. Normal functionality and existing test gates remain unchanged.

**Reopen criteria:** authoritative Google Nearby protocol update, reproducible redacted Pixel/Windows protocol trace showing a pre-commit cryptographic TCP binding, or explicit owner request to revisit the throughput vs security trade-off. Future Mode A work must require TCP-socket-specific proof, authenticated ownership commit and retry/rollback tests, and Android/Windows compatibility acceptance. Continue unrelated backlog tasks; localization remains the last feature before the final 1.0.0 gate. GitHub issue #8 is the canonical task tracker.

## Owner reaffirmation — 2026-10-26

Following the deeper compatibility review and clarification that postmigration
HMAC/sequence validation already exists, the owner chose **option 1: preserve
current fast Google-compatible BWU**, reaffirming A2.2. The proposed provisional
candidate/standard KEEP_ALIVE design remains unimplemented research; no BLE-only
policy or new TCP proof architecture was selected. See the
[final decision](bwu-compatible-candidate-proof-design-2026-10-26.md#final-owner-decision--2026-10-26-preserve-existing-fast-bwu).
Issue #8 remains **P0 OPEN / DEFERRED / NOT FIXED**, with no release-security
waiver or source/runtime change. The independently fixed #26 is not affected.

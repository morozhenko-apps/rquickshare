# P0 BWU TCP admission: security decision before implementation

Date: 2026-10-19
Primary mode: MODE A (architecture / security).
Repository baseline: `dev` at `57d0dc1f5faee4a5804cf493bb0728bf058f462c` (three documentation-only commits after CI-verified source `4f5705f4f888c3c2c6781d8999dc96a6b3d6f491`).
Issue: https://github.com/morozhenko-apps/rquickshare/issues/8

**Status: Option A APPROVED by user on 2026-10-19; required Google-compatible pre-commit proof NOT FOUND in public protocol. MODE A production implementation BLOCKED at the explicit compatibility gate.**
This remains documentation-only architectural work. No production behavior, tests, installed system, keys, release workflow or security policy have been changed. The user approved only conditional Option A, not an incompatible wire extension, downgraded authentication contract, BLE-only fallback, or residual-risk acceptance.

## 1. Reproducible source evidence and threat boundary

- `core_lib/src/manager.rs::route_bandwidth_upgrade_if_pending`: the primary TCP listener peeks a plaintext, length-delimited `CLIENT_INTRODUCTION`, extracts `endpoint_id` / `last_endpoint_id` and forwards them to `BwuRouter::route_with_alias`.
- `core_lib/src/hdl/bwu.rs::BwuRouter::route_with_alias`: a matching current ID or fallback prior ID **removes** its pending `oneshot::Sender<TcpStream>` and transfers the socket. No proof of the TCP socket's membership in the BLE encrypted session is checked at this boundary.
- `core_lib/src/hdl/inbound.rs::InboundRequest::do_bandwidth_upgrade`: the original encrypted BLE session registers the pending route and advertises a LAN upgrade. After it receives the routed TCP stream, it reads and validates the same plaintext introduction by string equality, sends plaintext ACK, conducts encrypted prior-channel drain and ultimately swaps to TCP. None of those steps demonstrably authenticates the TCP socket against the original BLE peer before irreversible route consumption.
- `core_lib/src/manager/session.rs::classify_inbound`: the classification task treats an apparently routed socket as consumed and exits; no recovery lease is returned to the caller.
- Existing real-socket router/classifier tests check parsing, one-shot routing, alias precedence, cancellation and byte isolation **when claimed identities are trusted**. They do not prove protection against a forged valid current/previous ID.

A TCP client capable of reaching the listener and learning or guessing a live ID may race the intended peer and occupy the pending route. This is a **source-evidenced availability/route-consumption concern**, not a demonstrated ability to decrypt, forge authenticated frames, or exfiltrate payloads. Hardware reproduction has not been performed.

External protocol cross-check:
- https://github.com/google/nearby/blob/main/connections/implementation/offline_frames.cc: `ForBwuIntroduction` serializes current/optional last endpoint ID; `ForBwuIntroductionAck` serializes a type-only ACK.
- https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/02354e29683393549b588a3ce2599a717f6da98e/connections/implementation/bwu_manager.cc: the Google open-source BWU manager handles these messages and dynamic endpoint aliases. This public code is **not proof** of all deployed Google Quick Share implementation details or a supported authenticated pre-commit extension.

## 2. Root-cause protocol (R1-R4; source-level only)

Observed expected behavior: only the original established BLE/secure-session peer can claim its BWU TCP upgrade; a forged claimant cannot consume its route, and legitimate transfer/retry remains possible.

Observed source behavior: TCP admission is decided using untrusted introduction fields; route removal precedes any session-bound proof.

Plausible contributors (not all independently confirmed as exploitable):
1. Plaintext endpoint IDs treated as sufficient routing authority.
2. Destructive one-shot pending-route removal before ownership verification.
3. Downstream validation repeats string equality against values originating from the same unauthenticated TCP socket.
4. Dynamic `last_endpoint_id` alias allows a second claimant identifier without independent session proof.
5. Shared primary TCP listener has no per-upgrade authenticated channel binding.
6. No documented transaction/lease rollback between route delivery, introduction validation and final transport swap.
7. Prior-channel encrypted drain authenticates BLE messages but does not by itself prove TCP socket ownership.

Primary causes: **1 + 2**, amplified by 3 and 7. The code inspection confirms the order of operations, not exploit success against an Android/Windows build.

Focused diagnostic plan (before any fix): use deterministic loopback sockets and real `BwuRouter` plus real `InboundRequest` subject under test. Register a live owner, send a syntactically valid forged current/previous ID first, then a genuine contender; observe pending-route count, receiver admission, completion/error and whether legitimate BLE-side state survives. Redact peer addresses/IDs and all crypto material in durable logs. Keep source-exact baseline tests unchanged; do not add arbitrary sleeps, relaxed assertions or silent retries. No such new diagnostic has been executed in this documentation step.

## 3. Non-negotiable security and compatibility invariants

I1. Wire-provided endpoint IDs and aliases are selectors, not proof of ownership.
I2. Unauthenticated ingress cannot irreversibly consume or replace another live owner's route.
I3. No TCP transport substitution without defensible channel binding to established encrypted-session authority.
I4. Replays, simultaneous peers, alias conflicts, late connections and cancellations cannot redirect bytes or stale session rights.
I5. A failed upgrade cannot silently corrupt encrypted sequence counters or publish a successful transfer. Preserve safe rollback/fallback when supported by the actual peer protocol.
I6. Do not announce Google-compatible BWU if physical Pixel/Windows proof is missing.

## 4. Architectural choices requiring explicit user approval

**Option A — protocol-compatible authenticated admission (recommended to investigate first).**
Keep current introduction/ACK framing and aliases only as hints. Introduce a session-scoped provisional admission lease and require an independently verifiable cryptographic proof bound to the existing encrypted session **on the candidate TCP channel before irrevocable commitment**. Validate from real peer traces that a standards-compatible proof arrives at a point where this can be done without deadlock, altered wire bytes or speculative timeout hacks. Otherwise **STOP again**, do not silently install a proprietary challenge or claim compatibility.
- Pros: potential preservation of Google Wi-Fi throughput and meaningful prevention of ID-only route hijack.
- Cons: adds real session state and lifecycle transitions; feasibility depends on actual Google peer sequencing.
- Risks: premature handoff/deadlock, signature/HMAC replay, sequence-counter divergence and protocol incompatibility if proof is unavailable.

**Option B — secure BLE-only for Google-compatible sessions.**
Explicitly reject/skip unauthenticated Wi-Fi LAN BWU admission and continue the already authenticated BLE path. Do not advertise a fast Wi-Fi upgrade that the receiver cannot safely complete. Preserve ordinary BLE sharing where interoperable.
- Pros: small security surface; no untrusted TCP route-consumption; deterministic unit/integration testing.
- Cons: major throughput/latency regression for larger files; some Google peer workflows may not support the fallback as assumed.
- Risks: actual BLE payload performance, peer protocol reliance on BWU, need physical device verification.

**Option C — authenticated rQuickShare-specific BWU extension with Google BLE fallback.**
Require a new per-session nonce + cryptographic TCP admission proof for peers that explicitly support it; never infer extension support from endpoint ID. Use standard BLE/no-BWU operation for Google peers lacking such proof.
- Pros: explicit, testable mutual channel binding for cooperating clients.
- Cons: proprietary extension does not make Google's existing BWU authenticated; adds maintenance/interop complexity.
- Risks: fallback bugs, feature negotiation downgrade, future multi-platform compatibility and versioning overhead.

A mere bounded provisional route, TCP source-address check, randomized port, or requeue-after-failure **alone** is insufficient to satisfy I2/I3 and must not be marked a security fix. Explicit residual-risk acceptance would require a separate release/security decision, not an implicit fallback.

Decision: **Option A selected**, conditional on passing its protocol-proof feasibility gate. The latest public Google code contains **no pre-ACK cryptographic binding in the introduction/ACK frames**, so that gate is **NOT PASSED**. Do not replace missing proof with a provisional lease, timeout, source-IP restriction or UKEY2 checks on a later channel, and do not implement B or C without renewed user approval.

## 4.1. Option A feasibility result (2026-10-19, public Google code)

Reviewed Google Nearby Connections `main` and the actual rQuickShare `dev` code, not just prose descriptions:

- [`google/nearby/connections/implementation/offline_frames.cc`](https://github.com/google/nearby/blob/main/connections/implementation/offline_frames.cc), `ForBwuIntroduction` (~lines 496-516): wire payload includes endpoint ID, optional previous endpoint ID, and encryption-support flag; these fields are sent before an authenticated TCP binding. `ForBwuIntroductionAck` (~lines 518-529) contains only the BWU event type, with no challenge/response or MAC.
- [`google/nearby/connections/implementation/bwu_manager.cc`](https://github.com/google/nearby/blob/main/connections/implementation/bwu_manager.cc), `OnIncomingConnection` (~lines 643-742): reads introduction, sends plaintext ACK, resolves endpoint/alias and invokes `RunUpgradeProtocol`. No socket-specific cryptographic proof is checked before selecting that endpoint.
- Same Google source, `RunUpgradeProtocol` (~lines 753-817): pauses/replaces the new channel and drains the old channel using the SAME UKEY2 context/sequence counters; the previous encrypted channel exchanges `LAST_WRITE_TO_PRIOR_CHANNEL` and `SAFE_TO_CLOSE_PRIOR_CHANNEL`. This proves encrypted control messages on the prior session, **not the origin of the untrusted TCP socket**.
- Our `InboundRequest::do_bandwidth_upgrade` likewise validates introduction strings, ACKs plaintext, drains old BLE and then swaps transport. Waiting for an authenticated TCP payload before ACK/old-channel drain is not supported by the evidenced Google sequence and could deadlock. Waiting until after the switch leaves an unauthenticated candidate able to consume/hold the BWU attempt, violating I2.
- Google [draft PR #4462](https://github.com/google/nearby/pull/4462) (opened July 14, 2026) explicitly describes untrusted `CLIENT_INTRODUCTION` controls and a concurrent-upgrade deadlock. Its proposed fixes retain initiator-owned encryption settings and reject duplicate paused-channel upgrades. **The draft does not provide a pre-commit TCP-channel authentication mechanism**; its separate encryption-downgrade issue must not be confused with a proven rQuickShare decryptability flaw. As of this read it is draft, not evidence of a deployed release.

**Assessment:** The evidence does not establish a way to satisfy BOTH I2 (no irreversible unauthenticated route consumption) and I3 (cryptographic channel binding before commitment) while preserving the observed Google BWU wire sequence. A fully interoperable Option A therefore cannot responsibly be implemented based on existing public protocol data. It is **not** mathematically proven impossible for every deployed Android/Windows build: independent authenticated extensions in those builds have not been checked on hardware.

**STOP/next admissible evidence:** obtain exact sanitized hardware protocol traces or new authoritative protocol data demonstrating a supported pre-commit proof, including its ordering relative to introduction/ACK/old-channel drain. Without that, request a NEW decision between B (secure BLE-only with physical fallback verification) and C (authenticated rQuickShare-only extension with BLE fallback). An option-A-only local route lease might reduce contention but is **not a security fix**, must not be reported as P0 closed, and must not be merged as such.

## 5. Implementation/validation plan after approval

1. Record Option A approval and the **failed public-source pre-commit proof gate** in this document and issue #8; do not start production implementation while it remains blocked.
2. Establish trace-based source-level diagnostic coverage without disclosing peer identifiers or keys.
3. Inventory every affected public route/upgrade method and its executable branches. Plan Positive and all applicable N1-N12 and interaction scenarios before changing code (MODE B inventory/branch map as required by current SSOT).
4. Implement a durable session-ownership contract in Core. Keep producer, daemon, GTK and existing wire compatibility explicit; no symptom suppression.
5. Add real-socket forged current/previous ID, replay, parallel owner, cancellation, stale generation, rejection/retry, BLE/TCP crypto continuity and failure-path tests. Do not mock the domain subject.
6. Perform final stable checks once, separately run eight known race-sensitive tests as their own tracked group, then release-required tagged full Mode B gate. Verify result at exact candidate SHA.
7. Run physical Pixel/KDE and available Google Windows acceptance, validate install/upgrade/rollback; only then close #8 and consider merging/releasing.

Rollback point: documentation-only planning is independently reversible. Any later code and test commits must be atomic; documentation changes are committed separately. Changes to deployed wire security or a supported compatibility contract trigger renewed STOP FACTOR.

## Execution evidence for this planning step

Read-only GitHub connector inspection of `dev` source and existing tests; external Google source inspection. **No local tests, CI run, deployment, hardware test, production code patch or release executed.** Existing exact-source Preflight and Native GTK success on `4f5705f4` remain historical evidence and do not validate a future security fix.

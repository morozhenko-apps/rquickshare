# MODE A research: non-destructive BWU candidates with standard keepalive proof

Issue #8. Owner explicitly requested a compatible solution on 2026-10-26 and
rejected merely repeating the old fast-BWU-versus-BLE-only choice. This is a
**new design candidate, not an implemented security fix or proven Pixel support**.
No production route/admission/crypto behavior changes are authorized by this memo.

## What the deeper source audit changes

Pinned Google Nearby source: `0826a4a4994b465b1be96d19f13c3552c9604f6b`.

- [BwuManager](https://github.com/google/nearby/blob/0826a4a4994b465b1be96d19f13c3552c9604f6b/connections/implementation/bwu_manager.cc): outgoing path sends introduction,
  waits for INTRODUCTION_ACK, then RunUpgradeProtocol installs/pauses the new
  channel and sends authenticated LAST_WRITE on the old channel. If remote
  LAST_WRITE arrives early, ProcessLastWriteToPriorChannelEvent remembers it;
  it does not declare a nonexistent local candidate ready.
- [BaseEndpointChannel](https://github.com/google/nearby/blob/0826a4a4994b465b1be96d19f13c3552c9604f6b/connections/implementation/base_endpoint_channel.cc): Pause blocks writes, not reads. Encryption
  context is shared across migration. This alone does not give an early proof.
- [EndpointManager](https://github.com/google/nearby/blob/0826a4a4994b465b1be96d19f13c3552c9604f6b/connections/implementation/endpoint_manager.cc): one reader continues on the old channel until it closes;
  KEEP_ALIVE requests receive encrypted ACK with the same `seq_num` when the
  channel is unpaused. Separate periodic keepalive writes also exist. An early
  keepalive while paused may be read without an ACK: never assume otherwise.
- [Wire schema](https://github.com/google/nearby/blob/0826a4a4994b465b1be96d19f13c3552c9604f6b/connections/implementation/proto/offline_wire_formats.proto): KeepAliveFrame has `ack=1`, `uint32 seq_num=2`.
  Local rQuickShare currently models only ack; adding standard seq_num echo
  is wire-compatible parsing parity, not a new proprietary message.
- [Google PR #4462](https://github.com/google/nearby/pull/4462) was still open/draft/unmerged at readback. Its encryption-disabling and
  duplicate-channel fixes do not authenticate a candidate TCP connection.

The earlier **pre-old-channel-close** proof gate still cannot be satisfied from
the inspected sequence. But that does not establish that the logical route must
be irrevocably assigned before proof: physical BLE retirement and TCP ownership
commit can be separate states. This distinction enables a new design to test.

## Proposed state machine (requires a material security-contract decision)

1. On the established encrypted BLE session, verify standard nonzero seq_num
   keepalive echo support before offering protected BWU. This is capability
   evidence only, never TCP ownership proof. Missing echo cannot silently fall
   through to insecure admission. No new message type or Google app change.
2. Register a generation-owned bounded **candidate mailbox**, not a one-shot TCP
   route. Current/previous endpoint strings select only this mailbox. Validate
   introduction framing and send the normal plaintext ACK separately to each
   provisional candidate. ACK does not select a winner or advance crypto state.
3. Wait for the genuine peer's encrypted LAST_WRITE on BLE. That proves its
   existing session is ready to migrate, not which TCP candidate is genuine.
   An attacker who merely supplies an ID cannot trigger this peer readiness.
   Complete the normal LAST_WRITE / SAFE_TO_CLOSE / plaintext DISCONNECTION
   exchange once. Keep the logical session/route generation alive; acknowledge
   that Google may retire physical BLE before TCP authentication completes.
4. After the old crypto stream is drained, freeze the counters at that boundary.
   Each candidate gets its own temporary verification context and frame buffer.
   No application payload, consent, secret metadata or file publication goes
   through an unverified candidate. Never let invalid/replayed input increment
   global counters (the existing mutable decrypt method is unsuitable here).
5. A **first authenticated frame alone is insufficient**: an observer could
   replay a genuine frame onto another candidate. Use candidate-distinct fresh
   challenges in the existing encrypted KEEP_ALIVE `seq_num` and verify the
   matching authenticated ACK on the same candidate. One uint32 is not a full
   cryptographic nonce: design four independent nonzero challenges (128 bits
   total), bound their transcript/role/generation, and verify every echo.
   Fork the outbound context at the same drained sequence boundary, use fresh
   CBC IVs for every actual frame, and keep per-candidate inbound state isolated.
   Unrelated valid keepalives/payloads may interleave: buffer within explicit
   resource budgets; do not apply them until proof/commit. Response order and
   outbound queue ordering must be validated against real Google scheduling.
6. Atomically commit exactly one proven candidate and its consumed crypto
   counters, cancel its siblings, consume the generation route, and replay the
   retained authenticated inbound frames into the normal handler exactly once.
   Invalid proof closes only that candidate; it cannot steal another candidate's
   route or corrupt its counters. Session/user/service cancellation owns the
   whole generation and prevents stale proofs from committing into a new one.

This uses standard Google frame types, but **new local security semantics and
extra keepalives are not automatically interoperable**. It needs deterministic
tests and Pixel/Windows traces before production claims. A transparent attacker
who actively relays the entire encrypted exchange is a different network threat;
this must not be sold as device identity verification or elimination of all DoS.
Finite socket/byte/time budgets also mean resource-exhaustion DoS is not solved
merely by removing one-shot route theft.

## Compatibility and failure boundaries to decide

- Existing direct LAN transfers need not change. Google peers implementing
  encrypted seq_num echo should keep fast BWU after a short proof exchange;
  actual latency and echo support on the owner's Pixel are unmeasured.
- For peers that fail the **pre-offer BLE capability** check, keeping that
  established transport avoids a broken migration, but can reduce throughput.
  Whether all target Google builds support this capability is unknown. Never
  silently label insecure legacy BWU as authenticated fallback.
- Once Google retires BLE, a failed proof cannot promise BLE rollback. Preserve
  the logical registration for another candidate within the phase budget; if
  none proves ownership, terminate truthfully, rather than deliver data on an
  unverified stream. This is the explicit revision to the earlier Option A
  promise of retaining a usable old transport until new-stream proof.
- Probe delivery racing Google's Resume can delay/omit an ACK. A complete
  design can use the first valid **post-drain** encrypted inbound frame only as
  a readiness event: Google's writer could produce it only after Resume. Then
  start the candidate-specific challenge, without treating that initial frame
  as ownership proof. A copied readiness frame can trigger a challenge but
  cannot answer a different candidate's nonce. No timer sleep/retry is needed.
  If the sender awaits sharing output, its normal keepalive can supply readiness;
  an optional encrypted keepalive probe after drain can solicit it sooner, but
  an ACK must not be assumed while paused. Measure the natural keepalive delay
  and validate sender/receiver ordering rather than asserting low latency.

## Required evidence before accepting the design

1. Pure, side-effect-free authenticated decode / counter preview; wrong MAC,
   malformed data, wrong sequence, stale epoch and replay never mutate owner
   state. Four challenge echoes bind candidate transcript, not just an ID.
2. Real-socket fake-first / real-second, exact/alias, parallel sessions, all
   false candidates, stale registration and candidate flood budgets. The two
   current expected-vulnerability sentinels must become ordinary passing tests
   only after the production classifier no longer consumes unproven routes.
3. Reproduce the pinned Google sender ordering, including paused writes,
   old-reader retirement, concurrent keepalive/payload output and delayed Resume.
   A hand-written peer model alone is not a Google interoperability result.
4. Demonstrate no deadlock with a peer waiting for receiver sharing-control
   output; verify counters when payload precedes proof ACK, bad candidate sends
   copied valid ciphertext, and user/service cancel at every transition.
5. Exact-source stable/security/stress gates and physical Pixel plus Google
   Windows acceptance: preflight echo, no proprietary frames, cold/retry latency,
   small text/file, large file, cancel/retry and rejected contender recovery.
   No packet dumps with keys, raw payloads or peer identifiers in repository.

## Disposition

**Research progress / architectural STOP before production #8 changes.** The
owner's compatibility request supersedes treating permanent BLE-only as the
default answer. Recommended next decision is to develop and validate this
standard-keepalive candidate design with explicit post-drain proof/fail-closed
semantics, rather than repeat A2.1/A2.2. Alternative: require proof before *any*
old-channel retirement; that stricter contract still needs a supported upstream
extension or other new evidence and cannot be claimed implemented here.

Independent #26 cancellation fix is tracked in
[the implementation report](bwu-cancel-security-followup-2026-10-26.md).
This design has no executable implementation and does not close #8. It narrows
the next investigation with actual mechanisms, replay requirements and failure
criteria instead of claiming impossibility from the introduction fields alone.

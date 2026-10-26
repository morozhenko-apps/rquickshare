# Current connection implementation corrections

Mode A. Owner narrowed scope on 2026-10-26 to correcting the existing implementation.
Baseline: `054acf0a383666991007cf85d456a6b7bd214d3c`. See the
[source audit](quickshare-airdrop-compatibility-audit-2026-10-26.md).

## Scope and evidence

Payload categorization, mixed attachments, Wi-Fi credentials, split APK, STREAM,
Remote Copy extensions, accounts, QR, new off-LAN transports and AirDrop stay in
the future backlog. A generic FileMetadata type does not prevent ordinary file
byte transfer. Existing validation, resource limits and peer failures still apply;
this is not a guarantee that every arbitrary file transfer always succeeds.

Local Ubuntu/Git access verified; dev clean/synchronized, no open PR, no competing
owner observed. Issues #9 and #19 were refreshed: internal event/probe ports and
withdrawal contracts already exist and their physical acceptance remains open.
No installed service or package change is included in source work.

Audit hypotheses checked against code: serialized discovery, first-address-only
selection, ACK-to-ACK handling, missing BWU safe-close condition, unattributed
GATT ingress and bulk-send control starvation. The first three have direct source
evidence; performance/phone timing attribution remains separate. Preserve #8
A2.2 and #16 verified-only identity. No new wire fields in this slice.

## Milestones, decisions and rollback

1. [x] Analyze current event/probe seams, both authenticated handlers and existing
   test/CI policies. Document owner deferrals before code.
2. [ ] Correct keepalive ACK handling in both directions. Require the keepalive
   body, answer absent/false ACK, do not answer true ACK. Preserve protobuf/wire
   fields and existing scheduling. Remove the confirmed outbound allocation used
   only to log the chunk length; preserve actual payload construction.
3. [ ] Make discovery probes owned concurrent futures in the existing event loop.
   Validate TXT/eligible IPv4 before probing, sort addresses, race reachable
   alternatives and cancel losing attempts. Remove/re-resolve cancels the old
   fullname's candidate; only the current candidate may publish. Preserve cache
   tombstones and outer SDK shutdown join. No new detached task or identity merge.
   Bound speculative work to 32 services, 16 IPv4 addresses per service, each
   connect attempt to five seconds. These are explicit discovery resource bounds,
   not sleeps/retries or transfer deadlines: completion publishes immediately,
   expiry frees slots, later genuine resolve events can try again. Never replace
   an active identical candidate merely because the SDK repeats its event.
4. [ ] Correct BWU committed migration: after LAST_WRITE starts, every failure is
   terminal for the established session; require peer LAST_WRITE and SAFE_TO_CLOSE
   within one existing five-second drain budget or fail, never swap on iteration
   exhaustion. Defer ordinary encrypted writes until TCP owns the stream; retain
   parsing/cancellation on BLE, bound the deferred bytes to SANE_FRAME_LENGTH.
   Send plaintext old-channel disconnect without advancing crypto counters and
   await peer disconnect/EOF with a bounded close opportunity instead of 200 ms
   sleep. A confirmed-safe old channel may be dropped when its close budget ends.
   Test silence, exhausted control traffic, partial body at deadline, EOF, bad
   authentication, cancel, reordered peer transitions and successful sequence
   continuity. Preserve the existing legacy admission choice (#8).
   Correct bulk sender duplex ownership within the same TCP session: cancel-safe
   persistent header/body reads race cancel-safe individual socket writes. Retain
   the partial write cursor, parse authenticated peer controls while backpressured,
   queue nested responses behind the current frame in encryption sequence order,
   and abort terminal peer Cancel/EOF without appending a new frame inside a
   partial payload. Bound queued control frames/bytes; keep one writer and avoid
   detached workers. Test partial input across writes, remote cancel before and
   during blocked output, keepalive interleaving and no false Finished. Local
   legacy Cancel also wakes the pump; native owner cancellation retains its
   existing enclosing token. Separate material security/transport choices.
   GATT subscriber/write binding needs a verified BlueZ contract before choosing
   its security design. Do not improvise trust based on device address.
5. [ ] Final gate after planned code/tests: nightly format, Core stable once with
   workspace ext4 TMPDIR (documented baseline tmpfs issue), eight risk-sensitive
   tests individually once (not known flaky), Core strict all-feature Clippy;
   exact-source six-job CI after normal dev push. Fix failures, repeat affected
   checks only as required. Physical tests remain distinct.
6. [ ] Record source/CI/physical boundaries and next steps; separate atomic
   docs/code commits, expected-tip checks and normal dev push.

Rollback uses inverse independent commits for ACK/copy and discovery. Risks:
canceling event futures with queued SDK input, stale results resurrecting removed
routes, false peer visibility, probe saturation and missing native acceptance.
The event source future must remain pinned until delivery; do not cancel/recreate
it whenever a TCP probe finishes. Boundaries are internal, with unchanged public
endpoint payload, explicit consent and trust contracts.

## Planned regression matrix

Keepalive: actual inbound/outbound handlers with absent/false/true ACK; missing
body rejected, expected encrypted response decoded and counters checked, ACK has
no output and does not require encryption keys. Addresses: no IPv4/self-only,
mixed self/remote, missing/invalid TXT, all unreachable, first pending/error with
second reachable. Lifecycle: slow peer does not block fast peer/removal; remove
pending prevents late presence; re-resolve supersedes old metadata/address; exact
duplicate does not reset pending work; cache withdrawal preserved; cancellation
and browse closure drop owned probes; bounded service/address work and expiry;
event future survives unrelated completion. Use oneshot/channels and paused
Tokio time, no physical multicast or sleeps. Test real state machine with injected
external network results. Positive and N1/N2/N3/N4/N5/N9 apply; N6/N7 protect
malformed metadata and route ownership, N10 preserves minimal tombstones. N8,
N11 and N12 do not apply to this in-memory discovery/ACK slice.

## Progress and evidence

ACK handlers/copy removal and owned discovery candidates are implemented locally;
compile-only check found fixture type-inference errors (being corrected), not a
passing test gate. Existing resolve/remove fixture now awaits actual admitted
presence before removing; an independent new fixture covers removal before probe
completion with no phantom presence. No assertion is weakened to hide a failure.
This plan does not claim that additional connection
problems, new transport capabilities or hardware acceptance are complete.

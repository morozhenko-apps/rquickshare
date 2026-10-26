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
2. [x] Correct keepalive ACK handling in both directions. Require the keepalive
   body, answer absent/false ACK, do not answer true ACK. Preserve protobuf/wire
   fields and existing scheduling. Remove the confirmed outbound allocation used
   only to log the chunk length; preserve actual payload construction.
3. [x] Make discovery probes owned concurrent futures in the existing event loop.
   Validate TXT/eligible IPv4 before probing, sort addresses, race reachable
   alternatives and cancel losing attempts. Remove/re-resolve cancels the old
   fullname's candidate; only the current candidate may publish. Preserve cache
   tombstones and outer SDK shutdown join. No new detached task or identity merge.
   Bound speculative work to 32 services, 16 IPv4 addresses per service, each
   connect attempt to five seconds. These are explicit discovery resource bounds,
   not sleeps/retries or transfer deadlines: completion publishes immediately,
   expiry frees slots, later genuine resolve events can try again. Never replace
   an active identical candidate merely because the SDK repeats its event.
4. [x] Correct BWU committed migration: after LAST_WRITE starts, every failure is
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
   GATT contract readback: locked bluer 0.17.3 exposes adapter/device on every
   WriteValue request; its Notify callback exposes no subscriber address. Restore
   the existing single-bridge transport invariant internally: the first valid
   Weave CONNECT claims ingress for that adapter/device, other writers fail
   InProgress, pre-CONNECT data fails, nonzero offsets fail, and an owned notify
   session gets an exclusive receiver. Drop/cancellation replaces the channel and
   clears the transport owner atomically, so old queued bytes cannot survive.
   Preserve CONNECT-before-Notify ordering. This is the equivalent of assigning a
   TCP socket's input to its one connection; it does not authenticate a device,
   grant trust, group routes, authorize consent or add subscriber-specific notify.
   Existing UKEY2/consent/security contracts are unchanged, so this correction is
   an internal routing fix, not the future identity/transport architecture choice.
   Test two writers/adapters, malformed CONNECT, offsets, full queue, overlapping
   receiver admission, drop/retry and queued old input. Unidentified notification
   subscribers and idle prehandshake lifecycle remain explicitly unproven; do not
   claim complete per-peer notification isolation from these APIs.
5. [x] Final gate after planned code/tests: nightly format, Core stable once with
   workspace ext4 TMPDIR (documented baseline tmpfs issue), eight risk-sensitive
   tests individually once (not known flaky), Core strict all-feature Clippy;
   exact-source six-job CI after normal dev push. Fix failures, repeat affected
   checks only as required. Physical tests remain distinct.
6. [x] Record source/CI/physical boundaries and next steps; separate atomic
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

First final Core gate: 855 passed / 1 failed / 8 filtered. The newly explicit
outbound Disconnection handler overwrote an already Finished text transfer when
the peer's close arrived after completion. Fixed the handler's terminal guard,
added an authenticated active/Finished/Cancelled/Rejected regression, and reran
the affected managed test (1/1) then required stable gate: 857/857 +4+4 passed,
8 risk-sensitive tests excluded. Assertions stayed unchanged. GATT ingress was
then clarified against actual bluer source, justifying the additional correction
and a fresh final gate after its implementation. No known-flaky classification.


## Implemented source checkpoint

Executable HEAD `088d543` contains four independent executable commits:

| Commit | Production behavior / regression ownership |
| --- | --- |
| `b0c62d9` | Inbound keepalive ACK gating; committed BWU errors are terminal; both transition signals required; application replies deferred until TCP; event-based old-channel close |
| `f4cb225` | Outbound ACK gating; resumable body reader and duplex write pump; remote/local cancel and peer EOF/disconnect while backpressured; terminal outcomes absorbing; unused chunk copy removed |
| `e320961` | Nonblocking owned mDNS probes, sorted IPv4 alternatives, removal/replacement cancellation, retained SDK event future and bounded speculative work |
| `088d543` | Single-peer GATT ingress, offset/packet bounds, exclusive receiver and drop-safe fresh channel on session retirement |

The existing root-cause source audit is now addressed for QS-AUDIT-01/02/03,
QS-AUDIT-04 **write ingress**, and QS-AUDIT-05 **control responsiveness / redundant
copy**. This does not establish physical acceptance or all performance hypotheses.
QS-AUDIT-06 new transports and payload/account/QR/AirDrop expansion stay deferred.
No public API/proto, persistence, peer trust, explicit consent, manual visibility
window or #8 A2.2 admission policy changed.

GATT still uses BlueZ's global notify callback without subscriber identity. The
transport address selects whose writes form the current stream; it never proves
who the user/device is. No full notification isolation or authenticated alias
association claim. The first valid CONNECT may arrive before Notify; that queue
is preserved until the notify session owns it. A session's Drop replaces the
whole channel, covering cancellation and queued-byte cleanup without async work.
An idle subscriber/pre-CONNECT owner can still occupy the existing bridge; a
separate connection-lifecycle investigation is needed before changing admission
or timeout policy. `gatt_write_queue_rejected` now denotes any ingress rejection
(including offset/foreign writer), not exclusively queue saturation; logs expose
only the static rejection type, no peer address or packet.

## Regression and validation evidence

**31 added test methods**, plus one existing resolve/remove fixture synchronized
on actual presence before removal. No existing assertion was weakened.

| Regression group | New methods | Protected scenarios |
| --- | ---: | --- |
| Inbound keepalive | 2 | absent/false/true ACK, authenticated framing, missing body, silent ACK without send keys |
| Outbound keepalive / terminal | 2 | same ACK/body branches, exact crypto counters and absorbing Finished/Cancelled/Rejected |
| Committed BWU | 6 | silence, missing signal, frame exhaustion, partial body, bad MAC, EOF, cancel, deferred response bound and TCP sequence continuity; both peer signal orders |
| mDNS | 10 | slow/fast peers, reachable alternate after pending/error, all failed, duplicate, remove/re-resolve, invalid replacement, source closure, retained event future, IPv4/self filters and probe bounds/expiry |
| Resumable body | 2 | every split in a small header/body, consecutive frames, invalid size and truncation |
| Duplex bulk | 4 | cancel/disconnect/EOF/bad MAC under backpressure, local legacy cancel, keepalive after complete file frame, queue count and byte boundaries |
| GATT ingress | 5 | foreign peer/adapter, pre-CONNECT/invalid/offset/oversize input, overlapping receiver, old queue drop/retry, bounded queue and canceled task |

The table counts methods, not a coverage percentage. Positive, N1/N2 framing and
resource boundaries, N3 network failure, N4 duplicates/retry, N5 lifecycle races,
N6 malformed wire metadata, N7 stream ownership and N9 cancellation are exercised.
N10 has static redacted new errors/tombstones; this is not a repository-wide log
privacy audit. N8 timezone, N11 billing and N12 storage are outside the changed
logic; existing storage tests remain in the stable suite. Assertions kill ACK
inversion, premature swap, OR-instead-of-AND transition checks, dropped partial
read progress, queue-bound equality errors, stale candidate publication and
terminal-state overwrite. No mutation engine or project-wide coverage claim.

Executed local commands and results on the final code tree:

- `cargo clippy --manifest-path core_lib/Cargo.toml --all-features --all-targets -- -D warnings`
  **exit 0**. Initial checks found a large control enum (reduced to a boolean
  result) and side-effect `map_err` (changed to `inspect_err`); warnings were not
  disabled. Existing bluer future-compatibility notice remains non-gating.
- `TMPDIR="$PWD/core_lib/target/connection-tmp" bash scripts/ci-core-test-groups.sh stable`
  **exit 0: 862 stable methods passed, 0 failed/ignored, 8 filtered; 4/4 + 4/4
  integration passed**. Prior failed/857-test intermediate gates are retained
  above. Workspace ext4 TMPDIR avoids the documented pre-existing tmpfs storage
  contract discrepancy; no storage test was skipped or weakened.
- `cargo +nightly fmt --manifest-path app/main/src-tauri/Cargo.toml --all --check`
  **exit 0**; `git diff --check` and `bash scripts/ci-check-workflow-policy.sh`
  **exit 0**.
- `while IFS= read -r test_name; do TMPDIR="$PWD/core_lib/target/connection-tmp" cargo test --manifest-path core_lib/Cargo.toml --all-features --lib -- "$test_name" --exact || exit; done < <(bash scripts/ci-core-test-groups.sh list)`
  **exit 0: all eight selected tests passed once each, exactly one test per
  invocation**. This group is risk-sensitive, not known flaky. Protected 10x
  release/Mode B gates were not requested or executed. No separate known-flaky
  suite is configured for this slice, and no newly failing test was reclassified.

CI for exact executable SHA `088d54343768e30e321822f6aa9dd64c748bcdac`
is **terminal SUCCESS, all six required checks**:
[Preflight 40334258112](https://github.com/morozhenko-apps/rquickshare/actions/runs/40334258112),
[Native GTK 40334258097](https://github.com/morozhenko-apps/rquickshare/actions/runs/40334258097).
Core job `121611473153` independently reports **862 passed / 0 failed / 0 ignored /
8 filtered**, plus both **4/4 integration** binaries. Format, Core Clippy, Tauri
Clippy, frontend and Native GTK are all completed/success. All 31 added methods
were also cross-checked by name against the final local stable log. No installed
binary, Pixel or throughput success is inferred from automated tests.

## Physical handoff and remaining limits

After six exact-source checks pass, build an uninstalled candidate with
`python3 scripts/native_deb.py build --profile dev --output <dedicated-directory>`;
record its SHA-256 and preserve the prior installed rollback artifact. Do not
install the older unaccepted `6ae62f6` candidate as though it includes these fixes.
Physical acceptance needs cold/retry text, URL and multi-file transfer, successful
BLE-to-TCP and TCP-unavailable paths, sender/receiver cancel then immediate retry,
and mDNS disappear/reappear with an unreachable alternate address. Record exact
installed binaries and redacted stage markers. Issues #9/#18/#19/#23/#25 remain
open for their applicable actual device/SDK gates.

Disk worker/buffer reuse/progress coalescing require measurements on a confirmed
transport and remain under #24. The old-channel close opportunity is bounded at
five seconds if a confirmed-safe peer supplies neither disconnect nor EOF; it
finishes immediately on the event, rather than always sleeping 200 ms. Discovery
has 32 pending services and 16 addresses per service with five-second connect
budgets; saturation never invents presence and a later resolve may retry.
Per-peer global-notify ambiguity and prehandshake idle ownership remain explicit.
No new network backend, account feature, release tag, master promotion or service
restart occurred. Rollback uses the inverse of the four source commits in reverse
order; documentation history preserves the rationale and intermediate failures.


## Final candidate and publication evidence

**Source/automated scope COMPLETE; installed/physical acceptance PARTIAL.**
The package build ran only after all six exact-source CI checks succeeded:

```sh
git diff --quiet 088d543 -- core_lib native_desktop app scripts .github
python3 scripts/native_deb.py build --profile dev --output /home/morozhenko/Downloads/rquickshare-connections-088d543
```

Both commands exited 0; the builder compiled all three binaries with stable
Cargo `--locked`, verified Debian metadata/archive/ELF smoke contracts and wrote:

- Candidate: `/home/morozhenko/Downloads/rquickshare-connections-088d543/rquickshare-native_0.9.0-1.debug_amd64.deb`.
- SHA-256: `5e575d3cd6e242587da477e52b2488cf3058df6d9ee7cdafba926f58c5fb2525`.
- Package identity remains `rquickshare-native 0.9.0-1.debug`; no release bump/tag.
- Build disclosed existing bluer future-compatibility and nine warnings in the
  desktop's no-default-feature Core dependency build, plus dpkg-shlibdeps loader
  diversion notices. These are not a failed build or a clean-warning claim;
  strict required all-feature Clippy passed separately.

A read-only `dpkg-deb -x` plus SHA-256 comparison confirms installed daemon and
GUI differ from the new candidate; unchanged preview helper matches. Installed
package reports the same version string, so version alone cannot prove upgrade.
Installed daemon SHA-256 is
`2b1aaa9803ff6b5d273e15c88bc3199564eeee2b1c8ada527ff48da51401a652`;
candidate daemon is
`ff8f1eba8247b6eb7b47883e44d3bfcd2669fad7dbb00ed5fe126169feaf1d78`.
No package installation, daemon restart, hardware traffic or current-service
modification was performed. Old rollback artifacts remain available.

Docs-first commits: `c6ee99e` (scope deferrals/plan), `a3b8403` (BWU invariants),
`02a33e5` (duplex ownership), `22056e2` (GATT contract and failed-gate evidence).
Four source commits are listed above. Publication was a normal dev-only push
from expected `054acf0` to `088d543`, with a fresh origin-tip check before each
source commit. Final evidence is a separate docs-only commit. No master/tag/PR
mutation, release publication or issue closure.

Final documentation checks: `git diff --check` exit 0; a Python relative-link
existence check passed **41 local Markdown targets** across the three edited
records. This is not an external-link/anchor crawler. All owned commands finished
with native exit status, terminal sessions closed; no server/watcher remains.
Next action is guarded installation of this candidate with prior rollback, then
actual Pixel acceptance using the matrix above. New source supersedes the older
uninstalled candidate; functional hardware issues remain open until their proof.

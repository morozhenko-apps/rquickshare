# Inbound initialization blocked by optional BWU

Mode A, issue [#23](https://github.com/morozhenko-apps/rquickshare/issues/23).
Baseline: `effa21a23a48b1ed1c9a51253b44fe6806d3212c` (`dev`).

## Analysis and environment

Local Ubuntu shell in Codex, writable repository, Git/gh and connected GitHub,
stable/nightly Rust available. One worktree, clean synchronized dev, no open PR
or observed competing writer. Commands run sequentially and are awaited through
the runner. No background service, installed package, master or release change.
Source baseline `213fd3db` check-runs were read back: six completed successes.
Historical baseline evidence is not validation of the new source.

Read AGENTS.md, active-backlog.md, the canonical Pixel/KDE measurements,
StartNotify investigation and the BWU admission decision. Missing documentation:
the receive-loop starvation mechanism and cancellation-safe interleaving plan;
this record supplies them before implementation.

Expected: an optional Wi-Fi upgrade must not stop negotiation/consent on the
established encrypted BLE stream. Actual: `run_inbound_worker` awaits
`do_bandwidth_upgrade` after connection response; that method waits exclusively
on a TCP oneshot for 15 seconds. No BLE frames or consent commands are processed
until it returns. Field offer=28103, timeout=43103, consent=43280 supports this
specific delay. It does not explain every historical cold-discovery failure.

Ranked hypotheses:
1. Exclusive TCP wait starves BLE negotiation: confirmed by call-chain inspection.
2. Peer reports upgrade failure on BLE, but host cannot consume it: supported by
   Google protocol source; whether this Pixel sent that frame is unmeasured.
3. Phone lacks a route to the advertised LAN: plausible; field WLAN state unknown.
4. StartNotify callback blocks: previously fixed, returning in same trace tick.
5. Expiring visibility aborts the session: contradicted by completion after Hidden.
6. Stale GATT ownership blocks first connection: possible for other runs, no busy
   marker in this successful trace; not grounds to change peer isolation.

Google's [BWU manager](https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/2df45c63c272afb24e501e40d4e3c4d9dc8751bb/connections/implementation/bwu_manager.cc)
handles UPGRADE_FAILURE on the previous channel. This supports consuming the
existing authenticated event, not inventing a new protocol or shortening timers.

## Plan, compatibility and rollback

1. [x] Analyze source, actual issues, baseline CI and ownership; document plan.
2. [ ] Add a focused failing diagnostic using the real managed encrypted sender
   and receiver with BWU enabled and no TCP route; consent must arrive before
   the 15-second upgrade deadline. No normal suite before the final gate.
3. [ ] Let the existing inbound event loop be interrupted only at a safe frame
   header/control boundary by route readiness/deadline. Continue authenticated
   frames, explicit consent and terminal handling while waiting. Honor peer
   UPGRADE_FAILURE immediately and clean up the pending route. Preserve partial
   frame headers across route/deadline transitions and prior-channel drain.
4. [ ] Add deterministic regressions for normal negotiation, early authenticated
   failure, silent-peer timeout, TCP arrival, partial headers/body, EOF/malformed
   frames, explicit accept/reject/cancel and route cleanup. Propagate established
   session errors instead of treating them as recoverable upgrade errors.
5. [ ] Run final stable Core gate once, strict format/Clippy, separate eight
   race-sensitive methods once (not known-flaky by repository classification).
   Push dev with expected-tip check; verify six exact-source CI checks.
6. [ ] Record evidence, update backlog/issue with remaining hardware acceptance.
   Do not close #23 until installed Pixel cold/retry acceptance is measured.

Production scope: inbound.rs, GATT worker error propagation, static trace marker;
related tests use real protocol handlers with duplex/loopback external I/O.
No public wire/API, persistence, peer identity, visibility or consent contract
changes. The approved A2.2 compatibility decision and known #8 risk remain intact.
Normal TCP migration is retained. This fixes internal scheduling, not throughput.
Do not introduce retries, timer tuning or new transport trust assumptions.

Risk: canceling a frame-body read would lose consumed bytes; never select over
the whole `handle()` future. Only a retained FrameHeader read may be interrupted.
An already selected body/control action completes before observing route readiness.
Transport errors in that work must end the session; malformed candidate TCP
introduction retains the existing fallback behavior. Rollback uses separate inverse
code/test and documentation commits on dev; no deployed state to restore.

## Regression inventory and acceptance

The selected-mode scope is a defect fix, not a whole-repository Mode B completion.
Planned artifacts: inbound interruptible event boundary, BWU wait/cleanup,
authenticated negotiation handler, existing drain reader, managed GATT worker.
Each branch maps to route ready/closed/deadline, handled event/peer failure/error,
partial header/body continuation, and consent accept/reject/cancel cases.
Interactions: route vs partial BLE frame; deadline vs partial header; keepalive vs
deadline; authenticated failure vs pending route; terminal consent vs route cleanup.

Positive: full encrypted consent/transfer and TCP migration; N1/N2: malformed and
fragmented bounded framing; N3: missing/closed route and EOF; N4: one-shot route
cleanup/re-registration; N5: route/header interleaving; N7: authenticated failure
only, no admission-policy changes; N9: explicit reject/cancel; N10: static marker.
N6/N8/N11/N12 are not introduced by this scheduling change (no persisted format,
wall clock/locale, entitlements or storage-policy changes); existing suites remain.
No tests mock protocol handlers. Paused clocks and explicit future polling are
preferred; integration deadlock deadlines are failure bounds, not success delays.

Physical gate: candidate built after six-green CI, verified rollback and installed
binary provenance; one cold BLE request plus retry, common WLAN recorded. Compare
request/offer/consent/failure-or-route stages and phone UI. A host-side test cannot
prove Android discovery or actual installed latency. Keep raw peer data private.

## Execution evidence

Implementation and final validation pending. No new package installed.

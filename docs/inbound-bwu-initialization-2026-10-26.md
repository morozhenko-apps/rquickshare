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
2. [x] Add a focused failing diagnostic using the real managed encrypted sender
   and receiver with BWU enabled and no TCP route; consent must arrive before
   the 15-second upgrade deadline. No normal suite before the final gate.
3. [x] Let the existing inbound event loop be interrupted only at a safe frame
   header/control boundary by route readiness/deadline. Continue authenticated
   frames, explicit consent and terminal handling while waiting. Honor peer
   UPGRADE_FAILURE immediately and clean up the pending route. Preserve partial
   frame headers across route/deadline transitions and prior-channel drain.
4. [x] Add deterministic regressions for normal negotiation, early authenticated
   failure, silent-peer timeout, TCP arrival, partial headers/body, EOF/malformed
   frames, explicit accept/reject/cancel and route cleanup. Propagate established
   session errors instead of treating them as recoverable upgrade errors.
5. [x] Run final stable Core gate once, strict format/Clippy, separate eight
   race-sensitive methods once (not known-flaky by repository classification).
   Push dev with expected-tip check; verify six exact-source CI checks.
6. [x] Record evidence, update backlog/issue with remaining hardware acceptance.
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

Diagnostic command: `cargo test --manifest-path core_lib/Cargo.toml --all-features
--lib given_pending_wifi_upgrade_when_ble_sender_negotiates_then_consent_and_text_complete_without_tcp
-- --exact hdl::gatt::managed_tests::given_pending_wifi_upgrade_when_ble_sender_negotiates_then_consent_and_text_complete_without_tcp`.
Exit 101: exactly one test executed, failed after 5.00 seconds at the explicit
initialization deadline. Production still exclusively waited for TCP at that
point. This confirms the starvation independently of Pixel UI assumptions.

Implementation uses private `handle_with_interrupt`, preserving the ordinary
handler's randomized command/socket selection and processing bodies outside the
interruptible select. The existing pending TCP timeout is pinned once, not reset
on activity. `BwuSessionError` preserves fatal/terminal established-session errors
through the optional-upgrade fallback. A private address-parameterized helper
allows deterministic loopback upgrade tests without depending on host NIC choice.
The managed worker tests additionally assert a live pending route at consent, so
skipping an unavailable upgrade cannot falsely satisfy the regression.

Added 14 test methods: 11 BWU wait/drain tests and 3 managed worker tests, with
tables for every header split, selected-body boundaries and malformed input kinds.
Existing accept/reject tests also exercise the refactored shared event boundary.
Static compilation found and corrected test-only fixture visibility/byte decoding
and signed frame-length constant typing before final test execution.

Mutation review: restoring exclusive wait kills consent/text/reject regressions;
ignoring UPGRADE_FAILURE kills zero-time fallback; restarting timeout kills exact
15-second assertion after traffic; losing a prefix/body kills authenticated
sequence/frame assertions; swallowing session errors kills terminal peer states;
omitting route cleanup kills has_pending/re-registration; suppressing migration
kills real TCP socket/crypto-counter assertions. No mutation engine run claimed.

Source commit: `6ae62f62b17b058160d3246873ecdd34377ec9f0`, normal-pushed to dev
after expected-tip checks. No force push, master or tag mutation.

Initial local stable command exited 1: **830 passed, 1 failed, 8 filtered**.
All 14 new tests passed. Existing
`receive_reliability::given_inaccessible_or_unsafe_receive_storage_after_introduction_when_preparing_then_no_accept_or_partial_owner_is_left`
failed at line 734. Targeted execution on a source archive of baseline `effa21a`
reproduced the same failure (exit 101, 1 failed, 824 filtered). The first archive
compile lacked three include_str assets; adding their unchanged baseline files
allowed the actual reproduction. No assertion or storage production code changed.

Root diagnostic: `/tmp` is **tmpfs**, workspace is **ext4** (`stat -f` reports
ext2/ext3). On tmpfs, `openat(unlinked_directory_fd, ".", O_TMPFILE|O_RDWR)`
succeeds with parent nlink=0; the identical syscall in workspace storage fails
EPERM. The storage test assumes the latter. This environment-dependent existing
storage defect remains visible; do not call it a flaky test or a BWU regression.
The native CI already uses a workspace TMPDIR. Repeat the final Core gate with
`TMPDIR=./core_lib/target/bwu-ci-tmp`
to validate on the same filesystem class, while retaining the tmpfs failure above.

`cargo +stable clippy --manifest-path core_lib/Cargo.toml --all-features --all-targets
-- -D warnings`: exit 0. Existing bluer 0.17.3 future-compatibility warning remains.
## Final source validation

All six exact-source checks completed successfully for `6ae62f6`:
[Preflight 40287507193](https://github.com/morozhenko-apps/rquickshare/actions/runs/40287507193)
and [Native GTK 40287507248](https://github.com/morozhenko-apps/rquickshare/actions/runs/40287507248).
Core job `121454548714` log independently confirms **831 passed, 0 failed,
0 ignored, 8 filtered**, plus **4/4 modern_protocol and 4/4 native_daemon**.
Rust format, Core Clippy, Tauri Clippy/desktop tests, frontend and GTK are all green.

Executed local final commands (repository root):

```sh
TMPDIR="$PWD/core_lib/target/bwu-ci-tmp" bash scripts/ci-core-test-groups.sh stable
cargo +nightly fmt --manifest-path app/main/src-tauri/Cargo.toml --all --check
bash scripts/ci-check-workflow-policy.sh
cargo +stable clippy --manifest-path core_lib/Cargo.toml --all-features --all-targets -- -D warnings
```

All exit 0. Final stable log: **831 + 4 + 4 passed**, 8 race-sensitive excluded.
Each exact name from `bash scripts/ci-core-test-groups.sh list` was separately run
once with `TMPDIR="$PWD/core_lib/target/bwu-ci-tmp" cargo test --manifest-path
core_lib/Cargo.toml --all-features --lib "$test_name" -- --exact`: eight commands,
each exit 0 and exactly one pass. These are race-sensitive tests, not a declared
known-flaky suite. No release 10x/stress or mutation-engine claim.

Evidence correction: an intermediate workspace run reported only 817 stable
tests after the baseline archive shared Cargo's target directory. That result
and the first race batch are **not accepted as current-source evidence**.
After forcing recompilation by touching current `core_lib/src/lib.rs` (no content
change), the final inventory correctly contained 839 methods, 831 stable passed,
and all eight exact race tests were rerun once. Independent CI never shared that
archive cache and confirms the same current-source counts. Logs are in
`/tmp/rquickshare-bwu-2026-10-26/stable-final.log` and `race-final-1.log` through
`race-final-8.log`; CI links above provide persistent external evidence.

Additional non-required probe: `cargo +stable clippy --manifest-path
core_lib/Cargo.toml --no-default-features --all-targets -- -D warnings` exited 101
on unused experimental imports/fields/helpers and unguarded test references to
`read_plain_frame_from`. This configuration is not the required all-features
Core or Tauri no-default gate. It remains a disclosed limitation, not a clean
pass; those existing feature paths were not broadened in this scoped fix.

The source fix and automated gates are complete; **overall #23 is PARTIAL**
until installed Pixel acceptance. The initial tmpfs storage failure remains an
independent tracked limitation, not silenced by workspace validation. No package
installed or physical latency measured. Rollback: inverse commit `6ae62f6`,
with separate documentation rollback; previous installed package stays intact.
Issue evidence: [comment 6413626764](https://github.com/morozhenko-apps/rquickshare/issues/23#issuecomment-6413626764).

## Uninstalled acceptance artifact

After all six CI checks passed, executed:

```sh
python3 scripts/native_deb.py build --profile dev --output <candidate-output>/rquickshare-bwu-6ae62f6
sha256sum <candidate-output>/rquickshare-bwu-6ae62f6/*.deb
git diff --check
```

All exit 0. Builder compiled all three binaries, verified Debian structure,
runtime dependencies and isolated help/settings smoke checks. Artifact:
`<candidate-output>/rquickshare-bwu-6ae62f6/rquickshare-native_0.9.0-1.debug_amd64.deb`.
SHA-256: `3d8f8e608527b7b08108e25ff34e52b2ef4c761ebefa4def8aa462749ab92bcc`.
Matching `.deb.sha256` is beside it. The source identity is `6ae62f6`; do not use
the reused Debian debug version as binary identity. Build log retains existing
Core feature/dead-code warnings, bluer future-compatibility and dpkg-shlibdeps
libc diversion warnings; final package verification still succeeded.

No apt install, installed binary replacement, service restart, Android action,
signing or publication occurred. Previous installed `0.9.0-1.debug` remains.
Before installing, verify the existing rollback package and compare its three
ELFs to installed paths as specified by the canonical hardware handoff. Then
install the candidate with transfers idle, verify its three installed ELFs and
perform the remaining cold text/retry acceptance. Preserve the old rollback.
All owned build/test commands completed; Terminal closed. No watcher remains.

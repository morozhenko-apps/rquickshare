# MODE A: committed BWU cancellation and compatible security review

Baseline: `8f818d53d59bc8f2a9c709a8051fac708adab1a7` (executable `088d543`).
Local Ubuntu Codex, actual shell/Git/network available; one owned command at a
time. Clean single dev worktree, no open PR, three upstream documentation commits
reviewed and fast-forwarded. No evidence of an active overlapping source writer.
Installed binaries/services/radios remain untouched. Issues #26/#25 and #8.

## Scope and decisions before source edits

Owner reopened backlog defects and #8 subject to Google compatibility. New payloads,
accounts/QR and AirDrop remain deferred. This authorizes investigation, not a
proprietary challenge or a silent switch to BLE-only operation.

#26 expected: Cancel actually writes both encrypted Sharing fragments on the
owned transport or reports delivery failure. Actual: worker drops committed BWU,
then `send_local_cancel` queues fragments and reports them flushed. Plausible
causes supported by source: (1) whole-loop user cancellation drops migration;
(2) deferred enqueue returns the same success as transport write; (3) TCP candidate
lives only in that future; (4) cleanup is bypassed on future drop; (5) final writes
lack a phase-wide deadline; (6) service and user cancellation share an abort path.
Primary causes 1/2 confirmed by #26 source path; add a minimal diagnostic before
fixing. No sleep/retry, forced BLE write after LAST_WRITE or softened assertion.

Correction preserves the existing wire/state contract. Interrupt the optional
preparation on user cancellation. Once committed, the same worker owns migration
until bounded completion, while global shutdown may still abort. On success,
observe the pending user cancellation before another application read and send
Cancel over TCP. While cancellation is pending, decrypt/drain migration framing
but do not process payloads or publish files. A failed/interrupted committed
writer is unusable: fail delivery truthfully, never fall back to BLE. Retain
existing five-second synchronization and five-second close budgets, with their
sum bounding the whole committed operation including writes. These are limits,
not fixed waits. Guard `send_local_cancel` against deferral/failed ownership.

## Security feasibility (independent of #26)

Google Nearby main read on 2026-10-26: `0826a4a4994b465b1be96d19f13c3552c9604f6b`.
`connections/implementation/bwu_manager.cc` OnIncomingConnection still reads
plaintext introduction, sends ACK, selects current/previous endpoint ID, and
RunUpgradeProtocol replaces/pauses the new channel before prior-channel drain.
Only after SAFE_TO_CLOSE does it resume the new channel. No precommit TCP-specific
cryptographic proof was found. Source:
https://github.com/google/nearby/blob/0826a4a4994b465b1be96d19f13c3552c9604f6b/connections/implementation/bwu_manager.cc

Google https://github.com/google/nearby/pull/4462 remains OPEN/DRAFT/unmerged at
readback (updated 2026-07-20). It addresses peer-controlled encryption disabling
and duplicate upgrades, not authenticated TCP admission. rQuickShare never uses
`supports_disabling_encryption` to disable encryption: production does not read
that flag. Add a real migration regression for absent/false/true flags and valid
AES/HMAC sequence continuity; this protects an existing invariant, not a newly
fixed exploitable downgrade. No claim about every proprietary Android/Windows build.

#8 remains a material compatibility STOP. Existing approved Option A cannot meet
its precommit proof requirement using these fields. Waiting for encrypted TCP
before old-channel drain deadlocks the evidenced sequence; waiting afterwards
already permits route theft. Random IDs, source IP, bounded queues and rearming
are not authentication. Do not implement such substitutes or claim #8 fixed.
Continue independent #26 work. Owner choice is needed only to change the agreed
fast-BWU contract/security boundary, not to fix cancellation internally.

## Milestones and validation plan

1. [x] Read live backlog, issue #8, #26 diagnostic plan and Google source/PR;
   refresh refs and preserve independent upstream docs.
2. [x] Establish local minimal #26 diagnostic (expected failure, not a stable pass).
3. [x] Refactor phase ownership, delivery guard and bounded shutdown; regression tests.
4. [x] Final stable suite once, strict Core Clippy, nightly format, workflow policy;
   separately execute eight race-sensitive tests once (not known flaky).
5. [x] Separate source/docs commits, normal dev push after expected-ref check;
   source-exact CI readback; issue/backlog handoff. CI completion and physical
   acceptance remain open as specified in the final checkpoint below.

### Planned branches and regressions

- Deferred/failed writer: error, no sequence increment or BLE bytes (N5/N9).
- Optional preparation Cancel: real BLE Cancel, pending route released (existing
  managed regression plus focused boundary case).
- User Cancel at local LAST_WRITE, peer LAST_WRITE, SAFE_TO_CLOSE and close wait:
  finish handshake, no extra BLE application frame, two decrypted TCP fragments,
  monotonic sequence, one terminal Cancelled (parameterized N5/N9).
- User Cancel plus queued peer payload: no payload processing/Finished (N5/N9).
- Committed EOF/bad MAC/timeout/blocked writer: no BLE fallback or false delivery,
  bounded completion and resource release (N3/N6/N7).
- Service cancellation, including simultaneous user cancel: prompt future drop,
  no manufactured Cancel; actual worker integration (N5/N9).
- Flag absent/false/true: real prepare/commit accepts standard introduction and
  keeps postmigration AES/HMAC/counters; plaintext payload cannot enter (N7).
- Existing no-key/terminal/second-fragment-write-error and deferred size boundary
  tests remain unchanged; no skips/expected-failure conversion for #26.
- #8 current/alias expected-vulnerability sentinels remain visible and unaltered;
  green tests do not prove this known vulnerability fixed.

Applicable negatives: N2 phase/queue bounds; N3 EOF/deadline; N4 existing repeat
cancel/session reuse; N5 races; N6 invalid crypto/framing; N7 encryption invariant;
N9 cancellation; N10 existing fixed trace vocabulary. N1 framing validation uses
existing parser suite; no new input schema. N8 locale/time and N11 billing do not
apply. N12 existing storage regressions stay gating; no storage changes or claim
of completing the broader file-publication race matrix. This MODE A slice narrows
the previous MODE B proposal to the reproducible migration-ownership defect.

Risk: cancellation delivery may wait for the existing bounded committed phase;
peer-visible failure is unavoidable if that phase fails. Public API/wire, consent,
trust and persistent schema stay unchanged. Rollback is inverse dev commits;
source-only rollback cannot undo any future separately authorized installation.

## Executed implementation and local evidence

Source commit `263aa3d` modifies `hdl/inbound.rs`, `hdl/gatt.rs` and two focused
BWU test modules. The actual GATT worker now observes local cancellation between
operations; preparation itself is cancellable, committed migration retains its
writer, and service shutdown remains immediately cancellable. Interface-address
resolution is injected as a function solely to make the real worker testable
with loopback; no domain handler is mocked, no production feature flag exists.
The old internal convenience method was folded into explicit-address preparation;
InboundRequest is not exported by the library's public API.

The writer-unavailable state is set before LAST_WRITE and cleared only after a
successful complete migration. `send_local_cancel` rejects deferred/unavailable
writers and emits the existing fixed `local_cancel_write_failed` event instead
of a false `local_cancel_flushed`. Failed migration remains terminal; no old-BLE
fallback. The local Cancel path is service-cancellable even while notifying.

Nine new test methods exercise fifteen table cases: one minimal deferred-writer
regression; four committed cancellation boundaries; three encryption-disable
flag variants; two service/user stop combinations; failed writer; backpressured
LAST_WRITE; optional TCP introduction cancellation; prior EOF; silent-peer phase
deadline. The four boundary cases also assert a single terminal Cancelled event,
both decrypted Cancel fragments, exact sequence continuity and no application
processing after the pending local cancellation. Missing/false/true disabling
flags retain encryption and reject plaintext after a real migration. This is
regression protection of an already-safe invariant, not a claimed downgrade fix.

Commands actually executed:

- Minimal `cargo test --manifest-path core_lib/Cargo.toml --all-features --lib
  given_deferred_bwu_writer_when_local_cancel_runs_then_delivery_fails_without_ble_bytes`:
  **exit 101 before fix**, 0 pass/1 fail, exact assertion detected false delivery.
- `cargo check --manifest-path core_lib/Cargo.toml --all-features --tests`:
  exit 0. Initial follow-up found unused test import/internal wrapper, removed
  rather than suppressing lint.
- `cargo clippy --manifest-path core_lib/Cargo.toml --all-features --all-targets
  -- -D warnings`: **exit 0**. Existing bluer 0.17.3 future-incompatibility notice
  remains visible; it is not a new Clippy failure.
- First final stable gate: **exit 1, 870 passed/1 failed**. The new silent-peer
  fixture paused the clock after a live-clock deadline existed, observing 5.001s
  instead of exactly 5s. Fixed the fixture to start with paused time; the exact
  equality assertion stayed unchanged. Focused affected test **exit 0, 1 pass**.
- Required final rerun `TMPDIR="$PWD/core_lib/target/connection-tmp" bash
  scripts/ci-core-test-groups.sh stable`: **exit 0, 871 stable unit methods,
  4+4 integration methods passed, 0 failures/ignored, 8 separately filtered**.
  Workspace TMPDIR preserves the existing ext4 storage contract; no skip added.
- All eight names from `bash scripts/ci-core-test-groups.sh list`, each once via
  `cargo test --manifest-path core_lib/Cargo.toml --all-features --lib -- "$race"
  --exact`: **eight exit-0 results, one test each**. These are race-sensitive,
  not a known-flaky group. No separately classified known-flaky group exists;
  no 10x release stress/mutation/physical suite executed.
- `cargo +nightly fmt --manifest-path app/main/src-tauri/Cargo.toml --all --check`,
  `bash scripts/ci-check-workflow-policy.sh`, `git diff --check`: each **exit 0**.

Logs: `/tmp/rqs-bwu-cancel-diagnostic.log`, `...-clippy.log`, `...-stable.log`,
`...-clock.log`, `...-stable-final.log`, `...-races.log` (prefix
`/tmp/rqs-bwu-cancel`). Owned commands completed; no runtime service changed.

Source and prior docs-first commit `90a19cb` were normally pushed to dev after
checking remote expected SHA `8f818d5`. Exact-source CI:
[Preflight 40367247177](https://github.com/morozhenko-apps/rquickshare/actions/runs/40367247177),
[GTK 40367247126](https://github.com/morozhenko-apps/rquickshare/actions/runs/40367247126).
At 17:24 UTC: Rust format, frontend, Core Rust tests and Native GTK completed
SUCCESS (**4/6**); both Clippy jobs remain in progress. Core job `121728173535`
independently confirms **871 +4+4 PASS**, all nine new methods individually `ok`,
and the two existing #8 sentinels still `should panic ... ok` (known defect, not
secured admission). No all-six-green claim yet. Physical acceptance is pending.

## Owner correction and continuing security work

The owner rejected repeating the old A2.1/A2.2 decision and asked to find a
compatible solution. No BLE-only change was selected. Deeper Google reader,
writer and keepalive analysis produced a **new candidate design**:
[standard keepalive proof with non-destructive candidates](bwu-compatible-candidate-proof-design-2026-10-26.md).
It separates old-transport drain from logical route commitment and includes
candidate-specific challenge echoes, rather than mislabeling a replayable first
encrypted frame as sufficient socket proof. This is research progress; #8 is
not fixed and no old acceptance or security-contract exception was inferred.

## Final handoff checkpoint (source 263aa3d, no installation)

Source CI has **5/6 verified completed SUCCESS**: Rust format, frontend, Core
tests, Core Clippy and Native GTK. Tauri job `121728173910` is still IN PROGRESS
(last direct run/jobs read: Install Tauri WebKit dependencies); it has not yet
reported a lint/test result. **PARTIAL, not COMPLETE**. Next external check:
read terminal status of that exact job on Preflight `40367247177`, then record
all six only if it succeeds. No background automation or completed-CI claim.
The source-correction issue must also remain open for physical acceptance.

Commits: docs-first `90a19cb`, executable/tests `263aa3d`, evidence/new security
design `f701ce9`, followed by the documentation-only owner-discussion checkpoint.
No executable drift since 263aa3d; normal dev-only pushes, clean source state,
master/tags untouched. All owned shell commands/process sessions completed;
terminal closed. No service, device, settings, package or signing state changed.

Owner's latest #8 response is discussion, not architectural approval: investigate
keeping slow BLE available if a fast path is not authenticated in time. The
post-drain keepalive design does not yet satisfy this fallback guarantee and is
not being implemented. Read its owner checkpoint before resuming. Issue comments
[26](https://github.com/morozhenko-apps/rquickshare/issues/26#issuecomment-6422807899)
and [8](https://github.com/morozhenko-apps/rquickshare/issues/8#issuecomment-6422799072)
preserve the earlier intermediate checkpoint; newer readback supersedes it.

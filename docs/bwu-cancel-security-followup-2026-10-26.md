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
2. [ ] Establish local minimal #26 diagnostic (expected failure, not a stable pass).
3. [ ] Refactor phase ownership, delivery guard and bounded shutdown; regression tests.
4. [ ] Final stable suite once, strict Core Clippy, nightly format, workflow policy;
   separately execute eight race-sensitive tests once (not known flaky).
5. [ ] Separate source/docs commits, normal dev push after expected-ref check;
   source-exact CI readback; issue/backlog handoff. Physical acceptance stays open.

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

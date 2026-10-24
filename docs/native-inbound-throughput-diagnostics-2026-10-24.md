# Native inbound transfer throughput diagnostics (issue #24)

Date: 2026-10-24. Mode A: diagnostic-only production instrumentation with focused tests.
Source baseline: `436422e8d86571e40cea8b4d3f67e15024ae19ce`.
Scope: Pixel 6a -> Ubuntu KDE rQuickShare native 0.9.0, incoming file payloads after explicit Accept.
Related but separate: #23 intermittent cold BLE/Weave connection latency; #22 premetadata GTK display.

## Evidence and non-evidence

The operator repeatedly used the same approximately 55 MB file, reporting ~8 seconds after Accept in the normal configuration, approximately 6.9 MB/s end-to-end by eye. The earlier A/B run disabling deferred slot-0 Bluetooth advertising still showed slow progress. One verified BLE->TCP bandwidth-upgrade trace has `bwu_tcp_switched` before `inbound_consent_entered`; however, the latest 8-second transfer had no matching transport trace, so its actual transport cannot be asserted. A slow GTK progress bar may or may not match slow byte delivery.

Potential causes (not yet established): (1) unoptimized Rust dev profile; (2) per-file-chunk state snapshot/broadcast/yield; (3) TCP receive pacing or Wi-Fi routing; (4) crypto/frame decode CPU cost; (5) per-chunk filesystem writes or final sync; (6) GTK event projection/update cadence; (7) Pixel-side sender pacing. The best current source-supported candidates are dev-profile CPU overhead and per-chunk state/event work. **Neither is a confirmed root cause.**

## Diagnostic contract, no behavior changes

Use the existing `RQUICKSHARE_RECEIVER_TRACE=1` opt-in trace, unchanged exact format:
`rqs-receiver event=<static_allowlisted_name> elapsed_ms=<nonnegative_integer>`.
No second logger; no peer id, MAC, IP, PIN, filename/path, payload byte contents, crypto material, per-file metadata, throughput values or identifiers. No persistent settings or added telemetry and no per-chunk log.

Stages for an inbound file session:
- `inbound_accept_flushed`: encrypted ACCEPT locally written (not Android acknowledgment), before `ReceivingFiles`.
- `inbound_first_file_chunk_persisted`: after first nonempty file chunk is written and progress state emitted.
- `inbound_file_progress_25`, `inbound_file_progress_50`, `inbound_file_progress_75`: cumulative committed file bytes reach 25%, 50%, 75% of the **entire advertised file total**. Each event is emitted at most once per inbound session, even if one chunk crosses multiple thresholds. Comparisons use overflow-safe integer arithmetic; zero-byte totals produce no threshold markers.
- `inbound_last_file_published`: final member securely published/synced after validation, before terminal UI update.
- `inbound_files_completed`: terminal Finished state propagated after publication, not merely the sender's claimed EOF.
The already present `bwu_tcp_switched` indicates a successful swap before these stages when applicable. Absence is not proof of BLE payload delivery, as native direct TCP ingress exists.

The emitted stage names must remain harmless without opt-in; computation must be trivial and allocation-free per chunk. This is strictly observation: do not change chunk parsing, filesystem write ordering, progress notification semantics, transport switching, cancellation or deadlines. Existing explicit consent, safe publication and session ownership remain invariant. No frame/body logging.

## Test plan and risk matrix

- Positive: first persisted nonzero file chunk event once; 25/50/75 milestones exactly when cumulative bytes cross thresholds; all three once on one large chunk; successful final publish/completed markers after actual operations.
- N1/N2: zero totals; zero-size chunks; one byte, exact 25/50/75 boundaries, large u64 values and overrun safety; no integer overflow.
- N4/N5: repeated cumulative value, overlapping calls, fast multi-threshold single update; no duplicate events from the same session; separate sessions reset state.
- N6/N9: cancellation/invalid payload/partial transfer never emit completed; no premature completion on the first file of a multi-file set.
- N7/N10: privacy-safe, fixed allowlisted trace names with monotonic integer millisecond only; no personal data or file metadata in log.
- N12: underlying file write or publication failure must not produce a false completion event; existing error handling remains unchanged.
- N3/N8/N11 are not directly applicable to this passive trace: transport failures are handled by the existing state machine, and locale/time/monetization are not inputs to milestones. No real wall-clock or network assertions in helper tests.

Implementation: add a lightweight private per-session milestone tracker in `core_lib/src/hdl/inbound.rs`, add fixed variants to `core_lib/src/receiver_trace.rs`, focused deterministic tests, no GUI or persistent schema changes. The test helper should allow injecting an observer callback to validate exact events without running environmental tracing. Known-flaky/race-sensitive tests remain separated as the repository requires; execute STABLE only at the final gate. Do not claim coverage percentage from test counts.

## Verification and rollout

1. Documentation first, separate `docs:` commit, issue stays OPEN.
2. Implement source+focused tests in one atomic `feat:`/ `test:` commit on dev; no auto-promotion to master.
3. Verify **exact-source** Preflight (Rust fmt, stable Core, frontend, two Clippy) and native GTK = six terminal successes. CI does not prove real Pixel performance.
4. Only after green, build and compare a new optional debug package with the saved rollback and all three installed executables; owner performs an opt-in **same-file timed Accept->Completed** test from a fresh daemon.
5. Evaluate stage durations and completion/bytes before architectural optimization. A separately verified optimized `release` vs `dev` comparison on the same source can subsequently isolate build-profile overhead, but is not a prerequisite for gathering first diagnostic evidence.

Rollback: revert only the diagnostic source changes without reverting accepted security, receive policy, GTK, or BWU changes. No release tag, signing, APT publication, master or R2 actions until separately approved.

## Execution status

- [x] Repo rules, issue, source paths, and exact baseline read.
- [x] Plan and invariant inventory recorded before code.
- [ ] Diagnostic code and deterministic focused tests.
- [ ] Source-exact CI final gate.
- [ ] New physical package / Pixel trial with quantifiable intervals.

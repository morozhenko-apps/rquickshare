## Current source-exact stable verification — 2026-10-24

Executable source **`afce867644baab8e4f5e9ffa66e70b9a27b09ff9`**; final source [Preflight 40115424146](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424146) and [GTK 40115424114](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424114).
- Core Rust stable job `120935659444` terminal SUCCESS: **796 passed, 0 failed, 0 ignored, 8 filtered**; both integration binaries **4/4** each. All three diagnostic regression tests individually `ok`, including full encrypted response+LAST_CHUNK read and opt-in marker formatter. Strict Rust format SUCCESS, Core Clippy SUCCESS, frontend SUCCESS, Native GTK SUCCESS.
- **Tauri Clippy job `120935659581` still IN PROGRESS as of this documentation checkpoint**, therefore **5/6 verified, not full 6/6**. Do not ask operator to install until terminal source-exact sixth gate is confirmed.
- No physical Pixel trace run on this source. The last user-reported data remains **3/3 Pixel `Failed`**, despite preceding drain change; **root cause not established**. Next step is the single user-approved trace on a fresh installed debug candidate, not another protocol/timing guess. See [plan](native-pixel-reject-wire-trace-2026-10-24.md).

## 2026-10-24 diagnostic code validation handoff

Current **executable source SHA** `afce867644baab8e4f5e9ffa66e70b9a27b09ff9`. Only docs will follow until evidence arrives. Four distinct code/test commits since the previous fully-validated package:
- `43d9e3ab`: two scoped source files, privacy-safe rejection markers, full two-frame encrypted delivery test and discriminating EOF/timeout drain outcomes; **no packet/timeout or consent behavior changes**.
- `17de08b0`: nightly test-format-only one-line adjustment.
- `8a7c5e8f`: tests return the `RejectionDrainOutcome` from Tokio spawned closure rather than `()`; first CI exposed this compile problem.
- `afce8676`: nightly test-only closure formatting correction to one line for exact formatting profile.

CI on `43d9e3ab` had rustfmt failure; CI on `17de08b0` had *test-only* `expected (), found RejectionDrainOutcome` compile error; CI on `8a7c5e8f` required the final nightly closure formatting. **Do not report these intermediate CI runs as successful.** [Final exact-source Preflight 40115424146](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424146) and [GTK 40115424114](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424114) are running at this checkpoint; verify **all six** check-run terminal SUCCESS before asking the operator to rebuild/install. No new device test has been performed; Pixel still FAILED on 3/3 previous release attempts. No local artifact path, contents hash, installed executable or journal for this new diagnostic source is verified.

Once CI is green, request one diagnostic dry-run package built from current dev, preserving old `0.9.0-1` rollback and passing extracted-vs-installed binary comparisons, then restart user services under `systemctl --user set-environment RQUICKSHARE_RECEIVER_TRACE=1` before the app starts. User supplies ONLY `rqs-receiver event=... elapsed_ms=...` lines filtered from journal, not raw logs; `systemctl --user unset-environment` after testing. A line `reject_last_chunk_flushed` confirms local write/flush of final encrypted frame, **not application-level receipt by Pixel**. Absence of that marker localizes failure earlier; `reject_peer_eof/read_error/deadline` shows how our transport observed completion, not GMS result. Full root cause remains OPEN.

## 2026-10-24 SECOND PHYSICAL FAILURE (3 of 3) — protocol proof investigation

Operator response after installing previous two-second-drain candidate: **all three Pixel→KDE→Decline attempts still showed `Failed` on Pixel**. The previous delay hypothesis **did not resolve observed behavior**. KDE→Pixel→Decline already PASS; manual-only Receiver and compact repeat/history remain accepted. #20 closed with user acceptance; #18 remains OPEN.

Current priority: prove the in-flight encrypted Sharing `Reject` **both chunks** are flushed, and classify how the connection ends, not increase delay or change status arbitrarily. [Detailed evidence, ranked hypotheses, branch map and test matrix](native-pixel-reject-wire-trace-2026-10-24.md). **Opt-in evidence source** `43d9e3ab4abd730ae58f0171aed857c58af70b97` changes `core_lib/src/receiver_trace.rs` and `core_lib/src/hdl/inbound.rs` only: existing `RQUICKSHARE_RECEIVER_TRACE=1` allowlisted monotonic event names (`reject_consent`, `reject_write_start`, `reject_payload_flushed`, `reject_last_chunk_flushed`, `reject_write_failed`, `reject_peer_eof`, `reject_peer_read_error`, `reject_drain_deadline`). Adds deterministic real encrypted **second-chunk** sequence/id/size/flag confirmation, and enum-valued bounded EOF/deadline drain tests. No wire schema, peer trust, packet bytes, delay or storage change. First source CI failed only one nightly formatting line; follow-up **test-only** `17de08b0524d81ca980e22640499e4e07c18f30a`. Exact-source terminal CI NOT YET VERIFIED at this checkpoint; do not direct user to rebuild/install until final gate. No trace has been collected from the Pixel yet, so no root-cause claim.

After six terminal checks PASS, rebuild one new local debug .deb from dev and verify source package hashes and installed binaries against rollback, then run **one isolated** Pixel→KDE→Decline with opt-in env. Extract **only** `rqs-receiver event=[a-z_]+ elapsed_ms=[0-9]+` and omit full journal; unset diagnostic environment on completion. Use stage gaps and EOF/deadline to establish failure, then design actual architectural correction. No release promotion.

# Pixel→KDE Reject still fails: opt-in wire-stage diagnosis (2026-10-24)

## Evidence / Mode B and scope

Operator installed a byte-verified `rquickshare-native 0.9.0-1.debug` candidate incorporating prior scoped `Reject` drain change (code `3a35097f`, 6/6 source CI), then tried **Pixel→KDE→Decline three times**, with Google Pixel displaying **Failed** all three times. This **falsifies the assumption that a 2-second transport drain alone resolves the observable failure**. In the reverse direction KDE→Pixel→Decline correctly shows Declined, and operator previously confirmed manual-only Receive + 60s timer and compact-card History behavior. Issue [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) remains OPEN; no owner test after this new diagnostics change yet. Earlier one-off Pixel Declined result is inconsistent with three recent Failed tests and cannot be treated as acceptance.

**Primary mode B: diagnostic testing, not a speculative protocol or timing change.** Source of truth: [active backlog](active-backlog.md), [prior drain design](native-pixel-decline-drain-2026-10-24.md). The local `InboundRequest::apply_consent(false)` emits Sharing `ConnectionResponseFrame::Reject` via `send_encrypted_frame`, which in turn sends a full `PayloadTransfer` frame and a separate terminal `LAST_CHUNK` frame, then retains the socket for <=2s while draining peer data. Existing self-decoding test validates one encrypted response frame but **does not prove the second chunk or that the remote Android accepted either frame**. Remote Google Quick Share runs outside our code; no GMS device log or sanitized incoming decoder result was provided. The upstream open-source implementation delays closing rejected incoming sessions, but that is insufficient to establish root cause here.

## Ranked hypotheses and deciding evidence

1. **Frame transport / completion problem:** one of the two encrypted output frames or flushes fails, is interrupted, or remote receiver disconnects before consuming both. Instrument command and **both** framed writes. A successful Rust write/flush does not guarantee Android application-level delivery.
2. **Wire-level expectation mismatch:** Android Quick Share may require different `PayloadTransfer` chunk metadata, response fields, frame version or sequencing, even though the in-project decoder accepts it. Prove against independent public Google implementation/fixtures, and only modify wire protocol after incompatible contract is established. No automatic fallback to Cancel/Failure.
3. **Wrong role or stale consent token:** UI Decline could be on a different active inbound session/generation. Log fixed stage names only; compare local state and actual on-wire write sequence for one isolated attempt. Never group by device name.
4. **BLE vs migrated TCP ownership race:** transport migrates between command acceptance and writing response; opt-in existing `bwu_tcp_switched` marker alongside rejection stages to correlate, not infer from file size.
5. **Close/cancellation race:** active worker receives cancellation before response completes or linger ends; check stage absence and peer EOF vs deadline. The existing bounded drain remains in place unless evidence says otherwise.
6. **GMS UI/status mapping:** even a correct frame may appear as Failed for reasons visible only on Android; unavailable evidence, do not assert remote behavior from local successful flush.

## Instrumentation: minimal, privacy safe and shared with existing receiver trace

Reuse **existing** `RQUICKSHARE_RECEIVER_TRACE=1`, unchanged line protocol `rqs-receiver event=STATIC elapsed_ms=N`, output to stderr/journal. Do not invent a second logger or ID that would expose endpoint, filename, data, PIN, Bluetooth addr or network IP. Additional allowlisted static markers, opt-in only:

- `reject_consent` — valid local decline command reached real inbound `apply_consent(false)`
- `reject_write_start`, `reject_payload_flushed`, `reject_last_chunk_flushed`, `reject_write_failed` — phase of encrypted Sharing REJECT writing/flush
- `reject_peer_eof`, `reject_peer_read_error`, `reject_drain_deadline` — terminal 2s peer drain outcomes

**Exact placement:** the second/last-chunk flush marker must be after the final await of the send (not after preparing payload). These new output events may occur on other `send_encrypted_frame` calls only if guarded by a private boolean/phase parameter; do not log payloads or instrument unrelated traffic as `reject_*`. Prefer a small internal observer function called from `reject_transfer` and `apply_consent` to retain single encoder and no duplicate wire logic. Do not add arbitrary sleeps, retries, premature socket shutdown, extra frames or global status changes.

**Tests:** existing real encrypted `Reject`/duplex peer test validates the same payload. Add/adjust deterministic test verifying the *second* encrypted `LAST_CHUNK` is framed, decrypted with correct incremented sequence number, has the same payload ID/total size, zero body and last flag, and is observed while the responder remains live before peer EOF; assert static diagnostics event formatter recognizes every marker with no personal fields. Ensure peer EOF and exact two-second deadline branch outcome coverage and read-error path if practical.

## Completion and hardware gate

- [x] User three failures recorded; earlier 2s fix falsified as sufficient.
- [x] Plan documented before source changes, Source=unchanged at docs checkpoint; separate docs and code commits.
- [x] Add opt-in fixed-event markers and cross-stream dual-chunk regression; code `43d9e3ab`, test-only nightly formatter `17de08b0` and `afce8676`, test-only corrected async fixture return `8a7c5e8f`. Final strict CI still pending.
- [ ] One final exact-source stable Core/GTK/format/Clippy/frontend CI; race-sensitive suite separated for full final Mode B (existing 8 filtered). Do not repeatedly poll pending CI.
- [ ] User rebuilds **one** debug package only after green CI, with preserved verified rollback; install only after user-controlled idle+Quit and byte compare packaged executable. No automatic `master`, release or R2.
- [ ] One isolated Pixel→KDE→Decline under `RQUICKSHARE_RECEIVER_TRACE=1`; extract **only** allowlisted `rqs-receiver event=... elapsed_ms=...` lines from user journal, then unset env. Do not request raw journal, user addresses, pairing codes, raw logs or file content.
- [ ] Compare stage sequence and peer-close reason to falsify/confirm hypotheses. **Do not change protocol/status until evidence**. Record follow-up issue/root fix and physical acceptance when established.

Rollback: revert only tracing/test commit (and optional docs), not previously verified application fixes. No security or persistent schema alteration.

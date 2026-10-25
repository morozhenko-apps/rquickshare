# Native receiver-initiated Cancel: protocol correctness / MODE B

Date: 2026-10-25. Priority: P0. [Issue #25](https://github.com/morozhenko-apps/rquickshare/issues/25). Related: [#23](https://github.com/morozhenko-apps/rquickshare/issues/23), [#18](https://github.com/morozhenko-apps/rquickshare/issues/18). Source baseline: `af951088` installed on Alcotester. Candidate implementation HEAD at document creation: `b00471ef`, **not installed or hardware accepted**.

## Precise user evidence and root cause

The owner **pressed Cancel on KDE** during a Pixel→KDE incoming video. Pixel displayed **Success** although Linux did not report a valid full-file completion (the relevant trace's first inbound reached 25%, lacked `inbound_last_file_published` / `inbound_files_completed`). A subsequent small transfer succeeded in the same daemon on a non-GATT transport. Thus repeated LAN connection works, but **cross-device terminal agreement is incorrect**.

Repository scan, exact native path:
1. `native_desktop/src/panel.rs` click → `handle.cancel(transfer)`.
2. `core_lib/src/service/runtime.rs` `ControlAction::Cancel` for an already bound native inbound → `binding.control.cancel()`.
3. `core_lib/src/sessions.rs` cancels its `CancellationToken`.
4. LAN `core_lib/src/manager/session.rs::execute_inbound()` OR BLE `core_lib/src/hdl/gatt.rs::run_inbound_worker()` exits via `until_either_stops`, previously without emitting a Sharing `Cancel`.
5. Android sees transport termination but no explicit sender-visible Cancel. Android's implementation of resulting Success is **not independently proven**; omitted cancellation frame is verified in source.

Other relevant branches: legacy inbound `ChannelAction::CancelTransfer` sends `Disconnection`; received `sharing_nearby::FrameType::Cancel` sets local `State::Cancelled`; successful receiver file completion requires final chunk flag, exact declared byte count and completed published files. Existing `send_encrypted_frame_with_trace` sends the Sharing bytes payload in exactly two separately authenticated Nearby `PayloadTransfer` frames: data and terminal LAST_CHUNK. Existing `drain_rejected_peer` waits for remote EOF or its established bounded rejection delivery deadline.

## Invariants and risks

- **Never** issue `State::Finished` or send a success-equivalent terminal after local Cancel if declared accepted bytes/files are not fully committed.
- Sender-visible authenticated Sharing Cancel must **precede** retiring a locally canceled established encrypted inbound transport; frame must include an authenticated LAST_CHUNK for the Sharing control payload. Closed/unauthed sessions must not generate fabricated encrypted frames.
- A hard global service shutdown is distinct from user Cancel: do not stall shutdown or send an unsolicited Cancel for it.
- Terminal `Finished`, `Rejected`, `Disconnected`, `Cancelled` are absorbing with respect to late local Cancel. Preserve isolated per-transfer tokens and the BLE single-owner permit.
- No raw file data/filenames/addresses/tokens/authentication material in opt-in traces; no unbounded read/write queues or detached tasks.
- Clean up unlinked incomplete files, preserve already published files, keep consent authorization and integrity/HMAC counters exactly once.
- Existing bounded delivery grace is a **transport shutdown upper bound**, not a discovery/speed/advertising heuristic. A timeout is not an acknowledgment of receipt; physical Android terminal UI must be verified.

## Candidate changes on dev

- `core_lib/src/hdl/inbound.rs`: `send_local_cancel()` sends Sharing `FrameType::Cancel` via the already proven encrypted 2-frame Nearby serialization, then waits for sender EOF or the existing 2s bounded drain; skips wire notification for unencrypted or terminal sessions.
- `core_lib/src/manager/session.rs`: after a **user-session** token cancels (not lifetime token), attempt `send_local_cancel()` before recording Cancelled, and report failure without converting to Finished.
- `core_lib/src/hdl/gatt.rs`: identical behavior for BLE Weave, with worker/task ownership preserved.
- `core_lib/src/receiver_trace.rs`: fixed allowlisted stages for last-chunk flush/failure/EOF/drain deadline.
- Test fixture decrypts real secure messages from an in-memory duplex stream and checks first and terminal control chunks, absence of early worker completion, peer EOF retire and Cancelled state; pre-key/terminal skips.

## Branch map / test inventory

| Production decision / branch | Existing evidence | Required assertion / status |
| --- | --- | --- |
| native KDE Cancel active inbound binding | source path verified | verify cancellation token enters correct worker; automated integration not yet complete |
| LAN cancel after authenticated Accept | candidate source | wire-exact encrypted Sharing Cancel test + actual native LAN sender test |
| BLE cancel after authenticated Accept | candidate source | wire-exact encryption and physical BLE fallback test with GATT ownership |
| native cancel pre-UKEY2 | candidate skip logic | no unkeyed encrypted control, local Cancelled, clean release |
| cancel during consent vs Reject | reject regressions existing | distinct outcome, no false `Finished` |
| cancel during file chunk read | new duplex test, Source | termination does not publish incomplete bytes |
| cancel mid-BWU handshake / after TCP switch | existing BWU tests | no prior-channel message on wrong stream, no token mismatch; audit needed |
| cancel when final chunk and Cancel race | existing terminal checks | Finished iff final chunk was fully committed before cancellation; otherwise Cancelled; additional targeted test needed |
| cancel after terminal Finished | new negative test | no outgoing Cancel, no downgraded terminal |
| sender-initiated Sharing Cancel | existing inbound parser tests | local Cancelled; protocol acknowledgment path unchanged |
| sender peer disconnect before Cancel | open source branch | no false success; closed/broken wire reports warning and Cancelled |
| sender never acknowledges/EOF | existing bounded drain fixture | deterministic advance, no indefinitely retained TaskTracker |
| local service shutdown concurrent with Cancel | candidate selector | shutdown not blocked by grace; add integration negative |
| rapid cancel + next inbound (same daemon) | LAN physical test PASS for reattempt only | sender terminal UI must show Cancelled, retry successful; BLE proof outstanding |
| 60s Receive visibility expires mid-transfer | observed live | cancellation/ongoing session ownership isolated from new discovery |
| two peers + overlapping BLE Notify | existing one-permit tests | no peer packet mixing or permit leakage |

### Positive/N1–N12 regression matrix

P1: valid mid-file KDE Cancel sends two encrypted control fragments; sender decrypts Cancel. P2: valid finished file publishes only after exact final chunk. P3: next LAN transfer reuses same daemon cleanly. P4: after canceled BLE send, fresh Notify can be admitted without restarting the daemon.

N1: pre-encryption Cancel sends no fabricated control. N2: post-Finished Cancel sends nothing, Finished preserved. N3: duplicate local Cancel is idempotent. N4: cancel just before final chunk must never publish incomplete file. N5: cancel after final commit must not erase Finished. N6: peer closes before first Cancel fragment. N7: peer closes between first and LAST_CHUNK. N8: forged sender Cancel metadata or incomplete bytes payload must fail closed. N9: peer stays open, bounded grace ends and task releases owner. N10: service shutdown concurrent with user Cancel does not hang owner join. N11: BLE to TCP BWU in progress cannot write authenticated Cancel on the wrong transport. N12: canceling transfer A does not alter consent, files or encryption of transfer B.

### Interaction matrix

| Actor | Native LAN | Native BLE Weave | BLE→TCP BWU | No authenticated keys |
| --- | --- | --- | --- | --- |
| KDE user Cancel | Control → secure Cancel → bounded finish | Control → secure Cancel → retained bridge until cleanup | route owning migrated stream only | local Cancelled; no encrypted send |
| Pixel sender Cancel | encrypted Sharing Cancel inbound | encrypted Sharing Cancel inbound | live stream only | fail closed |
| Service shutdown | cancellation + bounded worker join | cancellation + no orphan Notify worker | do not migrate if stopping | immediate termination |
| Normal success | exact final file commit | exact final file commit | exact final file commit | impossible before consent |

### Coverage estimation / mutation review / gates

- **Coverage measurements: unavailable**. The production-file map above is a scenario ownership map, NOT a claimed percentage. Do not invent a coverage percentage.
- Target meaningful mutants: suppress secure Sharing Cancel; drop LAST_CHUNK; allow plaintext send; call cancellation notification after setting terminal; send on unrelated BWU socket; mark Finished after a canceled partial; skip bounded drain; turn native shutdown into user Cancel; release BLE permit before callback completes. Relevant tests must fail under each mutant. Mutants not yet executed.
- Mandatory completion: finish N1–N12, branch and interaction cases, source file-by-file coverage readback, mutation review, one final stable test suite and exact-source six CI green. Do not run ad-hoc full tests repeatedly while implementation is in progress.
- Then hash-verified locally built native `.deb`, compare all three installed binaries against package, keep rollback. Physical Pixel: same LAN KDE Cancel from mid-transfer must show **Cancelled** on Pixel (not Success), and immediately retry without restart; BLE no-common-LAN path only if Google chooser actually discovers Linux. Existing 0xFEF3 radio and GATT read proof is not Google chooser proof. No promotion to master/tag/R2 before these gates.

## MODE B continuation plan: testable cancel ownership and failure boundaries (2026-10-25)

**Environment:** ChatGPT remote GitHub connector, not an interactive session on Alcotester. Rust stable and known-flaky tests execute in GitHub Actions; no local user's terminal or installed binary is accessible. Current executable source `e876e7ef` has source-exact Preflight and GTK workflow success; in-person Pixel receiver-side Cancel behavior has NOT been retested. This section is a **MODE B test scope**, not an additional feature request.

### Focused repository scan and exhaustive testable artifact inventory

| Artifact / path | Testable branches and invariant | Priority | Level | Planned number |
| --- | --- | --- | --- | ---: |
| `native_desktop/src/panel.rs` | Inbound active Cancel button invokes control once; queued/terminal disabled | P0 | GTK widget + runtime | Existing UI dispatch tests; new wire tests below |
| `core_lib/src/service/runtime.rs::controls` | queued pending cancel, binding cancel, unknown binding fails; successful ACK and stale consent not overwritten | P0 | runtime integration | Existing runtime tests + targeted guard below |
| `core_lib/src/sessions.rs::SessionControl::cancel` | token cancellation idempotent; no post-cancel accept allowed; lease ends | P0 | unit integration | Existing control tests + shared guard cases |
| `core_lib/src/manager/session.rs::execute_inbound` | LAN end: cancellation token/user vs service shutdown vs peer EOF vs finished vs terminal state | P0 | contract | shared predicate table (at least 8 cases) |
| `core_lib/src/hdl/gatt.rs::run_inbound_worker` | BLE end: same user/service/end decision, single notifier bridge lease; no cross-peer writes | P0 | contract | same predicate table + existing BLE task tests |
| `core_lib/src/hdl/inbound.rs::send_local_cancel` | terminal or plaintext skip, keyed auth, 2-part Sharing Cancel, EOF, deadline, error on write, ended after partial send | P0 | encrypted duplex | at least 6 explicit scenarios plus terminal table |
| `core_lib/src/hdl/inbound.rs::send_encrypted_frame_with_trace` | data+LAST_CHUNK exact offsets/IDs/flag, crypto counters, no malformed content | P0 | protocol contract | existing Reject + new Cancel tests |
| `core_lib/src/hdl/inbound.rs::drain_rejected_peer` | close, read-error, peer never closes, data arrives before EOF | P0 | async paused-clock | existing 2 tests + cancel-specific deadline and peer EOF |
| `core_lib/src/receiver_trace.rs` | privacy-safe allowlisted static stages, no user data fields | P1 | unit | existing exhaustive enum test |

### Branch map, positive and negative tests for new work

| Branch | Positive | Negative category | Assertion / expected |
| --- | --- | --- | --- |
| user Cancel while worker active | P1 | N4 repeated and N9 interruption | should notify sender exactly once |
| no user Cancel | P2 | N5 race | no sender Cancel, worker continues |
| service shutdown | P3 | N9 shutdown | no additional sender Cancel |
| service and user tokens simultaneously canceled | P4 | N5 concurrency | service shutdown priority, no late write |
| `State::Finished/Rejected/Cancelled/Disconnected` | P5 | N4 idempotency | no duplicate Cancel after any terminal |
| no encryption keys yet | P6 | N7 security | no synthetic encrypted Cancel |
| authenticated `ReceivingFiles` | P7 | N7 security | peer decrypts exact Sharing Cancel + LAST_CHUNK |
| disconnect after 2 complete fragments | P8 | N3 remote EOF | peer observes cancel before connection drops |
| remote hangs after 2 fragments | P9 | N3 timeout | deterministic bounded drain and final cleanup |
| socket closed before first chunk | P10 | N3 closed write | returns error, never falsely claims flush/success |
| socket closed after first chunk | P11 | N3 partial-control | incomplete control not counted as successful delivery |
| peer injects unrelated bytes before closing | P12 | N6 corruption | drain does not parse/admit follow-on payload |
| cancel in progress vs file EOF | P13 | N5 race | only Finished after all bytes committed; otherwise Cancelled |

N1 input parser and N2 size boundaries use existing `PayloadHeader` contract tests; N8 locale/time, N11 monetization are not applicable to transport cancellation; N10 static privacy tests apply; N12 partial-file publish/storage uses existing publication tests and remains to integrate with cancel. All independent applicable cases above are **planned**, not complete until merged tests and CI.

### Interface / mutation / coverage approach

Extract a **pure, behavior-preserving guard** `should_notify_sender_on_cancel(result_absent, user_cancelled, service_cancelled, state)` into `manager/session.rs` and call it from both LAN and BLE paths. This is a minimal testability seam, not a new public API or protocol choice. Matrix tests will kill removal/inversion of each condition and mistakenly treating terminal as nonterminal. Existing canceled control handling and async worker ownership remain unchanged.

For `send_local_cancel`, verify the *wire* with a real `tokio::io::duplex` and peer-side decryption, not mocked business logic; use paused Tokio time to complete bounded drain deterministically without sleeping. Distinguish peer read failure from a clean EOF and failure on either write, and assert never sending packets before authentication or after terminal. Every scenario should have exact terminal side effects. Tests remain in the existing Rust modules to preserve repository test conventions. Mutant review: remove complete first chunk, omit terminal LAST_CHUNK, switch Sharing Cancel to Response, disable HMAC/encryption, remove peer-drain, flip cancel-condition, remove service-priority check, issue Cancel after Finished, publish partial file. A meaningful test must fail for each applicable mutant; record unresolved mutants honestly.

**Final validation only after implementation:** final source-exact Preflight 5 stable jobs + Native GTK strict job; separately note eight excluded/quarantined race-sensitive tests rather than falsely calling them green. Require physical Pixel Cancel status and same-daemon retry before issue closure. No package/release/master/tag changes in MODE B test implementation.

## 2026-10-25 MODE B implementation checkpoint (tests committed, final code CI not yet verified)

**New code/test HEAD:** `82767655701e38661096955ab16b3660432004f7`. This checkpoint does not amend the still-installed `af951088` native executable.

**Changes since the already successful `e876e7ef` baseline:**
- `core_lib/src/manager/session.rs`: shared pure `should_notify_sender_on_cancel()` predicate with an explicit user/service/operation/terminal truth table; three deterministic unit tests, including 7 negative cancellation combinations across 3 active states and all 8 Boolean combinations over 4 terminal states. This guards against races between daemon shutdown, duplicate user control, late terminal states and active session cancellation.
- `core_lib/src/hdl/gatt.rs`: uses the **same** predicate as native LAN so it cannot silently diverge on shutdown or terminal handling. No BLE ownership/advertiser changes.
- `core_lib/src/hdl/inbound.rs`: replaced the weak two-state skip test with 7-state/crypto gate and an actual no-wire-read assertion; added real duplex tests for (i) authenticated encrypted two-fragment Cancel followed by peer EOF (existing), (ii) no-peer-EOF bounded delivery grace with paused clock, (iii) remote socket closed before writing any secure Cancel, (iv) missing crypto material fails closed, (v) peer disconnect after first encrypted Cancel fragment before terminal LAST_CHUNK can flush. Real downstream decryption used where possible, no subject-under-test mocking.

**Actual newly added tests in this continuation:** 3 shared guard tests + 5 inbound tests = 8. The previous patch already had 1 encrypted successful handshake test plus 1 guard skip test (the latter was replaced by a stronger table). Current source therefore includes **9 specific Cancel-focused tests** touched or added across both patches, plus earlier native runtime, managed session, file publication and Reject tests that were NOT re-counted as new. This is a static inventory only until CI confirms successful execution.

**Coverage map (no invented numeric coverage):**

| Production file | Scope covered by source tests | Remaining explicit risks |
| --- | --- | --- |
| `core_lib/src/manager/session.rs` | 3 truth-table tests for all predicate branches; existing managed TCP worker integration | full integration when a live Cancel races a partial transport write |
| `core_lib/src/hdl/gatt.rs` | common predicate and existing managed worker/permit tests | physical BLE cancel+retry and concurrent notify/stop with user Cancel |
| `core_lib/src/hdl/inbound.rs` | encrypted Cancel + LAST_CHUNK, benign skips, peer EOF, closed before and during send, deadline, no key | remote-side Android UI mapping, cancel-mid-BWU, file-close storage fault |
| `core_lib/src/service/runtime.rs` | existing Cancel intent tests and token cancellation | sender-visible Cancel receipt integration in one uninterrupted native session |
| `core_lib/src/sessions.rs` | existing idempotency/absorbing terminal status tests | simultaneous terminal completed-file commit vs Cancel full path |
| `native_desktop/src/panel.rs` | existing Cancel action wiring; correct service runtime call | user-visible interaction on real KDE/Pixel |
| `core_lib/src/receiver_trace.rs` | existing exhaustive static event allowlist | hardware `local_cancel_*` marker confirmation |

**Mutation review (static, not executed):** remove sender Cancel first fragment → peer decrypt test fails; remove LAST_CHUNK → peer read stalls/fails; remove encryption → SecureMessage/decrypt fails; flip any Boolean predicate condition → guard matrix fails; send after terminal → sentinel no-wire assertions fail; remove 2s deadline → paused-clock test hangs; report positive delivery with peer closed before/after first frame → actual worker error assertions fail. Open mutants: cancel during BWU transport transition and midway through local socket write of unrelated encrypted frame.

**Gates not complete:** No fresh CI result yet on `827676557`; test suite & real Pixel not claimed. N5 BWU race and N12 disk-write crash are not yet covered by new tests (existing file tests only). Test counts are not a coverage percentage. MODE B completion must remain **PARTIAL**, not marked done until missing critical tests and physical Cancel + BLE repeatability are addressed.

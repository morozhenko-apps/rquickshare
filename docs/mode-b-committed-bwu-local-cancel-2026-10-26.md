# MODE B: receiver Cancel while a BLE→TCP BWU migration is committed

Date: 2026-10-26. Environment: remote read/write GitHub connector, not the owner's local Ubuntu services or Pixel. Branch: `dev`. Baseline executable: `088d54343768e30e321822f6aa9dd64c748bcdac`, [six exact-source CI checks PASS](connection-corrections-2026-10-26.md). Existing verified but **not yet installed** candidate: `rquickshare-native_0.9.0-1.debug_amd64.deb`, SHA-256 `5e575d3cd6e242587da477e52b2488cf3058df6d9ee7cdafba926f58c5fb2525`. Owner's installed candidate remains older until independently verified otherwise.

## 1. Scope and evidence

The owner approved corrections to **existing** Quick Share connections, with new payload types, account/QR, transports, AirDrop and the #8 A2.2 BWU ownership/security decision outside this work. This is a focused testing/contract-gap investigation, not approval to change the wire, introduce an authentication mechanism, disable Cancel, or extend the BWU timeout.

**Existing demonstrated LAN acceptance:** on 2026-10-25, the receiver's KDE Cancel sent a real authenticated Sharing Cancel and Pixel showed “Сброшено” instead of “Success”; `local_cancel_flushed=183808`, `local_cancel_peer_eof=184202` (394 ms peer EOF after full send). See [canonical hardware report](native-pixel-kde-field-measurements-2026-10-25.md) and [#25](https://github.com/morozhenko-apps/rquickshare/issues/25). That physical case was **non-GATT, non-migrating LAN** and does not cover concurrent BWU.

**Source-level counterexample found during the 2026-10-26 audit** (no physical reproduction claimed):
1. `core_lib/src/hdl/inbound.rs::do_bandwidth_upgrade_at` sets `self.deferred_upgrade_writes = Some(Vec::new())` **after receiving an already-accepted TCP socket and before committed LAST_WRITE synchronization**. `finish_bandwidth_upgrade` sends LAST_WRITE and waits for peer LAST_WRITE + SAFE_TO_CLOSE. Its successful path replaces `self.socket` with TCP and drains `deferred_upgrade_writes` to TCP; on a normal error, the outer method clears the queue.
2. `core_lib/src/hdl/gatt.rs::run_inbound_worker` wraps the entire inbound request handle/BWU loop with `until_either_stops(&lifetime, &cancellation, ...)`. If KDE Cancel signals the session's token in the **committed migration window**, the outer select drops the in-progress BWU future **without** the normal error cleanup code in `do_bandwidth_upgrade_at` running to completion.
3. Afterwards that same GATT worker calls `request.send_local_cancel()` for user cancellation. The method emits Sharing `FrameType::Cancel` using `send_encrypted_frame` twice: the Bytes payload plus LAST_CHUNK, and logs `local_cancel_flushed` on the method's `Ok(())`.
4. However `InboundRequest::encrypt_and_send` explicitly implements an application-frame deferral rule: while `deferred_upgrade_writes.is_some()`, a non-BWU frame is appended to a **memory vector** and returns `Ok(())`, without encrypting or writing to the BLE socket or new TCP socket. Both frames of the Cancel can therefore be merely queued; `send_local_cancel` will **emit a misleading `local_cancel_flushed`** even though the sender never received either frame. Its 2s `drain_rejected_peer` operates on the old BLE socket; when the whole request drops the queued Cancel can be lost.
5. The previous exact-source Cancel duplex tests initialize requests without a committed BWU (`deferred_upgrade_writes == None`), and newly added `bwu_wait_tests` cover *peer-initiated* Cancel during committed BWU, but not **KDE-initiated** Cancel at this interleaving. Thus all 6 CI checks can pass while the described local-cancel misreporting is present.

**Confirmed:** existence of an executable logical path which reports flushed bytes when only in-memory queued. **Not confirmed:** Pixel's actual Android UI outcome during this narrow race, frequency, packet-level failure, whether TCP was reachable in the incident, or whether this race caused an earlier user-observed failure. No arbitrary sleep or protocol guess is acceptable.

## 2. Risk and invariants

- `local_cancel_flushed` must mean that all authenticated Cancel frames **actually left the currently owned transport**, not merely that a writer accepted them into the BWU deferral buffer.
- Once migration LAST_WRITE is committed, app frames must **not be written to old BLE**; preserving sender/server crypto sequence counters and explicit consent remains mandatory.
- User Cancel and service/global shutdown have different semantics. On a canceled session, no false `Finished`, forged terminal acknowledgment, premature incomplete-file publication or cross-peer writer ownership.
- A failed *committed* BWU is terminal, per [connection correction contract](connection-corrections-2026-10-26.md); legacy A2.2 route admission and #16 verified identity trust must not be changed as an incidental fix.
- Exactly one owner of the BLE Weave ingress and output, TCP socket and encrypted frame writer at a time; clean queue/permit teardown on cancel, EOF, failed migration, stale sessions.
- Both a successful new cancellation result and a failure to notify must be observable via **redacted fixed-stage traces**, never raw PII or cryptographic data.

## 3. Repository scan / exhaustive artifact inventory

| Artifact / tested behavior | Risk | Level | Planned scenarios |
| --- | --- | --- | ---: |
| `core_lib/src/hdl/inbound.rs::encrypt_and_send` | buffered write `Ok` vs transmitted `Ok` | unit/duplex | 4 |
| `InboundRequest::send_local_cancel` / `send_encrypted_frame` | two-part encrypted Cancel/LAST_CHUNK on live channel | integration | 6 |
| `InboundRequest::do_bandwidth_upgrade_at` and `finish_bandwidth_upgrade` | commit point; old channel prohibited; queue cleared or drained exactly once | integration | 8 |
| `core_lib/src/hdl/gatt.rs::run_inbound_worker` | user vs service cancellation while migration future is pending | integration | 5 |
| `core_lib/src/hdl/gatt/ingress.rs` | exclusive peer/session ownership and cancellation drop | unit/integration | 4 existing contract readback |
| `core_lib/src/manager/session.rs::should_notify_sender_on_cancel` | terminal guard and shutdown priority | unit | existing table, 3 extension cases |
| `core_lib/src/service/runtime.rs::controls`, `sessions.rs::SessionControl` | KDE control propagation vs peer Cancel, idempotent terminal | integration | existing contracts + 4 |
| `core_lib/src/receiver_trace.rs` | flushed/waited/failed logs truthful without PII | unit | 4 |
| `core_lib/src/hdl/inbound/bwu_wait_tests.rs` | current real in-memory framing/authentication/BWU tests | integration | 5 new focused scenarios |
| `native_desktop/src/panel.rs` | local Cancel action and terminal display | GTK integration | existing widget contract + 2 |
| `core_lib/src/received_files.rs` + staging/commit | partial artifacts and final publication | integration | existing storage regressions + 3 |

The numbers are *planning estimates*, not claimed new tests or coverage. Do not add weak token tests. Include complete functions/branch-to-test map and true N1–N12 matrix before test implementation as required by the MODE B SSOT.

## 4. Branch map and deterministic test oracle

| Branch / interaction | Expected outcome | Main source seam |
| --- | --- | --- |
| Cancel during optional TCP socket wait before LAST_WRITE | BLE still owned; encrypted Sharing Cancel should genuinely flush on BLE or fail visibly | `wait_for_bwu_socket` + native worker |
| Cancel after TCP introduction ACK but before LAST_WRITE | owner must be explicit; no ambiguity about queued vs transmitted | BWU entry/deferral |
| Cancel after LAST_WRITE, before peer LAST_WRITE | no old-channel app writes; Cancel must not be silently queued or falsely logged delivered | migration commit phase |
| Cancel after peer LAST_WRITE, before SAFE_TO_CLOSE | maintain crypto/wire boundaries, bounded complete/abort | migration drain |
| Cancel after both peer signals but before TCP socket swap | terminal outcome coherent on one owned transport | swap synchronization |
| Cancel after TCP swap and queued frames drained | only TCP receives Cancel, once, sequence-valid, bounded closure | `encrypt_and_send` |
| Cancel concurrent with file final chunk | `Finished` iff bytes durably committed *before* terminal; else `Cancelled` | file publication |
| Service shutdown during any stage | no user-initiated Cancel synthesized; bounded task ownership/cleanup | `until_either_stops` |
| Remote Sharing Cancel during drain | local Cancelled; no false Finished or extra frame | `process_transfer_setup` |
| Peer disconnect / dead TCP during migration | terminal error rather than fake successful BWU/Cancel | migratable stream |
| Cancel then new peer/service | old deferred queue cannot leak into next Notify/TCP | ingress session drop |

**Reproduction without radios:** Use the existing `tokio::io::duplex`, local `TcpListener`, paused clock and explicit cancellation-token transitions in `bwu_wait_tests`. Inject token cancellation only after a deterministic committed-phase barrier; do not use sleeps or real Google services. An initial *negative* test should assert that local Cancel cannot produce a successful `local_cancel_flushed` unless the peer decrypts **both** payload fragments from the currently owned stream. In the current path, a synchronous `send_local_cancel()` with `deferred_upgrade_writes=Some(Vec::new())` already returns success without emitting a byte; this is the minimal source-level regression fixture. Do not claim the present system passes that intended assertion.

### Positive/N1–N12 interaction matrix (planned)

| Case | Applies? | Planned assertions |
| --- | --- | --- |
| Positive | yes | real TCP-swap and BLE-not-committed Cancel, peer decrypts complete authenticated frame + LAST_CHUNK |
| N1 invalid input | yes | malformed/empty/oversize deferred framing, invalid peer BWU controls |
| N2 boundaries | yes | deferral max/max+1, 0/1 frames, queue/sequence boundaries |
| N3 network errors | yes | EOF, closed TCP, short frame, timeout, partial transport write |
| N4 retry/idempotency | yes | repeated user Cancel; new session after canceled old queue |
| N5 concurrency/races | **critical** | all above BWU commit stages, final-file completion and user/service simultaneous stop |
| N6 corrupt/outdated data | yes | damaged encrypted BWU/Sharing frames and truncated LAST_CHUNK |
| N7 permissions/security | yes | untrusted BWU endpoint, wrong writer, invalid MAC, no secret material |
| N8 locale/time | N/A | locale/date cannot alter this binary framing/token logic |
| N9 user interruption | yes | KDE Cancel, Pixel Cancel, app shutdown vs background receive |
| N10 privacy | yes | `local_cancel_*` redacted logs do not leak identifiers |
| N11 monetization | N/A | no billing/entitlement path |
| N12 storage faults | yes | incomplete file staging, disk fault while Cancel races commit; keep already published files |

### Coverage map / mutation targets / execution gates

All production files above must be mapped to actual applicable functions/branches and outcomes **after** implementing tests. Coverage **not measured**; do not infer a percentage from green CI. Useful mutations: `Some` deferral treated as `None`; report `Ok` after enqueue but before write; drop second Cancel fragment; send encrypted control on old BLE after LAST_WRITE; fail to clear queue on error/drop; permit reuse while old session active; `Finished` overwritten by Cancel; terminal-state frame sent without keys. The correct test oracle must kill these without mocking domain logic.

**Execution:** this investigation has **only source review**. No new code, tests, CI runs, device install or radio changes were executed by this investigator. Baseline `088d543` remains 6/6 green and uninstalled. An architecture-preserving fix is expected to distinguish *buffering* from *delivery* and coordinate cancellation across the existing migration owner. **Do not implement before resolving ownership on committed migration**: the existing contract prohibits old BLE application frames after LAST_WRITE, so “just force-send Cancel on BLE” is incorrect. If repairing the state machine materially changes its public wire protocol or cancellation guarantee, apply the SSOT architectural STOP FACTOR and obtain the owner's choice. Until then keep the defect OPEN with this exact test fixture and constraints.

## 5. Physical acceptance after a verified source fix

Do not deliberately force a user-observable migration race with a large video as a diagnostic. After a code-level fix + exact six-green CI: verify candidate SHA and installed binaries, then test one normal shared-WLAN BLE→TCP file, KDE Cancel while already receiving, immediate same-daemon retry, and optional no-common-LAN pure BLE Cancel only if Google Quick Share actually discovers the Linux device. Log `bwu_tcp_received`, `bwu_prior_drain_started`, `bwu_prior_safe_to_close`, `bwu_tcp_switched`, `local_cancel_flushed`, `local_cancel_peer_eof`, `inbound_files_completed` without payloads or identity. The run only confirms an observed interleaving if the monotonic stages actually show it; do not infer the race from UI alone.

The existing `#25` LAN result remains **PASS once** and **must not be downgraded** by the new uncovered interleaving. The #23 BLE/cold connection and #18 chooser issues remain distinct. No master, tags, release or installed service changed by this record.

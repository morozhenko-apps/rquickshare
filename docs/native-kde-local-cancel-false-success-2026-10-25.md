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

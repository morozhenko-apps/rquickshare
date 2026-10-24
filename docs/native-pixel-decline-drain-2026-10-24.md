# Pixel→KDE Decline regression: protocol response flush and bounded peer drain

**Mode A — scoped transport lifecycle correction**, 2026-10-24. [#18](https://github.com/morozhenko-apps/rquickshare/issues/18), [active backlog](active-backlog.md), [native hardware acceptance](native-current-source-physical-acceptance-2026-10-24.md).

## 1. Physical evidence and what is not proved

On installed diagnostic package `rquickshare-candidate-e67134a` verified byte-for-byte against all three installed executables and protected by 6/6 successful Preflight/GTK exact-source CI, the user tested:
- **PASS:** KDE sends to Pixel; Pixel presses Decline; KDE now displays `Declined`.
- **FAIL:** Pixel sends to KDE; KDE presses Decline; Pixel displays `Failed` (regression/intermittent previous `Declined`).
- **PASS (user reports "остальное работает")**: manual-only Receive hidden at launch; explicit 60s mode/expiry and restart; compact card suppression of prior Completed when new inbound starts, with History/files preserved. These are operator-supplied acceptance observations, not automated device traces; exact locale and transfer content remain independent.
- **No raw peer packet trace/Pixel GMS status code** was captured. Android UI label is not proof of whether a valid encrypted Reject was actually processed.

Current `core_lib/src/hdl/inbound.rs::apply_consent(false)` updates Linux `State::Rejected`, `reject_transfer(Status::Reject)` sends and flushes an encrypted Sharing `Response.connection_response.status=Reject`, then immediately returns `AppError::NotAnError`. Both BLE managed `run_inbound_worker` and native TCP session executor own a worker that drops its socket after this terminal exit. The Google transport may observe premature EOF/disconnection while its REJECT frame is still queued on the path or application event loop, interpreting it as Failed. A TCP socket dropped with unread peer data can also result in RST. No explicit reject-delivery acknowledgement is defined in the current Sharing protocol. Do not claim Google always behaves identically across versions.

**Independent external protocol evidence**: Google's published open-source `NearbySharingServiceImpl::Reject` schedules `CloseConnection` after `kIncomingRejectionDelay`, sends `ConnectionResponseFrame::REJECT`, then marks `kRejected`. Its published tests assert that the response is sent before delayed teardown. Source: https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/refs/heads/main/sharing/nearby_sharing_service_impl.cc and https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/refs/heads/main/sharing/nearby_sharing_service_impl_test.cc. This **supports a transport-lifecycle root cause**, but does not prove the exact Pixel GMS implementation or upstream timer value.

## 2. Root cause, alternatives, selected minimal architecture

1. **Premature reject socket closure:** direct terminal return after flushed `Response::Reject` does not allow sender time to observe frame before transport teardown. Repro intermittent and matches Google's delayed teardown design. This is the most directly source-supported local defect.
2. **Malformed or incorrectly sequenced encrypted response:** less supported by source because `reject_transfer` uses existing generic encrypted data flow and Pixel previously showed `Declined`; direct synthetic decrypt/protobuf regression will cover validity.
3. **BLE bridge shutdown/backpressure:** abrupt owner drop may lose buffered bytes even when `send_frame.flush` returned. Bounded drain protects current transport, instrument if device still fails.
4. **Platform UI mapping:** GMS versions can map remote close/fault to Failed; cannot force UI label without physical verification.

**Selected internal option:** keep the original encrypted `Reject` response and terminal Linux state. After a **successful** response write/flush, *retain the exact same socket* for a **bounded, cancel-safe peer-close drain window** (at most 2 seconds). Finish immediately on peer EOF or read error; otherwise discard received post-rejection bytes in a fixed-size buffer (never decode them as new transfers, never write files or send Accept/Cancel/Disconnection). The deadline bounds hostile/quiet peer resource use. The managed worker's existing `CancellationToken` wrapper must still interrupt the drain for shutdown and revoke resources. Do **not** spawn background tasks or use an unconditional `sleep`; do **not** change the general success, bandwidth upgrade, reject reason, pairing, storage, or UI handling. Explicitly do not assume an acknowledgement exists; the drain affords an opportunity for remote receive/close, not cryptographic delivery proof.

This is an internal transport **lifecycle** fix, not a public protocol schema or trust-boundary change. The 2-second ceiling is a conservative bounded compatibility window, not claimed to be Google's precise private constant. Safety: no more than one 2-second drain for a single explicitly rejected established session; no success path delay, no retention of file content or peer identifiers, and no new advertising. Existing session ownership/cancellation wins over the drain.

## 3. Branch map, tests, and completion gates

| Branch / interaction | Expected behavior | Proof |
| --- | --- | --- |
| Valid live inbound explicit decline | Linux Rejected terminal; exactly one encrypted protobuf Sharing Response Reject sent and flushed; not prematurely dropped before peer read | **new** real request/peer crypto fixture and controlled socket |
| Peer closes after reading Reject | drain returns promptly, worker ends Rejected; no forced 2-second wait | **new** EOF test |
| Peer stays connected, sends post-terminal bytes | receiver ignores bounded data, no transfer state/metadata/files mutated; deadline stops it | **new** deadline/drain test |
| Peer closes abruptly / read errors | drain ends promptly without changing Rejected into Disconnected | **new** read error or EOF test, existing terminal tests |
| Worker cancellation / shutdown during drain | outer cancellation wrapper cancels/join; no orphan task or delayed daemon stop | existing native session ownership + added targeted cancellation test if feasible |
| Accept transfer | original response and file success path unaffected | existing large Core suite |
| Unsupported attachment / capacity failure | separate status logic stays unchanged | existing negative tests |
| Incoming via BLE vs TCP | both use the same inbound handler, no assumptions about pinned Bluetooth/Google Android timing | existing managed BLE/TCP and physical tests |

Positive: explicit user Reject, completed crypto frame. N1 empty socket/EoF; N2 0/1/many bytes; N3 slow/silent peer; N4 repeated reject does not spawn detached tasks; N5 cancel races; N6 old generation shutdown; N7 encrypted authentication unchanged; N8 label remains from existing locale UI; N9 timer precisely bounded; N10 no peer address/PIN/filename logs; N11/N12 not applicable. Interaction matrix: {valid encrypted response vs write failure} × {EOF, blocked, extra data} × {BLE/TCP managed cancellation}; terminal `State::Rejected` invariant and no new sent frames after original Reject.

Per-production-file coverage: `core_lib/src/hdl/inbound.rs` (real `apply_consent` + private bounded drain helper, plus same-file Core Tokio tests). `core_lib/src/hdl/gatt.rs` and `core_lib/src/manager/session.rs` unchanged, covered by existing managed worker suites; native GTK unchanged. Mutation review must detect: skip flush/Reject, immediate close, exceeding deadline, EOF kept open, Accepted treated rejected or extra frame emitted. No invented line coverage percentage; calculate only after actual tooling evidence.

1. [x] Root cause source scan and external design reference; user physical outcomes captured.
2. [x] Docs-first minimal plan and invariants; preserve rollback of one production+test commit.
3. [ ] Modify only inbound reject terminal path to add bounded peer drain and deterministic crypto/EOF/deadline tests.
4. [ ] Source-exact fmt, stable Core, two Clippy, GTK, frontend; separate flaky-sensitive tests as final global release gate. Do not busy-poll CI.
5. [ ] User rebuilds an **uninstalled** candidate from new source only after exact CI is verified, then explicitly reinstalls with original rollback intact and all 3 executable comparisons. Pixel→KDE Decline should report `Declined` repeatedly; both directions, one accepted transfer and manual visibility remain green. Physical result required before closing the remaining part of #18.
6. [ ] Update #18 and active backlog with source SHA/CI and hardware acceptance. #18 still includes cold/warm discovery measurement outside this regression. `master`, production tags and R2 untouched.

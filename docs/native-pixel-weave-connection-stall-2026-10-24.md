# Pixel post-discovery BLE Weave connection stall: 82-second video proof and diagnostic plan

**Date:** 2026-10-24. **Owner hardware:** Pixel 6a → installed Ubuntu 26.04 KDE native 0.9.0-1.debug. [Focused issue #23](https://github.com/morozhenko-apps/rquickshare/issues/23), [#18 wider cold/reliability issue](https://github.com/morozhenko-apps/rquickshare/issues/18), [active backlog](active-backlog.md).

## Observed facts; separate UI observations from protocol inference

The user-provided `rqs-discovery.mp4` is **82.457 seconds**, 1080×2400. A rough per-3-second visual review supports:
1. Pixel Quick Share is searching initially; PC receiver **Alcotester** appears promptly after manual Receive on KDE, around t≈12 seconds video elapsed.
2. User taps PC to send one video; Google Quick Share displays **Connecting…** around t≈15 seconds without PIN/consent prompt, persisting for ~50 seconds.
3. Around t≈66–69 seconds, first attempt becomes **Failed**. Manual Receive on KDE expired around that phase, but *causation is not established*.
4. User starts a new 60-second Receive window and retries; around t≈78–81 seconds Pixel advances to **Waiting…** with a confirmation PIN shown. Do **not** reproduce the displayed PIN, account name/email or other private text in logs/issues. Operator reports second try went through; last video frame itself proves only arrival at Waiting screen, not completed byte transfer.

**Important revision to previous working hypothesis:** rQuickShare discovery advertisement appears quickly after Receive; **first recipient connection establishment is slow/stuck**. The earlier 53/17-second Linux advertisement→GATT-read intervals were not Pixel search latency and do not prove a discovery algorithm defect. Do not optimize mDNS/advertisement scanning heuristically.

## Production source scan and branch/invariant inventory

- `core_lib/src/hdl/gatt.rs` registers the BlueZ GATT write and notify callbacks. One `tokio::mpsc` accepts all incoming Weave writes; one `Arc<Mutex<Option<Receiver>>>` is held by an active session until release; a single `Semaphore` permit gates concurrent notifier subscriptions, with `try_acquire_owned` currently **silently returning** on busy. Weave can be in *notify subscription pending*, *bridge busy*, *control CONNECT request not delivered/parsed*, *confirm not sent*, *framed offline packet not bridged*, or *Core UKEY2 waiting*.
- `weave_session` requires notify subscriber + Weave CONNECT control frame, sends CONN_CONFIRM, then spawns real managed inbound worker and bridges framed packets to Core `InboundRequest::handle`. Only after connection request→UKEY2→encrypted metadata does Core enter `WaitingForUserConsent` and UI display PIN. A missing PIN narrows the failure to before consent without proving which phase.
- Existing `RQUICKSHARE_RECEIVER_TRACE=1` only includes advertiser, slot-0 read, GATT header registration and protocol Reject milestones, not connection-start stages. It does not reveal whether GATT characteristic write/notification callbacks fire when Pixel shows Connecting.
- No evidence yet that duplicate subscriptions/bridge single permit cause failure. A one-permit admission must remain guarded; blindly enlarging queue, removing permit or resetting the GATT server during active sessions risks peer cross-talk and cancelled-consent security.

**Diagnostic option A:** add opt-in fixed names to **existing** receiver trace in `core_lib/src/receiver_trace.rs` and calls at real boundaries in `core_lib/src/hdl/gatt.rs` and `core_lib/src/hdl/inbound.rs`:
`gatt_weave_connect_write`, `gatt_notify_opened`, `gatt_bridge_busy`, `gatt_session_opened`, `gatt_connect_parsed`, `gatt_confirm_sent`, `gatt_first_frame_forwarded`, `inbound_connection_request_decoded`, `inbound_ukey2_client_init`, `inbound_consent_entered`.
All events format as `rqs-receiver event=<STATIC> elapsed_ms=<NUMBER>` with no IDs, addresses, PINs, payloads or file details. For **one isolated first attempt**, missing stage identifies the first boundary not reached. For second attempt in same daemon, compare order to first. Static events can interleave if a third peer is present; therefore ask user for isolated test with no other devices initiating transfers. Do not use raw logcat or BlueZ dumps.

## Mode B matrix and completion

| Case | Evidence / gate |
| --- | --- |
| GATT notify registered, bridge empty | marker emitted, one active permit; no protocol change |
| Second notify while active bridge occupied | explicit `gatt_bridge_busy` marker, no second worker or unverified peer routing |
| GATT Weave control malformed or early | write callback observed even if parse never succeeds; `gatt_connect_parsed` only for accepted version |
| Successful control frame | `gatt_confirm_sent` only after real BlueZ await resolves |
| Offline payload | `gatt_first_frame_forwarded` once per session, only after framed service-hash and length validation |
| Core accepts initial connection request | `inbound_connection_request_decoded` after successfully decoding actual `ConnectionRequest` |
| UKEY2 progress and consent | separate client-init and consent-entry markers; no PIN or crypto logged |
| Restart, timeout, one active→retry | real worker cancellation and singleton bridge ownership unchanged; compare first/second pass |
| UI metadata not received | #22 separate GTK premetadata fix; do not turn missing PIN into fake file count |
| No receiver availability after 60s | next attempt requires new explicit manual Receive; do not auto-extend advertising |

Tests: opt-in marker name uniqueness, privacy-safe string format, GATT control classification, zero/one/two permit occupancy with explicit release; plus existing Core Weave managed integration matrix. Positive/N1–N12: malformed requests, concurrent notifier, cancellation/restart, idempotent retry, no identity merging, no logging secrets, no consent bypass, no buffer overflow, no partial filesystem write. No invented line coverage or claim of real Pixel GATT correctness from mock BLE tests.

Execution: docs first, code-only diagnostics commit, strict source-exact CI six checks. Request one full 60-second recording/trace with **one cold failed attempt and one retry**, only after green CI and an explicitly installed new debug .deb with rollback preserved. A new trace at failure boundary drives architectural root-cause fix/tests; no speculative retry cadence or timer extension. Physical acceptance needs actual Pixel first attempt Connecting→PIN promptly, not just GATT startup log. No `master`, releases, signed APT or R2 changes.

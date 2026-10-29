# Pixel 6a ↔ Ubuntu KDE Native: field measurements and P0 incident record

**Tests performed:** 2026-10-24–25. **Consolidated:** 2026-10-26. **Scope:** Google Quick Share on Pixel 6a sending to rQuickShare Native receiver on Ubuntu KDE (Alcotester). **Branch:** `dev`. **Type:** hardware incident evidence, not a protocol specification, benchmark, or claim of completed release validation.

**Issues:** [#25 — KDE Cancel incorrectly reported as Success on Pixel](https://github.com/morozhenko-apps/rquickshare/issues/25), [#23 — intermittent first BLE Weave connection](https://github.com/morozhenko-apps/rquickshare/issues/23), [#18 — discovery/retry reliability](https://github.com/morozhenko-apps/rquickshare/issues/18), [#24 — throughput](https://github.com/morozhenko-apps/rquickshare/issues/24) (explicitly deferred). The issues are related **but separate**. A passing LAN Cancel test does not close BLE discovery.

## 1. Executive result and confidence

| Observation | Evidence level | Result |
| --- | --- | --- |
| KDE clicks **Cancel** mid-transfer; Pixel reports **Success** (old package) | owner UI report + first incomplete Linux transfer; no old sender-side packet capture | **FAIL before patch** |
| The old native KDE Cancel path canceled the session token without emitting an encrypted Sharing Cancel | source-level call-chain proof | **CONFIRMED root protocol omission** |
| New KDE Cancel sends complete authenticated Sharing Cancel and Pixel displays **«Сброшено»**, not Success | exact installed hash comparison + opt-in stage log + owner Android UI observation | **PASS once, LAN/non-GATT** |
| Immediately send second file **after the new fixed Cancel without restarting either app** | not yet measured; a previous pre-fix same-daemon LAN repeat did pass | **NOT TESTED with fixed binary** |
| Linux BLE advertising reaches the Pixel and GATT slot0 is readable | nRF Connect on actual Pixel, screenshots | **PASS** |
| Genuine Google Quick Share completed an inbound BLE/Weave/UKEY2 file transfer | instrumented host stages, earlier installed StartNotify fix | **PASS once** |
| Google Quick Share reliably lists Alcotester **without a shared Wi-Fi LAN** | repeated owner reports of absence despite generic BLE scanner seeing FEF3 | **FAIL / unresolved** |
| BLE Cancel→retry without restarting both apps | no matched two-attempt BLE cancellation trace | **NOT TESTED** |
| Wi-Fi LAN BWU during isolated BLE transfer | a 15,000 ms timeout, then fallback to BLE | **TCP UPGRADE FAILED in one run** |

**Critical caveat:** UI timestamps, Bluetooth advertisement intervals, and `elapsed_ms` journal stages come from different clocks/tests. Do **not** align them as one synchronized trace, extrapolate sample counts, or interpret an absent marker as a captured protocol frame. No network-wide throughput measurement was performed.

## 2. Field environment and method

- Sender: **Google Pixel 6a**, Google Quick Share. Receiver: native rQuickShare on **Ubuntu KDE**, computer label Alcotester.
- KDE receiver was manually made **Receive/Visible**. Its observed manual visibility window is **60 seconds**. Advertising can stop during an already active transfer without aborting the established session.
- Two *different network conditions* were tried: **common home LAN**, where direct (non-GATT) incoming requests appear; and **Pixel disconnected from home WLAN with Wi-Fi and Bluetooth radios enabled**, intended to isolate BLE discovery. **The network state of the earlier successful BLE file trace was not recorded conclusively**, so lack of BWU TCP may be caused by missing common LAN; do not claim it is a universal throughput regression.
- Instrumentation: optional `RQUICKSHARE_RECEIVER_TRACE=1`, emitting fixed stage names with monotonic `elapsed_ms` since that **daemon process**, redacted by construction. The Android-side results are user-observed screenshots, Google Quick Share UI, and the nRF Connect BLE scanner. No decrypted peer-side Quick Share logcat, Android internal status implementation, raw identity/PIN or synchronized over-the-air capture was recorded.
- **Important log hygiene:** one earlier diagnostic command started the daemon **twice** and rewrote the timestamp file, mixing an old `hidden=321010` marker with the new process `start=0`. Later same-daemon traces also began with an old-process `mdns_withdrawal_emitted=1063748`. Exclude events preceding each `start=0`; do not reconstruct the canceled-video attempt from the successful subsequent image trace.

## 3. Earlier post-discovery first-connection failure (#23)

A supplied **82.457-second Pixel screen recording**, sampled visually rather than with a synchronized HCI clock (rough estimate ±2 seconds), showed:

| Pixel-video relative time | Observation |
| ---: | --- |
| ~12 s | Alcotester **already appears** in Google Quick Share after manual Receive |
| ~15 s | User selects it; Pixel shows **Connecting…** |
| ~66–69 s | First attempt fails without PIN/consent, after roughly **50+ seconds** stuck |
| ~78–81 s | With Receive reactivated, another attempt advances to PIN and **Waiting…**; eventual success reported separately |

**Interpretation:** on this video, **first discovery is fast; connection negotiation is the stalled stage**. The roughly coincident 60s KDE visibility expiry does not prove causality.

### Physical BLE advertising and GATT on Pixel

BlueZ HCI recording showed both advertisement roles configured successfully, using legacy PDUs:

| Role | Advertising parameters | Proven by |
| --- | --- | --- |
| Nearby connectable **0xFEF3** | legacy **ADV_IND**, configured **100–150 ms** interval | BlueZ HCI successful parameters/data + actual Pixel nRF Connect scanner |
| Background **0xFE2C** | legacy **ADV_NONCONN_IND**, configured **1280 ms** interval | BlueZ HCI configuration; second Pixel Nearby scanner entry *consistent with* but not independently UUID-confirmed |

In nRF Connect, the initial **“No filter” still excluded Google advertisements**. After disabling Google exclusion: Pixel showed one strongly received Nearby advertisement associated with the paired host (~**−41 to −50 dBm**, observed ~**153 ms**); its details explicitly identified **16-bit service UUID 0xFEF3** and 17 bytes of service data. Another Nearby entry was around **−38 to −43 dBm** and ~**1285 ms**, **without independent FE2C UUID confirmation**. Observed scanner intervals are not precisely synchronized with controller-configured intervals.

nRF Connect **CONNECTED** to Alcotester, enumerated `Nearby Share Service 0xFEF3`, `Weave RX` (NOTIFY/INDICATE), `Weave TX` (WRITE), and successfully **READ GATT slot0** `00000000-0000-3000-8000-000000000000`; the read value was nonempty. UI showed **Indications enabled**. This verifies BLE RF, generic GATT connect/service enumeration and slot read; it **alone** does not prove Google's discovery filtering, Weave protocol compatibility, PIN, or full sender acceptance.

**Source-level correction already installed for this earlier phase:** pinned `bluer v0.17.3` invokes the GATT `StartNotify` callback and awaits its result. Our old callback awaited the long-lived notify worker; fixed source `af951088a9de6b67a42eda2ebbec0b471e5479d8` launched the worker under managed task ownership then **returned promptly**. See [detailed StartNotify investigation](native-pixel-weave-connection-stall-2026-10-24.md).

### One successful real BLE/Weave file flow (after StartNotify fix, before Cancel fix)

All values below are monotonic **milliseconds from the start of this particular daemon**, **not** from pressing Send:

| `elapsed_ms` | Exact stage |
| ---: | --- |
| 4,852 | `visible` |
| 4,873 | `advertisement_started` |
| 20,016 | `gatt_slot0_read` |
| 23,914 | `gatt_notify_requested`, `gatt_notify_opened`, `gatt_notify_callback_returning`, `gatt_session_opened` |
| 23,974 | `gatt_weave_connect_write`, `gatt_connect_parsed`, `gatt_confirm_sent` |
| 24,154 | `gatt_first_frame_forwarded`, `inbound_connection_request_decoded` |
| 24,217 | `inbound_ukey2_client_init` |
| 28,103 | `bwu_offer_sent` |
| 43,103 | **`bwu_tcp_timeout`** |
| 43,280 | `inbound_consent_entered` |
| 48,063 | `inbound_accept_flushed` |
| 64,852 | `hidden`, `advertisement_stopped` |
| 78,037 | **`inbound_last_file_published`, `inbound_files_completed`** |
| 78,038 | `gatt_notify_worker_exited` |

Derived measured intervals (from the same clock):
- Notify request → callback returning: **same 1ms resolution tick (0 ms reported)**; CONNECT/CONFIRM **60 ms** afterward; first Core offline connection request **240 ms** after Notify.
- BWU offer → TCP timeout: **exactly 15,000 ms**, consistent with the existing configured `Duration::from_secs(15)`; **no `bwu_tcp_switched`**. The file still completed over existing BLE.
- First `visible` → `hidden`: **60,000 ms**. The in-flight file completed **13,185 ms after Hidden**, so visibility expiry did not abort this transfer.
- Owner saw a **~30-minute ETA for a video in a separate earlier attempt**, canceled it, and then saw a stale saved computer tile/failed fresh discovery until app restarts. **This estimate is not measured throughput** and **the failed canceled-video run is not fully captured in this trace**. Do not treat the later image completion as video evidence.

This single successful run demonstrates actual callback return, Weave, UKEY2, consent, and file completion. It does **not** establish repeated cold reliability or BLE Cancel recovery.

## 4. KDE-initiated Cancel falsely marked Success (#25): before fix

The owner clarified explicitly: **Cancel was pressed on the KDE receiver, not on Pixel**. Pixel showed **“Success”**, but the first inbound transfer was incomplete. The old native runtime:

`native_desktop/src/panel.rs` Cancel button → `core_lib/src/service/runtime.rs` `ControlAction::Cancel` → `binding.control.cancel()` → `core_lib/src/sessions.rs` cancellation token → `until_either_stops` in LAN `core_lib/src/manager/session.rs` or BLE `core_lib/src/hdl/gatt.rs`.

**Proven pre-fix defect:** this managed-session path terminated socket handling without first sending authenticated **Sharing `FrameType::Cancel`** to the sender. The Android application's internal rule for converting such disconnect into Success was **not** inspected; the owner-observed sender UI is the evidence.

### Pre-fix LAN/non-GATT same-daemon trace

| `elapsed_ms` | Attempt A: incomplete | Attempt B: independent small file |
| ---: | --- | --- |
| 18,421 | Manual `visible` (common initial window) | — |
| 35,033 | `inbound_connection_request_decoded` | — |
| 35,054 | `inbound_ukey2_client_init` | — |
| 35,389 | `inbound_consent_entered` | — |
| 36,718 | `inbound_accept_flushed` | — |
| 37,441 | `inbound_first_file_chunk_persisted` | — |
| 37,886 | `inbound_file_progress_25`; **no completion event** | — |
| 65,672 | — | `inbound_connection_request_decoded` |
| 65,681 | — | `inbound_ukey2_client_init` |
| 65,980 | — | `inbound_consent_entered` |
| 70,694 | — | `inbound_accept_flushed` |
| 71,364 | — | `inbound_first_file_chunk_persisted` |
| 72,996 | — | **`inbound_last_file_published`, `inbound_files_completed`** |
| 78,422 | `advertisement_stopped`, `hidden` | same 60.001s visibility expiry |

No `gatt_*` or BWU events occurred on these two requests; transport is **non-GATT, consistent with direct LAN/TCP**, rather than a proven BLE Cancel/retry.

**Conclusions limited to the evidence:** Attempt B completed in the same daemon **30,639 ms** after attempt A's request, and **47,251 ms after initial visible**. Thus *a new LAN transfer after a non-completing first transfer* succeeded without daemon restart. The log has **no dedicated original `local_cancel_*` stages**, so cancellation frame absence is proved by source path, not by a packet capture. It does **not** prove that the Pixel correctly understood Cancel; the owner reported the opposite.

## 5. Fix implementation, exact-source CI and package provenance

**Fixed executable SHA:** `213fd3db7c3fac9dd2e51662d6250dc361006967` on `dev`. Functional change:
- `core_lib/src/hdl/inbound.rs::send_local_cancel()`: send authenticated encrypted Sharing `Cancel` using existing two-part Nearby Bytes payload serializer, **including terminal LAST_CHUNK**, then keep the channel for peer EOF or existing bounded **2s** drain before cleanup.
- Native LAN `manager/session.rs` and BLE `gatt.rs` both invoke the same path **only on user-requested session cancellation**, never unconditional service shutdown or a completed transfer. Shared `should_notify_sender_on_cancel` guards state and cancellation origin.
- Opt-in allowlisted `local_cancel_flushed`, `local_cancel_peer_eof`, `local_cancel_drain_deadline`, `local_cancel_write_failed` etc. contain only static names and monotonic milliseconds. Tests decrypt real encrypted duplex frames and assert matching payload ID and LAST_CHUNK, negative no-auth/terminal, broken peer, deadline, and shared state truth tables.
- The fixed wire path is **code-covered for LAN and BLE**, but **only LAN has been accepted on real Pixel hardware**. The 2s limit is an upper bound, **not proof of Android acknowledgment**.

**Final source-exact CI, all six terminal PASS**:

| Workflow | Checks | Result |
| --- | --- | --- |
| [Preflight #40227898996](https://github.com/morozhenko-apps/rquickshare/actions/runs/40227898996) | Rust format, Core Rust tests, Clippy core_lib, Clippy Tauri, frontend lint/typecheck/unit | **5/5 PASS** |
| [Native GTK #40227898978](https://github.com/morozhenko-apps/rquickshare/actions/runs/40227898978) | Native GTK widget tests and strict lint | **1/1 PASS** |

On Alcotester, the owner ran guarded `rquickshare-cancel-acceptance.sh` and recorded all six checks PASS. It validated the previous rollback `.deb`, built and installed `rquickshare-native_0.9.0-1.debug_amd64.deb` from the pinned source, and **byte-compared all three installed executables** (`/usr/bin/rquickshare`, `/usr/libexec/rquickshare-daemon`, `/usr/libexec/rquickshare-preview-helper`) to candidate package members: three `INSTALLED_BINARY_PASS`; `INSTALL_VERIFIED=PASS`.

| Package | SHA-256 | Purpose |
| --- | --- | --- |
| New native Cancel candidate `0.9.0-1.debug` | `38932d873894725f121eb859cc42a02e2d3be4792a4f743d45c534d82ce2cfeb` | **Physically installed / tested** |
| Old StartNotify fixed native package `0.9.0-1.debug` | `a4905ce65c1f8906d07ff6a93fcf35d36f832d947de9e2424eccfab94cbffaad` | Verified **rollback**, not the new source |

Same package Debian version was installed with `apt install --reinstall`; **version string alone cannot distinguish** the two binaries. The source SHA, .deb digest and exact installed-byte comparison are therefore essential provenance.

## 6. Post-fix physical Pixel→KDE Cancel trace: PASS once (LAN)

The user reports that **Pixel now displays “Сброшено” after pressing Cancel on KDE**, rather than **“Success”**.

Monotonic `elapsed_ms` from the **newly started installed fixed daemon**:

| `elapsed_ms` | Exact stage |
| ---: | --- |
| 148,353 | `visible` |
| 148,374 | `advertisement_started` |
| 179,309 | `inbound_connection_request_decoded` |
| 179,333 | `inbound_ukey2_client_init` |
| 179,759 | `inbound_consent_entered` |
| 182,689 | `inbound_accept_flushed` |
| 183,210 | `inbound_first_file_chunk_persisted` |
| 183,488 | `inbound_file_progress_25` |
| **183,808** | **`local_cancel_flushed`** |
| **184,202** | **`local_cancel_peer_eof`** |

**Derived from the same instrument clock:**

| Metric | Computation | Value |
| --- | --- | ---: |
| Consent entry after inbound request | 179,759 − 179,309 | **450 ms** |
| First persisted file bytes after Accept flush | 183,210 − 182,689 | **521 ms** |
| Cancel flushed after 25% checkpoint | 183,808 − 183,488 | **320 ms** |
| **Peer EOF after authenticated Cancel flush** | **184,202 − 183,808** | **394 ms** |
| Cancel flush to completion/Finished | no completion event | **Not applicable: canceled** |

Observed: no `inbound_files_completed`, no `local_cancel_write_failed`, no `local_cancel_drain_deadline` in the filtered trace. The sender closed before the 2s upper-bound drain; **394ms is time to observed peer EOF, not an application ACK round-trip**. No `gatt_*` stages for the transfer, so this is **LAN/non-GATT only**. After capturing the trace the owner **disabled tracing and restarted both user services**.

**Acceptance supported:** receiver Cancel was sent over the authenticated transport, Pixel displayed cancellation and the receiver never reported file completion. **Not established:** retry after this fixed Cancel without restarting either app (only one request in trace), BLE cancel behavior, or frequency/reliability over multiple attempts.

## 7. Outstanding cases, priority and next discriminating measurement

| Gate | Current status | Exact missing evidence |
| --- | --- | --- |
| #25: Pixel KDE Cancel false-success, common-LAN | **PASS once** | repeat under same candidate if seeking reliability sample |
| #25: Cancel → immediate second send in **same daemon**, no application restart | **OPEN** | two inbound requests in one uninterrupted trace, first `local_cancel_flushed`, second `inbound_files_completed`, UI statuses |
| #25: BLE-only KDE Cancel → second BLE send | **OPEN / blocked by chooser intermittency** | Google Quick Share must first actually discover the Linux host off-shared-LAN; then `gatt_notify_*`, `local_cancel_*`, `gatt_notify_worker_exited`, next Notify; no forced protocol writes |
| #23: cold first BLE Weave handshake repeatability | **OPEN** | multiple controlled cold attempts distinguishing Google picker list, GATT/Notify, code and consent; no arbitrary timers |
| #18: no-shared-WLAN Google chooser listing | **FAIL reported, unresolved** | actual Google discovery semantics, not a generic scanner absence |
| #25 MODE B: Cancel during BLE→TCP BWU swap / final file commit / storage fault | **PARTIAL** | deterministic owner/stream/commit race tests plus mutation execution/coverage evidence |
| #24: video ETA/throughput | **DEFERRED** | real byte/time measurement only after connection reliability; one 15s BWU timeout is not measured throughput |

### Safely reproducing one more Cancel→retry trace

1. Keep Pixel and Linux on the **same home WLAN**, disconnect nRF Connect, start *one* instrumented daemon with `RQUICKSHARE_RECEIVER_TRACE=1`. Do not restart or rewrite `rqs-...start.txt` between the two sends.
2. In Receive, send a moderately sized file from Pixel, accept on KDE, cancel **on KDE after actual file progress**. Immediately send a small image from Pixel in the **same daemon** (reactivate manual Receive only if its visibility expired). Record each Pixel status. Do not use BLE-only mode for this LAN retry acceptance.
3. Extract only `journalctl --user -u rquickshare-daemon.service --since "$(cat "$HOME/Downloads/rqs-kde-cancel-acceptance-start.txt")" --no-pager -o cat | grep -Eo 'rqs-receiver event=[a-z0-9_]+ elapsed_ms=[0-9]+'` or equivalent fixed-marker filter. An `inbound_files_completed` for the second request proves its completion; absent stage has to be interpreted with the UI and per-process trace caveats above. Unset the diagnostic environment afterward.

**Do not** upload raw Android logcat, full GATT slot contents, pairing codes, phone addresses, filenames or contents. Keep `dev` code unchanged while recording acceptance; no `master`/tag/release changes based solely on this report.

## Evidence and canonical links

- [#25 hardware confirmation, fixed package provenance and exact cancellation stages](https://github.com/morozhenko-apps/rquickshare/issues/25#issuecomment-6403538342).
- [#23 BLE advertisement parameter/HCI + protocol comparison](https://github.com/morozhenko-apps/rquickshare/issues/23#issuecomment-6397536107); [Pixel nRF Connect GATT read](https://github.com/morozhenko-apps/rquickshare/issues/23#issuecomment-6398059667).
- [#23 real BLE→Weave→UKEY2→completed file, BWU timeout and visibility boundary](https://github.com/morozhenko-apps/rquickshare/issues/23#issuecomment-6398278815).
- [#23 pre-fix two inbound sessions in one daemon, no GATT path](https://github.com/morozhenko-apps/rquickshare/issues/23#issuecomment-6401488649).
- [MODE B source, invariants, branch/test/interaction matrix](native-kde-local-cancel-false-success-2026-10-25.md); [StartNotify source analysis](native-pixel-weave-connection-stall-2026-10-24.md); [Android-side discovery video](native-android-side-discovery-measurement-2026-10-24.md).
- Owner-provided screenshots, redacted console extract `Pasted text(20261025-212531).txt`, and the earlier test chat are the primary observations; **the full raw artifacts have not been committed to the repository** for privacy. This document preserves the stage values and derived calculations needed to reconstruct the conclusion.

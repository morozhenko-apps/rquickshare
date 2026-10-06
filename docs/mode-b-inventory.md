# Mode B inventory, branch map, and test matrix

_Last updated: 2026-10-06_

This document is subordinate to `docs/mode-b-test-plan.md` and records Stages 3-8 for the current maintenance/Pixel receiver scope.

Legend:
- `C` = covered by an existing automated test.
- `P` = planned automated coverage.
- `S` = package/live smoke only because the behavior depends on BlueZ, WebKit, desktop lifecycle, or a real peer.
- `NA(reason)` = not applicable to this artifact.

## Stage 3 - Test inventory

| Artifact | Risk | Level | Current evidence | Planned delta |
|---|---|---|---|---|
| `parseListeningPort` | P2 | Unit | Empty, min/max and common invalid values covered | Add NaN/Infinity and whitespace/format abuse cases |
| `BleAdvertiser::should_advertise` | P1 | Unit | Visible/Temporary/Invisible covered | None |
| `BleAdvertiser::{new,run,get_advertisement}` | P1 | Integration/Smoke | No deterministic BlueZ adapter seam | Keep smoke-only before hardware validation |
| `receiver_service_data` | P0 | Unit | Layout and long ASCII truncation covered | Add multibyte UTF-8 truncation/length-bound coverage |
| `ReceiverAdvertiser::{new,run}` | P0 | Integration/Smoke | No deterministic BlueZ adapter seam | Smoke visibility, retry, periodic re-register, FE2C/FEF3 coexistence |
| `BwuRouter::new` | P1 | Unit | Exercised indirectly | No dedicated test needed beyond router behavior |
| `BwuRouter::register` | P0 | Unit/Integration | Unique registration/cancel covered | Add empty ID, multiple endpoints, concurrent duplicate registration |
| `BwuRouter::route` | P0 | Integration | Successful consume covered | Add unknown endpoint and dropped receiver paths |
| `BwuRouter::cancel` | P1 | Unit | Existing endpoint covered | Add unknown endpoint/idempotent cancel |
| `BwuRouter::has_pending` | P1 | Unit | Empty/non-empty covered | Add multi-endpoint lifecycle |
| `parse_connection_request` | P0 | Unit | Short/malformed/version mismatch and clamp covered | Add exact min/max, wrong command, inverted range, trailing bytes |
| `connection_confirm` | P1 | Unit | Indirect only | Add exact encoding test |
| `complete_framed_message_len` | P0 | Unit | Short/complete/zero covered | Add incomplete declared frame, max, max+1, trailing next-frame bytes |
| `ReceiverGattServer::{new,run}` | P0 | Integration/Smoke | BlueZ-bound | Smoke GATT registration, slot-0 offset reads, queue overflow behavior |
| `weave_session` | P0 | Integration | Main branch logic not directly unit-testable through `CharacteristicNotifier` | Record testability defect; cover pure framing/reassembly seams before broad refactor |
| `validate_received_file_name` | P0 | Unit | Safe, traversal/absolute/control/oversize covered | Add exact 255-byte boundary and multibyte byte-length boundary |
| `prepare_inbound_files` | P0 | Unit/FS integration | Duplicate names, duplicate IDs, negative size, transactional failure covered | Add pre-existing destination collision chain and total-size boundary where practical |
| `parse_wifi_password_payload` | P1 | Unit | Valid 16-byte and malformed variants covered | Confirm invalid UTF-8, exact/truncated trailer variants |
| `read_plain_frame_from` / `send_plain_frame_on` | P0 | Unit/Integration | Round-trip and zero-length read covered | Add max/max+1, truncated stream, zero-length send |
| `validate_client_introduction` / `peek_client_introduction` | P0 | Unit | Valid intro, incomplete prefix and invalid event/payload variants covered | Add explicit wrong endpoint semantics at BWU handoff layer |
| `InboundRequest::{new,enable_bandwidth_upgrade,take_bwu_pending}` | P1 | Unit | Exercised indirectly | Add one-shot pending flag behavior |
| Inbound secure-session/state-machine methods | P0 | Integration | Regression coverage exists around introductions, payload guards, P-256 validation and consent | Keep branch-by-branch map below; fill uncovered deterministic branches only |
| `InboundRequest::do_bandwidth_upgrade` | P0 | Integration/Smoke | Indirect BWU routing evidence, no complete deterministic handoff test | Add pure frame/order assertions where possible; real migration remains hardware smoke |
| `MigratableStream::{poll_read,poll_write,poll_flush,poll_shutdown}` | P0 | Integration | BLE read/write/flush/shutdown covered | Add TCP variant round-trip/shutdown; flush survivor remains equivalent for concrete transports |
| `is_cancel_request` | P1 | Unit | Direction/id/action covered | None |
| `prepare_outbound_files` | P0 | Unit/FS integration | image/apk/unknown, missing, empty, unique IDs covered | Add audio/video, non-UTF8 filename, unreadable/open failure when deterministic |
| Outbound secure-session/state-machine methods | P0 | Integration | Consent accept/reject/malformed plus previous hardening regressions | Fill only deterministic branch gaps discovered below |
| `route_bandwidth_upgrade_if_pending` | P0 | Integration | Successful route without consuming bytes covered | Add no-pending, malformed/ordinary frame with pending route, unknown endpoint |
| `error_for_ui` | P2 | Unit | No dedicated test | Add 512-character boundary/truncation |
| `TcpServer::{new,run,connect}` | P1 | Integration/Smoke | Main listener path exercised indirectly | Keep accept-loop/cancellation live; test pure route classifier separately |
| `checked_payload_buffer_size` | P0 | Unit | 0, max, -1, max+1 covered | Add 1 and max-1 table entries for mutation strength |
| `parse_mdns_endpoint_info` | P1 | Unit | standard/compact/empty/truncated/too-short covered | Add invalid base64 and invalid UTF-8 |
| `normalize_p256_coordinate` | P0 | Unit | short pad, signed 33-byte, invalid lengths covered | Add exact 32 and bad 33-byte prefix explicitly if not already parameterized |
| `get_download_dir` | P1 | Unit/Integration | Poison recovery code untested | Test only if global-lock isolation can be deterministic |
| `is_virtual_interface` | P0 | Unit | known tunnel prefixes and common LAN names covered | Add case-insensitive variants |
| `local_lan_ipv4` | P0 | Unit/Integration | Environment-dependent only | Extract pure selector seam; cover private/public/fallback/IPv6/loopback/tunnel interactions |
| `is_not_self_ip` | P2 | Integration | Environment-dependent | Leave integration-only unless selector seam makes it cheap |
| `RQS::{new,run,discovery,stop_discovery,change_visibility,set_foreground,stop,set_download_path}` | P1 | Integration/Smoke | Orchestration not unit-covered | Treat BlueZ/mDNS/task lifecycle as integration/smoke; pure state setters may be unit-tested if isolated |
| Frontend `ContentStatus.vue` | P2 | Component | Ready/drop, file select, discovery idempotence | No pre-smoke gap identified |
| Frontend `Heading.vue` | P2 | Component | Host/version/settings/update link | No pre-smoke gap identified |
| Frontend `SettingsModal.vue` | P2 | Component | load/save/clear port, invalid port, startup toggles, download folder | Extend only if `parseListeningPort` contract changes |
| Frontend `SideMenu.vue` | P2 | Component | visibility states and cancel event | No pre-smoke gap identified |

## Stage 4 - Branch map

### Listening port parser
1. Trimmed input empty -> automatic port.
2. Non-integer -> reject.
3. Integer < 1024 -> reject.
4. Integer > 65535 -> reject.
5. Integer inside inclusive range -> accept.

### Receiver service data
1. Device type is masked to three bits.
2. Name length <= 237 bytes -> preserve full byte sequence.
3. Name length > 237 bytes -> truncate to protocol limit.
4. Endpoint-info length must fit one byte.
5. Service-data framing length must match the encoded connection advertisement.

**Observed defect candidate:** byte truncation is not UTF-8-boundary-aware. A multibyte name can be cut mid-codepoint while the PR contract currently claims safe UTF-8 truncation.

### BWU router
1. Empty endpoint -> reject registration.
2. New endpoint -> register.
3. Duplicate pending endpoint -> reject.
4. Route known endpoint -> remove pending entry and deliver socket.
5. Route unknown endpoint -> return socket.
6. Route with dropped receiver -> return socket.
7. Cancel known endpoint -> remove.
8. Cancel unknown endpoint -> no-op.
9. Multiple independent endpoint IDs -> independent lifecycle.

### Weave connection request
1. Length < 7 -> reject.
2. Control bit absent -> reject.
3. Command != connection request -> reject.
4. Protocol version 1 outside requested range -> reject.
5. Requested packet size < 20 -> clamp to 20.
6. Requested packet size 20..509 -> preserve.
7. Requested packet size > 509 -> clamp to 509.

### Framed-message length
1. Prefix < 4 bytes -> incomplete.
2. Declared length 0 -> reject.
3. Declared length > 5 MiB -> reject.
4. Prefix + body incomplete -> incomplete.
5. Prefix + body complete -> return first-frame total.
6. Trailing bytes after a complete frame -> still return first-frame total.

### Weave session
1. Receiver unavailable -> error.
2. Ignore pre-handshake junk until a valid connection request.
3. Handshake channel closes -> error.
4. GATT notification stops -> clean exit.
5. Packet channel closes -> clean exit.
6. Empty packet -> ignore.
7. Control ERROR -> error.
8. Other control packet -> ignore.
9. FIRST resets reassembly.
10. Reassembly overflow or > safety limit -> error.
11. Non-LAST -> continue buffering.
12. Reassembly < 3 bytes -> clear/ignore.
13. Socket introduction -> ignore as control.
14. Socket disconnection -> clean exit.
15. Unknown service hash -> clear/ignore.
16. Missing framed length -> error.
17. Declared 0 or > max -> error.
18. Declared/body mismatch -> error.
19. Valid inbound framed message -> forward to duplex.
20. Duplex EOF -> clean exit.
21. Outbound buffer overflow -> error.
22. Complete outbound frame -> prepend service hash and fragment.
23. FIRST/LAST bits set correctly across fragments.
24. Send counter wraps without corrupting payload.

### Inbound filename/file preparation
1. Empty name -> reject.
2. Byte length > 255 -> reject.
3. slash/backslash/NUL/control -> reject.
4. Exactly one normal path component -> accept.
5. Any other component form -> reject.
6. Negative file size -> reject.
7. Duplicate payload ID against existing/current batch -> reject.
8. Destination free -> use original name.
9. Destination exists/reserved -> increment numeric prefix until unique.
10. Destination suffix overflow -> reject.
11. Total transfer size overflow -> reject.
12. Full batch succeeds -> return complete prepared set; failure must not partially mutate request state.

### Wi-Fi password parser
1. Payload < 4 -> reject.
2. Prefix != 0x0A -> reject.
3. Declared password length arithmetic overflow -> reject.
4. Declared payload/trailer beyond available bytes -> reject.
5. Trailer marker != 0x10 -> reject.
6. Invalid UTF-8 password -> reject.
7. Valid payload -> return password.

### Plain BWU frame
1. Read: zero or > 5 MiB -> reject.
2. Read: valid length + full body -> return.
3. Read: truncated prefix/body -> I/O error.
4. Send: empty -> reject.
5. Send: > 5 MiB -> reject.
6. Send: length not representable as u32 -> reject.
7. Send: valid -> write prefix, body, flush.

### CLIENT_INTRODUCTION classifier
1. Incomplete prefix -> None.
2. Zero/oversized length -> None.
3. Incomplete body -> None.
4. Complete valid BWU CLIENT_INTRODUCTION with non-empty endpoint -> Some(endpoint).
5. Complete malformed/non-BWU/empty-endpoint frame -> None at peek layer.
6. Strict validator returns an error for malformed/non-BWU/empty endpoint.

### Migratable stream
1. BLE variant delegates read/write/flush/shutdown.
2. TCP variant delegates read/write/flush/shutdown.
3. Transport swap happens by replacing enum variant while request-owned crypto state remains untouched.

### Outbound file preparation
1. Non-file path -> skip.
2. Open failure -> skip.
3. Metadata failure -> skip.
4. image/video/audio/apk/other -> correct attachment type.
5. Missing filename -> error.
6. Non-UTF8 filename -> error.
7. Payload ID collision -> regenerate.
8. File size not representable in i64 -> error.
9. Total transfer size overflow -> error.
10. Empty input -> empty deterministic plan.

### BWU listener classifier
1. No pending route -> return socket immediately.
2. Pending route + EOF -> normal inbound socket.
3. Pending route + invalid/oversized length -> normal inbound socket.
4. Pending route + incomplete frame until deadline -> normal inbound socket.
5. Pending route + ordinary Quick Share frame -> normal inbound socket.
6. Pending route + valid CLIENT_INTRODUCTION for registered endpoint -> consume route, do not consume bytes.
7. Valid CLIENT_INTRODUCTION for unregistered endpoint -> consume/classify socket without creating a normal inbound session.

### Payload-size guard
1. < 0 -> reject.
2. 0 -> accept.
3. 1..max -> accept.
4. max -> accept.
5. max+1 -> reject.

### LAN address selection
1. Interface enumeration fails -> None.
2. Virtual/tunnel interface -> ignore.
3. IPv6 -> ignore.
4. loopback/link-local IPv4 -> ignore.
5. private non-virtual IPv4 -> return immediately.
6. public non-virtual IPv4 -> remember as fallback.
7. later private address beats earlier public fallback.
8. only public candidates -> first fallback.
9. no eligible candidates -> None.

## Stage 5 - Positive/N1-N12 matrix

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Port parser | C | P | C | NA(no network) | NA(no retry) | NA(pure) | NA(no storage) | NA(no auth) | NA(no time) | NA(no UI interruption in parser) | NA(no PII) | NA(no billing) | NA(no storage) |
| Receiver service data | C | P | C | NA(no network call) | NA(no retry) | NA(pure) | NA(no persisted data) | NA(no auth) | NA(no time) | NA(no UI) | NA(no PII log) | NA(no billing) | NA(no storage) |
| Receiver advertiser lifecycle | S | NA(no external input parser) | S | S | S | S | NA(no persisted data) | NA(no auth) | NA(no time semantics) | S | NA(no PII) | NA(no billing) | NA(no storage) |
| BWU router | C | P | P | NA(no network semantics beyond socket ownership) | P | P | NA(no persisted data) | P(endpoint identity) | NA(no time) | P(cancel/drop) | NA(no PII) | NA(no billing) | NA(no storage) |
| Weave request/framing helpers | C | P | P | NA(pure helpers) | NA(no retry) | NA(pure helpers) | P(malformed frames) | P(protocol misuse) | NA(no time) | NA(no UI) | NA(no PII) | NA(no billing) | NA(no storage) |
| Weave session | S | P | P | S | P | P | P | P | NA(no locale/time) | S(disconnect) | NA(no PII by contract) | NA(no billing) | NA(no storage) |
| Inbound filename/files | C | C | P | NA(no network in helper) | P(duplicate) | NA(single-thread helper) | C | C(path abuse) | NA(no time) | P(partial failure) | P(filename/log review) | NA(no billing) | P(write/path failures) |
| Wi-Fi payload parser | C | P | P | NA(pure parser) | NA(no retry) | NA(pure) | P | P(malformed peer input) | NA(no time) | NA(no UI) | P(secret must not be logged) | NA(no billing) | NA(no storage) |
| Plain BWU frame | C | P | P | P(truncated/offline I/O) | NA(no retry policy) | NA(single stream) | P | P(peer framing abuse) | NA(no time) | P(peer closes) | NA(no PII) | NA(no billing) | NA(no storage) |
| CLIENT_INTRODUCTION | C | P | P | P(incomplete socket data) | P(repeat intro) | P(route race) | P | P(endpoint mismatch/malformed) | NA(no time) | P(peer closes) | NA(no PII) | NA(no billing) | NA(no storage) |
| Inbound secure state machine | C | P | P | P | P | P | P | P | NA(protocol has no locale/time rule) | P(cancel/disconnect) | P(no secrets in logs) | NA(no billing) | P(file writes) |
| Migratable stream | C | NA(byte stream input) | P(buffer/EOF) | P(I/O errors) | NA(no retry) | P(swap timing) | NA(no persisted data) | NA(no auth) | NA(no time) | P(shutdown) | NA(no PII) | NA(no billing) | NA(no storage) |
| Outbound file preparation | C | P | P | NA(no network in helper) | P(payload collision) | NA(single-thread helper) | P | P(path/name abuse) | NA(no time) | P(cancel handled elsewhere) | P(file path log review) | NA(no billing) | P(open/read failures) |
| Outbound secure state machine | C | P | P | P | P | P | P | P | NA(no locale/time) | C(cancel/deny) | P(no secrets in logs) | NA(no billing) | P(file read failures) |
| BWU listener classifier | C | P | P | P | P | P | P | P(endpoint identity) | P(deadline behavior) | P(peer closes) | NA(no PII) | NA(no billing) | NA(no storage) |
| Payload-size guard | C | C | C | NA(pure) | NA(no retry) | NA(pure) | NA(no persisted data) | P(peer abuse) | NA(no time) | NA(no UI) | NA(no PII) | NA(no billing) | NA(no storage) |
| mDNS parser | C | P | P | NA(pure parser) | NA(no retry) | NA(pure) | P | P(malformed peer record) | NA(no time) | NA(no UI) | P(device-name handling) | NA(no billing) | NA(no storage) |
| P-256 normalization | C | C | C | NA(pure) | NA(no retry) | NA(pure) | P | C(peer key abuse) | NA(no time) | NA(no UI) | NA(no PII) | NA(no billing) | NA(no storage) |
| LAN selector | P | P | P | P(interface enumeration failure) | NA(no retry) | P(interface-order interaction) | P | P(VPN/tunnel exclusion) | NA(no time) | NA(no UI) | P(no tunnel leakage) | NA(no billing) | NA(no storage) |
| RQS orchestration | S | P(config inputs) | P(port/buffer boundaries) | S | S | S | P | P | NA(no locale/time) | S | P | NA(no billing) | S(download path) |
| Frontend components | C | C | C | P(clipboard/plugin failures where applicable) | P(double action) | P(rapid events) | P(stale settings) | NA(no auth boundary here) | NA(no time-sensitive UI in scope) | P(close/back/drop) | P(no secret rendering/logging) | NA(no billing) | P(settings/download failures) |

## Stage 6 - Interaction matrix

Mandatory interaction sets:

1. Visibility x advertiser type x repeated transfer:
   - Visible/Temporary/Invisible x FE2C/FEF3 x first/repeated receive.
   - Automation: pure visibility predicate only.
   - Smoke: BlueZ coexistence and re-registration.

2. BWU route state x incoming frame:
   - no pending / pending other endpoint / pending same endpoint
   - ordinary frame / incomplete intro / malformed intro / valid intro.
   - Highest risk: valid intro for a stale or dropped registration.

3. Transport x session state:
   - BLE/TCP x pre-UKEY2/post-UKEY2/post-upgrade x read/write/shutdown.
   - Keys and sequence counters must remain request-owned rather than transport-owned.

4. Filename x destination state x metadata validity:
   - safe/unsafe name x free/existing/reserved destination x positive/negative size x unique/duplicate payload ID.
   - Any invalid member of a batch must leave request state unchanged.

5. Frame length x actual bytes x transport closure:
   - 0/1/max/max+1 x short/exact/trailing x open/EOF.
   - No allocation before validation.

6. Outbound source x filesystem state:
   - image/video/audio/apk/other x exists/missing/unreadable/non-UTF8 x one/many files.

7. LAN interfaces:
   - private/public x physical/virtual x IPv4/IPv6 x order permutations.
   - Private eligible address must win over public fallback regardless of order.

8. User action x outbound state:
   - cancel/accept/reject x Initial/WaitingForConsent/transferring/Finished.
   - Foreign transfer IDs and LibToFront messages must not cancel the active transfer.

## Stage 7 - Coverage estimation

Expected pre-smoke additions from the current gaps:

- Port parser: 1 table-driven test.
- Receiver advertisement data: 1 UTF-8 boundary test, potentially accompanied by a small production fix.
- BWU router: 3-4 tests.
- Weave pure helpers: 4-6 tests.
- Inbound filename/file preparation and framing: 4-6 tests.
- Migratable TCP variant: 2 tests.
- Outbound preparation: 2-4 tests.
- BWU listener classifier: 3-5 tests, preferably after extracting a deterministic classification seam.
- Payload-size guard: 1 table-driven expansion.
- mDNS/P-256 utilities: 2-3 tests.
- LAN selection: 5-7 table-driven cases after a pure selector seam.
- Error truncation: 1 table-driven test.

Estimated additional automated scenarios before hardware smoke: **29-40**. This is an estimate, not a quota. Tests may be consolidated with parameterized cases only when they validate the same branch/rule.

## Stage 8 - Coverage map

| Production file | Important functions/branches | Current state | Remaining reason |
|---|---|---|---|
| `hdl/blea.rs` | visibility predicate, receiver data, BlueZ lifecycle | Partial | BlueZ lifecycle is hardware-bound; UTF-8 boundary gap is deterministic |
| `hdl/bwu.rs` | register/route/cancel lifecycle | High line coverage | Negative/concurrency lifecycle branches still need explicit tests |
| `hdl/gatt.rs` | request parsing, frame length, weave reassembly/fragmentation | Partial | Core session is coupled to BlueZ notifier; pure branches can be extracted/tested |
| `hdl/inbound.rs` | filesystem guards, frame parsing, client intro, secure state machine, BWU | Partial | Large state machine includes hardware/crypto integration branches and deterministic gaps |
| `hdl/migratable.rs` | BLE/TCP delegation | High | TCP variant lacks explicit tests |
| `hdl/mod.rs` | state/data definitions | Mostly declarative | No standalone behavior beyond state containers |
| `hdl/outbound.rs` | file plan, cancel/consent, secure send state machine | Partial | Filesystem failures and some state branches remain |
| `lib.rs` | task orchestration, visibility, discovery, shutdown | Low | mDNS/BlueZ/task orchestration; package/live integration is more representative |
| `manager.rs` | BWU classification, accept/send loops | Partial | Classifier negative branches and accept-loop lifecycle remain |
| `protocol.rs` | payload size guard | Strong branch coverage | Add inner boundary values for mutation confidence |
| `utils.rs` | mDNS, P-256, download dir, LAN selection | Partial | LAN selection is environment-coupled; some malformed input cases remain |
| `app/main/src/vue_lib/network.ts` | listening port parser | Strong | NaN/Infinity explicit cases remain |
| Frontend components | settings/status/header/menu flows | High component coverage | OS/plugin/window lifecycle remains smoke/integration territory |

Stages 3-8 are now documented. Test implementation may start from the planned deterministic gaps without waiting for hardware smoke.


## Consolidated dev scope extension - Full Quick Share

After PR #3 was integrated into `dev`, the Mode B scope expanded. The following artifacts are mandatory before the final completion gate.

### Stage 3 extension - Test inventory

| Artifact | Risk | Level | Current evidence | Planned delta |
|---|---|---|---|---|
| Modern `sharing.nearby` protobuf fields | P1 | Contract | Round-trip tests cover modern introduction fields, QR handshake material, and resume attachment details | Add explicit backward-compatibility decode of legacy/minimal frames and enum/default-field boundaries where generated-code behavior can regress |
| `classify_outbound_text` | P1 | Unit | URL, phone number, plain text covered | Add case/whitespace boundaries, false-positive phone inputs, short digit strings, address-like text |
| `prepare_outbound_text` | P0 | Unit | Empty rejection and UTF-8 byte-size preservation covered | Add max/max+1 payload-size boundary, URL payload type, 64-character title truncation, whitespace-only policy |
| `byte_payload_frames` | P0 | Unit/Contract | Multi-chunk framing, offsets, final marker, reconstruction covered | Add empty body, exact chunk, chunk+1, max/max+1 payload boundaries and sensitive/header invariants |
| `ManagedEphemeralFile` / `OutboundPayload::EphemeralFiles` | P0 | Unit/FS integration | Security defect identified: raw strings previously acquired deletion capability | Add constructor path-boundary abuse cases, missing-file idempotence, verified cleanup, and ensure normal `Files` payload never deletes source files |
| Tauri `managed_temp_path` | P0 | Unit/Security | Prefix restriction test exists | Add path traversal / sibling path / extension / non-temp-root abuse cases |
| Tauri `save_clipboard_image` | P1 | Integration/Smoke | No deterministic clipboard backend seam | Package/live smoke with image clipboard; keep backend integration smoke-only unless DI is introduced |
| Tauri `remove_ephemeral_file` | P0 | Unit/Integration | Prefix guard is shared but command path is not directly covered | Add allowed managed path removal, missing-file idempotence, forbidden path rejection |
| `ContentStatus.shareClipboard` | P1 | Component | Text success, image fallback, no-content error covered | Add whitespace text + valid image fallback and discovery-already-running for clipboard path |
| Frontend `clearSending` ephemeral cleanup | P0 | Unit | No direct test found | Add EphemeralFiles cleanup, cleanup failure resilience, normal Files no-delete, state reset invariants |
| Outbound text consent/send state machine | P0 | Integration | Existing consent tests cover file/empty flows; text helper tests do not prove state-machine integration | Add accepted text sends BYTE payload then Finished+disconnect; cancellation before text send; rejected text follows normal rejection path |
| Wi-Fi SAE inbound handling | P1 | Unit/Integration | SAE branch added to production parsing | Add SAE password payload regression matching WPA/WEP parsing behavior |

### Stage 4 extension - Branch map

#### Outbound text classification
1. Trim surrounding whitespace for classification only.
2. `http://` and `https://`, case-insensitive -> URL.
3. Phone candidate strips whitespace, `-`, `(`, `)`.
4. At least seven digits and only digits/optional `+` -> phone number.
5. Short/invalid phone-like input -> plain text.
6. Everything else -> plain text.

#### Outbound text preparation
1. Empty string -> reject.
2. Non-empty UTF-8 -> preserve original payload bytes.
3. Payload byte length > protocol maximum -> reject.
4. URL classification -> `TextPayloadType::Url`; other classifications -> `Text`.
5. Metadata title uses at most 64 Unicode scalar values.
6. Metadata size equals byte length, not character count.
7. Generated payload id is copied consistently into metadata.

#### BYTE payload framing
1. Body length 0 -> terminal frame only.
2. 1..256 KiB -> one DATA frame + terminal frame.
3. Exact 256 KiB -> one DATA frame + terminal frame.
4. 256 KiB + 1 -> two DATA frames + terminal frame.
5. Body at protocol max -> accepted.
6. Body > protocol max -> reject before frame allocation.
7. Every DATA frame carries same payload id/total size/type.
8. Offsets are monotonic and exact.
9. Nonterminal chunks have flags=0.
10. Final empty chunk has offset=total size and flags=1.

#### Ephemeral outbound cleanup
1. Raw `EphemeralFiles` strings have no deletion authority.
2. Each raw path must pass `ManagedEphemeralFile` validation before `OutboundRequest` construction succeeds.
3. Only a direct child of the system temp directory with the application-owned `rquickshare-clipboard-*.png` name may become managed.
4. Normal `Files` never acquire deletion authority.
5. Existing managed ephemeral file -> removed on drop or explicit cleanup.
6. Already missing managed file -> idempotent success.
7. Traversal, sibling, wrong-prefix, wrong-extension and ordinary user paths -> reject.
8. Removal failure -> warn/error without panic and without broadening the allowed boundary.
9. Frontend cancellation/reset asks Tauri to remove only managed ephemeral files; Tauri delegates the same core validation instead of duplicating policy.

#### Clipboard UI
1. Nonblank clipboard text -> emit Text, ensure discovery.
2. Blank text -> try image.
3. Text backend error -> try image.
4. Image saved -> emit EphemeralFiles, ensure discovery.
5. Image backend error -> inline error, emit no payload.
6. Discovery already running -> do not restart it.
7. Clipboard error is cleared before a new attempt.

#### Managed clipboard-image path
1. File must live under application temp root.
2. Filename must match the application-owned PNG prefix.
3. Sibling/parent traversal or arbitrary temp file -> reject.
4. Missing managed file removal -> idempotent success.
5. Existing managed file removal -> delete exactly that file.

### Stage 5 extension - Positive/N1-N12 matrix

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Modern protobuf compatibility | C | P | P | NA(pure encode/decode) | C(round-trip) | NA(pure) | P(legacy/missing fields) | P(malformed enum/fields) | NA(no time semantics in scoped fields) | NA(no UI) | P(sensitive flags must preserve semantics) | NA(no billing) | NA(no storage) |
| Outbound text classification/preparation | C | P | P | NA(pure) | NA(no retry) | NA(pure) | NA(no persisted data) | P(untrusted text input) | NA(no time) | NA(no UI) | P(text must not be logged as secret content) | NA(no billing) | NA(no storage) |
| BYTE payload framing | C | P | P | NA(pure framing) | P(re-send/idempotent frame construction) | NA(pure) | P(malformed sizes) | P(peer/protocol boundary) | NA(no time) | NA(no UI) | P(sensitive/header invariant) | NA(no billing) | NA(no storage) |
| Ephemeral file cleanup | C | P | P | NA(no network) | P(double cleanup) | P(drop vs frontend cleanup) | P(missing file) | C(prefix/path ownership boundary) | NA(no time) | C(cancel/reset) | C(do not delete arbitrary user file) | NA(no billing) | P(remove failure/read-only path) |
| Clipboard UI flow | C | C | NA(no numeric limit) | P(plugin/backend failure) | P(repeated paste) | P(rapid retry) | P(stale clipboard/backend state) | NA(no auth) | NA(no time) | C(error/retry/reset) | P(no clipboard content in logs) | NA(no billing) | P(temp-file failure) |
| Outbound text state machine | P | P | P | P(peer disconnect) | P(cancel/retry) | P(cancel vs accept) | P(malformed consent) | P(protocol misuse) | NA(no time) | P(cancel) | P(text secrecy/log review) | NA(no billing) | NA(no persistent storage) |
| Wi-Fi SAE receive | P | P | P | P(truncated transfer) | NA(no retry) | NA(single session parser) | P | P(malformed credentials) | NA(no time) | P(reject/cancel) | P(password must not be logged) | NA(no billing) | NA(no storage) |

### Stage 6 extension - Interaction matrix

9. Clipboard content x backend state x discovery state:
   - text/image/empty x text-plugin success/failure x image-command success/failure x discovery idle/running.
   - Exactly one outbound payload may be emitted per user action.

10. Outbound text size x chunk boundary x classification:
   - plain/URL/phone x 0/1/chunk/chunk+1/max/max+1 bytes.
   - Classification must not affect byte framing or size validation.

11. Ephemeral ownership x cleanup trigger:
   - managed/unmanaged path x frontend clear/drop/double cleanup x exists/missing/removal error.
   - Arbitrary user files must never be deleted.

12. Protocol generation compatibility:
   - legacy absent fields / modern populated fields x encode/decode.
   - New optional fields must not change defaults of old peers.

### Stage 7 extension - Coverage estimation

Additional deterministic scenarios for the consolidated Full Quick Share scope:

- text classification/preparation: 6-9 cases;
- BYTE framing boundaries: 5-7 cases;
- ephemeral cleanup security/idempotence: 5-7 cases;
- clipboard component interactions: 2-4 cases beyond existing coverage;
- outbound text state-machine integration: 3-4 cases;
- SAE receive regression: 1-2 cases;
- protobuf backward/default compatibility: 2-4 cases.

Estimated delta: **24-37 scenarios**, preferably table-driven where the branch/rule is identical.

### Stage 8 extension - Coverage map

| Production file | Important functions/branches | Current state | Remaining reason |
|---|---|---|---|
| `core_lib/src/proto_src/wire_format.proto` | modern optional fields/enums/default compatibility | Partial contract coverage | Legacy/default-field compatibility and malformed boundaries remain |
| `core_lib/src/hdl/outbound.rs` | text classification, metadata, BYTE framing, ephemeral cleanup, text consent/send | Partial | Pure helpers covered; state-machine and boundary matrix incomplete |
| `core_lib/src/hdl/inbound.rs` | SAE credential parsing | Partial | Explicit SAE regression missing |
| `app/main/src-tauri/src/cmds/clipboard_image.rs` | path ownership, image save, removal | Partial | Path abuse/removal branches deterministic; clipboard backend smoke-bound |
| `app/main/src/composables/ContentStatus.vue` | clipboard text/image/error/discovery interaction | Good | Blank-text image fallback + already-running interaction remain |
| `app/main/src/vue_lib/utils.ts` | ephemeral cleanup during clear/reset | Missing direct coverage | Deterministic unit seam already exists through VM invoke contract |

The final Mode B gate must use this extension together with the original Stage 3-8 inventory. Hardware smoke does not replace these deterministic cases.

## Hardware smoke findings - 2026-10-06

Real Pixel-to-Linux and Linux-to-Pixel smoke on Ubuntu 26.04 KDE/Wayland produced the following evidence:

- Linux -> Pixel clipboard-image transfer succeeds over the normal LAN path and managed clipboard temporary files are removed after completion.
- Pixel -> Linux file receive establishes the BLE Weave session and completes UKEY2, but the offered Wi-Fi LAN bandwidth upgrade times out after 15 seconds. No TCP connection reaches the application's listener before the timeout, so the transfer falls back to BLE and progresses in roughly 509-byte payload chunks.
- The peer sends `BANDWIDTH_UPGRADE_RETRY` / `UPGRADE_FAILURE` after the failed Wi-Fi handoff. These frames are currently logged as unhandled and should be treated as expected fallback diagnostics rather than protocol-fatal errors.
- Two Pixel -> Linux text attempts fail before a receiver GATT/Weave session is opened. No `TextMetadata` reaches `InboundRequest::process_introduction`, so the current evidence does not support changing the inbound text parser.
- The completed-file `Open` action passes a local destination path to the frontend shell URL opener. This violates the shell opener scope and produces an invalid-URI/open failure. Local receive destinations need a dedicated backend capability rather than broad shell URL permissions.

Next deterministic/live checks:

1. Add FEF3 slot-0 read, Weave write/notify-subscription, and negotiated packet-size diagnostics without logging payload contents.
2. Keep inbound text parser unchanged until the next instrumented hardware run proves that a sharing frame reaches it.
3. Verify Linux firewall/listener reachability for the advertised BWU TCP port before changing the BWU router or protocol.
4. Fix local received-destination opening through a backend-only command that computes the configured/default download directory itself; do not accept an arbitrary filesystem path from the frontend.
5. Re-run Pixel -> Linux text and repeated Pixel -> Linux file receive without restarting the application.

### Follow-up smoke evidence

A later Pixel -> Linux text attempt reached the receiver successfully and exposed a separate state-machine bug:

- The text `IntroductionFrame` is decoded and the UI is moved to `WaitingForUserConsent`.
- Before the user clicks Accept, the peer sends another completed BYTES payload with a different payload ID. The current receiver sees `text_payload.is_some()` and incorrectly assumes every completed BYTES payload is the announced text body, then terminates the BLE session with `Unexpected text payload id`.
- Reference Quick Share receiver behavior (NearDrop) first matches the completed BYTES payload ID against the announced text payload ID; non-matching BYTES payloads continue through the transfer-setup frame decoder. rQuickShare must preserve that distinction.
- The frontend currently remains in `WaitingForUserConsent` after this GATT session error, so subsequent Accept clicks are sent into a dead session. GATT inbound failures must surface `Disconnected` to the frontend.

The same run also proved the Wi-Fi BWU reachability failure is host-firewall related on the smoke machine: UFW is active with default incoming deny, while rQuickShare is listening on a random high TCP port (40119 in the retest; 42971 in the earlier run). No rQuickShare rule is present. For the current smoke, use the existing fixed-port setting plus an explicit UFW rule; do not change the user's firewall automatically from the application.

### Successful post-fix Pixel smoke

A later hardware run with the fixed port reachable through UFW confirms the transport and text-state fixes:

- Cold BLE receiver path: Android reads FEF3 slot 0, opens the Weave notify session, completes UKEY2, accepts the advertised Wi-Fi LAN upgrade, connects to the fixed TCP port, completes LAST_WRITE / SAFE_TO_CLOSE, then the inbound session migrates from BLE to Wi-Fi LAN.
- Once the TCP socket reaches rQuickShare, introduction parsing and consent presentation are fast (roughly 0–1 second on direct TCP sessions).
- The remaining perceived pre-consent delay is before or during Android discovery/transport selection. On the observed cold path, slot-0 read -> GATT notify subscription took about 9 seconds and Wi-Fi upgrade negotiation took about another 8 seconds before consent. Treat this as a separate discovery/transport-latency optimization task; do not weaken the now-working protocol sequence without a measured experiment.
- Warm/direct TCP inbound file transfer completed about 1.5 MB in roughly one second after acceptance, confirming that the previous 509-byte BLE crawl was caused by the blocked BWU port rather than file-transfer chunking.
- Inbound text now completes successfully after acceptance, with the full text payload reaching `Finished`.
- The repaired local `Open` action successfully opens the configured download directory.

Notification UX follow-up:

- Consent notifications currently contain only the sender name even though the channel metadata already includes text description, file names and total bytes.
- Consent notifications should show a bounded preview/type summary so the user can decide without opening the main window.
- Successful inbound text should raise a completion notification with a bounded preview and a `Copy` action that copies the full received text.
- Successful inbound files should raise a completion notification with file/count summary and an `Open folder` action.
- Notification previews must be bounded to avoid oversized desktop notifications; the underlying payload must not be truncated for actions such as Copy.

### Reveal received item UX

Post-smoke UX follow-up for completed inbound files:

- The completed-file action should reveal the received item itself in the desktop file manager instead of opening the download directory at its initial scroll position.
- On Linux, use the standard `org.freedesktop.FileManager1.ShowItems` D-Bus interface so Dolphin/Nautilus can open the parent folder and select the file.
- The frontend must not regain arbitrary filesystem-open authority. It may pass only the received basename from trusted transfer metadata; the backend resolves the configured download directory, rejects non-basename/path-traversal input, verifies the candidate exists under that directory, and constructs the file URI.
- If the file-manager D-Bus interface is unavailable or the named file cannot be resolved, fall back to opening the configured download directory.
- For multi-file completion, revealing the first received item is sufficient for the current UI; multi-selection can be added later.


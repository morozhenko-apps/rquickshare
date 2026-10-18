# MODE B P0 physical acceptance matrix — installed Linux / Android / Windows

Date: 2026-10-18. This is a **planned physical-system test
inventory**, not executed acceptance evidence. It is deliberately
separate from green automated Core/Native GTK/Frontend test
counters. No source or installed application changes.

## Scope and threat model

Baseline program in repo: native GTK UI (Wayland/X11),
daemon, mDNS discovery, BLE/GATT capability negotiation,
BWU TCP upgrade, Android Google Quick Share, optionally
Windows Quick Share. The code base currently has verified
private D-Bus broker and isolated GTK/Xvfb tests; **none**
of those establishes physical device compatibility.
Untrusted nearby devices and changes in screen lock,
visibility and network path can overlap an in-progress
transfer. Only a real paired peer's handshake may confer
send authority; a plaintext 'last_endpoint_id' alone
is not an authentication proof (Issue #8).

Each physical run records exact client/release versions,
commit/installed binary hash, Ubuntu/KDE/Wayland-or-X11,
Android model/OS/Quick Share build, transport/interface,
timestamps, direction, size, result code and **redacted**
native app/daemon logs. Never upload personal file
contents, BLE addresses, display names, private IPs,
tokens, endpoint identifiers or received text verbatim.
Note clearly whether the app needs *pairing*, not
assume pair state survives network re-creation.

## Stage 3/4 P0 physical scenarios

| ID | Setup / ordered action | Expected observable result |
|---|---|---|
| H01 | Linux and Android idle; first discovery from fresh launch | Exactly one reachable peer per actual endpoint; old mDNS/BLE entries age out; panel reflects correct ready state |
| H02 | Fresh pair, if needed, then Android→Linux short UTF-8 text | Explicit correct consent flow; exact accepted text, no spurious unsupported-type failure; rejected text does not persist or appear later |
| H03 | Repeat Android→Linux non-ASCII emoji/European Portuguese text | Byte-accurate accepted content without stray replacement, duplicate or unintended auto-open |
| H04 | Android→Linux 1 small image; Linux→Android 1 small image | Both directions succeed with correct sizes and contents; clear progress, one terminal result |
| H05 | Linux→Android folder with nested files and two equal basenames in different directories | Relative paths preserved, no overwrite outside destination, accurate byte totals and per-file result |
| H06 | Android→Linux multi-file batch (2 to 10), consent/accept once | No silently dropped members or duplicated queued transfers; missing directory handled safely |
| H07 | Reject incoming transfer on Linux before first byte | No content persisted or consent replay; sender sees terminal rejected |
| H08 | Reject on Android from Linux send, then explicitly retry | No implicit retry; new explicit selection/revision succeeds once |
| H09 | Cancel sender during partial large file on both directions | No corrupted final file; partials reported/cleaned according to documented policy; next transfer remains possible |
| H10 | Cancel receiver in the middle of a multi-file batch | Correct partial/failure status and no unintended second-file admission; no race-triggered success |
| H11 | Turn Wi-Fi off/on between BLE discovery and BWU TCP upgrade | Transfer fails safely or explicitly recovers; never authenticates a mismatched peer via old alias |
| H12 | Change Wi-Fi network while TCP transfer is in flight | No deadlock, duplicate ownership, or misdirected replay; terminal response and resource cleanup |
| H13 | Restart Linux daemon after pairing but before consent | Stale session and consent revision cannot act in new epoch; rediscovery recovers |
| H14 | Restart Linux daemon after files chosen but before send | Old Selection denied; user gets Recover/discard outcome; no silent automatic resubmit |
| H15 | Android goes out of range, returns, renames nearby endpoint | Exactly one correct discovery row after expiry/rebind; wrong peer name cannot inherit live Selection |
| H16 | Lock KDE desktop while incoming consent card is shown | Card hidden; consent not auto-granted; unlock requires a fresh live verified round and manual action |
| H17 | Turn on notification inhibition, manually open card | Automatic presentation stays disabled, manual access remains allowed if unlocked; no stale automatic consent |
| H18 | Trigger native GTK copy/cut URI clipboard, change owner while file list is streaming | No previous owner file is sent after source loss; record the *mid-read* window rather than owner loss before read |
| H19 | While Linux is preparing files, close chooser/GTK card and immediately choose another set | Cancelled old generation cannot send or display stale error; new selection has its own owner and paths |
| H20 | Unplug/change display or switch Wayland monitor scaling while card is visible | No offscreen card, stale x/y assumption or unauthorized auto-reveal; manual open remains accessible |
| H21 | Discovery-only peer with invalid/oversized GATT/TCP negotiation | No application crash, no peer substitution, no successful phantom send; log a bounded protocol failure |
| H22 | Repeated transfers (10) in both directions after intentional failure H09 | No progressive stale peer/DBus/watch task accumulation; no regression in pairing/discovery or notification lifecycle |

## Stage 5: positive/N1–N12 / combinations

Positive: H02–H06.
N1 malformed/untrusted: H21.
N2 size/count/edge: H03/H05/H06/H09/H21.
N3 transport: H11/H12/H15.
N4 repeat/retry: H08/H14/H22.
N5 concurrency/ownership: H13/H14/H18/H19.
N6 corruption: H03/H05/H09.
N7 authentication: H11/H13/H15/H21.
N8 locale: H03.
N9 cancel/lifecycle: H07/H09/H10/H13/H14/H19.
N10 privacy/lock: H02/H07/H16/H17/H18.
N11 entitlements N/A (no payment flow).
N12 file/OS: H05/H06/H09/H10/H20.

Meaningful cross-dimensional interactions:
direction {Android→Linux, Linux→Android}
× payload {text, image, one file, nested folder, multi-file}
× transport {BLE discovery + BWU, Wi-Fi dropped/changed}
× consent {accepted, rejected, cancelled}
× epoch {live, stopped/restarted} × desktop
{unlocked, locked, notification-inhibited} ×
display {Wayland, X11}. A one-device/manual
pass cannot close all combinations.

## Execution ledger (intentionally unverified)

Every H01–H22 begins **OPEN — NOT RUN**.
When manually executed, record:
`ID | environment/source SHA | actions | expected |
actual | PASS/FAIL/BLOCKED | redacted log reference |
issue/commit`. Avoid inventing status,
auto-promoting to PASS from isolated GTK
smoke, or treating physical tests as a substitute
for formal MODE B Stage 8 branch coverage.

Current architectural preconditions:
BWU identity authentication Issue #8,
mDNS lifecycle/SDK event seam Issue #9.
If an untrusted previous endpoint alias
is accepted as authentication, stop;
that requires an explicit Mode A product
security decision, not a test assertion rewrite.
Midstream clipboard-owner cancellation
still needs a controllable true GInputStream
or physical OS test rather than merely
replacing GDK bytes between completed reads.

Global Stage 11 complete final-source ten
consecutive repository passes, Stage 12
all-method/branch/negative map and physical
acceptance **remain OPEN/NO**.

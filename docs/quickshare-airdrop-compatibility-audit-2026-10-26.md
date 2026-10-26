# Quick Share compatibility and AirDrop feasibility audit

Mode A, research/planning slice requested by the owner on 2026-10-26.
Baseline: `b19031a94332223d90d67556cd4960851f4882f1` on dev; executable baseline
`6ae62f62b17b058160d3246873ecdd34377ec9f0` (six-green CI, uninstalled candidate).

## Scope and execution plan

Investigate the complete discovery/connection/authentication/consent/transport/
payload/termination path against public Google Nearby source and current product
documentation. Distinguish confirmed source defects, missing capabilities,
performance hypotheses and measured physical behavior. Assess AirDrop separately.
This slice does not promise or implement all parity features at once.

1. [x] Read repository guide, live backlog, roadmap, identity and BWU decisions;
   refresh remote ownership/open issues. Clean dev, one worktree, origin has only
   dev/master, no open PR or observed concurrent owner. Local Ubuntu shell,
   network/Git/connected GitHub and web research available. Sequential terminals.
2. [x] Map production paths and test/physical evidence; inspect current primary
   upstream sources. Record exact code anchors and immutable source revisions
   where available, rather than treating proto fields as implemented features.
3. [x] Rank defects and optimization candidates with mechanism, confidence,
   risk, required regression and acceptance measurement. Preserve approved A2.2
   legacy BWU compatibility and verified-only identity choices.
4. [x] Build Quick Share parity matrix and AirDrop feasibility/options with
   dependency/privilege/hardware constraints. Architectural choices are proposals,
   not approved implementation. No proprietary handshake or cloud dependency
   introduced without a specific subsequent decision.
5. [x] Update live backlog and existing roadmap links; record a concrete ordered
   implementation queue. Validate documentation links/whitespace and workflow
   policy; commit documentation separately and normal-push dev after tip checks.

Code changes: none planned in this research slice. Documentation changes: this
audit, backlog and roadmap pointer. Architecture: alternatives documented only.
Risks: confusing Google public C++ source with Android GMS implementation,
unsynchronized clocks, unsupported modern AirDrop claims, or silently weakening
consent/trust to obtain interoperability. Rollback: inverse docs-only commit.
No new test suites, hardware traffic, service restarts, package installation,
master/tag/publication, credential extraction or system-network changes planned.

## Findings and evidence

The receive initialization fix is source-complete but not physically accepted.
This audit found additional independent implementation defects and missing
capabilities; none is asserted to explain every observed Android delay. Findings
below reference the executable baseline above. No production changes were made
in this slice. The IDs below are documented backlog items, not completed fixes.

### Evidence model and diagnosis

Expected: one slow candidate must not block other discovery; control traffic and
Cancel must progress during transfer/upgrade; channel replacement must follow
peer synchronization; each transport session must own its ingress. Actual source
violations and capability gaps are recorded separately below.

Six plausible causes were checked before recommending changes:

| Hypothesis | Focused check | Result / confidence |
| --- | --- | --- |
| Discovery blocks behind an unreachable interface | Follow `run_event_loop` and injected probe call | Confirmed serialized probe and first-IPv4-only choice; hardware contribution unmeasured |
| BLE observation is mistaken for a complete discovery route | Trace monitor output through manager and outbound connector | Confirmed notification-only observation and TCP-only outbound path |
| Control traffic stalls during upgrade or bulk send | Inspect old-channel drain and complete-file sender loops | Confirmed missing SAFE_TO_CLOSE condition and deferred socket reading; latency impact needs a trace |
| Keepalive handling produces unnecessary protocol traffic | Compare both handlers with pinned Google implementation | Confirmed ACK-to-ACK behavior; Google ignores ACKs, so no claim of an observed Google ACK loop |
| A stale or unrelated GATT writer affects the next session | Trace write callback, shared queue and semaphore lifetime | Confirmed missing ingress peer attribution and retained queue; no cross-peer exploit reproduced |
| Bulk throughput is constrained by copies/disk/progress work | Inspect chunk construction, synchronous file calls and notifications | Redundant copy confirmed; bottleneck ranking requires matched transport/CPU/disk measurements |

No new packet captures or logging were necessary for this source audit. A live
latency attribution still requires the redacted stage trace and peer outcome.
No timers were changed and no source regression suite was run for documentation.

### QS-AUDIT-01 — discovery responsiveness and address selection

**Confirmed source issue; first implementation priority alongside keepalive.**
[mDNS discovery](../core_lib/src/hdl/mdns_discovery.rs), `run_event_loop`, selects
`get_addresses_v4().iter().next()` and awaits its probe in the event loop. The
production probe uses `TcpStream::connect` without an application deadline. Thus
one slow connection delays later resolved/removal events, and a dead first
address hides a device with a working alternative. The selected set iteration
order is not a reachability policy. IPv6-only routes are not supported here.

Implement bounded asynchronous per-service probes, generation-bound results and
removal cancellation. Try eligible advertised addresses with a deterministic
policy; a probe budget must have a documented connectivity rationale. Do not
merge distinct peers or confer trust from IP/name. Start with the existing IPv4
contract; scoped IPv6 needs interface-aware endpoints and separate acceptance.

Regressions: blocked A does not delay reachable B or removal; all advertised
addresses unreachable versus later address reachable; remove/re-resolve during
probe; stale success cannot resurrect a tombstone; bounded work under floods;
shutdown releases all probes. Measure discovery and click-to-connect separately
on multiple NICs, VPN, disconnected routes and IPv6-only networks. Existing
[#9](https://github.com/morozhenko-apps/rquickshare/issues/9),
[#11](https://github.com/morozhenko-apps/rquickshare/issues/11),
[#19](https://github.com/morozhenko-apps/rquickshare/issues/19) remain evidence
dependencies; do not overwrite their already implemented safeguards.

### QS-AUDIT-02 — keepalive ACK semantics

**Confirmed protocol handling defect; small compatible fix.** Both
[inbound](../core_lib/src/hdl/inbound.rs) and
[outbound](../core_lib/src/hdl/outbound.rs) KeepAlive branches unconditionally
call `send_keepalive(true)`, including for ACK=true. Google responds only to
ACK=false/absent and echoes its sequence number. Two rQuickShare peers can keep
acknowledging one another if a keepalive starts the exchange; this has not been
physically reproduced. Google peer behavior prevents inferring the same loop
for Android.

First fix ACK gating in both directions. Separately reconcile the local
[wire schema](../core_lib/src/proto_src/offline_wire_formats.proto) with Google's
optional `uint32 seq_num = 2`; retain absent-field compatibility and document the
wire contract before changing generated types. Test absent/false/true ACK,
missing/zero/max sequence, no unsolicited reply, and both authenticated handlers.
This does not justify inventing a new keepalive interval or timeout.

### QS-AUDIT-03 — finish BWU migration by state, not loop exhaustion

**Confirmed state transition issue; high correctness priority.** The recent
15-second optional-route wait fix does not cover the later drain. In
`Inbound::try_bandwidth_upgrade`, the `0..16` old-channel drain uses a separate
5-second timeout for each read. Timeout or exhausting all iterations reaches the
same disconnect/swap path as SAFE_TO_CLOSE; there is no required confirmation
flag. Trickle frames can extend this stage far beyond five seconds. The code
then sends plaintext disconnect, sleeps 200 ms and installs TCP.

Further risks in that path: cancellation of an in-progress body read can lose
consumed bytes; partial old-channel header state must not carry into TCP;
ordinary frame processing after LAST_WRITE can send old-channel application
traffic; established-session errors from this stage need the same fatal/optional
distinction as the fixed route wait. These are source risks, not measured losses.

Use an explicit migration state machine: distinguish precommit fallback from
post-LAST_WRITE failure, preserve partial reads and encryption sequence ownership,
pause application writes where required, require the peer transition, and close
the old channel on a proven event. Compare Google's paused replacement channel
and LAST_WRITE/SAFE_TO_CLOSE lifecycle; do not just remove the 200 ms sleep.
Regression cases: timeout/exhaustion without confirmation never reports upgrade
success; confirmation split across reads; peer cancel/EOF/bad MAC during drain;
concurrent application data; successful upgrade preserves ordering and counters.

The manager's partial-prefix classification also has a 750 ms budget and 5 ms
polling delay. Event-driven buffered classification is a secondary optimization
only if ordinary inbound bytes and admission semantics are preserved. The
[#8](https://github.com/morozhenko-apps/rquickshare/issues/8) untrusted route
ownership problem is distinct: approved A2.2 remains unchanged, and a new
cryptographic binding/wire challenge requires the owner's architectural choice.

### QS-AUDIT-04 — attribute GATT writes to a session

**Confirmed ownership gap; security design required before implementation.**
[GATT](../core_lib/src/hdl/gatt.rs) creates one bounded `Vec<u8>` write channel,
shares its receiver across notify sessions, and ignores `_request` in the write
callback. One semaphore limits workers, but does not identify the writer of
queued bytes. Queue lifetime exceeds session lifetime. `weave_session` waits
for CONNECT or notifier termination, without a handshake phase deadline.

This supports stale/mixed ingress and availability concerns, not a claim that
encrypted data can be forged. Preserve cryptographic fail-closed handling.
Design peer-attributed admission, a session generation and explicit queue
ownership/teardown. BlueZ device identity may bind transport ingress; it must
never become the verified user/device identity required by #16. Assess how BlueZ
associates notification subscribers before selecting the design. Define bounded
phase/resource ownership without an arbitrary sleep.

Tests must cover two writers, disconnect/reconnect, queued old fragments,
malformed pre-CONNECT traffic, idle subscriber and simultaneous subscription.
Also audit receive fragment counter semantics against the actual Weave contract;
an absent counter check alone is not yet proof of an interoperable fix.
Track with [#23](https://github.com/morozhenko-apps/rquickshare/issues/23) and
[#18](https://github.com/morozhenko-apps/rquickshare/issues/18).

### QS-AUDIT-05 — true duplex bulk transfer and measured efficiency

**Confirmed scheduling limitation and redundant copy; performance impact open.**
The outbound file loop checks locally queued actions but does not return to its
peer socket reader until the bulk send completes. Incoming remote Cancel and
keepalive can therefore wait behind file writes/backpressure. Use a frame-level
duplex scheduler with one owner of encryption sequence and writes; avoid detached
concurrent writes. Test remote cancel mid-file, blocked writer, peer EOF,
keepalive during transfer and immediate retry, with no false Success.

The same loop allocates 512 KiB per chunk and copies `sending_buffer` only to log
its length, then copies payload bytes again. File reads and inbound positional
writes/final persistence are synchronous in async handlers. Progress updates
also propagate per chunk. Remove the proven redundant allocation first; profile
buffer reuse, bounded blocking disk work and coalesced display updates separately.
Preserve final durability, cancellation responsiveness and exact terminal state.
No throughput improvement is claimed from source inspection. Use
[#24](https://github.com/morozhenko-apps/rquickshare/issues/24) after stable
connection acceptance; retain [#25](https://github.com/morozhenko-apps/rquickshare/issues/25)
cancel/retry gates. General logs currently include file paths and peer/network
metadata; safe stage markers do not make all logs privacy-safe. Plan redaction
before collecting broad debug logs.

### QS-AUDIT-06 — discovery is not yet a complete off-LAN transport

**Confirmed capability gap; new transport architecture required.**
[BLE scan](../core_lib/src/hdl/ble.rs) and
[passive monitor](../core_lib/src/hdl/ble_monitor.rs) emit `Sender<()>` alerts,
not independently connectable recipient endpoints. Fallback scan uses a 5-second
scan/25-second idle cycle and global 30-second alert cooldown. These constants
apply to that fallback path, not all Android discovery or all passive monitoring.
Changing them cannot create the missing route.

Inbound GATT/Weave exists, while
[outbound session connector](../core_lib/src/manager/session.rs) uses TCP and
outbound ConnectionRequest advertises WifiLan. `MigratableStream` has BLE and
TCP variants; an outbound BLE client, Bluetooth Classic route and interoperable
Hotspot/Wi-Fi Direct backend are not implemented here. Same-WLAN success is not
proof of no-common-LAN operation. Google's public connection manager supports
multiple media, with policy-controlled upgrades; its SDK is not a drop-in
implementation of the Quick Share application protocol.

First establish exact Android discovery/bootstrap requirements using authorized
redacted traces and public reference sources. Then design endpoint/transport
capabilities, cancellation and OS network ownership. Test both directions with
no shared AP, Wi-Fi enabled but unassociated, changed network and unavailable
Bluetooth. Preserve the approved manual Receive visibility window. Do not
silently toggle the user's Wi-Fi or claim every peer supports every medium.

## Compatibility matrix and acceptance definition

Source presence, a unit test, one physical run and general interoperability are
different evidence levels. No percentage or full-parity claim is justified yet.

| Area | Current evidence / missing work | Acceptance needed |
| --- | --- | --- |
| Same-LAN discovery and transfer | Implemented; physical Pixel transfers exist; QS-AUDIT-01 gaps | Cold/warm discovery, both directions, multiple NICs/VPN, disappear/reappear |
| BLE bootstrap and LAN upgrade | Inbound implemented; new wait fix not installed; QS-AUDIT-03/04 | Cold/retry text and file; successful upgrade and rejected/unavailable TCP |
| No shared LAN | Notification-only scan/outbound TCP limitation | Real Google chooser discovery and transfer both directions without common AP |
| Consent, reject, cancel, retry | Explicit consent; one LAN Cancel accepted, repeat/BLE gates open | Sender and receiver terminal states agree, immediate retry without restart |
| Text/URL/files/folders/multiple recipients | Existing source and scoped checks; not universal physical proof | Empty/large/multiple files, nested folders, Unicode, concurrent recipients, partial failure |
| Remote Copy | Outbound advertises use case; full peer capability behavior unproven | Google clipboard behavior, negotiated support and explicit local user action |
| Accounts / Contacts / Your devices | Anonymous authentication path; verified identity provider missing | Google certificates, account lifecycle/revocation, fail-closed consent policy |
| QR | Schema/roadmap exists; interoperable flow missing | Separate nearby Android QR from service-backed cross-platform QR |
| Advanced payloads | Stream/resume/hash negotiation, app split packages and Wi-Fi outbound/SAE incomplete | Version/capability-specific positive, unsupported and malformed cases |
| Previews | Supporting foundations do not establish wire/native presentation parity | Remains deferred under existing essential-feature scope |
| AirDrop | No accepted Linux transport/protocol backend | Separate Apple/device/OS/radio matrix below |
| Desktop platforms | Current native daemon/GTK packaging is Linux-specific | Windows/macOS adapters and independent acceptance; not implied by Core tests |

Both session constructors currently use anonymous peer authentication.
Google's certificate manager consumes an account, device registration and remote
certificate services. A local trust list cannot implement Google's Contacts or
Your devices. Preserve [approved verified-only identity A](native-pixel-acceptance-identity-locales-2026-10-23.md):
no name/IP/MAC grouping, silent recipient choice or account auto-accept without
authenticated session-bound proof. Provider, persistence and security changes
require a separate architectural decision. QR/account parity remain required
long-term goals; this audit does not relabel them as optional or implemented.

For each claimed supported combination record sender/receiver product/version,
OS/OEM, direction, visibility/account mode, radio topology, payload and source
SHA/installed binary. Measure chooser latency, click-to-session, UKEY2,
introduction-to-consent, accept-to-first-byte, payload and durable completion
separately. Use per-device monotonic durations; never subtract unsynchronized
phone/PC timestamps. Report sample count, cold/warm distributions and failures.
Unit/integration tests own scheduling, framing and cancellation invariants;
physical acceptance owns Google/OEM behavior and radio interoperability.

## AirDrop: feasible research, separate product implementation

Google currently documents direct AirDrop support on selected Android devices,
including Pixel 8a or later, with the Apple receiver in Everyone for 10 minutes.
The existing Pixel 6a is not listed for that direct path. Google's service-backed
QR alternative uses internet/cloud delivery; it must not be presented as native
local AirDrop or confused with nearby Android QR. [Google product help](https://support.google.com/pixelphone/answer/9286773?hl=en).

Apple documents BLE discovery, its peer-to-peer Wi-Fi link, TLS and separate
contact identity verification. [Apple security description](https://support.apple.com/en-au/guide/security/sec2261183f4/web)
is dated 2021, so use it as a baseline, not a complete specification for current
Apple versions. [Apple Wi-Fi Aware APIs](https://developer.apple.com/documentation/wifiaware)
do not by themselves establish native AirDrop wire compatibility.

The pinned Google Nearby tree now contains AWDL negotiation fields and an Apple
AWDL backend using `GNCNWFramework` with peer-to-peer enabled. That is useful
transport reference material, not evidence that importing Nearby yields a Linux
AirDrop implementation. Its AWDL stub and Apple adapter are platform-specific.

[OpenDrop](https://github.com/seemoo-lab/opendrop) is an experimental protocol
reference. Its documented limitations include missing BLE triggering, missing
Apple certificate/identity-record validation, automatic receive acceptance and
no multiple-file support. An unchanged wrapper cannot meet this project's
consent/security contract. Do not extract a user's Apple credentials to bypass
the identity design.

[OWL](https://github.com/seemoo-lab/owl), the AWDL reference, was archived on
2026-10-12 and says it is unmaintained. It requires active monitor mode with
frame injection and documents no concurrent AP connection and static channel
limitations. It cannot be assumed to work on a normal laptop adapter.

Read-only local inventory found Intel AX210/AX1675 (`8086:2725`); `iw list`
advertises monitor and P2P modes. This does not establish active-monitor ACK,
injection, channel scheduling, concurrent networking or current Apple acceptance.
No mode changes, packets, new dependency installations or radio tests were made.

### Architecture proposals for a later decision

Recommended boundary: separate Quick Share and AirDrop protocol adapters sharing
the existing consent, transfer lifecycle, safe storage and history domain.
Transport discovery candidates remain distinct from verified identities. Any
privileged radio worker needs a small authenticated IPC boundary, strict resource
limits and independent lifecycle. This is a proposal, not an approved refactor.

| Option | Advantages | Costs / risks |
| --- | --- | --- |
| A. Own maintained AirDrop adapter and isolated native radio backend, using references to prove protocol behavior | Explicit security/consent ownership, testable framing and lifetime, fits Rust domain | Largest engineering effort; current Apple and adapter interoperability still must be demonstrated |
| B. Maintain hardened forks of reference protocol/radio components behind a process boundary | Faster laboratory starting point; existing reverse-engineering evidence | Must replace automatic acceptance and incomplete validation, own archived radio code and additional runtime/dependency maintenance; not shippable unchanged |

Prefer A for production maintainability if hardware/protocol feasibility passes;
use reference implementations only as laboratory evidence before committing to
the backend. Neither option guarantees Contacts Only support. Start feasibility
with explicit Everyone-mode consent; preserve Contacts Only as separately
unproven scope. A companion application or custom Wi-Fi Aware exchange may be a
different product feature, but does not satisfy native AirDrop interoperability.

Feasibility exit gate: discovery and one consented file in both directions with
current iPhone/macOS; reject/cancel/no-auto-accept; no unknown identity marked
trusted; radio teardown restores networking; multi-file/bounds/path safety;
adapter/OS compatibility matrix. Account identity and privilege boundary require
owner choice before production implementation. No choice is needed to finish
this read-only research slice.

## Ordered implementation queue

1. Physically accept the already built `6ae62f6` candidate and record cold/retry
   Pixel text/file stages. Keep #23 open until actual proof; do not claim install.
2. Implement QS-AUDIT-02 ACK gating and QS-AUDIT-01 discovery scheduling/address
   fallback as separate documented, tested changes preserving existing contracts.
3. Correct QS-AUDIT-03 migration synchronization and QS-AUDIT-05 control-channel
   responsiveness. Design QS-AUDIT-04 peer ingress ownership before its security
   boundary changes. Keep #8's deferred identity proof explicit.
4. With stable connections, profile QS-AUDIT-05 disk/copy/progress costs and #24
   on the actual selected transport; retain explicit cancellation/durability.
5. Implement QS-AUDIT-06 after choosing the new transport/network ownership
   architecture. Complete QR/accounts and remaining negotiated payload parity
   in independent slices; no identity shortcuts.
6. AirDrop feasibility follows its separate adapter/radio gate, then an owner
   architecture decision and production implementation. Final locales/release
   gates retain their existing order. No master/tag/release authorization implied.

## Primary source provenance

Public Google Nearby `main` resolved to
`f3e0f907adef395fd20fd8554e103bcc7d1b8fd7` during this audit. It is a reference
implementation, not proof that a particular installed Google Play Services build
uses identical code. Immutable references:

- [Keepalive ACK handling](https://github.com/google/nearby/blob/f3e0f907adef395fd20fd8554e103bcc7d1b8fd7/connections/implementation/endpoint_manager.cc#L309)
  and [wire fields](https://github.com/google/nearby/blob/f3e0f907adef395fd20fd8554e103bcc7d1b8fd7/connections/implementation/proto/offline_wire_formats.proto).
- [BWU channel lifecycle](https://github.com/google/nearby/blob/f3e0f907adef395fd20fd8554e103bcc7d1b8fd7/connections/implementation/bwu_manager.cc),
  `RunUpgradeProtocol`, `ProcessLastWriteToPriorChannelEvent`, `ProcessSafeToClosePriorChannelEvent`.
- [Sharing medium policy](https://github.com/google/nearby/blob/f3e0f907adef395fd20fd8554e103bcc7d1b8fd7/sharing/nearby_connections_manager_impl.cc)
  and [certificate lifecycle](https://github.com/google/nearby/blob/f3e0f907adef395fd20fd8554e103bcc7d1b8fd7/sharing/certificates/nearby_share_certificate_manager_impl.cc).
- [Apple AWDL adapter](https://github.com/google/nearby/blob/f3e0f907adef395fd20fd8554e103bcc7d1b8fd7/internal/platform/implementation/apple/awdl.mm#L136)
  and [AWDL stub](https://github.com/google/nearby/blob/f3e0f907adef395fd20fd8554e103bcc7d1b8fd7/connections/implementation/mediums/awdl/awdl_stub.cc).
- [Nearby Connections SDK scope](https://developers.google.com/nearby/connections/overview)
  is not a Quick Share application-protocol compatibility guarantee.

## Validation and handoff

**Research/documentation scope COMPLETE; product compatibility PARTIAL.**
Validation on 2026-10-26:

- `git diff --check` exited 0.
- `bash scripts/ci-check-workflow-policy.sh` exited 0: dev fast-test and protected
  tag-only stress workflow contracts passed.
- The following Python check exited 0: **66 local Markdown file targets**, all
  original backlog/roadmap lines preserved in order, and only the three planned
  documentation files changed relative to baseline. It validates local file
  existence, not external URL availability or Markdown fragment anchors.

```sh
python3 - <<'PY'
from pathlib import Path
import re
import subprocess
from urllib.parse import unquote, urlsplit
paths = [Path('docs') / name for name in (
    'quickshare-airdrop-compatibility-audit-2026-10-26.md',
    'active-backlog.md', 'full-feature-roadmap.md')]
count = 0
for path in paths:
    for target in re.findall(r'\]\(([^\s)]+)\)', path.read_text()):
        url = urlsplit(target)
        if url.scheme or not url.path:
            continue
        assert (path.parent / unquote(url.path)).exists(), (path, target)
        count += 1
baseline = 'b19031a94332223d90d67556cd4960851f4882f1'
for path in paths[1:]:
    old = subprocess.check_output(['git', 'show', f'{baseline}:{path}']).decode().splitlines()
    new = iter(path.read_text().splitlines())
    assert all(any(line == candidate for candidate in new) for line in old), path
changed = subprocess.check_output(['git', 'diff', '--name-only', baseline]).decode().splitlines()
assert sorted(changed) == sorted(str(path) for path in paths), changed
print(f'PASS: {count} local Markdown file targets; original backlog/roadmap lines preserved; only 3 planned docs changed')
PY
```

Plan commit: `701b854` (`docs: plan full Quick Share and AirDrop compatibility audit`).
Final findings/backlog/roadmap form a second docs-only commit. Publication uses a
normal dev push after confirming expected remote `b19031a`; no force push or
master change. Rollback is an inverse docs-only commit, preserving prior journal
content and executable behavior. All owned shell commands completed; terminal
sessions closed. No watcher or background process was started.

Production source/tests, installed package and running services are unchanged by
this audit. No new runtime/STABLE/FLAKY suite,
physical AirDrop test or interoperability benchmark is claimed. Existing exact
source CI remains recorded in [the initialization fix evidence](inbound-bwu-initialization-2026-10-26.md).
All open functional issues remain open. This audit is the durable backlog for
QS-AUDIT-01 through QS-AUDIT-06; each implementation needs its own plan, regression
gate and applicable physical evidence before closure.

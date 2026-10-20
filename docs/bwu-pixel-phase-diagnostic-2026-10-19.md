# Pixel BWU protocol phase trace — source diagnostic (Option A)

Date: 2026-10-19. Mode: **MODE A / diagnostic instrumentation**.
Decision source: [P0 #8 Option A](p0-bwu-authentication-decision-2026-10-19.md).
Branch: `dev`. **Option A's authenticated pre-commit TCP proof gate remains NOT PASSED.** This diagnostic does not change BWU admission, cryptographic state, device configuration, installation, protocol bytes, APIs or fallback behavior.

## Repository scan and rationale

- Existing `core_lib/src/receiver_trace.rs` prints allowlisted, monotonic events on stderr **only** if `RQUICKSHARE_RECEIVER_TRACE=1`. It is already used in native Core and has an isolated output-shape test.
- `core_lib/src/manager.rs::route_bandwidth_upgrade_if_pending` classifies the TCP `CLIENT_INTRODUCTION` before `BwuRouter::route_with_alias` consumes its route.
- `core_lib/src/hdl/inbound.rs::InboundRequest::do_bandwidth_upgrade` is the real BLE→TCP receiver flow: announces upgrade path over secure BLE, receives a socket, reads plaintext introduction, replies with plaintext ACK, drains encrypted old BLE and replaces the transport.
- Native systemd unit is `rquickshare-daemon.service` with `ExecStart=/usr/libexec/rquickshare-daemon`. The existing `scripts/receiver-ab-smoke.sh` targets legacy Tauri and must not be reused as a native process launcher.
- The remote ChatGPT/GitHub environment cannot access the user's KDE session, USB ADB, radio or Pixel. No physical-device result may be inferred from source tests.

## Frozen non-invasive instrumentation contract

Extend the existing `receiver_trace::Event` enum with fixed BWU phase names; reuse the existing opt-in environment flag and `rqs-receiver event=<fixed> elapsed_ms=<monotonic>` format. **Never print peer IDs, aliases, MAC/IP, negotiated ports, filenames, paths, content, keys, crypto state bytes, filenames, tokens, or raw frame lengths.**

New named phases:
`bwu_offer_sent`, `bwu_tcp_classified`, `bwu_tcp_unmatched`, `bwu_route_consumed`, `bwu_tcp_received`, `bwu_tcp_timeout`, `bwu_intro_validated`, `bwu_ack_sent`, `bwu_prior_drain_started`, `bwu_prior_safe_to_close`, `bwu_tcp_switched`.

Semantics:
- `bwu_tcp_classified` means a syntactically recognized plaintext introduction; **NOT authenticated**.
- `bwu_route_consumed` means the router delivered a socket; **NOT authenticated**.
- `bwu_intro_validated` means string identity matched; **NOT cryptographic proof**.
- `bwu_prior_safe_to_close` means an encrypted acknowledgement on the **old channel**, not proof for the **new TCP socket**.
- `bwu_tcp_switched` means actual transport mutation, not a claim about security.
- `bwu_tcp_unmatched` may be emitted after ordinary hostile/unexpected TCP ingress; static text only.
- Trace events use a process-local monotonic anchor. With multiple concurrent sessions there is no attempt to correlate private IDs; results become inconclusive unless one isolated trial and a single active peer.

Instrumentation MUST be observational and synchronous, with no new sleeps, retries, conditional protocol behavior, network capture or extra locks. Existing `RQUICKSHARE_RECEIVER_TRACE=0` and unset flag produce no new event lines.

## Plan, safety and rollback

1. [x] D0 scan agent SSOT, native unit, existing trace logger, TCP classifier, BWU handshake and legacy tools. Commit this plan **separately from code**.
2. [x] D1 extended the existing trace `Event` and output-shape test; instrumented route classifier and inbound BWU phase transitions in source commits `e2531eda`, `69547f93`, `9c8c6577`. No authenticated-admission behavior changed.
3. [x] D2 static event/call-site audit performed (11 fixed variants and 11 call sites) with GitHub read-back. Exact code SHA `c4bd463b` completed green [Preflight 39333092113](https://github.com/morozhenko-apps/rquickshare/actions/runs/39333092113) and [Native GTK 39333092130](https://github.com/morozhenko-apps/rquickshare/actions/runs/39333092130). This is source CI, not physical Pixel acceptance.
4. [ ] D3 **physical** Ubuntu native build/version check, clean single Pixel send, owner confirmation, trace collection, and evaluate protocol proof gate. A source-only green build never completes D3.

Rollback: revert additive `receiver_trace` variants/call sites only. The event names are diagnostic, not a product wire contract. Never execute host service restart, installation, Bluetooth changes or ADB commands through a remote GitHub tool.

## Scoped regression inventory before implementation

| Artifact / branch | Evidence/test | Category |
|---|---|---|
| `receiver_trace::Event::name/line/enabled`: each new enum variant | Existing table-driven opt-in/shape test extends to all new variants; fixed 3-field lines | Positive, N1, N10 |
| `route_bandwidth_upgrade_if_pending`: parsed current/alias identity / route accepted / unmatched / ordinary path | Review exact trace insertion vs existing real-socket tests; no route mutation | Positive, N5, N7 |
| `InboundRequest::do_bandwidth_upgrade`: offer / accepted TCP / timeout / identity match / ACK / drain / safe / final transport switch | Review each branch and existing protocol tests; hardware necessary for actual negotiated sequence | Positive, N3, N4, N5, N7 |
| Diagnostic flag disabled and no PII in static phase lines | Existing logger test plus explicit scrutiny of all event call sites | N1, N10 |
| Actual Pixel compatible cryptographic pre-commit proof | **OPEN hardware/protocol gate**, never mark PASS based on event order alone | N3, N6, N7, N10 |

N2=not applicable (no user-sized diagnostic payload); N8=monotonic elapsed only, no wall clock; N9=existing shutdown semantics retained, hardware cancel remains OPEN; N11=not applicable (no entitlement); N12=journal storage controlled by host, tracing does not create files. Interaction review: trace flag {unset,0,1} × BWU path {ordinary, matched, unmatched, timeout, successful} × concurrent owners {single, simultaneous=uncorrelated}. A mutation that includes any variable untrusted string in the logger must fail source review; changing routing must fail existing socket-owner tests. No global Mode B completion claim.

## Local physical procedure AFTER the matching candidate is installed

1. Ensure no active transfers. Confirm executable and package ownership on the **user's host**, not here: `dpkg-query -W rquickshare-native rquickshare` and `systemctl --user cat rquickshare-daemon.service`. If the currently installed daemon predates the instrumentation, its trace is unavailable; do not infer it was recorded.
2. Enable future daemon processes only: `systemctl --user set-environment RQUICKSHARE_RECEIVER_TRACE=1`. **A currently running daemon does not inherit this retroactively.** Close the native tray/daemon gracefully with no active transfers and start the instrumented daemon using the normal supported activation flow; do not kill unknown owners or restart during transfer.
3. In a separate terminal, note a trial-start timestamp and follow **allowlisted** output (never persist a complete, possibly sensitive journal): `journalctl --user -u rquickshare-daemon.service -f -o cat --no-pager | grep --line-buffered -E 'rqs-receiver event=(bwu_|start)' `. Record only the allowlisted fixed event lines. Stop just the *monitor* with Ctrl+C after the one disposable Pixel image/file transfer; do **not** Ctrl+C the app.
4. For the single Pixel trial, enable normal visibility, choose a one-off harmless file (no private content), explicitly verify device/code/consent and transfer result. Record separately whether the Android display indicated successful transfer and whether the Linux receiver ended with `bwu_tcp_switched`.
5. Disable future trace flag: `systemctl --user unset-environment RQUICKSHARE_RECEIVER_TRACE`. Verify with `systemctl --user show-environment | grep '^RQUICKSHARE_RECEIVER_TRACE=' || true`.
6. Share only the filtered fixed-name event lines and a short manual outcome summary; do not upload raw journal, tcpdump/pcap, logcat, keys or personal payloads by default.

The trace distinguishes actual phases and unintended stalls, but **cannot establish cryptographic proof of TCP socket origin**. If no supported pre-commit proof is seen or its existence independently established, Option A remains blocked and #8 open. Any further wire-capture authorization requires separate, minimally scoped safe instrumentation.

## Evidence

Documentation plan committed as `aa3db04d` before code. Source commits:
- `e2531eda` extended `receiver_trace.rs` with 11 fixed phase names and the table-driven output-shape fixture.
- `69547f93` added 3 trace calls at TCP recognition/routing decision in `manager.rs`.
- `9c8c6577` added 8 trace calls at BLE→TCP receiver upgrade phases in `inbound.rs`.

After source changes, the live GitHub connector fetched all three exact `dev` file blobs and confirmed 11 names, 11 test enum cases and 11 instrumentation sites. This is **static source inspection, not compiler/test execution**.

GitHub Actions initially observed at post-code SHA `9c8c6577`:
- [Preflight 39326407110](https://github.com/morozhenko-apps/rquickshare/actions/runs/39326407110): **PENDING** at observation; no pass/fail claim.
- [Native GTK 39326407154](https://github.com/morozhenko-apps/rquickshare/actions/runs/39326407154): **SUCCESS**, confirmed on post-code SHA `9c8c6577` at later observation; this does not substitute for Core Preflight or Pixel hardware.

Previous intermediate source pushes triggered runs that were cancelled due to the existing concurrency policy. Do not interpret them as failures or passes for the final source. The last verified green source prior to instrumentation was `4f5705f4` and does not verify these calls.

2026-10-19 exact source CI findings:
- [Preflight 39326407110](https://github.com/morozhenko-apps/rquickshare/actions/runs/39326407110) on `9c8c6577`: **FAILURE solely in Rust format**. GitHub job `118313729923` showed rustfmt requested only a single-line rewrite of `BwuPriorSafeToClose` trace call in `core_lib/src/hdl/inbound.rs`. Other 4 Preflight jobs, including Core Rust tests, strict Core Clippy, Tauri Clippy and Frontend, were **SUCCESS**. This is a real red CI for that SHA, not a green gate.
- Formatting-only source commit `c4bd463b` corrects exactly that expression; no trace semantics, domain routing, BLE/TCP, protocol bytes, tests or dependencies changed. **Exact corrected-SHA CI outcome not yet verified** at this documentation update. Do not mark whole Preflight PASS based on previous source or a local build.
- User-provided Ubuntu 26.04 KDE build transcript: missing GTK/Graphene dev dependencies installed; `python3 scripts/native_deb.py build --profile dev` reports `Native Debian package verified: .../rquickshare-native_1.0.0-1.debug_amd64.deb`. This is a successfully staged/verified **local uninstalled** artifact; it was built before source formatting commit `c4bd463b`, so its exact commit provenance must be recorded before hardware acceptance. No proof of installed version, running daemon, Pixel BWU, peer identity, authentication or actual transfer is inferred from the build alone.

Subsequent 2026-10-19 checks:
- GitHub read-back of workflow status for **code SHA `c4bd463b`** confirms both Preflight and Native GTK `completed/success`. Documentation-only head `4be1cb7` was a descendant of the tested source.
- User's host supplied `git pull --ff-only`, yielding documentation head `4be1cb7` with no local divergence reported. Existing locally staged development package hash: `1167522dac95a74723ccb8aa40192c7e28354576aa054698ffcce5a6731bfcf4`; this binary predates the trivial source rustfmt correction, so it is **not** the final exact-source candidate for device testing.
- Installed package is `rquickshare 0.11.5+native30` and both `rquickshare-daemon.service` and `rquickshare-desktop.service` are **active**. `sudo apt-get -s install <staged debug package>` resolved dependencies and would remove `rquickshare` and install `rquickshare-native 1.0.0-1.debug` (0 other upgrades), but performed no mutation. An active service is NOT evidence of zero active or prepared transfers.
- **Before any installation**: locate and validate an independent exact `0.11.5+native30` rollback .deb (source doc `native30-card-history-polish-2026-10-17.md` records historical native30 checksum `db1a44ae9548985a656f1a9139777117b8996105b01643a35b0fce8abc81d0a0`, not proof that the identical file survives locally); verify idle session and staged sources, stop current services cleanly, build the corrected-source diagnostic package in a unique directory, then simulate that actual replacement .deb.
- No `apt install`, `systemctl stop/restart`, ADB, Google peer send, provenance check of the installed native30 bytes or real BWU event log has yet been executed in this diagnostic step.

## Physical installation gate — owner-host evidence 2026-10-20

User-provided shell output after `git pull --ff-only` reports successful `python3 scripts/native_deb.py build --profile dev --output native_desktop/target/packages/bwu-pixel-final` and successful `python3 scripts/native_deb.py verify` for:
- **Diagnostic candidate** `native_desktop/target/packages/bwu-pixel-final/rquickshare-native_1.0.0-1.debug_amd64.deb`, SHA-256 `c9d75a5b7becadacc02d5152f1e589d484258a6011ad7e1afe03dc003ab9b627`.
- **Verified native30 rollback** `native_desktop/target/packages/rquickshare_0.11.5+native30_amd64.deb`, metadata Package=`rquickshare`, Version=`0.11.5+native30`, Architecture=`amd64`, SHA-256 `db1a44ae9548985a656f1a9139777117b8996105b01643a35b0fce8abc81d0a0`, matching the historical recorded package.
- Owner-host `apt-get -s install` plan: remove `rquickshare 0.11.5+native30`; install/configure `rquickshare-native 1.0.0-1.debug`; **no additional package upgrades**. This is a preview, not a service-idle check or a completed installation.
- GitHub exact source code SHA `c4bd463b` passed both 2026-10-19 CI gates. The later commits are documentation-only. Candidate was rebuilt after the code correction, with the reported path+checksum above. Explicit device test availability confirmed by the user.

**Physical test runbook**: verify these two exact SHA-256 values again; user confirms zero in-flight transfers and no staged selections; stop only `rquickshare-desktop.service` and `rquickshare-daemon.service` cleanly; set opt-in trace environment, install exactly the candidate with system APT; daemon-reload/start the user desktop unit and verify active daemon, client and installed package identity. Collect only fixed allowlisted trace event names and monotonic timestamps from a single disposable Pixel→Ubuntu transfer; user confirms code/consent and receipt separately; unset manager trace flag. Preserve rollback package unchanged, and use ordinary APT replacement after an idle gate if recovery is necessary. Any host commands are owner-executed; GitHub connector cannot manipulate systemd, APT, Pixel or local journal. P0 #8 crypto pre-commit proof remains unproven.

## Owner-host installation confirmed, event startup only — 2026-10-20

User-provided executed shell confirms: `sha256sum -c` PASS for exact diagnostic candidate `c9d75a5b7becadacc02d5152f1e589d484258a6011ad7e1afe03dc003ab9b627`, owner reported no in-flight or staged sends; clean `systemctl --user stop`, `RQUICKSHARE_RECEIVER_TRACE=1` manager environment, APT removed `rquickshare 0.11.5+native30` and installed `rquickshare-native 1.0.0-1.debug`, user daemon reload and desktop restart. Both user units reported `active`; expected installed version reported. APT printed `_apt` local-path unsandboxed-root warning; installation itself exited successfully.

Filtered `journalctl --user -u rquickshare-daemon.service` displayed **`rqs-receiver event=start elapsed_ms=0`** once, confirming trace-enabled daemon startup. **No `bwu_*` events or Pixel transfer result were reported**. This is not an interoperability failure: the user captured the journal but did not report executing a Pixel→Linux transfer. Requested next step: with the installed daemon still running, follow the fixed events in a terminal, carry out exactly one consented disposable Pixel→Linux transfer, and report both device outcomes plus filtered events. Do not record any PASS/FAIL hardware or BWU origin proof until there is real transfer evidence.

Source check: `scripts/native_deb.py` builds `core_lib/rquickshare-daemon` without `--no-default-features`; `core_lib/Cargo.toml` default features include `experimental`; `core_lib/src/hdl/gatt.rs` enables BWU and calls the guarded `do_bandwidth_upgrade` worker. Therefore this package build is not statically disabling BWU. A successful ordinary Wi-Fi-LAN transfer may still produce **zero BWU events** if there was no BLE upgrade negotiation; that must be distinguished from a failed protocol path.

The preceding sections' installation/absence-of-BWU observations are historical checkpoints. Subsequent physical tracing is recorded below; `Option A` cryptographic pre-commit socket authentication remains **UNPROVEN**, and P0 #8 remains OPEN.

## Actual Pixel BLE→Wi-Fi LAN BWU phase trace — 2026-10-20

The user supplied fixed-phase `journalctl --user -u rquickshare-daemon.service -n 0 -f -o cat` output from the installed `rquickshare-native 1.0.0-1.debug` diagnostic candidate. This **is physical-device evidence of the transport switch**, unlike the prior `start`-only read. User said these events appeared during the subsequent attempt; Android post-send success and independent received-file SHA-256 are **not yet reported**.

| Monotonic elapsed_ms | Sanitized event | Delta since offer |
|---:|---|---:|
| 639623 | `bwu_offer_sent` | 0 ms |
| 639827 | `bwu_tcp_classified` | 204 ms |
| 639827 | `bwu_route_consumed` | 204 ms |
| 639827 | `bwu_tcp_received` | 204 ms |
| 639827 | `bwu_intro_validated` | 204 ms |
| 639827 | `bwu_ack_sent` | 204 ms |
| 639827 | `bwu_prior_drain_started` | 204 ms |
| 639984 | `bwu_prior_safe_to_close` | 361 ms |
| 640184 | `bwu_tcp_switched` | 561 ms |

Interpretation: the local BLE receiver sent a BWU offer and handed a TCP socket to the session, accepted a syntactically/identity-matching *plaintext* `CLIENT_INTRODUCTION`, received safe-to-close acknowledgement over the prior authenticated channel, then **mutated** the local transport to TCP. This is a concrete interoperable BLE→TCP migration sequence and a 561 ms offer-to-swap trace from one session. It does **not** prove that the TCP peer possessed a cryptographic secret before `bwu_route_consumed`; on the contrary, current implementation consumes the route based on untrusted introduction IDs. The static traces cannot distinguish a legitimate initiator from an attacker spoofing the introduction.

Pending: separate user report of actual file receipt and sender-side completion, independent hash if needed; architectural proof of Google-compatible authenticated *pre-commit* new-socket admission, or an explicit new user decision when Option A is infeasible. Retain exact package/rollback hashes and the saved fixed-phase-only trace, not full journal/endpoint identifiers. No protocol/security source change made based on this trace.

Related **deferred UI request**, separately tracked as [#13](https://github.com/morozhenko-apps/rquickshare/issues/13): Devices attention pulse for collapsed incoming requests, focus actual sender when opened/already expanded, move awaiting-consent count from Receive to Devices. Do not implement in this BWU diagnostic/security slice.


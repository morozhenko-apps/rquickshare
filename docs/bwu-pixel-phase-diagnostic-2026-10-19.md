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
2. [ ] D1 extend existing trace `Event` and its output-shape test; instrument route classifier and current inbound BWU phase transitions, keeping sensitive fields out.
3. [ ] D2 final static inspection, test execution on exact source if available, independently record CI status, then document evidence in a separate docs-only commit.
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

Planning/scan only at initial creation; execution evidence added only after CI or hardware actually runs.

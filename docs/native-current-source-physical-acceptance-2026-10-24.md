## 2026-10-24 second reboot test: Decline 6/6, cold discovery still slow, misleading premetadata row

**Fresh operator report:** after manually stopping/restarting native user services and enabling the opt-in fixed-event trace before starting the process, **three more Pixel→KDE Declines correctly displayed `Declined`**. Total latest result **6/6 successful across two distinct daemon sessions**. Second session stage pairs `reject_last_chunk_flushed → reject_peer_eof`: (70139→70539) **400ms**, (91284→91679) **395ms**, (110484→110933) **449ms**. All three local REJECT payloads and LAST_CHUNKs flushed; no write failures or 2-second deadlines recorded. The user disabled the tracing environment afterward. This does not prove reliability or causality in the face of three earlier Failed observations. [Issue #18](https://github.com/morozhenko-apps/rquickshare/issues/18) remains OPEN, including **slow first Pixel discovery**.

**New screenshot:** compact GTK displays desktop icon, `Transfer · 0 files · Cancel` before payload metadata has been received; nearby availability countdown shows `Visible to nearby devices · 0:12`. Countdown is remaining manual visibility time, **not discovery elapsed**. Source root cause: `TransferDetails::default` means **Unavailable** and initializes `kind=Files, files=[]`, while `panel/rows.rs::payload_label` rendered that placeholder as `0 files`. [Issue #22](https://github.com/morozhenko-apps/rquickshare/issues/22) opened; scoped GTK-only code [`7dc44071`](https://github.com/morozhenko-apps/rquickshare/commit/7dc4407173aa0fcdb86d00b5adc1c7e634265518) now renders `Connecting…` and `Waiting for details` instead; preserves genuine cancellable session. [Design, alternatives, N1–N12 and coverage inventory](native-pending-transfer-metadata-ui-2026-10-24.md). **Source CI and new installed package acceptance not yet completed at writing**; do not close #22 based on original screenshot or source change alone.

**Exact next diagnosis for initial discovery:** existing daemon `RQUICKSHARE_RECEIVER_TRACE=1` contains `runtime_start_requested`, `runtime_core_ready`, `runtime_discovery_worker_ready`, `mdns_browse_started`, `first_mdns_route_admitted`, `visible`, `advertisement_started`, `gatt_slot0_read`, `gatt_header_ready`. Reproduce on *one fresh daemon* and share ONLY these allowlisted elapsed-time markers plus separately observed time from manually opening Pixel Quick Share to first KDE listing on Pixel. The earlier user `grep reject_*` **intentionally filtered these markers out**. Avoid premature Bluetooth/SDK retry/timer changes without knowing which of startup, advertising, SDK browse, TCP probe, Pixel scan or GTK projection actually stalls. No master, release tag, APT publish or R2 changes.


## 2026-10-24 installed diagnostic: Pixel Declined 3/3; correlated frame-stage evidence

The operator supplied a full local build-and-reinstall transcript and filtered, privacy-safe daemon trace (support artifact **Pasted text(10).txt**, October 24). Build from **dev `29ce6d3db1e1df037da9dbdbccf7613505236a97`**, containing executable trace changes `afce867644baab8e4f5e9ffa66e70b9a27b09ff9` and docs-only follow-ups. The operator's preflight script checked **all 6** source-exact GitHub check-runs SUCCESS (independently verified now: [Preflight 40115424146](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424146), [Native GTK 40115424114](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424114)). Core stable: **796 passed, 0 failed, 8 filtered**, 4/4 and 4/4 integration; GTK SUCCESS. Local build/embedded `.deb` verifier and independently checked package SHA-256 **OK**. Original rollback `0.9.0-1` SHA **OK**. `apt install --reinstall` completed with 0 removals; version `0.9.0-1.debug`; **all three installed ELF executables `cmp` MATCH** extracted local diagnostic `.deb`. The `_apt` permission warning is nonfatal.

**Owner explicitly reports all three Pixel→KDE→Decline attempts now display `Declined` on Pixel (3/3 PASS).** The earlier build with essentially the same wire/drain logic had **3/3 Failed**, so this is **not enough to claim permanent/causal repair**. The new build's code is principally fixed-stage instrumentation and enhanced test proof, **not a protocol schema or timeout change**. Do not credit tracing itself as a valid protocol fix or claim GMS acknowledged a frame.

Filtered `RQUICKSHARE_RECEIVER_TRACE=1` output reported for each observed consent:
| Attempt | reject_consent | reject_write_start | reject_payload_flushed | reject_last_chunk_flushed | reject_peer_eof | Last chunk → EOF |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 34383 ms | 34383 | 34384 | 34385 | 34792 | **407 ms** |
| 2 | 135685 ms | 135685 | 135686 | 135686 | 136130 | **444 ms** |
| 3 | 140736 ms | 140736 | 140737 | 140738 | 141213 | **475 ms** |

There are **no** `reject_write_failed`, `reject_peer_read_error` or `reject_drain_deadline` markers in these 3 filtered attempts. The daemon locally flushed **both** encrypted Sharing Response payload and LAST_CHUNK. The peer then signaled EOF approximately 0.4–0.5 s later. Sender peer EOF **does not independently prove Google Android decrypted/accepted either message**; the UI's `Declined` is separate operator observation. The operator ran `systemctl --user unset-environment RQUICKSHARE_RECEIVER_TRACE` after collecting diagnostics; **no permanent debugging environment is requested**. Settings, files, history and protocol unchanged.

**Disposition:** keep [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) OPEN for intermittent reproducibility and cold/warm discovery measurement. With 3 prior failures and 3 latest successes, a high-confidence reliability/causal gate needs repeat cold-start tests across daemon restarts and ideally a matched FAIL trace (if it recurs); check especially partial/absent `reject_last_chunk_flushed`, unexpected `reject_drain_deadline` vs peer EOF and BWU route. Avoid adding arbitrary delays, changing Reject to Cancel or attributing unproved failure to Pixel UI. Normal Core 796/796 + GTK check gate is green, but **no release-wide 10x Mode B / 8 filtered races** yet. No master/tag/APT/R2 operations.

## 2026-10-24 SECOND PHYSICAL FAILURE (3 of 3) — protocol proof investigation

Operator response after installing previous two-second-drain candidate: **all three Pixel→KDE→Decline attempts still showed `Failed` on Pixel**. The previous delay hypothesis **did not resolve observed behavior**. KDE→Pixel→Decline already PASS; manual-only Receiver and compact repeat/history remain accepted. #20 closed with user acceptance; #18 remains OPEN.

Current priority: prove the in-flight encrypted Sharing `Reject` **both chunks** are flushed, and classify how the connection ends, not increase delay or change status arbitrarily. [Detailed evidence, ranked hypotheses, branch map and test matrix](native-pixel-reject-wire-trace-2026-10-24.md). **Opt-in evidence source** `43d9e3ab4abd730ae58f0171aed857c58af70b97` changes `core_lib/src/receiver_trace.rs` and `core_lib/src/hdl/inbound.rs` only: existing `RQUICKSHARE_RECEIVER_TRACE=1` allowlisted monotonic event names (`reject_consent`, `reject_write_start`, `reject_payload_flushed`, `reject_last_chunk_flushed`, `reject_write_failed`, `reject_peer_eof`, `reject_peer_read_error`, `reject_drain_deadline`). Adds deterministic real encrypted **second-chunk** sequence/id/size/flag confirmation, and enum-valued bounded EOF/deadline drain tests. No wire schema, peer trust, packet bytes, delay or storage change. First source CI failed only one nightly formatting line; follow-up **test-only** `17de08b0524d81ca980e22640499e4e07c18f30a`. Exact-source terminal CI NOT YET VERIFIED at this checkpoint; do not direct user to rebuild/install until final gate. No trace has been collected from the Pixel yet, so no root-cause claim.

After six terminal checks PASS, rebuild one new local debug .deb from dev and verify source package hashes and installed binaries against rollback, then run **one isolated** Pixel→KDE→Decline with opt-in env. Extract **only** `rqs-receiver event=[a-z_]+ elapsed_ms=[0-9]+` and omit full journal; unset diagnostic environment on completion. Use stage gaps and EOF/deadline to establish failure, then design actual architectural correction. No release promotion.

## FINAL source-exact CI for bounded inbound Reject drain — 2026-10-24

**Executable source `3a35097f5c0afe3140d2b52f81ebc79677faf1a9`: ALL SIX check-runs terminal SUCCESS.** [Preflight 40111072020](https://github.com/morozhenko-apps/rquickshare/actions/runs/40111072020) and [Native GTK 40111071885](https://github.com/morozhenko-apps/rquickshare/actions/runs/40111071885) were inspected independently. Core stable **795 passed, 0 failed, 0 ignored, 8 filtered**, plus 4/4 and 4/4 integration tests; both newly added rejection tests logged `ok`: real encrypted Sharing REJECT delivered/read and peer EOF before transport retirement; silent/chatty drain bounded to 2s. Rust fmt, Core Clippy, Tauri Clippy+desktop tests, frontend and GTK all **SUCCESS**. The prior source `57739042` had only one nightly rustfmt test layout failure, corrected in `3a35097f` without production changes; earlier cancelled jobs are not the final CI status.

**Still NOT a Pixel fix validation:** none of this source has been built/installed/tested on the operator's Pixel/KDE. Next build fresh current `dev` after this 6/6 CI, verify SHA and rollback, reinstall only after transfers idle / tray Quit and byte compare all three extracted binaries. Physical Pixel→KDE Decline needs a new repeat test to see whether Pixel shows `Declined` consistently; also ensure KDE→Pixel Decline, manual Receive and successful sending do not regress. Do not close #18 until actual acceptance; cold discovery remains separately unmeasured. The full final release-wide Mode B (including 8 filtered race-sensitive tests and hardware scope) has not been claimed. No master/tag/R2 changes.

## 2026-10-24 operator bilateral decline feedback and next diagnostic build

User tested latest byte-verified installed package `rquickshare-candidate-e67134a`: **PASS** KDE→Pixel explicit Decline displays KDE `Declined`, and **FAIL** Pixel→KDE explicit Decline displays Pixel `Failed` again. User reports **all other previously requested physical tests work** (manual Receive only, 60s expiry/restart, clean repeat transfer/History). #20 now closed after source CI+physical acceptance; #21 startup acceptance passed, further live-settings reapply invariant open.

Root source: inbound explicit `Status::Reject` sends encrypted `Response` and immediately terminates worker, dropping BLE/TCP transport; Google's published service uses a delayed close after its REJECT. Scoped `dev` source `57739042` now sends same encrypted Reply then retains connection for bounded read/remote EOF of up to 2 seconds; no new transport frame, accepted transfer delay or peer identity change. Two Tokio regressions: real encrypted Reject observed by peer before close; terminal socket drains late bytes until 2s deadline or EOF. Nightly formatter required `3a35097f`. At checkpoint new exact-source CI pending; **not installed**, do not claim Pixel result. [Plan](native-pixel-decline-drain-2026-10-24.md). Preserve verified `0.9.0-1` original rollback .deb; no master/tag/R2.

## 2026-10-24 latest physical installation gate — verified by operator

The operator ran the full guarded replacement procedure and supplied **terminal success evidence**. The operator's installed binary came from **local candidate folder `rquickshare-candidate-e67134a`**, built from a documentation-only head after code fix `a3976ad7`. The later `ded1f1d2` commit changed only two managed BLE/TCP **test expectations**, not executable production paths; its **exact-source Preflight [40109101697](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101697) and GTK [40109101791](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101791) are now 6/6 completed SUCCESS** (independently verified and gated by the operator's successful `PASSED == 6` check). No false claim that the installed binary was built from a later test-only SHA is needed.

Observed `sha256sum -c` for both the original rollback `0.9.0-1` and new candidate reported `OK`. The package manager reinstalled **`rquickshare-native 0.9.0-1.debug amd64`** successfully with **0 removals**. `dpkg-deb -x` plus `cmp` against the new package produced **MATCH** for all three paths: `usr/bin/rquickshare`, `usr/libexec/rquickshare-daemon`, `usr/libexec/rquickshare-preview-helper`. Therefore the currently installed program files are exactly the packaged candidate, not stale leftovers. The nonfatal `_apt` unsandboxed-root local permissions notice is not a package failure.

**Physical behaviors after installation are NOT YET TESTED / REPORTED.** Stage A now: launch native app, assert no active Receive timer and Pixel cannot freshly discover KDE while Hidden (allow for Android cache), activate Receive manually, verify advertised 60-second timer and automatic expiry, close+reopen to ensure no temporary reset/auto-advertise. Stage B: KDE→Pixel send then Pixel explicitly Decline, expect live KDE `Declined` and History; Pixel→KDE inbound one successful transfer, then send same attachment again and verify completed prior row no longer shares compact card with new request, but History and saved file(s) remain. Separately verify normal Linux discovery of Pixel while Linux inbound visibility Hidden. No device pass or issue closure until operator returns observations. Original rollback installer remains present; do not delete. No master/tags/R2/published release.

# Native current-dev installed KDE/Pixel acceptance — 2026-10-24

**Owner request:** test and finish available Native sharing/discovery issues after #11 and #9 source CI. **Primary mode B (testing)**. This is an operator-run physical acceptance procedure, not an action performed by the GitHub-connected agent. Canonical live task index: [active-backlog.md](active-backlog.md). Historical broad matrix: [native-essential-acceptance.md](native-essential-acceptance.md); [hardware automation](hardware-test-automation.md).

## 1. Immutable baseline and precise limitations

Verified executable source (latest #11 + #9 code/test): `2545e0519bb8f236e1948993eef28b31d2e58ac0`. Only documentation followed on `dev`, initially at `3af116e3bbcbb4278e70ca457f35860385d2a377`; verify **actual fresh `dev` SHA and `git diff`** before building. Both [Preflight 40093504811](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504811) and [Native GTK 40093504760](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504760) concluded SUCCESS: Core 790 stable, 0 failed/ignored, 8 filtered for separate race-sensitive partition; integration 4+4; both Clippy, format, frontend and GTK all success. Green source CI does not verify live BLE, mDNS, Pixel, new package or local dependency installation.

Owner KDE machine: Ubuntu 26.04 KDE/Wayland, user-reported installed `rquickshare-native 0.9.0-1`; Pixel 6a; Windows peer unavailable unless provided later. Do not make claims about an actual fresh install or peer test until user supplies version and observed output. The Github connector cannot execute on the owner's local OS, authorize ADB, operate Quick Share, install packages or enter credentials. No autonomous remote terminal is present in this session.

**Build a diagnostic version** `0.9.0-1.debug` via the already implemented `scripts/native_deb.py build --profile dev`, **not** the identical-version release-like `0.9.0-1` from manual native-package workflow, and do not fake a release tag or stage to R2. Desktop SemVer `0.9.0`, package revision `1`, profile suffix `.debug` are read from actual native Cargo/release metadata. Debug variant is a real local package, not a published artifact; it will replace (rather than coinstall with) `rquickshare-native` if the operator explicitly installs it. Saving the old `0.9.0-1` package or independently verifiable rollback candidate before replacing it is mandatory. Do not unconditionally remove users' settings, install alongside historical `rquickshare`, or change their default audio/network settings.

## 2. Operator staging (no installation side effects)

Run from the already cloned repository on Alcotester. Preserve uncommitted work: abort rather than forcing a checkout or reset. Do not source an untrusted remote script, change tags, or kill any process owned by another agent.

```bash
(
  set -euo pipefail
  cd "$HOME/Documents/GitHub/rquickshare"
  test -z "$(git status --porcelain)" || {
    echo "STOP: dirty worktree, do not overwrite local work"; exit 1;
  }
  git switch dev
  git pull --ff-only origin dev
  echo "Source SHA: $(git rev-parse HEAD)"
  dpkg-query -W -f='Installed: ${Package} ${Version} ${Architecture}\n' rquickshare-native || true
  echo 'Current package rollback candidates:'
  find "$HOME/Downloads" -maxdepth 4 -type f -name 'rquickshare-native_0.9.0-1_amd64.deb' -print
  out="$HOME/Downloads/rquickshare-candidate-$(git rev-parse --short HEAD)"
  python3 scripts/native_deb.py build --profile dev --output "$out"
  (cd "$out" && sha256sum -c "rquickshare-native_0.9.0-1.debug_amd64.deb.sha256")
  python3 scripts/native_deb.py verify "$out/rquickshare-native_0.9.0-1.debug_amd64.deb"
  dpkg-deb --field "$out/rquickshare-native_0.9.0-1.debug_amd64.deb" Package Version Architecture
  echo "CANDIDATE=$out/rquickshare-native_0.9.0-1.debug_amd64.deb"
)
```

This creates local output under `Downloads` only, compiles using existing pinned Rust crates and runs strict package validation without starting or installing any user daemon. If dependencies or package shlibs metadata fail, preserve the first original build error, do not apply ad-hoc `--force-depends` or disable Bubblewrap/GTK checks. The user has not yet run this command in this session.

**Rollback prerequisite:** identify an intact original `rquickshare-native_0.9.0-1_amd64.deb` before any installation. The plain package name/version appearing in `dpkg-query` is **not** a recoverable installer. If the old package is missing, preserve the installed version and recover/verify the older binary artifact first. The debug package has a higher dpkg version, so rollback to `0.9.0-1` requires a deliberate downgrade, not an implicit `apt upgrade`.

Once a candidate and rollback installer are verified and the operator chooses to install: use normal GUI/tray **Quit** on an idle app, check daemon/client lifecycle, then `sudo apt install -- ./path/to/rquickshare-native_0.9.0-1.debug_amd64.deb`, not `dpkg --force-*`. Package installation never auto-enables the service. User action is separate and cannot be claimed from source/CI. Verify `dpkg-query -W`, `systemctl --user status` for daemon/desktop, and actual process executable (relaunch from app/tray). A previously running executable may still reference old unlinked binary until clean exit, so version alone is insufficient without lifecycle confirmation.

## 3. Physical matrix and truthful disposition

| ID / issue | Operator steps | Pass evidence | Mark OPEN/UNTESTED when |
| --- | --- | --- | --- |
| H01 #19 | Pixel discovery in KDE and KDE discovery in Pixel; remove/disable advertising on *one* peer, observe exact row disappear; restore, observe one stable row | No stale route, correct present/withdrawn and re-present; no unintended loss of unrelated route | Changes are only observed in old binary, the Android scan list caches peers or disappearance time is unmeasured |
| H02 #12/#9 | With all transfers idle, normal app Quit/reopen, repeat discovery generation, inspect one daemon / one tray owner and no mDNS stale cache after graceful shutdown | Stable restarts, no duplicate worker/log runaway, no SDK cleanup exception | No directly forced SDK browse close; synthetic loop test is not actual SDK failure |
| H03 #11 | Pixel small image both directions over same physical Wi-Fi, observe BLE→TCP success; repeat in comparable VPN-on and VPN-off configurations only if available | Correct bytes, no unroutable announced address, unchanged consent, BWU speed-up availability | Merely passed UKEY2/BLE without actual BWU or no VPN peer available |
| H04 #13 | With Devices collapsed, start inbound request; Devices pending counter/pulse, click opens and focuses correct Accept row. Repeat with expanded long list and two simultaneous consents *if second sender exists* | Exact current transfer visible after render, Receive caption unchanged, no misfocused Accept; no focus stolen during existing interaction | A single Pixel cannot produce two independent peer consents at once, so multiple-peer case UNTESTED |
| H05 #18 | Cold app/daemon start vs warm restart under fixed radio/visibility; measure UI activation, SDK browse, first accepted route; compare Pixel listing | Actual recorded timestamp/markers and correlated Pixel discovery, no guessed SLA | No trace enabled, or Pixel scan step not timestamped |
| H06 #18 | Pixel→Linux, explicit Linux Decline; then Linux→Pixel, explicit Pixel Decline | Linux terminal state and Android UI state recorded exactly, no file published and no auto-retry | Pixel says `Failed`: record observed label, do not manufacture wire fix from UI wording |
| H07 general | File both ways, multi-file, text/link, nested folder with simple synthetic fixtures | Exact bytes and hashes match, safe root/relative paths, consent before publication, no auto clipboard writes | Partial scenario or no independent hash proof |
| H08 reliability | Cancel during large inbound/multi-file, resume only by explicit new send; collision filename and offline/disconnect | No incomplete *final-name* file, completed members retained, no false Finished; no overwrite | Destructive fault injection risks real files, or no safe disposable directory |
| H09 package | Inspect package version, tray/daemon ownership, optional login-start unchanged, uninstall simulation/rollback dry run | Exact package SHA, one daemon/tray, settings survive when expected, safe rollback installer exists | Fresh package not installed or rollback artifact missing |
| H10 Windows | Google Quick Share Windows two-way discovery/transfer when hardware exists | Exact Windows build, code/consent and byte hashes | Current available peer only Pixel; mark Windows UNTESTED |

Minimal user feedback after first test batch: `source_sha`, `dpkg version`, checks H01–H06 with PASS/FAIL/UNTESTED and a short *observable* reason, optional screenshots showing labels only (hide peer names/codes), and the fixed-format trace lines below. Do not upload raw pairing PIN, file content, IP/MAC addresses, full journal or private screenshots.

## 4. Opt-in privacy-safe cold-start telemetry

The current code accepts `RQUICKSHARE_RECEIVER_TRACE=1` on **daemon process startup**, not on a running UI process after startup. Only five named fixed events `rqs-receiver event=... elapsed_ms=N` are intended. An owner-approved, transient testing session may set the systemd **user manager environment** before a clean daemon start:

```bash
systemctl --user set-environment RQUICKSHARE_RECEIVER_TRACE=1
# Start the installed app normally after graceful Quit and idle daemon stop.
# In another terminal after the test:
journalctl --user -u rquickshare-daemon.service --since '15 minutes ago' --no-pager |
  grep -Eo 'rqs-receiver event=[a-z_]+ elapsed_ms=[0-9]+' || true
systemctl --user unset-environment RQUICKSHARE_RECEIVER_TRACE
```

The user manager environment change is temporary but may be inherited by future daemon activations until unset. Do not force-kill an active transfer or mix one process's elapsed clock with another. Missing markers mean no measurement; do not invent latency or say the radio is healthy. [Precise trace contract](native-cold-discovery-timeline-2026-10-24.md).

## 5. Validation / coverage / next task

Mode B applicable Positive/N1–N12: basic BLE/TCP transfer, empty/malformed discovery, 0/1/multiple device rows, network interrupted/offline, retry/idempotency, concurrent consent and restart, stale route/cache, explicit security code consent, locale text, GUI close and lock, redacted logs, storage partial/permissions/collision; N11 monetization N/A. Automation Core and GTK source tests cover only a subset; the physical matrix is the necessary integration complement. Separate the physical pass from CI test count and from user observation.

An executable source bug requires reproducible symptom, alternative hypotheses and a scoped root-cause fix/regression plan **before changing production code**. For #16, authenticated route alias proof remains an architectural STOP and must never be faked by name/IP. For #8, owner chose A2.2; preserve fast BWU and leave security concern open. For actual fixed issues, update their trackers and the [active backlog](active-backlog.md) with true user acceptance, preserving source/build/installed SHA and exact operator observation; close only issues with fully passed applicable gates. Release #14 requires separate authorization, not a side effect of testing.

**Current checkpoint:** operator hardware/build actions **NOT YET EXECUTED** in this chat. A documentation-only physical plan does not add a test and should not trigger heavy Rust CI.


## 2026-10-24 operator-built candidate — completed offline build gate

User ran the documented **local** build on Alcotester from clean `dev` HEAD `87af15c7e9776c93eba6d519249fd8143131027d` (only documentation differs after the tested executable source `2545e051`). Their supplied terminal output is direct evidence of:
- Previous installed `rquickshare-native` version: `0.9.0-1`.
- Real `cargo +stable build` finished for `rqs_lib` and Native GTK, package assembled at `~/Downloads/rquickshare-candidate-87af15c/rquickshare-native_0.9.0-1.debug_amd64.deb`.
- `scripts/native_deb.py build --profile dev` ran its internal package verifier (PASS); the operator's independent `sha256sum -c` printed `OK`; subsequent `scripts/native_deb.py verify` printed PASS.
- `dpkg-shlibdeps` noted merged-usr libc6 loader diversions; the packaging operation succeeded and validated. A nonfatal `bluer v0.17.3` future Rust incompatibility warning appeared. **No Rust future-incompatibility report collected yet**; do not assert its specific cause.
- Native GTK compiles `rqs_lib` separately with `default-features = false`, so that compile displays nine `unused` warnings including `local_lan_ipv4`; **the daemon is built from core_lib with default `experimental` enabled** and `inbound.rs::send_upgrade_path_available` uses `local_lan_ipv4` for `WIFI_LAN`. Thus the warnings do not independently establish a dead BWU path. Source structure supports the call, but live BWU remains **UNTESTED** until Pixel transfer and trace evidence.
- **No local installation was reported**; no user service restart, test transfer, or previous-package rollback artifact verified. Do not infer these steps succeeded. Next gate: locate and independently verify the old `0.9.0-1` rollback `.deb`, then an explicit idle user-controlled package replacement and actual test matrix.

This checkpoint is *operator-supplied physical-host packaging evidence*, not a GitHub Actions artifact. Keep old/new package hashes locally; avoid committing host paths with private identifiers, peer/device secrets or full verbose journal.

## 2026-10-24 rollback artifact located — operator evidence

Before installing the current debug build, the operator searched only their local Downloads, native build output and APT cache and supplied the following **non-mutating terminal evidence**:

- Old installer found: `~/Downloads/rquickshare-acceptance-0.9.0.8RY0pX/rquickshare-native_0.9.0-1_amd64.deb` with dpkg metadata `Package: rquickshare-native`, `Version: 0.9.0-1`, `Architecture: amd64`.
- Old package SHA-256: `51f22e38d2b11efdb064a4835ac96fd49be0cd26077db511e8c502178fe51e00` (observed from local `sha256sum`, **not yet matched to an independent trusted distribution digest**). Before install, also verify the old archive can be read via `dpkg-deb --contents` and keep this file unchanged for rollback.
- New installer: `~/Downloads/rquickshare-candidate-87af15c/rquickshare-native_0.9.0-1.debug_amd64.deb` with metadata `Package: rquickshare-native`, `Version: 0.9.0-1.debug`, `Architecture: amd64`. The new package's own SHA-256/metadata/contents checks already passed in the preceding build checkpoint.
- Installation, user daemon/client stop/restart, Pixel test and real BWU **NOT YET REPORTED**. Never mark installed/native peer smoke PASS from these two packaging outputs alone.

**Next physical gate:** operator quits tray/desktop after all transfers idle, gracefully stops their own systemd user services, checks no binary remains, and uses `sudo apt install -- ./absolute/path/to/new.debug.deb`, then validates package version and both service owners. A downgrade to `0.9.0-1` is deliberate and requires the preserved original file; do not delete settings or source paths. The owner must report actual status before physical acceptance scoring.

## 2026-10-24 physical screenshot and scoped follow-up

- Operator on KDE/Pixel 6a reports: after launch `Receive` shows `Visible to nearby devices · 0:45`; on clicking Linux `Decline`, Android sender shows `Failed`; repeated transfer leaves a prior `Mercury · MP4 · 5,264,330 bytes · Completed` row alongside a new `Mercury · MP4 · 5,264,330 bytes · code / Accept / Decline` consent. Note screenshot says **MP4**, not image; do not claim file-kind misclassification until the submitted filename/type is independently established. User has not supplied a direct package-version readout after installation; only the earlier verified `0.9.0-1.debug` candidate and current physical screen are known.
- **#20 code change** `d886146aafd0b479eb5efad7fc44101c40090bb9` removes old completed results from compact card on new incoming nonterminal transfer, preserves History and queued result reveals. UI-only `native_desktop/src/panel/results.rs`, two regression methods. Needs exact CI terminal statuses and updated installed package on KDE before acceptance.
- **#18 Android Failed mapping** confirmed by operator, source sends encrypted `Status::Reject`; no proven Pixel reception or timing and no speculative wire change.
- **#21 startup policy** is separate from actual mDNS discovery and from Receive 60s UI. New-install default is Visible; stored temporary can restart a 60s window. Needs owner decision to preserve saved intentional Visible and make temporary truly one-shot. No source/settings modification yet.


## 2026-10-24 FINAL exact-source CI (supersedes earlier 5/6)

**Production+test source `2de1bb70d7b864e3faae849891176d510f678e35` — ALL SIX checks terminal SUCCESS.**

- [Preflight 40101208122](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101208122): Rust format `120893731492`, Core tests `120893731359`, Core Clippy `120893731514`, Tauri Clippy + Desktop Rust tests `120893731534`, frontend `120893731585` all completed success.
- [Native GTK 40101207886](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101207886): `120893552998` completed success; GTK 10/10 auxiliary + 92/92 unit/widget/controller assertions, including the **two new Results regression methods individually `ok`**.
- Core `790 passed, 0 failed, 0 ignored, 8 filtered`; 4/4 + 4/4 integration. The eight filtered race-sensitive tests are not marked as executed. Mode B full 10×, mutation engine and physical user tests have **not** been executed on this change.
- Rebuilding/installing the package on operator KDE is still OPEN; the currently shown MP4 screenshot predates the #20 fix. Old/rollback installed `0.9.0-1` and candidate debug package may share a version with new debug build, so **use source SHA + extracted-vs-installed executable hashes**, not dpkg version alone, before marking acceptance. No `master`, published artifacts or tag.

## 2026-10-24 verified installed package and observed UX findings

Owner supplied full `apt install --reinstall -- .../rquickshare-candidate-e2304e9/rquickshare-native_0.9.0-1.debug_amd64.deb` terminal transcript, including `0 removed`, `0.9.0-1.debug` reinstall, `dpkg-query` confirming `amd64`, and `cmp` **MATCH** for all 3 native executables. This confirms the debug code containing #20 was actually installed, though #20 behavior has not been re-tested. The local `_apt` unsandboxed read warning is nonfatal. Upon ordinary app start inbound visibility still auto-enabled because settings persisted 2; not stale installed executables. Pixel-side response to Linux decline now `Declined` (PASS for that direction). Pixel rejection of Linux outbound shows no GTK compact result (FAIL). Owner demands **manual-only inbound discoverability on startup**, preserving recipient discovery. New source `e47e6a9c` targets both with final CI + local .deb/install yet to be performed. [Plan](native-manual-receive-and-decline-2026-10-24.md), [#21](https://github.com/morozhenko-apps/rquickshare/issues/21), [#18](https://github.com/morozhenko-apps/rquickshare/issues/18).

## 2026-10-24 owner local build / previous-source CI failure / test-oracle correction

Owner reported **"собрал"** after being instructed to build current `dev` from `e67134ad` (production+tests through `a3976ad`). No terminal output or file hash from this latest local build was supplied, so its exact path and checksum are **not independently verified**; do not infer new installation or package replacement. **Do not install that build as acceptance candidate** because exact executable source `a3976ad778035d1580e1614a99d7bdffdee18a81` finished **5/6 checks success, Core Rust tests failure** (Core run 40105012066). Core job `120904955524`: **791 passed; 2 failed; 8 filtered**. The two failed tests were managed BLE and managed TCP integration oracles still expecting outbound `Disconnected` after recipient explicit Reject, while the corrected real protocol now produces `Rejected`. All 791 remaining source tests passed, including the new real `process_consent` Reject regression. This is a test expectation changed by an intentional state contract, **not evidence of a hidden protocol break or validation of the physical UI**.

**Test-only new commit** `ded1f1d2d86ff889174ec04666612bf112f5cc82` updates exactly the two affected integration test assertions to expect terminal outbound `Rejected` (keeping inbound `Rejected` and accepted `Finished` semantics and preserving actual BLE/TCP integration coverage). Zero production code files changed. Source CI [Preflight 40109101697](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101697) and [Native GTK 40109101791](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101791) were started; all terminal statuses were **not yet verified at this doc checkpoint**. Do not assert 6/6 until checked. Rebuild **once** from `dev` containing at least `ded1f1d2` after source gate becomes green, verify source SHA, package sha, original rollback and installed binary comparison; then owner tests manual Hidden startup, restart, 60s Receive and expiry, Pixel Decline→KDE Declined, repeated inbound compact result & History. No master/tag/R2/release updates.

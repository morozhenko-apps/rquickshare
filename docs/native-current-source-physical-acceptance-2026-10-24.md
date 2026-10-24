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

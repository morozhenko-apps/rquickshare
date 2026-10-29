# Native GTK 0.9.0-3 installed acceptance (after final green source and package gates)

## Eligibility

The owner already ran T01-T13 on **0.9.0-2** with PASS, T14 UNTESTED. That evidence applies only to the previous installed candidate. This checkpoint is for the **new** native 0.9.0-3 Debian revision and includes the changed GTK settings/localization and R2 public verifier. It is not a request to repeat every previous transfer scenario unnecessarily.

Preconditions must be recorded as facts, not assumptions: exact source SHA with terminal Preflight + Native GTK SUCCESS, clean native package-smoke, downloaded artifact SHA256 OK, Package/Version/Architecture = rquickshare-native/0.9.0-3/amd64, and apt-get -s shows no unrelated package removal. Never use the unsigned APT staging as an installed trusted repo. Stop if any dependency cannot be proven.

## Reduced owner physical matrix, no human 20-locale traversal

K01: Open new KDE client, inspect custom app titlebar icon in Settings and About, no in-page duplicate product icon/name, no About tab, no redundant separator/success footer. An actual save failure or uncertain apply MUST retain its warning.
K02: Language System default and explicit Russian/PT-PT, save and reopen. Cancel does not change active language. Restart app and daemon separately; preference survives. Language-only change must succeed even while daemon is offline and not trigger a transfer.
K03: With History, Send Text (with **unsent draft**), Send Image (with captured image) or Received Text open, switch locale from Settings and verify chrome changes while state/content remains. Never auto-read clipboard or consent. Representative Arabic RTL and long labels can be inspected once; no 20-human-language signoff is required.
K04: KDE tray context menu and card button labels change without daemon restart. Devices pending badge remains on Devices, not Receive; a new inbound request is focused correctly without auto-acceptance.
K05: Cold start and manual Receive: daemon remains Hidden after startup including persisted Temporary. Receive enables ~60s inbound visibility, expiry returns Hidden; daemon restart does not automatically rearm. Outbound sender discovery works while hidden.
K06: Pixel 6a → KDE & KDE → Pixel: send Unicode text + non-sensitive file, Accept, Decline. Verify explicit reject stays terminal Declined, not Success. Cancel a mid-file transfer, check no partial file success on the opposite endpoint, and next send still works. A Cancel concurrent with an actual BWU cannot be marked PASS from an ordinary TCP Cancel.
K07: Two same-named devices or multiple incoming requests: keep separate routes, do not merge trust by display name. Check pending count/attention scroll, no automatic consent. This is NOT #16 identity proof and is NOT authorization to change #16.
K08: Report cold start vs first Pixel appearance vs Connecting→PIN timing; measure Accept→Completed duration for one synthetic file and record exact bytes/time (not only progress-bar animation). Keep opt-in receiver trace strictly static and redact all peer/IP/PIN/path data. Try without WLAN only if current test constraints permit; never claim radio interoperability without observed evidence.
K09: Dolphin Send context action, file contents/hash match, cancel/restart and app removal/reinstall with preference retention. Run only as needed to detect changed-package regression; previous 0.9.0-2 T10 PASS can be cited historically.

## Evidence template

Source SHA: ___ ; native package workflow run ___ ; artifact SHA256 OK [yes/no] ; installed Debian version ___ ; Ubuntu KDE ___ ; Pixel Quick Share version if available ___.
Mark K01-K09 PASS/FAIL/UNTESTED, exact first failing step, expected/actual and redacted service log. Stop to investigate at the first reproducible critical failure. Do not promote master/tag/production signed R2 until a separate operator release approval. Leave #8 and #16 untouched, including their code and issue states.

## Current checkpoint (2026-10-29)

Only source changes are present in dev. Candidate 0.9.0-3 is not built or installed as of this document. Current CI for the test-only compiler-fix b85aeaad is not yet terminal-verified; no physical pass or package artifact is asserted.

## 2026-10-29 verified package-smoke result and owner handoff

- `native-package-smoke.yml` run [40916717323](https://github.com/morozhenko-apps/rquickshare/actions/runs/40916717323): terminal SUCCESS, workflow-dispatch on dev HEAD `0388a94b81a7dea3f276bc56a0ce0160b36c29fb`. All steps success, including native Core/Python tests, 10+171 GTK tests, strict rustfmt/Clippy, `native_deb.py build`, `native_deb.py` verification and unsigned APT staging preparation.
- Verified release artifact ID `12915614696`: `rquickshare-native-ubuntu-26.04.zip`, ZIP SHA256 `55c0c59ef5fe839ed912fdaf945bf72f1f5f8c05b2ec87a50131e53789b64de2`. Distinct unsigned APT staging ID `12915983929` is **not** the installer and was not published.
- Independent post-CI ZIP check matches GitHub digest; ZIP contains exactly `rquickshare-native_0.9.0-3_amd64.deb` and `.deb.sha256`. The package `.sha256` check passed. Debian package SHA256 **`757f439fc6b4bd98759b2e1f63bba391c53ef36bfebf1a73457c5a792dbfd6c7`**; `dpkg-deb -f` confirms package `rquickshare-native`, version `0.9.0-3`, architecture `amd64`. Package contents include `/usr/bin/rquickshare`, user daemon/desktop systemd units, daemon and preview helper, application/icon, DBus services and Dolphin KIO menu. `dpkg --compare-versions` confirms 0.9.0-3 newer than both 0.9.0-2 and 0.9.0-1.debug.
- The source-gate pair on last executable SHA `b85aeaad4af6ad2dbe0f64d5501541be90419c10` passed [Preflight 40900537851](https://github.com/morozhenko-apps/rquickshare/actions/runs/40900537851) and [GTK 40900537836](https://github.com/morozhenko-apps/rquickshare/actions/runs/40900537836); subsequent modifications before package dispatch were docs-only.
- This independent ZIP/package verification ran in an isolated container, **not** the owner's Ubuntu 26.04. Consequently local `apt-get -s install` and installed live GTK/Pixel tests remain pending. The user must preserve the existing 0.9.0-2 rollback artifact and inspect any APT proposed removals before installation. Do not present the unsigned APT staging as trusted distribution.
- No protected release tag, master promotion, real R2 upload or package installation executed by the assistant. Owner's #8 and #16 freeze remains in effect. T14 and cross-device acceptance still open.

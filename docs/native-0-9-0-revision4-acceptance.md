# Native 0.9.0-4: verified candidate and focused acceptance

## Candidate provenance and timestamps (2026-10-29 Europe/Lisbon, UTC+0)

- Source branch: `dev`, package commit `2b0103ddef9c5115452cb8c86693e08e1af5a69e`. Its executable ancestor was `45c246702ff447e2783489dc047c71cec61e83a0`; subsequent commits before package were documentation-only.
- [Preflight 40932148661](https://github.com/morozhenko-apps/rquickshare/actions/runs/40932148661) and [Native GTK 40932148761](https://github.com/morozhenko-apps/rquickshare/actions/runs/40932148761) started **2026-10-29 18:34:51 Lisbon**, both terminal SUCCESS. The isolated native GTK suite passed **172 of 172 tests** plus **10 of 10 additional tests**, no failures or ignored tests.
- [Native Linux package smoke 40933928808](https://github.com/morozhenko-apps/rquickshare/actions/runs/40933928808) was explicitly manually dispatched. Start: **2026-10-29 18:46:17 Lisbon**; terminal completion **18:53:44 Lisbon**; conclusion **SUCCESS**. All 17 workflow steps passed (build prerequisites, real unprivileged Bubblewrap, Core/Rust/Python, strict Clippy, GTK, package verify, unsigned APT prepare, artifact upload).
- Build log: **902/902 Core tests**, 4+4 additional Core tests, 65/65 Python native-package/publication contract test methods, 10/10 plus 172/172 GTK tests (other suites reported 0 tests). Build itself completed at 18:53:27, upload at 18:53:40. This reports CI observations; no user system has been installed or tested by the assistant.
- Release artifact ID `12924920625`: `rquickshare-native-ubuntu-26.04`, ZIP SHA256 `4adbadb33b459de904bd2c2d444b66dc24f3f25c74e418c29a8d766dce1a5747`. Its ZIP contains exactly `rquickshare-native_0.9.0-4_amd64.deb` and `rquickshare-native_0.9.0-4_amd64.deb.sha256`; independent ZIP digest matches the GitHub artifact digest, and the internal SHA256 manifest check passed.
- Package SHA256: **`943d9885860648f64eeb0d536ecd22b978d82689f644ee2b2044e071ad11b796`**. `dpkg-deb -f` independently confirmed package `rquickshare-native`, version `0.9.0-4`, architecture `amd64`; Debian ordering `0.9.0-4 > 0.9.0-3` passed. Package contains the GTK client, daemon, preview helper, systemd units, icon, Dolphin KIO integration and application launcher `Name=rQuickShare Native`.
- Separate unsigned APT staging ID `12923428540` was uploaded only for testing, not a signed or published repository. It MUST NOT be treated as trusted APT distribution.

## Owner installation boundary

No installation occurred on the owner's Ubuntu. Do not claim real dependency resolution/upgrade until the owner's `sudo apt-get -s install ./rquickshare-native_0.9.0-4_amd64.deb` shows no unrelated removals or missing dependencies. Keep the previously accepted `0.9.0-3` package for rollback, and stop live transfers before upgrade. Owner's current Ubuntu/KDE is the primary Linux target. The test ZIP and .deb are untrusted **pre-release candidates**, not publicly signed stable packages.

## Focused physical acceptance (owner, not one test per locale)

- A1. `rQuickShare Native` appears beneath the installed application icon and launches correctly; Dolphin Send action and tray remain.
- A2. About uses RepoTicker-style compact **single GTK window**: 72px icon, title, version, `Станислав Мосейко` / `Stanislav Moseyko (morozhenko)`, external link `https://morozhenko.com`, explicit upstream `Martichou/rquickshare` and `GNU GPLv3`; **no Credits page** and no Linux-only slogan. Close and reopen it; it must reuse the same view, not leave orphaned windows.
- A3. Russian UI nearby empty caption **is Russian**, as are incoming/transfers empty placeholders. Switch RU -> EN -> PT and back while existing device list and About are open; no stale captions; RTL quick check optional rather than a 20-language manual review.
- A4. Open Send Text draft, History, Send Image and possibly About while switching locale. Content, clipboard, image and transfer identities/consent remain untouched. Cancel language edit does not change active language.
- A5. Bidirectional Pixel 6a ↔ KDE text and image, single Decline and ordinary Cancel with no false Success. Do not treat ordinary Cancel as **proof** of #26 committed BLE-to-TCP cancellation race; keep observed timing and conditions.
- A6. Cold start manual-only Receive, tray and pending inbound action; do not auto-accept or expose receiving mode on startup.
- A7. If any failure, provide screenshot and redacted log/steps without secrets or peer identifiers, and record PASS/FAIL/UNTESTED. Do not run package upload/deploy on fail.

## Release gate and scope exclusions

Source and package gate: GREEN for this specific candidate SHA. Physical KDE/Pixel installed acceptance: PENDING. Repository-wide Mode B and `1.0.0` release: NOT COMPLETE. Frozen #8 and #16 are **excluded**, no source/issue modification and no claim they are resolved. Do not merge master, tag, sign, publish or write production R2 without explicit owner approval and agreed release readiness. The already accepted `0.9.0-3` stays intact.

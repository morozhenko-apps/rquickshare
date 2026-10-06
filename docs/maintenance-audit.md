# rQuickShare maintained fork — audit and Mode B hardening

_Last updated: 2026-10-06_

This document is the working log for the `morozhenko-apps/rquickshare` maintenance pass. It is intentionally kept separate from release notes: it tracks audit scope, upstream triage, CI gates, fixes, and the final Mode B verification before fast-forwarding `dev` into `master`.

## Current CI policy

- `dev`: fast preflight only — Rust formatting, Rust tests, Clippy, frontend lint/typecheck/unit tests.
- Internal `dev -> master` PR: duplicate heavy jobs are skipped; the push to `dev` is the authoritative preflight.
- Package/build smoke: manual only. No Tauri packaging on every development commit.
- Release artifact build: manual only.
- Mode B: stable tests run once; only explicitly flaky/stability-sensitive tests are repeated 10 times.
- `master`: updated only after the audit is complete and the final gates are green.

## Current gate status

A full `dev` revalidation is in progress after dependency, clipboard, logging and Tauri security hardening.

The last representative dependency-upgrade preflight had four green jobs (Rust format, core tests, core Clippy and frontend lint/typecheck/unit) and one Tauri Clippy failure caused only by five newly surfaced lint/deprecation findings. Those findings have been fixed; a fresh authoritative `push dev` preflight is now running.

The internal `dev -> master` PR validation is intentionally skipped to avoid duplicating the authoritative `push dev` preflight.

## Completed work

### CI and toolchain

- Removed automatic package builds from normal `dev` pushes.
- Moved package smoke to a manual workflow.
- Moved release packaging to a manual workflow.
- Updated GitHub Actions away from Node 20-backed actions and configured Node 24 for frontend CI.
- Removed the deprecated `arduino/setup-protoc` action; Linux CI installs `protobuf-compiler` directly.
- Removed deprecated/redundant cache/action usage found during validation.
- Aligned Rust formatting/Clippy toolchains with the repository's actual toolchain requirements.
- Removed duplicate Rust formatting work and duplicate internal PR validation.
- Enabled cancellation of superseded runs.

### Repository metadata / maintenance

- Aligned crate/package license metadata with the repository GPLv3 license.
- Disabled automatic release-please activity for the maintained fork; release preparation is manual.
- Replaced the stale Tauri 2.2 dependency set with current Tauri v2 packages (Tauri core/API/CLI 2.12.1 plus current compatible plugin minors).
- Regenerated Cargo.lock and pnpm-lock.yaml reproducibly on GitHub-hosted runners and removed the temporary sync workflows afterwards.
- Updated the frontend toolchain conservatively inside its existing major lines: ESLint 9, Tailwind 3, TypeScript 5, Vite 6 and Vitest 3.
- Removed the unused Vue DevTools/Electron development stack that was responsible for a large obsolete vulnerability tree.
- Added weekly/manual dependency security auditing for both Rust lockfiles and frontend dependencies.
- Current Rust cargo-audit is clean for both lockfiles.
- Current JavaScript production/runtime audit has no known vulnerabilities.
- Full JavaScript dev-tooling audit has one accepted high-severity build-only advisory: Tailwind CSS 3.4.19 pulls `braces 3.0.3` through its watcher/glob chain; the advisory currently has no patched `braces` release. This dependency is not shipped in the Tauri runtime and build inputs are trusted repository files. The runtime audit remains release-blocking; the full toolchain audit remains visible/reporting.

### Protocol / interoperability

- Added currently used Nearby frame types 8–12 to the protobuf enum.
- Added/retained compatibility for compact mDNS endpoint information and fallback device naming.
- Reviewed upstream interoperability PRs before integrating equivalent fixes instead of blindly merging stale branches.

### Linux BLE lifecycle

- Added background BLE scan duty-cycling to avoid keeping BlueZ in continuous discovery while the app sits in the tray.
- Made BLE advertising follow receive visibility rather than outbound discovery mode.
- Added foreground/background lifecycle signalling.
- Unified scan/advertising lifecycle so send discovery and receive visibility do not fight each other.

This work is intended to cover the same problem areas reported upstream in issues/PRs such as #429, #430 and #404.

### Inbound hardening

- Reject unsafe received filenames instead of joining remote path components directly into the download directory.
- Reject absolute paths, traversal components, separators/control characters and overlong received names.
- Reserve unique destinations for multiple inbound files with the same filename.
- Reject negative or oversized byte payload sizes before allocation.
- Reject byte chunks that exceed the peer-declared size.
- Reject file payload data before the user has accepted the transfer instead of reaching an `unwrap()`.
- Guard offset arithmetic against overflow.
- Reject duplicate payload IDs.
- Reject negative file sizes and total-size overflow.
- Use exclusive destination creation and fail clearly if the configured download directory is unavailable.
- Reject invalid peer P-256 public keys instead of panicking.

### Outbound hardening

- Reject negative or oversized peer-controlled byte payload sizes before allocation.
- Reject byte chunks that exceed declared payload size.
- Replace peer-controlled connection/consent `unwrap()` paths with checked errors.
- Reject invalid peer P-256 public keys instead of panicking.
- Avoid outbound payload-ID collisions.
- Reject non-UTF-8 filenames cleanly instead of panicking.
- Guard total outbound transfer size against overflow.

### Error handling / UI

- Surface transfer errors instead of reducing every failure to a generic unexpected disconnect.
- Stop receiver loops cleanly when their channel closes.
- Normalized transfer state text and Vue markup so lint is meaningful rather than permanently noisy.
- Added a browser Clipboard API fallback when the Tauri clipboard plugin fails on Linux/Wayland, with unit coverage for primary/fallback/failure paths.
- Capped the active on-disk log at 5 MiB per session while keeping stdout logging alive; oversized previous logs are still rotated at startup. Unit tests cover the capped writer.
- Enabled a restrictive production/development Content Security Policy and `freezePrototype` in the Tauri WebView. Production network access is limited to Tauri IPC plus the GitHub API used for release checks.

## Upstream PR / issue triage

Reviewed or identified as relevant:

- PR #442 — missing download-directory handling. Not applied verbatim: silently recreating a configured removable/missing destination is not always desirable; the maintained fork currently fails clearly when the selected destination is unavailable.
- PR #439 / issue #431 — compact mDNS endpoint info and newer frame types. Equivalent compatibility work is included.
- PR #430 / issue #429 — continuous BlueZ discovery interference. Equivalent duty-cycle work is included.
- PR #404 — BLE advertisement based on visibility. Equivalent lifecycle work is included.
- PR #418 — filesystem / transfer error propagation. Equivalent error-surfacing work is included where applicable.
- PR #408 — Tauri JS/Rust dependency alignment. Superseded: the fork now uses current Tauri v2 (2.12.1 core/API/CLI with compatible current plugins) and reproducibly regenerated lockfiles.
- PR #380 / issues #423 and #195 — clipboard fallback. Equivalent behavior is included with dedicated unit tests.
- PR #334 — cancellation during active file sends. Equivalent cancellation handling is included.
- PR #333 — Wi-Fi credential payload parsing. Equivalent handling is included with declared-length validation.
- Issue #268 — unbounded active log growth. Fixed with a 5 MiB runtime file cap.
- PR #420 — large transport/protobuf refactor. Deferred until the maintained fork is green because it is too broad to merge as a bugfix.

Upstream backlog is not considered closed yet. Remaining open PRs/issues must be classified as: applicable, already covered, obsolete/duplicate, feature request, or deferred refactor.

## Modern Pixel receiver BLE/GATT bootstrap

Upstream issue #425 contains current 2026 evidence that modern Pixel Quick Share can leave Wi-Fi during receiver discovery. In that state, mDNS-only Linux receivers may never appear or may fail before opening the TCP connection.

The implementation is now isolated in draft PR #2 (`feat/pixel-ble-receiver -> dev`) and ports the proven architecture from `martinalderson/rquickshare:feat/ble-receiver-connect-back` without overwriting the fork's hardened inbound code:

- receiver advertisement on service UUID `0xFEF3` using the same endpoint id and hostname as mDNS;
- visibility-aware connectable advertising with retry on transient BlueZ failures;
- Nearby GATT slot plus weave write/notify characteristics;
- bounded weave framing/reassembly and handshake validation;
- the existing hardened UKEY2 / Sharing receive state machine generalized over `AsyncRead + AsyncWrite`;
- a migratable BLE/TCP transport that preserves crypto keys and sequence counters;
- encrypted `UPGRADE_PATH_AVAILABLE` negotiation after UKEY2 establishment;
- validated plaintext `CLIENT_INTRODUCTION` / ACK on the new TCP socket;
- LAST_WRITE / SAFE_TO_CLOSE prior-channel handoff;
- LAN address selection that ignores Docker/VPN/tunnel interfaces;
- Wi-Fi-LAN transport swap for the actual payload;
- regression coverage for advertisement layout, weave handshake, framed lengths, LAN-interface filtering and BWU client-introduction validation.

The stable `dev` branch remains green while this larger interoperability change is validated separately. PR #2 must pass its own preflight and a real Pixel -> Linux smoke test before it is merged into `dev`.

## Remaining audit work

- Finish residual network/state-machine `unwrap()`/panic review; peer-controlled crypto/size/path panic paths have already been removed.
- Finish the upstream open PR/issue classification and keep large feature/refactor PRs separate from hardening.
- Validate the new CSP/freezePrototype behavior in the packaged Linux smoke test.
- Expand regression coverage around every bug fixed during this pass.
- Identify only genuinely flaky/stability-sensitive tests for 10x Mode B repetition.
- Run one manual Linux package build.
- Complete PR #2 validation and real Pixel -> Linux BLE/GATT -> Wi-Fi-LAN smoke.
- Perform install/start/send/receive smoke checks on Linux/Android.
- Run the final Mode B gate.
- Fast-forward `dev` to `master` only after the above is complete.

## Mode B exit criteria

The maintenance pass is complete only when all of the following are true:

1. Normal `dev` preflight is green.
2. New regression tests for fixed defects are green.
3. Stable tests pass once.
4. Designated flaky/stability-sensitive tests pass 10 consecutive runs.
5. Manual package build succeeds.
6. Linux application smoke test succeeds.
7. Android -> Linux receive path succeeds.
8. Linux -> Android send path succeeds.
9. No unresolved high-severity security/reliability finding remains in the audit.
10. `dev` can be fast-forwarded to `master` without merge noise.

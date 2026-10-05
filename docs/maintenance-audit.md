# rQuickShare maintained fork — audit and Mode B hardening

_Last updated: 2026-10-05_

This document is the working log for the `morozhenko-apps/rquickshare` maintenance pass. It is intentionally kept separate from release notes: it tracks audit scope, upstream triage, CI gates, fixes, and the final Mode B verification before fast-forwarding `dev` into `master`.

## Current CI policy

- `dev`: fast preflight only — Rust formatting, Rust tests, Clippy, frontend lint/typecheck/unit tests.
- Internal `dev -> master` PR: duplicate heavy jobs are skipped; the push to `dev` is the authoritative preflight.
- Package/build smoke: manual only. No Tauri packaging on every development commit.
- Release artifact build: manual only.
- Mode B: stable tests run once; only explicitly flaky/stability-sensitive tests are repeated 10 times.
- `master`: updated only after the audit is complete and the final gates are green.

## Current gate status

Latest `dev` preflight on `70f4a2c` is fully green:

- Rust format — green.
- Core Rust tests — green.
- Clippy `core_lib` — green.
- Frontend lint/typecheck/unit — green.
- Clippy `app/main/src-tauri` — green.

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

## Upstream PR / issue triage

Reviewed or identified as relevant:

- PR #442 — missing download-directory handling. Not applied verbatim: silently recreating a configured removable/missing destination is not always desirable; the maintained fork currently fails clearly when the selected destination is unavailable.
- PR #439 / issue #431 — compact mDNS endpoint info and newer frame types. Equivalent compatibility work is included.
- PR #430 / issue #429 — continuous BlueZ discovery interference. Equivalent duty-cycle work is included.
- PR #404 — BLE advertisement based on visibility. Equivalent lifecycle work is included.
- PR #418 — filesystem / transfer error propagation. Equivalent error-surfacing work is included where applicable.
- PR #408 — Tauri JS/Rust dependency alignment. Still to review against the current dependency graph before applying.
- PR #420 — large transport/protobuf refactor. Deferred until the maintained fork is green because it is too broad to merge as a bugfix.

Upstream backlog is not considered closed yet. Remaining open PRs/issues must be classified as: applicable, already covered, obsolete/duplicate, feature request, or deferred refactor.

## Remaining audit work

- Audit remaining network/state-machine `unwrap()`/panic paths and distinguish true peer-controlled paths from internal invariants.
- Review dependency age and known vulnerabilities, including Tauri/Rust/JS alignment.
- Finish the upstream open PR/issue classification.
- Expand regression coverage around every bug fixed during this pass.
- Identify only genuinely flaky/stability-sensitive tests for 10x Mode B repetition.
- Run one manual Linux package build.
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

# Native APT refresh keyring Path boundary (2026-10-30)

Primary mode: MODE A (production release and refresh reliability). This document records a scoped root-cause correction before executable changes.

## Evidence and diagnosis

The Native 1.0.0 publisher run [41059114750](https://github.com/morozhenko-apps/rquickshare/actions/runs/41059114750) succeeded through source validation and DEB packaging but failed in the signer-only keyring admission step with `AttributeError: 'str' object has no attribute 'is_symlink'`. No protected signing, R2 publication, or GitHub Release followed that failure. The exact same invalid caller remains in `.github/workflows/native-apt-refresh.yml`, even though the publisher caller is corrected in PR #39.

Plausible failure domains: raw CLI string passed to a Path-only validator; missing or corrupted keyring; inaccessible keyring file; an unexpected signing fingerprint; malformed imported subkey; invalid subkey passphrase; missing archive credentials. The traceback and source contract confirm the first hypothesis for this failure. They do not establish the condition of the secrets or archive service.

## Plan

1. Convert the APT refresh keyring CLI argument to `pathlib.Path` at the Python adapter boundary, matching the fixed tagged publisher.
2. Add a source-backed regression asserting Path conversion and retention of the strict signing-subkey check in **both** release and APT refresh workflows.
3. Keep the change limited to the refresh caller and its existing tests. Do not change signature validation, key custody, fingerprint, R2 credentials, publishing semantics, or scheduling.
4. Let the reviewed PR's normal Preflight and Native GTK checks run against the eventual combined SHA. Do not launch an independent stress/Mode B or publishing run for this patch.

## Invariants and rollback

- APT refresh must never accept a private-primary signing key or downgrade authenticity checks.
- The refreshed snapshot must continue to authenticate prior R2 state, sign and verify metadata, publish through conditional CAS, and verify the public InRelease.
- The original caller change is independently revertible before a release. Retagging, signing and production R2 writes are **not** part of this patch.
- The old `native/v1.0.0` annotated tag remains unchanged until protected-master review and a separate, explicitly authorized retag recovery. No public GitHub Release was listed at this checkpoint, but public R2 state requires independent readback.

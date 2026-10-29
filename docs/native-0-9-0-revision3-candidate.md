# Native 0.9.0-3 candidate preparation

Date: 2026-10-29. MODE A, native GTK client/UI and public R2 readback scope only. This is a disposable dev candidate, **not** a product SemVer bump, protected release tag, signed publication or immutable R2 overwrite.

## Why revision 3

The owner installed and accepted 0.9.0-2 (13 PASS, 0 FAIL, T14 UNTESTED) from Native Linux package smoke 40785201691. Since that package's source was built, dev source changed the GTK-only language selector, Settings and About layout, localized card/menu refresh and public release readback header. The next testable package must have a strictly greater Debian version than 0.9.0-2. Reuse of 0.9.0-2 bytes/version is forbidden. Set packaging/release.toml revision from 2 to 3 and update exact dpkg ordering regression.

## Scope and invariants

- Version field: same product 0.9.0, Debian revision 3 (rquickshare-native_0.9.0-3_amd64.deb).
- Compare current physical installed 0.9.0-2 and previous 0.9.0-1.debug; check candidate is lower than future 0.9.0-3.debug / 0.9.0-4 / 0.9.1 / 1.0.0.
- One source gate at final executable commit: Preflight, Native GTK formatting / lint / widgets and native package tests. The previous GTK 40837887677 stopped at nightly rustfmt; exact formatting-only corrections were applied in commits 939bef05 and 3a9ce0af. No later compile or widget results are inferred.
- Only after exact-source gates succeed: manually dispatch native-package-smoke.yml on dev. Verify artifact SHA256, version, architecture and APT install simulation before voluntary replacement. Do not install from unsigned APT staging.
- Preserve one daemon and tray client, consent, file byte/hash, cancel/decline, login-start, KIO Dolphin, persisted paths/data, locale staging and R2 no-publish boundary.
- Physical acceptance 13/14 is **historical** to 0.9.0-2, not automatically applicable to 0.9.0-3. T14 remains open and 20-locale human/RTL review not claimed.
- No security issue, peer authentication, Windows interop, signing or 1.0.0 feature is closed merely by this bump.

## Rollback and handoff

Revert the single revision-3 source/config/test commit to restore the unbuilt revision-2 metadata. Do not rewrite the existing real 0.9.0-2 artifact. Owner can keep the existing installed package until new candidate gates pass; no automatic upgrade, master/tag, R2 write or signing is performed by this plan.

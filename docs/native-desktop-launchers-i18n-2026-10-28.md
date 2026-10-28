# Installed Linux desktop entry localization (#17)

Date: 2026-10-28. MODE A; working branch `dev`, no installation, package publication, new branch or PR. Starting source `f34b24f57cbb2ba27ab4d43f5c39866ffd667882` with docs checkpoint `0b7159548406622dab9a8ebc7c096775a2939af4`. Source-exact Preflight 40649696855 succeeded; Native GTK 40649696863 failed on **one** nightly rustfmt-only assertion in `src/i18n/tray_chrome.rs`. Fix before any source-completion claim.

## Scope and source of truth

Native installer stages exactly two user-visible desktop entry files:
- `native_desktop/packaging/share/applications/io.github.morozhenko_apps.RQuickShare.desktop`: `Name=rQuickShare` is a protected brand and does not require a localized override. `Comment=Share files and text with nearby devices` is visible in launcher/search metadata; localize its comment only.
- `native_desktop/packaging/share/kio/servicemenus/io.github.morozhenko_apps.RQuickShare.Send.desktop`: localize both `[Desktop Entry] Name=Send with rQuickShare` and `[Desktop Action Send] Name=Send with rQuickShare…`; retain trailing ellipsis only for the action that opens a choice/dialog. Do not translate the rQuickShare brand.

Canonical 20 supported native locale IDs: `en,ru,es,pt,de,fr,it,pl,hi,zh-Hant,ja,nl,sv,tr,ko,bn,th,id,fil,ar`. Keep English unqualified base keys (fallback); non-English localized keys use freedesktop/POSIX syntax. Map `pt` (European intent) **only** to `pt_PT`, not generic `pt`, so `pt_BR` falls back to English. Map `zh-Hant` to `zh_TW` and `zh_HK`, not generic `zh` or `zh_CN`; `zh_CN` must remain English. Other languages use their unqualified ISO language codes. The native source locale resolver is not changed.

Security-sensitive and protocol keys must remain byte-for-byte unchanged: entry groups, `Exec=/usr/bin/rquickshare`, `Exec=rquickshare send -- %U`, `Type`, `Icon`, `MimeType`, `Actions`, `X-KDE-Protocols`, min/max URLs, priority, `Categories`, `Terminal` and `StartupNotify`. No new commands, CLI arguments, shell interpolation, trust checks, file/path transformations, or side effects.

## Planned acceptance checks

- Source-package Python test parses both entries with strict duplicate detection, verifies expected sections and exact base control metadata, enumerates all required translation locale keys (including both Chinese regions) and forbids `pt`, `pt_BR`, `zh`, `zh_CN`, `zh_Hans` overrides.
- Check every localized display value is nonempty, valid Unicode with no control/newline/D-Bus markup, every Dolphin action still visibly refers to `rQuickShare`, and only the `[Desktop Action Send]` label ends in `…`. Retain differences between the service header and actionable operation.
- Native package candidate workflow should validate **both** desktop entries with `desktop-file-validate`. The package test gate remains candidate-only/manual, never auto-publish or auto-install.
- Keep docs and source in separate commits, fix the observed rustfmt assertion in same source integration, and run source-exact Native GTK + Preflight once at end. The package candidate workflow requires a separate explicit dispatch, not implied by green source CI. Installed KDE launcher/Dolphin UI, localized font fallback, RTL and editorial 20/20 semantic accuracy are manual acceptance boundaries.

Rollback: revert only localized desktop entry keys and their offline tests; preserve execution keys, packaging identity, source code runtime, package version/revision, signatures and immutable artifacts.

## Implemented source and current evidence

Source `87808038781d3552b7224d276c231bbb100eafe0` adds localized `Comment` in the application entry and both `Name` captions in the KIO service menu, 20 POSIX translation variants per surface. The native Debian packager now validates all three translation sets alongside protected base fields and explicit forbidden control characters; two offline Python tests cover source completeness, binary/command preservation, and negative tampering/missing-locale scenarios. The candidate-only native package smoke workflow now invokes `desktop-file-validate` for **both** assets. Also corrected the exact prior Native GTK nightly formatting finding in `i18n/tray_chrome.rs` without changing labels.

The follow-up documentation commit contains no production changes. The source-exact native GTK and Preflight runs triggered by the source commit must complete before the source gate is credited; the manually dispatched package-candidate gate is intentionally not part of the automatic push validation. Neither automated i18n inventory nor uninstalled package validation proves installed KDE/Dolphin behavior or native-speaker context acceptance.

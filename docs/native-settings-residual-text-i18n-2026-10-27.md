# Native Settings residual static text localization, #17

Date: 2026-10-27. Primary MODE A, source-first audit / bounded feature implementation. Baseline `dev` `fce92e067d2a7d856bbe1fa6c1f845d76460e746`. User wants all pre-Mode-B work except #8 accepted exception and #16 approved Google-certificate A left OPEN/blocked; do not run final Mode B, publish or install.

## Repository scan and existing proof

Latest executable `84bcff96dbd56d93c2693871ca32fd71c35a560a` has source-exact Preflight `40563016992` SUCCESS and Native GTK `40563016773` SUCCESS. The 20 canonical locales and English fallback are in `native_desktop/src/i18n.rs`; settings chrome, safety warnings and 32 independent status/error branches already have separate catalogs.

Exact source `native_desktop/src/panel/settings.rs` (1,263 lines) still displays **15 unique static UI messages** in English through `label`, `set_text`, `set_label` or GTK window content. Audit scope is precisely:
- UI static guidance: `Leave empty to use the computer name. OK applies changes to new connections.`, `Starts sharing and the tray at sign-in, with the card hidden.`, `Default Downloads folder`, `Send and receive nearby files and text.\nGNU General Public License v3.0.`
- State/action captions: `Changes will apply after OK.`, `Start sharing`, `Waiting for login settings…`, `Sign-in choice will apply after OK.`, `Daemon and tray will start at sign-in; the card stays hidden.`, `Login start is off.`, `Login settings are unavailable.`, `Waiting for the service…`, `Sharing restart requested. Retry the connection from the card to read its state.`, `Choose an accessible local folder.`, `Cannot open the folder chooser.`
- The initial `Sharing settings are unavailable.` is an already defined `settings_state` source key: REUSE, no duplicate catalog key. The dynamically resolved device name and directory are already display-only FSI-isolated; preserve that architecture. The product name `rQuickShare`, program version, `CARGO_PKG_VERSION`, `general`/etc stable widget identifiers and the `GNU General Public License v3.0` legal designation must not be modified.

## Implementation contract

1. Add scoped `native_desktop/src/i18n/settings_residual.rs` holding exactly 15 static English source keys, 19 model-assisted draft locale rows and fallback to the source English string. No dynamic text lookup, no FFI/provider changes.
2. Update `i18n.rs` dispatch and cross-catalog collision tests. Update exact GTK callsites (two Default Downloads folder displays, constructor initial unavailable and login default reuse existing settings state). The `refresh_login` state decision and `refresh` Start-vs-Restart condition must remain byte-for-byte the same conditional logic; only return captions are translated.
3. Regression tests: 20-locale nonempty, no duplicates, source English fallback for unsupported pt-BR/zh-Hans, exact RU/PT captions, legal label retains `GNU General Public License v3.0`, `Start sharing` not equal `Restart sharing` from existing settings chrome, pending/login-off/unknown distinct and no 'saved and applied' semantic collisions. Retain selected visibility indices, saved user data, port parser, status matcher, login controller, directory chooser capability, explicit per-transfer consent.
4. Exact source Native GTK strict formatter, Clippy, widget tests + Preflight checks *after last executable edit*. No 'green' before terminal source-exact job results. Never run full Mode B as part of this work, no release, master, tags, R2 or installed package.

## Risk and rollback

Literal source keys are consumed only through `i18n::label(&'static str)`; the original `&str` statuses and `glib` closures stay unchanged. Translation errors can confuse action state and privacy warnings; tests forbid key collisions but do not replace human review. RTL/GTK narrow-card and PT-PT/Traditional Chinese semantics still require acceptance. Roll back this one executable source commit independently. Subsequent GTK rows/reveal/tray i18n remain in #17.

Status before executable code: NOT IMPLEMENTED.

## 2026-10-27 implementation checkpoint

Executable `723e7c033f87082b8e070d2feaf57832eaeba68a` adds `native_desktop/src/i18n/settings_residual.rs` (15 distinct static English keys × 19 non-English draft rows = **285 translations**), registered in `src/i18n.rs` with cross-catalog collision/Start-vs-Restart invariants and two direct locale/semantic tests. `panel/settings.rs` now routes **18 direct display sites** through `i18n::label`, including a two-line GPL license text, default Downloads label (two sites), on/off/unknown/pending login messages, Start sharing vs Restart sharing, help on user-input name, Settings Save/Reload/Folder Picker. Initial unavailable Settings caption reuses the existing SettingsView catalog, without creating a duplicate source key. The product name and GPL legal designation are unchanged.

A source-structural pass independently checked 19×15 translated rows, canonical 20 locale registration, key routing and visible Start/Restart captions. Unmodified critical code: `status(SettingsView)`, `failure(ControllerError)`, `login_failure`, `confirmation_applied`, user `device_name`, `gio::File` capability, input/port/visibility and explicit actions. This is source-only evidence; no native-speaker approval claimed.

Source-exact **Native GTK `40571862841`** and **Preflight `40571863031`** were in progress at checkpoint, not PASS by inference. No final Mode B, physical KDE/Pixel, Windows, master/tag/R2/install actions. #17 remains OPEN until transfer rows, reveal, notifications/tray/a11y/packaging and RTL language review are complete.

## Final source-exact Settings proof (2026-10-27)

Formatter-only follow-up `dc1da6a700087f8106b8748a05be9150098b0cd2` validated with **Native GTK `40572346978` SUCCESS** and **Preflight `40572346805` SUCCESS across all five jobs**. Source `723e7c03` initially failed Native GTK `40571862841` in seven strict `rustfmt` layout differences; no tests were run in the failed invocation. Exact formatter correction made no behavior, catalog value or assertion change. The new transfer-row source `3fceee03` starts a separate source-exact validation; no inference of pass there.

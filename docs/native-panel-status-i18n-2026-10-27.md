# Native GTK panel status projection localization (#17)

Date: 2026-10-27, MODE A. Documentation before source. Direct `dev`, source baseline `2137bd24fe27fd28a9e3a07ec6a08a7356153c84`. Previous operation notice source CI Native GTK 40592290719 and Preflight 40592290693 were **IN PROGRESS** at this planning checkpoint; no green claim, no master/tag/release and no final Mode B.

## Exact source inventory

`native_desktop/src/panel.rs::render` selects the following **twelve** status texts after checking its live model, before writing `gtk::Label status`:
1. `Preview only · service actions disabled`
2. `Transfer service unavailable. Check installation, then reconnect.`
3. `Service connection lost. Reconnect to review its current state.`
4. `Receiving files from the local sender. Abort to release this preparation.`
5. `Preparing selection…`
6. `Preparation failed. Review service state or select accessible files.`
7. `Visibility change outcome is uncertain. Use Retry visibility change.`
8. `Visibility was recovered as hidden. Review service settings before enabling discovery.`
9. `Transfer service is stopped. Open Settings to repair or start it.`
10. `Accepting request…`
11. `Declining request…`
12. `Cancelling transfer…`

These direct literals do **not** currently occur in the native i18n catalogs. A separate ten-message `panel.rs::error_message` ClientError map, two completion-save errors, empty-service views, inbound/outbound/tray/notification content and desktop launcher remain separate later inventory/implementation. This stage does not change any ClientError or status precedence.

## Proposed implementation and invariants

- Add pure `i18n/panel_status.rs` with twelve immutable canonical keys and 19 additional draft translated rows (228) across the existing twenty canonical locales. English and unsupported pt-BR/zh-Hans fall back to the exact English source. Use European Portuguese and context-accurate Arabic RTL, distinguish only waiting/pending from finished/success, and preserve explicit *hidden* and *uncertain* semantics.
- Wire the catalog in `i18n.rs`; the existing `Panel::render` final `crate::i18n::label(status)` boundary (introduced in `2137bd24`) performs display translation. **Do not edit** `Panel::render` logic, visibility/Receive lifecycle, empty/error guard order, daemon connections, transfer control, user consent, retry strategy, settings state or files.
- Full locale matrix: all 20 keys nonempty/unique, direct translations equal intended RU/PT/Arabic oracles, unsupported locale English fallback, unknown source does not invent a translation, no cross-catalog key collisions. Every locale preserves distinction among Preparing/Failed, uncertain/recovered-hidden, and accepting/declining/cancelling; no translated pending action claims completion. Existing live GTK integration controller suite verifies original state selection and expiration/consent behavior.
- Final exact-source Native GTK nightly rustfmt, strict Clippy, UI/widget unit test suite and Preflight only once after planned edits; isolate known flaky group and full Mode B later. Record exact SHA/check results, no inferred installed-physical signoff.

Rollback is the new catalog and i18n registration only, with status branch model unchanged. Physical Pixel/KDE, RTL semantic review, Windows acceptance, Google-compatible verified identity #16 and the explicitly retained #8 A2.2 risk remain independent OPEN gates. No user authorization to publish 1.0.0 is inferred.

## Initial implementation checkpoint

`5a29c3ccb8d8803fefea7e83df633847ed9119c6` adds `src/i18n/panel_status.rs` (twelve canonical status keys, 19×12 =228 non-English draft translations, two deterministic unit suites) and adds pure catalog routing/collision tests in `i18n.rs`. Existing `panel.rs` and other daemon/control modules byte-identical to previous release slice. Independent structural audit: 19 locale rows; each includes 12 unique, nonempty values and no interpolation placeholders; English and unsupported locale fallback remain exact. Final source-exact [Native GTK 40593845460](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593845460) and [Preflight 40593845531](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593845531) **IN PROGRESS**. No success claim until both terminal on exact source. Other failures or unrelated physical gates not reclassified.

## Strict formatter correction, exact source gate

Source `5a29c3cc`: [Preflight 40593845531](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593845531) completed **SUCCESS**. [Native GTK 40593845460](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593845460) **FAILED in strict rustfmt only** at the six-element difference-pair array in the status unit test; Clippy/widgets never ran on that SHA. Formatter-only `75c8e4e5d4cee82aab2299e181379fffef597061` preserves the exact twelve keys, 228 translations, all assertions and executable status selection. New exact [Native GTK 40596180592](https://github.com/morozhenko-apps/rquickshare/actions/runs/40596180592) **SUCCESS**. Matching [Preflight 40596180519](https://github.com/morozhenko-apps/rquickshare/actions/runs/40596180519) **PENDING**, no combined PASS yet. Human RTL/context/device verification is not implied.

# Native GTK settings: direct validation and consent warnings, #17

Date: 2026-10-27. MODE A source-only bounded localization; baseline dev `743eea77a154c501699c5d7f8ed89ff3ef871c2b`. Last exact executable preview test fixture code `1fe649ed` has **terminal Preflight 40539063998 SUCCESS and Native GTK 40539064009 SUCCESS**. This is not a final 1.0.0 device or Mode B gate.

## Architectural scope

The earlier static Settings chrome and status/error catalogs (25 + 15 + 17 keys across 20 locales) do not cover direct GUI validation/consent failures in `native_desktop/src/panel/settings.rs`. Existing controller `status(SettingsView)`, `failure(ControllerError)`, `login_failure(login::Error)` and save/login operation handlers MUST remain exactly unchanged.

This slice adds **12 exact, security-relevant static presentation labels**, one scoped module `native_desktop/src/i18n/settings_warning.rs` (19 draft non-English rows plus English fallback), and wires GTK literal displays to `i18n::label`. The source English keys, neither arbitrary variables nor error tokens, are:

1. `Each incoming transfer still requires your confirmation. Folder changes affect new requests.`
2. `Leave empty or use 0 for automatic selection. A changed port requires restarting sharing when idle.`
3. `Sign-in state changed or is unavailable. Reload it before confirming.`
4. `Device name is too long or contains unsupported characters or surrounding spaces. Leave empty for the automatic name.`
5. `Sharing settings are unavailable. Retry the connection from the card.`
6. `Sign-in change was not confirmed. Any acknowledged sharing settings remain saved.`
7. `Choose a visibility setting before saving.`
8. `Port must be empty or an integer from 0 to 65535.`
9. `Cannot create a save identity. Try again.`
10. `Cannot retain the selected folder. Choose it again.`
11. `Settings acknowledgement was incomplete. Reload before making another edit.`
12. `Cannot acquire the selected folder. Check access and choose again.`

## Risk / invariants / tests

- Preserve independent **explicit user confirmation** for every inbound transfer. Folder change must not claim old/pending requests retarget automatically; leave existing Receive dialog and `SettingsEdit` branch untouched.
- A port value accepts existing validation only (empty, 0, or digits 1–65535); never parse localized digits into config or change number bound. All translations of the port error must contain exact `0` and `65535`.
- Ambiguous login/settings acknowledgement cannot become successful application; source `confirmed`, callback ordering, `revision`, request identity, `folder chooser`, `login_dirty`, `recover` and restart remain unchanged.
- All 12×19 translated draft values complete, nonempty, no placeholders or markup, each security-sensitive error distinct from normal successful status. Source-level tests verify English fallback (including unsupported pt-BR/zh-Hans), menu disjointness, RU/PT literal examples, required port constants, and distinct consent/ack/restart statements.
- Direct GTK `set_text`, `label` and warnings only. Do not alter source payload, button action, backing files, untrusted paths, app schema, daemon, wire, identity or installation. Future dynamic value `Name for new connections: {}` uses typed bidi-safe projection in a separate slice.
- Exact-source strict nightly Rustfmt, stable Clippy and Xvfb GTK widget tests plus Preflight after last source edit. Non-flaky stable first; full MODE B, device Pixel↔KDE validation and native-speaker review separate. No claims of 20-language semantic approval.

## After this slice

Still untranslated in Settings are general advice, default folder caption, login pending/on/off captions, folder chooser errors, start/restart conditional title, direct waiting/operation states, About/license descriptions, dynamic resolved name/path. Other GTK windows/tray, notification/accessibility/desktop-menu locale coverage still need audit. Issue #17 remains OPEN/PARTIAL, and #16 identity and open P0/device-acceptance issues remain release blockers or explicit review items. No release, master, tag, R2 or local install mutations.

**Precode checkpoint:** NOT IMPLEMENTED, no source-specific CI for this new slice.

## Implementation checkpoint and source-exact validation (2026-10-27)

- Code-only implementation `7b8cc5991dc25aa6e6d14abefc1073f4bb390a61`: `native_desktop/src/i18n/settings_warning.rs` holds 12 exact canonical warning/consent sources ×19 non-English locale rows = **228 draft translations** (English unchanged). `i18n.rs` dispatch and all 13 GTK occurrences (one validation message is shown twice) use the catalog. Five direct warnings, validation and consent branches remain in place; actual `status()`, `failure()`, `login_failure()` selector functions verified byte-identical with original sources. The consent, port parsing, folder capability, login and storage actions were not modified.
- Static independent wiring audit PASS: 19×12 rows, 12 source keys, 12 exact GTK messages, complete catalog dispatch, caller source semantics unchanged; all canonical locales represented. Two unit regression functions check disjoint catalogue identifiers, positive RU/PT message meaning, unknown-locale English fallback, nonempty translations, port `0`/`65535` and negative consent/error collisions. **Human linguistic and physical layout/RTL validation not done.**
- Native GTK source-exact `40547964619` on `7b8cc599` **FAIL** in `rustfmt` only: two locale assertions and six GTK `set_text` call layouts. This failure stopped before Clippy/test execution; it is not evidence of an app runtime defect. Format-only follow-up `ac152f6941daaa4abd599351a14f77a8f63a8c86` adopts the runner's exact diff with no behavior or translation changes.
- New final executable-source **Native GTK `40548418258`** and **Preflight `40548418257`** triggered on `ac152f69` and were **IN PROGRESS / QUEUED** at this checkpoint. Do not count as success until terminal outcomes checked. Original Preflight `40547964628` may become cancelled/superseded; it does not validate final executable code. No Mode B, installed-package test or publication.
- Follow-on inventory: direct Settings pending/start/login/folder-advice messages, dynamic `Name for new connections: {}` and `directory_display` RTL projection, other GTK module/tray/a11y strings, packaging/desktop locale strings, native review 20/20. Issue #17 OPEN; #16/8 and the P0/physical follow-ups remain separately open. No release gate was waived.

Rollback: inverse source commit from `ac152f69` without altering existing settings or user data. No `master`, tag or R2 changes.

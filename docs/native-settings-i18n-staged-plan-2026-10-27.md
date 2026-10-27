# Native GTK settings localization: staged plan, #17

Date: 2026-10-27. MODE A source audit only. No settings executable changes in this checkpoint. Current dev `f08ae3cc3ec4e599c282b491ea5b3c93ff1e59d9` follows image-localization code `9e6251d3`, which already passed its Native GTK gate `40511449070`; source-exact Preflight `40511449089` still in progress at audit time.

## Source and safety model

`native_desktop/src/panel/settings.rs` has ~1,235 lines and around 44 candidates for visible captions, labels or status strings. Settings are **not** a simple text dialog: GTK retains unsaved intent while daemon owns persistence/apply; there are separate login-start and service-settings actions, recovery backup/replace, folder chooser and restart. Save receipt may be indeterminate and must never be mistranslated as `Saved and applied`. Source `status(SettingsView)`, `failure(ControllerError)`, `login_failure(login::Error)`, `selected_visibility`, `parse_port`, `confirmation_applied` and completion callbacks are existing authoritative contracts.

Split further localization into independent, reviewable slices:
1. **Static settings chrome (first)**: title/section names, action captions, entries/dropdown prompts, user-facing explanatory descriptions and menu labels. Catalog using one new `native_desktop/src/i18n/settings.rs` internal module for the exact same 20 canonical language codes and English fallback, avoiding uncontrolled replacements.
2. **Status and error messages (second)**: `status`/save/login/error/restart/folder warnings and recovery copy messages. Pass the *returned static source literal* through `i18n::label` at GTK call sites, never change the error/confirmation branch mapping. Make explicit proof that unknown results never claim saved/applied. No extra implicit retries, Save/Restart or uncontrolled settings writes.
3. **Dynamic user values/RTL (third)**: visible device name, directory path and `Name for new connections: {}` text. Use existing presentation-only `i18n::isolate_untrusted` and a typed message builder instead of interpolating raw file paths into a translatable static key; do not edit underlying value or permission. Preserve no markup.

### First static inventory (representative; verify exact source before edit)

The current top layout includes: `rQuickShare Settings`, `About rQuickShare`, `Select visibility…`, `Visible for one minute`, `Automatic computer name`, `Automatic`, `Device name`, `General`, `Visibility`, `Receive folder`, `Receiving`, `Port`, `Advanced`, `About`, `Start at sign-in`, `Apply login start`, `Reload login state`, `Reload saved values`, `Retry pending save`, `Restart sharing`, `Choose folder…`, `Use default`, `Back up and replace damaged settings`, `Connect to read the receive folder`, `OK`, `Cancel` and multiple privacy/permission descriptions. Existing `Visible`/`Hidden`/`Cancel` and potentially `About` must be reused only where source keys already exist; never duplicate them. Keep `gtk::DropDown::from_strings` index order and selection code 1..3 unchanged.

## Invariants and negative matrices

- S1: no invisible write, save, restart, login start or folder change from localization; the existing explicit action handlers remain sole owners.
- S2: `login_failure(Error::Uncertain)` and settings `StoreError::Indeterminate` must never display a successful save label. Only `confirmation_applied` can confirm final state, same as original code.
- S3: visibility selector retains the same index→value mapping `1..=3`; default/hidden/one-minute never reorder for visual sorting. Invalid selection still blocks save.
- S4: `Port` validation retains empty/0..65535, no translated numeral conversion to wire/input values.
- S5: damaged settings recovery requires existing explicit backup-and-replace consent; translations must not call Reset or imply backup already exists when it failed.
- S6: status text privacy: no settings tokens, folder paths, device names, request IDs or service credentials passed through static catalog, markup or logs.
- S7: 20 complete locale rows per new key, unknown territory fallback/English, single canonical key per distinct status. No machine-assisted string is labeled human-reviewed.
- Regression inventory: positive menu/tab rendering and RU/PT action terminology; N1 unknown locale, N2 unsupported pt-BR/zh-Hans, N3 invalid dropdown, N4 bad port, N5 directory denial, N6 damaged settings backup required, N7 ambiguous save, N8 stale revision/conflict, N9 missing/unavailable login service, N10 no confirmation, N11 long translated captions, N12 Arabic bidi and untrusted values. Coverage audited against each changed source file after final CI.

## Execution and rollback

Confirm final image CI before starting settings code, then add first small static catalog/action slice. Separate `docs:` and executable `feat:` commits using optimistic `dev` HEAD; execute final stable CI only after that slice's final code edit. Maintain historical CI receipts in the live backlog. Roll back localization without touching persisted settings. No protected `master`/tag/APT/R2 or installed package changes and no 1.0.0 completion claim.

**Current state:** planning only, settings localization NOT IMPLEMENTED. #17 OPEN/PARTIAL.

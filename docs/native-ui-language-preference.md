# Native GTK language preference: client-owned, owner-approved option 1

## Status and decision

2026-10-29, MODE A. Owner selected option **1**, a persistent GTK-only UI language preference. This is separate from Core/daemon settings, wire protocol, peer identity, consent, receive folder, sign-in activation and any release/signing/R2 changes.

Current i18n in `native_desktop/src/i18n.rs` resolves only environment variables (`LC_ALL`, `LC_MESSAGES`, `LANGUAGE`, `LANG`) at each lookup. Existing UI strings are snapshot at widget construction. Merely adding a dropdown without repainting static chrome and asynchronous tray menu would be misleading. The catalog supports precisely: `en, ru, es, pt, de, fr, it, pl, hi, zh-Hant, ja, nl, sv, tr, ko, bn, th, id, fil, ar`. `pt` is PT-PT and `zh-Hant` is Traditional Chinese; no inferred pt-BR/zh-Hans.

## Contract and stages

1. **Local state only**. Implement a dedicated, bounded, owner-private UI preference (System default or one of the 20 locale IDs). No additions to `SettingsEdit`, daemon JSON/schema, D-Bus or service methods. Determine system fallback with the existing locale resolver. Reject invalid/oversized preference content without unsafe default rewrites. Persist atomically, and never leak configuration, paths or device identifiers to logs. Do not allow untrusted paths or data to select UI resources.
2. **Staged Settings → General**. Add `Language` and `System default`, with 20 human-readable language names. Editing alone never changes the currently active locale. Cancel or window close discards it. An explicit successful OK persists the choice; errors retain the dialog and prior language. Language-only changes must work independently of daemon availability and never cause daemon settings writes, login-start toggles, replayed transfers or restarts.
3. **Live client retranslation**. Changing the accepted language must update current card, menus and tray labels and any existing secondary windows without reconstructing the transport/consent owner or interrupting transfers. Dynamically rendered text uses the current language. Static labels and accessible descriptions need explicit stable key bindings rather than reverse-guessing a translated string. Preserve focus, unsaved dialogs, selection, pending operations and card placement. If any surface cannot be updated safely, disclose it as incomplete rather than claiming live 20-locale coverage.
4. **User-requested UI polish, separate executable commit if needed**. The Settings About tab, Settings in-body header/logo and redundant successful save footer are scheduled for a scoped layout change, while preserving warnings, failed/indeterminate save states and recovery status. About remains a separate app entry and retains upstream GPL attribution; author identity must not misstate upstream copyright. Window decorations and icon should be tested on installed KDE; do not assert titlebar behavior from isolated GTK tests alone.
5. **Independent R2 User-Agent audit**. Compare the actual release upload HTTP client/headers with CFVigil's corrected upload path. Do not assume a corresponding bug before finding the sending call and origin/server expectation. No R2/key/bucket/publication changes in this task.

## Risk, branch inventory, and tests

- Selection: System default; each of the 20 explicit codes; unsupported/empty, region aliases; stale saved preference; invalid preference syntax; absent home/config directory; readable vs unreadable/oversized file; concurrent app instances; atomic write failure, insufficient permissions; cancel/close/OK and daemon offline.
- Language correctness: RU and PT-PT source-key smoke; unknown i18n source fallback; dynamic values and filename bidi isolation; Arabic RTL and narrow strings; reopening Settings/About, live card, DBusMenu revision and context menu refresh. Do not translate peer-provided values as localization keys.
- Safety: zero change to current `SettingsEdit`, login-start, transport, auth, identity, daemon ownership, consent, queue/order, file paths/content. No external network or release action when switching language.
- Final gate after last source edit: targeted pure parsing/persistence tests, GTK Settings staging tests and main client/snapshot tests, nightly rustfmt, strict native Clippy and STABLE suite once, separately tracked FLAKY tests. Preflight/Native GTK CI only at final source gate. Physical KDE 20-locale/RTL/user acceptance and untranslated surface inventory remain independent: `#17` and T14 are not complete from CI alone.

## Rollback / handoff

Separate `docs:` from code/test commits. For code rollback, revert only the client locale module and its GTK bindings, restoring environment-only language resolution; do not touch daemon settings. No master promotion, protected release tag, CI-triggered distribution or R2 writes without separate explicit authorization. Record evidence (exact SHA and test outcomes), and mark any unfinished integration as PARTIAL rather than completing the release.

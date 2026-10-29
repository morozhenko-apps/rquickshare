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

## Implemented source checkpoint (unverified CI)

Dev executable checkpoint: `a90ccb3a72518ba8f5589dad83b6ddf56ee55683` after the staged locale/UI/R2 source changes. Implemented `src/ui_language.rs` independent bounded file preference, startup read and atomic locale switch, Settings General dropdown, Cancel/OK staging, GTK card/context-menu refresh and a dedicated DBusMenu revision notification. Closed Settings is reconstructed on next open after a confirmed language change. Added parser, preference roundtrip, invalid/symlink, menu revision and GTK Cancel smoke assertions. This is NOT terminal rustfmt/Clippy/GTK CI evidence; no host compilation or installed runtime test occurred in this session.

The earlier separately requested Settings UI cleanup removes in-body duplicate product branding, About page and success-only footer. Separate About is updated to credit morozhenko apps and the native fork while retaining upstream provenance and GPL. Titlebar icon is set; physical KDE maximize/minimize decoration remains unverified.

Known incompleteness: the current `localize_panel()` updates only selected fixed GTK labels; other already-open secondary windows and hard-coded/uncataloged captions may remain in their old language until reopened. Human semantic review of all 20 languages, RTL/layout/long strings and the installed native UI gate are NOT complete. Do not close issue #17 or T14 or call this 1.0.0 signoff. CI must be evaluated at the final source gate; if it finds a compiler/test defect, apply root-cause protocol before generating a new candidate.

No master/tag/signed APT/R2 public release. Operator's 13/14 physical PASS belong to **previous** installed native 0.9.0-2, not this modified dev code.

## Owner's localization acceptance decision (2026-10-29)

The owner rejects a requirement for a human reviewer to manually switch through all 20 languages. Replace that requirement with an agent-led, source-based audit of the entire locale catalog and targeted automated semantic/invariant checks. Do not claim a native/fluent linguistic proof from table-shape checks alone. The physical operator acceptance is limited to the installed application's language switching, persistence, layout and consent behavior (representative RU/PT/Arabic if useful), not 20 human locale sessions. Keep built-in English fallback, bidi isolation and security wording exact. Never close #17 solely because the 19 translated branches exist.

Audit checkpoint: 19 static translation modules contain 206 unique source keys; every module has 19 non-English locale arms with the expected fixed row length, no missing or blank translated row. Additional modules are dynamic.rs and transfer_dynamic.rs and the main i18n.rs catalog; they require their own dynamic-format and semantics checks. English-identical terms such as OK/Port/File are not automatically errors. Major GTK i18n::label literals reviewed in panel, Settings, menus, history, rows, text and image composer; user language controls are in a client-only catalog. Human all-locale testing removed from the completion criteria; exact-source CI, semantics, RTL and installed behavioral acceptance remain mandatory.

## MODE A secondary-window retranslation remediation (2026-10-29)

Root-cause audit: the live card and GTK/DBus menus refresh after a confirmed locale save, but secondary windows snapshot localized GTK strings at construction. History stores neither its Refresh button nor its former title key, and Review/Composer/Image retain status strings as rendered translations rather than stable message IDs. Changing the locale while any of those windows is open leaves old-language chrome. This violates the already approved live-client presentation contract without touching the daemon.

Plan before code:
1. Keep all window and operation owners alive. Add explicit stable-key controls and a per-window `retranslate()` method, not arbitrary GTK widget tree scanning. History title, Refresh button and already-recomputed journal statuses refresh in-place. Text/Composer/Image need stable source keys for statuses; captured image dimensions remain untrusted data displayed as values, never used to select translations.
2. Call those methods from `Panel::retranslate` before the ordinary dynamic render. Preserve prepared source, clipboard text, consent, current image, event handlers, focus, and ownership. Update all four windows even if currently hidden; never reconstruct or auto-close a live editor.
3. Add unit/GTK tests for surviving handles, changed titles/chrome and no change to selected contents/action sensitivity; ensure language-only change remains independent of daemon. Keep manual 20-language review forbidden; automatic semantic/structure audit plus installed representative UI checks.
4. At final gate, nightly Rust formatting, strict Clippy and native GTK widget suite, Python native release tests, and one stable Preflight. If any test fails, use the failure log to diagnose and fix root cause without masking it. Build 0.9.0-3 only after green source gate.

No changes to public API, daemon persistence, IPC, authentication or transport. Rollback: revert secondary-window methods and call sites independently from the previously authorized client-only preference store. No master/tag/R2 signing or publication.

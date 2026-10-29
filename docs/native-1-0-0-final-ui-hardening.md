# Native 1.0.0 finishing: cached empty state, About and launcher name

Date: 2026-10-29. Owner accepted 0.9.0-3 on Ubuntu KDE + Pixel 6a: five reported PASS categories (Settings/locale staging, open auxiliary windows, tray/visibility, bidirectional text+image, Decline/Cancel outcomes), followed by a screenshot of an English placeholder beneath Russian quick buttons. This single failure overrides any earlier claim of complete 20-locale UI acceptance.

## MODE A root-cause and decision

- `native_desktop/src/panel/rows.rs`: `render_rows` caches `Entry` for empty pages by a stable key `empty:{selection}:{English source}`; `or_insert_with` creates a GTK label using `i18n::label(title)` only once. When the user changes an explicit locale, `Panel::retranslate` invokes `render` but the cached label persists in the previous language. `i18n/row_empty.rs` **already has every locale including Russian**; adding translations would not fix the bug.
- Proposed remedy: preserve keyed empty card and *update the label on every render from the stable source key*. No use of translated text as a lookup key, no clearing of device/transfer caches, no impact on consent/control handlers. Factor a pure-ish GTK label synchronization helper or directly refresh an accessible existing widget with a tested invariant.
- `native_desktop/src/panel/menu.rs::show_about` currently builds a standard GtkAboutDialog with `.authors([...])`, creating the unwanted Credits UI. Remove `authors` metadata, while **keeping GPL-3.0-only, upstream fork attribution and author identity in the main descriptive text**. Change program name to `rQuickShare Native`. Remove Linux-only phrase; explicitly credit Stanislav Moseyko (Стaнислав Мосейко) and Martichou/rquickshare as the upstream base. Keep the developer website and icon. Translation of About description must follow the same active locale as the client; in-place `Panel::retranslate` refreshes any already open About dialog.
- `native_desktop/packaging/share/applications/io.github.morozhenko_apps.RQuickShare.desktop`: change visible `Name=` to `rQuickShare Native` so the KDE application-launcher title beneath the icon matches the actual product. Keep Exec, icon, desktop ID, D-Bus integration and package IDs stable. Do not rename executable or break installed launcher migration.

## MODE B tests / invariants

1. Cached empty row: initial selected locale and second different locale must repaint *the same owned widget* with no invalidation of transfer identity, loss of incoming control, accessibility or selection. Empty states nearby/incoming/transfers must all change correctly.
2. About: no authors/credits list, no Linux-exclusive text, exactly identified author, upstream fork and GPL; title version and icon consistent; existing About dialog reflects selected language on change. No unsafe markup or external links added to messages.
3. Desktop: exact Name field, unchanged Exec/Icon/desktop DBus ID, desktop-file-validate passes and installed package places the same launcher.
4. Structural locale audit for new About strings: English + canonical 19 locales, no empty source, all contain immutable author identity and upstream provenance, no user-controlled names, English fallback. Previous 20-human-language review remains explicitly forbidden.
5. N1 invalid locale, N2 long label and narrow card, N3 daemon offline, N4 repeat locale A-B-A, N5 reentrant render, N6 malformed locale prefs, N7 dynamic text vs trusted IDs, N8 RTL, N9 close/reopen, N10 redaction, N11 N/A (no billing), N12 persistence. Interaction: drawer open/closed × empty/pending/active × locale × existing/uncreated About.
6. Preserve 0.9.0-3 for rollback. Build new **0.9.0-4 candidate** after exact-source Preflight/GTK, rather than silently changing already accepted 0.9.0-3 bytes. Promote to product 1.0.0 only after explicit separate source/package and physical gates; do not tag/publish/sign/R2/master without authorization. This task does **not** touch frozen #8 or #16 and does not claim general Android/Windows trust completeness.

## Completion gates

Source changes and tests tracked in dedicated code commits; this document in a separate doc commit. Mode B inventory and mutation review required; exact CI green not inferred until verified. Distinguish source and hardware evidence, and stop after triggering final CI. No release deployment authorized.

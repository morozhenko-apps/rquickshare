# Native tray visible chrome localization (#17)

Date: 2026-10-28. MODE A. Direct `dev`, independent documentation and code commits. Scope: StatusNotifierItem tooltip and DBusMenu labels/counts only; source baseline `bcfb990a692e72e1f4f251557703cd7d53f26713`.

## Source inventory and existing decisions

- `native_desktop/src/tray/menu.rs::Action::label` already calls `i18n::label`, but nine distinct user-visible keys lack catalog entries: `Open card`, `Send text…`, `Send image…`, `Retry visibility change`, `Send a folder…`, `Incoming requests`, `Compact view`, `Full view`, `Reconnect service`.
- `menu.rs::properties` independently constructs English `Hide card` (already localized elsewhere) and two English count captions: `Incoming requests (N)`, `History (N)`. The latter base word is already cataloged; do not duplicate it. Counts are a privacy-free in-memory `Projection`, not unread notifications or historical-success claims. Preserve the exact counters.
- `native_desktop/src/tray.rs::Item::tool_tip` exposes `Send and receive nearby files` untranslated; its icon name, item ID, title, category, SNI status and D-Bus interface identifiers are **not** user-translatable protocol fields.
- The GUI uses a daemon-owned custom-card notice model. This task is tray chrome localization **not** new desktop system notifications, provider ownership, consent actions or activation behavior.
- Source `bcfb990a` Preflight `40645441436` **SUCCESS**; Native GTK `40645441460` **FAIL** exclusively on two exact nightly rustfmt diffs in `panel/tests.rs` and `panel.rs`. Apply both production-compatible diffs as part of the next source commit before the final workflow gate. No widget/compiler PASS asserted for that source.

## Implementation and test sequence

1. Correct the two source-exact `rustfmt` differences without changing data, test assertions or transfer actions; keep a rollback boundary distinct in changed-file scope.
2. Add an isolated tray-only static catalog for the ten missing captions (nine actions + tooltip), with the canonical 20 `LOCALES` and deterministic English fallback. Reuse `i18n::label` and the established `History` translation; never insert duplicates or alter metadata/control identifiers.
3. Use a typed `TrayCountKind` display function to append exact decimal `0..=u16::MAX` counts without pluralization claims, unrelated numeric rounding or changes to the projection. Use the same function in DBusMenu `properties` for incoming and history; reuse the existing `Hide card` translation for dynamically toggled Open.
4. Translate only SNI `tool_tip` description, preserving icon, title and protocol status fields. No new system settings, provider lifecycle, network I/O, secrets, permissions, wake scheduling, untrusted peer values or IPC action writes.
5. Pure locale matrix tests: 20/20 key coverage, no duplicate keys, absent/unknown fallback, context-distinct labels, count boundaries and both kinds; existing private-broker DBusMenu and SNI tests continue to assert exact menu property and action policy. Run source-exact Native GTK (nightly format, strict lint, GTK widget tests) + Preflight **once** at end. Source green is not KDE physical, RTL, native-speaker, or installed-menu acceptance.

Rollback: drop the new tray catalog and formatter calls, revert the three bounded presentation callsites; preserve all action IDs, enabled guards and daemon notice lifecycle. Do not modify master, releases, signed packages or installed assets. The native desktop launcher/servicemenu translation and semantic review remain separately scoped.

## Implementation checkpoint

Production `f34b24f57cbb2ba27ab4d43f5c39866ffd667882` contains `i18n/tray_chrome.rs` (ten caption keys × 19 translated rows, English fallback), typed projection count display, translated SNI tooltip and DBusMenu dynamic menu labels, plus catalog and existing broker/GTK-facing test updates. Includes only the two literal formatting corrections required by the previous exact Native GTK run. No change to D-Bus identity, enabled action flags, process activation or transfer state. Source-exact workflows were triggered by the source commit; their results are not yet accepted. Installed Plasma menu, Arabic RTL and editorial review remain distinct gates.

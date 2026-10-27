# Native GTK history metadata localization, issue #17

Date: 2026-10-27. MODE A feature slice. Baseline dev `21e119a2c1d7e01f193eeeea4719ee2a3ccb498f`; previous received-text review and composer slices merged, with exact-source Preflight and GTK workflow results SUCCESS on executable SHA `9cfe63fa`. No open PR was observed at the start.

## Source inventory and architecture

`native_desktop/src/panel/transfers.rs` renders an independent GTK history metadata window. Eighteen new **static English labels** are uncatalogued: `Refresh`, `Clear completed`, `Reset history`, `Service disconnected. Live states may be stale; reconnect before controlling a transfer.`, `History could not be saved. Sharing is still available. Refresh to retry, or reset history while preserving a recovery copy.`, `Loading history…`, `Saved history is unavailable with this service version.`, `History could not be read safely. Refresh to retry; sharing remains available.`, `Completed metadata is kept for up to 30 days and 1000 operations. File and message contents are not stored.`, `Unnamed device`, `Completed`, `Declined`, `Cancelled`, `Failed`, `Disconnected`, `Interrupted`, `In progress` and `Metadata only. Content review and file actions require the original live transfer capability.`. Existing `History`, `Receive`, `Send` MUST reuse MENU_KEYS.

Add a **separate internal source module** `native_desktop/src/i18n/history.rs` for its 18-key translation table rather than growing the existing 1,159-line `src/i18n.rs` with 19 more giant match arms. Preserve the 20 canonical locales, locale selection/English fallback, secure error semantics and current GTK action callbacks. Add no dependency, settings, persistent schema, IPC or protocol change. `i18n::label` remains the single label lookup; journal codes are mapped to static English source keys *before* translation and never translated as peer-controlled arbitrary source strings.

## Safety invariants, scenarios, tests

- **Never** map failure, interrupted, disconnected, rejected or unknown journal status to `Completed`. Unknown status remains `In progress`. Keep `Declined` distinct from `Cancelled` and `Failed` in every catalog. Unavailable/disconnected journal does not silently enable clear/reset.
- `Clear completed` vs `Reset history` must be distinct even in locale translation. Reset remains available only on existing `journal.error == 1`; do not change token/permission/readiness gating, recovery snapshot or actual menu actions.
- Privacy warning must continue to say history has **metadata only**, for **30 days** and **1000 operations**, and **not** file/message contents. These numeric constants remain unchanged in all locale texts and are automatically checked.
- Keep `record.peer` and `record.kind` as **untrusted display values**: no translation of arbitrary values, markup interpolation, grouping by identity, clipboard side effect, or new dynamic message substitution.
- `{} · {} bytes`, file counts and consent PIN are **deferred** to a proper typed/plural-aware formatter plus explicit LTR isolation and RTL layout review; raw dynamic fields remain exactly as before in this slice. Existing `date()` uses local GLib date formatting. Do not invent CLDR plural rules or change numerical interpretation.
- Positive P1/P2: 20 distinct locale rows with 18 nonempty translations, real source lookup, matched labels/statuses. Negative N1–N12: unknown locale/label→English, every status code including unknown, saved/not-saved/disconnected precedence, action ambiguity, metadata-only privacy warning, 30/1000, and no fail→success collision. Preserve GTK column alignment.
- Mutation review: if terminal `failed` maps to `Completed`, translation erroneously equates status pairs, or tooltip claims contents stored, regression oracles should fail.
- Exact-source rustfmt, strict native GTK Clippy + Xvfb tests, and Preflight stable tests on final code, with risky/flaky policy unchanged. Machine translations are draft copy; human native review, Arabic RTL/screen-width/CJK/Indic fonts and physical KDE/Pixel acceptance remain required.

## Implementation order and rollback

1. This documentation-only commit on dev with expected SHA.
2. Add module, label integration, unit/collision/privacy tests and GTK-history mapping in one **code-only** commit, using optimistic expected-SHA update; do not interfere with #16, #8, Windows, protected master, publication or installed packages.
3. Fix only exact source errors returned by CI and read each final run as terminal before labeling it PASS. Update live backlog separately from code.
4. Roll back executable commit without touching metadata/user history; docs inverse is independent.

**Before code:** design only; #17 OPEN / PARTIAL; no feature/CI/physical claim.

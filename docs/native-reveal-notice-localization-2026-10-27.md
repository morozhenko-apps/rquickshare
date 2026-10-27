# Native GTK reveal/portal notice localization, #17

2026-10-27 MODE A. Work isolated on branch `agent/locale-reveal-notices-20261027` because another agent actively advanced `dev` during source inspection. Base SHA `1a422353a45aa337e5150e6f8ba7459a723d357a`. No full Mode B, physical acceptance or release.

## Repository scan and risk inventory

`native_desktop/src/panel/reveal.rs` is a capability-guarded, cancellation-aware file reveal workflow. The exact seven direct GTK notice sources are:
- `Cannot show this file right now. Review the current transfer and try again.`
- `Cannot show this file. It may have moved, changed, or become unavailable.`
- `Cannot access this received file right now.`
- `Containing folder shown.`
- `Folder request cancelled.`
- `Your desktop could not show this folder. Try again.`
- `Opening containing folder…`

The `Show in folder` button must **reuse** the existing `i18n/transfer_rows.rs` translation, never duplicate a catalog key. Do not translate `reveal-{index}` GTK widget key or file identifiers.

## Invariant analysis / changes

Create `native_desktop/src/i18n/reveal_notices.rs` with seven exact English source keys × 19 draft non-English locale rows, English fallback and scoped contract tests. Register this in `src/i18n.rs` and cross-catalog disjointness tests. Wrap only existing displayed strings using `crate::i18n::label`, preserving exact `Panel::notice` branches and explicit file-reveal action.

Safety invariants:
- R1: no file or folder opened without existing `reveal_allowed()` gates for current epoch, transfer, revision, scope, terminal status, user readiness and ownership; no auto reveal on successful transfer.
- R2: cancellation must never become `Containing folder shown.`; failed/stale/denied file reference must not become success.
- R3: `ResultKind::Contents` gets silent successful completion rather than a false "Containing folder shown." (keep original branch).
- R4: strings use literal GTK labels; no untrusted file/path interpolation, HTML/Pango markup, persistent data or permission changes.
- R5: 20 locale catalog complete with Portuguese pt-PT, zh-Hant, Arabic; absent pt-BR/zh-Hans fallback English; no distinct error/phase translations collide; `Show in folder` reused from transfer-rows.
- R6: no unexpected async task/cancellation/timeouts, ingress admission, portal or manual-mark changes.

## Tests and evidence gate

Focused positive and N1-N12 checks: source-key uniqueness, 19×7 nonempty rows, English fallback, RU/PT/Arabic oracles, cancelled vs shown vs requested vs unavailable distinct, disjoint with all existing keys and strings. Source compare verifies `current`, `matches`, `reveal_allowed`, `Pending::close`, `reveal_target` branch conditions unchanged. Native GTK strict rustfmt, Clippy and widget suite + Preflight run **on the exact executable source**, then update docs separately. Previously scripted source mapping is not CI evidence. Physical KDE file portal and RTL semantic review remain open.

## Execution / branch coordination

Separate docs and code commits. Open a PR targeting `dev`; merge only after source-exact required checks and no parallel changes collision. Do not overwrite active agent's changes or promote `master`. Revert this bounded locale module and display-only call sites without changing downloaded files or the portal authorization behavior.

**Initial state:** design only, not implemented.

# Native panel transfer dynamic presentation: recipient, exact bytes and phase (#17)

Date: 2026-10-27. MODE A, read-only source inventory and future implementation scope. Direct `dev`; source baseline `35ab8ac21fc11c8097c98d263623e9ac52dc2c92` with exact Native GTK `40597510638` and Preflight `40597510685` pending at inventory time. Do not cancel source-exact CI or claim an installed UI pass.

## Verified presentation source

`native_desktop/src/panel.rs::transfer_card` has three additional display surfaces beyond the newly localized guarded card captions:
- `Recipient: {recipient}` created from `transfer.recipient` using raw `format!`. The endpoint name must remain a display value only and must not be trusted as authenticated peer identity (#16).
- `{transferred_bytes} / {total_bytes} bytes locally written/received/sent`, using actual `transfer.details` counts and `Direction::Inbound` plus `PayloadKind::Files`. Branches distinguish persisted local file bytes, inbound received non-file bytes, and outbound sent bytes. Never convert a byte count or claim finalized/saved file bytes merely from transferred totals.
- `phase_text(transfer.phase)` rendered verbatim as a transfer status. Any proposed phase label translation must separately inventory exact `Phase` sources and verify terminal vs non-terminal distinctions before modifying it.

Existing `i18n::isolate_untrusted` already removes Unicode direction controls **in a display-only projection**, enclosing user/peer strings with FSI/PDI. Other screen-specific dynamic formatters (count, PIN, exact B) live in `i18n/dynamic.rs`; reuse rather than introduce competing sanitizer or mutate original transfer metadata.

## Future implementation contract

1. Audit actual `Recipient` value provenance, all source phase enum captions, exact status usage and existing GTK tests. No grouping by name/IP, no certified identity assertion, no changes to protocol/IPC.
2. Implement typed `i18n` display formatting functions with canonical twenty locale IDs: localize the fixed recipient caption and three precise direction/content verbs, safely isolate untrusted recipient text, preserve all exact integer byte values (including 0/u64::MAX) and never round or change arithmetic. Keep the original raw recipient ID and file paths unchanged for actions and persistence.
3. For every supported locale, test both transfer directions, files vs non-file inbound distinction, invalid bidi characters and isolates in a malicious display name, exact bytes at boundaries, English/unsupported locale fallback, no markup/clipboard/path side effects.
4. Separate phase caption audit and implementation if terminal conditions or implied content publication cannot be proved; no broad regex replacement. Native source-exact nightly rustfmt/strict Clippy/widgets plus Preflight. Full Mode B and physical Pixel/KDE/Arabic RTL acceptance remain later gates.

This is **plan only**, not implementation or release approval. #8 keeps the expressly documented compatible-A2.2 risk, #16 actual Google-compatible certificate provider remains OPEN, and #17 still needs complete UI, human semantics, desktop launcher and notifications plus hardware verification. No master/tag/R2 change.

## Verified phase mapping and preview boundary

A separate source audit of `native_desktop/src/panel.rs::phase_text` found eleven exact statuses: `Queued`, `Connecting`, `Waiting for consent`, `Receiving`, `Sending`, `Cancellation requested`, `Completed`, `Declined`, `Cancelled`, `Failed`, and `Disconnected`. Keep the enum classification and terminal vs non-terminal semantics authoritative. In particular, `Cancellation requested` must not be collapsed into completed `Cancelled`, and `Declined` differs from `Failed` and `Disconnected`. No phase localization is implemented here.

`Panel::render_preview` separately builds example cards with hardcoded `Recipient: {recipient}`, `2048 / 12345 bytes locally sent`, `Queued`, `Cancel` and sample PIN tooltip. These preview artifacts require their own localized source treatment, not a global replacement of live transfer labels. This checkpoint remains documentation only.

## Chosen architecture and bounded acceptance

`Completed`, `Declined`, `Cancelled`, `Failed`, `Disconnected` and `Receiving` already occur in unrelated history/settings catalogs. Do not add these phase keys to the shared `i18n::label` dispatcher. Implement a phase-specific pure `phase_for(locale, phase_text(Phase))` mapping, preserving all eleven original source phases and the enum-to-phase function. Unknown phase strings and unsupported locales retain the source. State selection, expiration and actions stay unmodified.

Add a typed `ProgressKind::{Written, Received, Sent}` to the same presentation-only module, and localize only its three units while preserving the original exact `transferred_bytes / total_bytes` numbers; wrap numbers in LRI/PDI for mixed direction. Use the established `isolate_untrusted` projection for `Recipient`; never derive sender identity from this display label. Cover all twenty locales, hostile bidi controls, 0 and u64::MAX, unknown/fallback codes, and eleven distinct phases including requested-vs-final cancellation.

Wire only live `Panel::transfer_card` first. The hardcoded preview examples and phase tooltip must remain separately tracked until their UI/demo acceptance is audited, rather than risk overwriting real transfer output with a fake fixture. User strings and keyboard/RTL layout need later physical review. Rollback is isolated to the presentation module and its three wrapper/UI callsites; no migration or protocol rollback should be required.

## Preview parity implementation contract — 2026-10-28

Source `a1fe6cac` passed Preflight `40607713252` but Native GTK `40607713688` failed before widget compilation on **nightly rustfmt only** (twelve exact assertion/array layout patches in `i18n/transfer_dynamic.rs`). No source completion claim. Apply those formatter diffs without rewriting translations, and use this same final source change to finish preview parity rather than launching intermediate redundant CI.

The synthetic `render_preview` card still contains hardcoded English for its disabled consent buttons, demonstration recipient, progress, queued phase, disabled Cancel button, hardcoded PIN summary and tooltip. Bring them into parity with the already implemented native locale helpers:
- Existing `i18n::label` for consent/Cancel and the tooltip safety guidance; `transfer_recipient`, `transfer_progress` and `transfer_phase` for sample recipient/progress/phase. Preserve synthetic exact fixture counts `2048 / 12345`, the fixture's original names, its disabled action state and the explicit separation from live network sends.
- Use existing validated `i18n::pin_code("4821")` and `i18n::byte_count(12345)` for sample PIN/size; isolate the sample filename in the tooltip using `isolate_untrusted`. PIN LRI/PDI and exact bytes must remain intact in RTL.
- Update isolated GTK preview widget expectations from the English hardcoded PIN to the canonical presentation formatter. Test disabled consent controls, preview row semantics and absence of unsafe markup; no platform IPC, session state, transfer completion, or permission guards altered.
- Final exact-source Native GTK and Preflight once after all changes. No CI claims until they terminate. Physical/mobile/RTL UI acceptance remains separate. Rollback: reverse the preview-only presentation wiring and the formatting-only test layout; neither affects the underlying transfer protocol or sender attribution.

## Root-cause correction: real details-card PIN and untrusted metadata (2026-10-28)

Native GTK CI on production `87808038781d3552b7224d276c231bbb100eafe0` finished **FAIL**, with 142 passing and one failing native GTK `panel::tests::given_actual_builder...`, at `src/panel/tests.rs:397` (`reviewed.contains(i18n::pin_code("4821"))`). All lint and format stages passed before this. Direct source inspection found `render_preview` now uses validated `i18n::pin_code` for the nearby row, but `panel.rs::details_card_mode_with_files` *still displays raw* `format!("Code: {}", details.pin)`, both for preview and real inbound details. The changed test correctly exposed source/formatter inconsistency, not a flaky timing failure.

Scope a production fix, not test weakening: in the authoritative details-card builder, render the validated four-digit `pin_code` or suppress an invalid token; preserve exact consent revision and accept/decline readiness. Replace raw sender/filename display with existing `isolate_untrusted`, without changing stored metadata, file action indices or paths. Reuse exact `byte_count` and `file_count` presentation for summaries and per-file bytes. Localize only four remaining context-specific details captions (waiting for metadata; unsupported review; advertised device; review all files) across 20 locales, source English fallback, and keep accessible widget names/manifest expansion, no markup, file actions and IPC unchanged.

Update current actual GTK widget assertions to the canonical localized PIN, FSI/PDI isolation, exact B units, source metadata unmodified, and invalid PIN absent. Preserve 256-row bound/expander behavior and disabled preview controls. Verify all current locale rows and avoid collision with catalog keys. Then trigger one final source-exact Native GTK/Preflight gate; completed green code does not establish physical KDE/Pixel/RTL acceptance. Branch consolidation before this checkpoint imported only a historical reveal audit, with no source reversion.

## Native transfer-row phase presentation parity (2026-10-28)

Current `cb1994c0` Preflight `40668796978` completed **SUCCESS**, Native GTK `40668797037` **FAIL** during source formatting *before* lint/widgets. The sole nightly `rustfmt` diff is a grouped `assert!(labels(&row).iter().any(...))` in `native_desktop/src/panel/tests.rs` around line 350. Apply exactly the formatter output, without weakening the assertion or changing the verified sender projection.

Source review found another bounded i18n drift: the native transfer-row builder `panel/rows.rs::transfer_row` uses raw `phase_text(Phase)` for the visible terminal `row-status` and the `row-icon` tooltip/accessibility name, and the keyed transfer and nearby-device progress bar tooltips independently use raw `phase_text`. The primary panel transfer card already uses a dedicated `i18n::transfer_phase` mapper for the exact same eleven phases. This is strictly a presentation-layer source divergence; all enum values, badges, consent readiness, transfer IDs, progress fractions, controller actions and expiration rules remain authoritative and unchanged.

MODE A implementation boundary:
1. Fix exact formatter mismatch from `40668797037` in existing GTK test source.
2. Reuse `i18n::transfer_phase(phase_text(phase))` at all four presentation surfaces: row icon tooltip+accessibility name, terminal status, in-row progress tooltip and discovered-peer active progress tooltip. For normal idle discovery, retain the existing available/unavailable labels; for live transfer, show its real localized current phase, not a conjectured status.
3. Add assertions to the existing isolated GTK keyed-layout verification for the actual terminal status and associated accessible tooltip. Add a pure all-phase state matrix assertion for the 11 enum variants with distinct cancellation requested/final outcomes, preserving label fallback. Do not create a second translation catalog or invent success states.
4. Perform one source-exact Native GTK + Preflight verification after code commit, no CI polling, and leave KDE/Wayland/RTL/native-speaker accessibility acceptance separate.

Risk/rollback: revert only the `rows.rs` translated text projections, the formatter-only test layout and the affected test assertions. No changes to protocol, persisted settings, device ownership, consent or network I/O are allowed. This checkpoint is not a completion or release signoff.

## Transfer-row phase source checkpoint

Source commit `61027d860b4bef5411d76914adaeb0f09adfe0f4` resolves the exact previously reported nightly assertion-layout diff and wires the existing eleven-phase `transfer_phase` catalog to the native transfer-row status, accessible icon tooltip and both progress tooltip locations. Existing GTK keyed-layout smoke now compares actual label/tooltip projections against the canonical phase resolver for all eleven `Phase` values; requested and final cancellation remain distinct. No protocol or action control changes; presentation-only rollback paths remain `panel/rows.rs` and the formatting-only adjustment in `panel/tests.rs`. Validation for this source SHA is pending; source completeness is not installed GTK or locale/RTL acceptance.

## Saved-file metadata and empty-state alignment (2026-10-28)

Source `61027d86` Preflight `40696406856` **SUCCESS**, Native GTK `40696406855` **FAIL** in the widget suite (144 passed, 1 failed; format/lint passed). `panel/reveal/tests.rs:156` still expects unisolated filename and the obsolete `bytes` suffix, whereas production `details_card_mode_with_files` now renders the escaped plain filename under FSI/PDI and `byte_count(8)` as `8 B`. That test is stale; do not restore the unsafe source behavior.

Further source examination: `panel.rs::saved_files_card`, for the **already saved** inbound files, still uses raw `file.name`, English `bytes`, English `Saved X of Y files` and `Saved files` expander title. The expansion callback is not a filesystem permission boundary; only the explicitly added reveal button issues the guarded action. A shared display formatter must isolate hostile filenames and show exact byte values without changing capability identities, file indices or recorded metadata. Localize the saved count using the already translated `Saved files` key plus an exact left-to-right-isolated `saved/total` fraction (no inferred successful transfers, no rounding and no fake plural semantics). The total may differ from the saved count: do not coerce or hide it.

Also the three empty-state captions in `panel/rows.rs::render_rows` remain visible English. Add a presentation-only exact three-key 20-locale catalog, use the same `i18n::label` routing, keep the existing stable internal row key and preserve `selection`, `ready`, `phase`, scrolling, disabled consent controls and state equality.

MODE A tests: source-exact former failing reveal smoke asserts isolated name + exact bytes, exact saved ratio + zero/partial counts and unchanged file button indices; localize 3 empty-state captions across 20 canonical locales with explicit English fallback and uniqueness/meaning tests; saved count helper tests `0,1,2,u16::MAX,usize::MAX` and Arabic LRI/PDI. Include duplicate catalog key check and negative cases for bidi/control characters without writing to disk. Final source-only Native GTK/Preflight once, no polling. Physical KDE/accessibility voice, RTL long labels, peer exchange and full Mode B remain separate gates.

Rollback only presentation and related tests: remove new `row_empty` catalog, revert `saved_files_card` string formatting and the empty caption lookup, and adjust the matching GTK expectations. No daemon, route identity, consent or release changes.

## Saved-file and empty-row implementation checkpoint

Code `1e42eb49b2d7bc636b2b6ce3466e1fde41d20067`: 20-locale `row_empty` catalog with three distinct static empty captions, exact `saved_files_tally` using the previously translated `Saved files` key and isolated fraction, and `saved_files_card` now uses bidi-isolated display-only filenames and exact byte values. File-action indices and original `FileDetail` remain untouched. The source-exact former failing reveal widget assertion now matches the same guarded presentation path; a saved filename containing U+202E proves display isolation without changing the original fixture. Added count bounds, unsupported-locale fallback and catalog-collision tests. No source build/CI green claim until the new exact-SHA run completes. Installed Wayland/KDE, hardware peers, semantic localization and final MODE B are still separate gates.

## Saved-file tally formatting gate correction (2026-10-28)

Source `1e42eb49b2d7bc636b2b6ce3466e1fde41d20067` Preflight `40716042625` completed **SUCCESS**. Source-exact Native GTK `40716042818` completed **FAIL** in nightly rustfmt only, before lint/widget execution: two output-format hunks in `native_desktop/src/i18n/row_empty.rs` at lines 112 and 162. Follow-up source-only `1b17f4089869087e7919361fd0502c46a6e7aabb` applies both exact formatter changes; no translation or production logic was altered. Matching Preflight `40732361824` and Native GTK `40732361870` were **IN PROGRESS** at the initial workflow lookup. Do not credit them as successful before completion, and do not poll active workflows. The candidate package smoke, installed Dolphin/KDE and Android peer checks remain distinct.

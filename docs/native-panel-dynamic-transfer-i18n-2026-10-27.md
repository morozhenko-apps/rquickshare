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

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

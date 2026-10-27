# Native GTK card controls and partial-file captions localization (#17)

Date: 2026-10-27. MODE A, direct `dev`, docs-first scope. Existing client-error source `2badb6c5` has exact Native GTK 40596891097 and Preflight 40596891115 pending; do not cancel them with further executable source before terminal verification.

## Verified source and catalog inventory

Eight canonical English-only visible strings in `native_desktop/src/panel.rs` have no exact source key in any current `src/i18n/*` catalog:

1. `Preview only` (no synchronized service view)
2. `Waiting for complete service state` (no synchronized service view)
3. `Decline` (incoming consent control)
4. `Accept` (incoming consent control)
5. `No complete files were saved. Ask the sender to send again.` (failed/cancelled inbound files, zero fully published files)
6. `Completed files were kept. Ask the sender to resend the remaining files.` (failed/cancelled inbound files, nonempty completed list)
7. `Review text` (finished inbound transfer with current review revision)
8. `Cancel folder request` (explicit current reveal operation)

Existing `Cancel` is already a base menu translation; reuse it, never duplicate. This is source-specific inventory, not the full native UI audit. Dynamic `Recipient: {recipient}` and exact byte/progress/unit labels are explicitly out of scope pending a typed locale/bidi-safe formatter, as are `phase_text` enum statuses and tray/notifications.

## Invariants and bounded implementation

- Add isolated `i18n/panel_cards.rs` with eight exact source keys and 19 draft non-English translations per key. English source and unsupported `pt-BR`/`zh-Hans` fallback unchanged. 
- Translate GTK display only at the eight static source callsites, and reuse `i18n::label("Cancel")`. Preserve transfer model, revision/epoch, `can_accept()` and explicit consent round, rejection/acceptance event route, cancel handler, reveal operation ownership and cancellation semantics, no automatic clipboard or file opening.
- Preserve partial-file truth: zero completed files must never claim anything saved; nonempty completed list must not claim every file was saved or an incomplete transfer finished. No implicit retry.
- Test full 20-locale matrix, complete nonempty/disjoint source keys, RU/PT/Arabic oracles, uncertainty/partial-success distinction, unsupported locale fallback, static marker count, and existing real GTK transfer/controller tests. Never mock action guards under test.
- Final exact-source Native GTK nightly rustfmt, strict Clippy/widget suite and Preflight after implementation; no automatic final Mode B, packaging or hardware claim. Document only source exact evidence; close #17 after full native/RTL/physical acceptance, not this slice.

Rollback by reversing the isolated presentation catalog and label-only wiring. #8 BWU A2.2 residual risk is an explicit unresolved exception; #16 Google-compatible certificate provider remains blocked externally. No `master`, release tag, R2, or production publication.

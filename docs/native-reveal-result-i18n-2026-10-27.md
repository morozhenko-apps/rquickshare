# Native reveal, result and unavailable-text notice i18n (#17)

Date: 2026-10-27, MODE A. Documentation before code. Base `dev` `96735efc9f2b9d05848facb22172efcdfdd65077`; transfer rows `1a422353` exact Preflight `40574149980` and GTK `40574150162` had not yet reached terminal verdict at planning time, so no new source ref update may cancel them blindly. User requests all pre-Mode-B tasks except explicitly accepted #8; #16 compatible-Google-certificate source remains OPEN, do not replace it with proprietary pairing.

## Source inventory and invariants

`native_desktop/src/panel/reveal.rs`, `results.rs`, and `text.rs` expose **10** distinct English notices through existing user action and permission/error branches:

1. `Cannot show this file right now. Review the current transfer and try again.`
2. `Cannot show this file. It may have moved, changed, or become unavailable.`
3. `Cannot access this received file right now.`
4. `Containing folder shown.`
5. `Folder request cancelled.`
6. `Your desktop could not show this folder. Try again.`
7. `Opening containing folder…`
8. `Completed. The saved result is unavailable for automatic folder selection; review History.`
9. `Completed. Text is no longer available; review History.`
10. `Text is no longer available for review. Unlock and check the current transfer.`

`Show in folder` already exists in the recently added transfer_rows catalog; reuse this exact canonical key in `reveal.rs::file_button`.

### Implementation contract

- Add `native_desktop/src/i18n/reveal_notices.rs` catalog: 10 exact source keys ×19 non-English locale rows (190 draft translations), English fallback including deliberate pt-BR/zh-Hans unsupported locale behavior, direct RU/PT/Arabic oracles. No runtime dependency, protocol or GTK widget change.
- Apply `i18n::label` **after** existing notice-source selection at exactly these 10 static callsites, plus reuse `i18n::label("Show in folder")` on the permitted file button. Preserve silent cancellation for stale/closed windows; never turn a declined/aborted portal request into successful `Containing folder shown.`.
- Preserve portal file capability, `reveal_allowed`, transfer ID+revision/source checks, owned cancellation token and expiry, `ResultKind::Contents`, `safe`/manual consent and public file/link privacy branch logic exactly. No implicit retry, no navigator launch until the pre-existing user action, no clipboard copy.
- Positive / negative unit tests: each locale/key nonempty, unique/disjoint from old catalogs, no status success/failure/cancel collision, known PT-PT/RU, English fallback, no completed-content claim from metadata-only records, full 20-language key matrix and literal brand/trust invariants. Keep existing reveal tests exercising real guarded resource ownership; do not mock portal source under test.
- Validate final source with Native GTK strict rustfmt/Clippy/widget suite and Preflight. Do not run final MODE B, release, master/tag/R2/installed Pixel; physical portal cancellation/reveal and human RTL review remain later gates.

## Rollback and staged release

Single atomic executable source/test commit and separate docs-only checkpoint; revert catalog and label adaptations without touching settings, files or OS portal. Other #17 tray/notification/outbound/countdown/desktop-entry locale surfaces remain open. #25/#26 transport Cancel accuracy must not be conflated with this desktop *folder request* cancellation notice.

**Before code:** only planned, NO source work or CI for this notice slice.

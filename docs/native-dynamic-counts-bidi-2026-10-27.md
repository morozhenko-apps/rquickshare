# Native GTK dynamic counts and bidi-safe transfer text, issue #17

Date: 2026-10-27. Primary MODE A (Feature / Refactor), code owner: ChatGPT GitHub integration. Start `dev` `758536c8865f6d800a3e13645634a9c98e3c4ccf`, no open PR. Previous static history localization source `34e168e2` passed source-exact Preflight 40485357024 and Native GTK 40485357147. This slice is **not** complete 20-language GTK localization or a released binary.

## Inventory and rationale

The static i18n catalog cannot safely translate interpolation such as `{n} files`, `{n} bytes`, `Code: {pin}`, `{name} · {size} bytes`. Today these appear in `native_desktop/src/panel/rows.rs` (count summary, consent PIN and tooltip), and `native_desktop/src/panel/transfers.rs` (journal kind and exact byte count). Dynamic filenames and peer names are user/peer-controlled and must never become lookup keys, HTML/GTK markup, or asserted trusted identity. Active session PIN is four ASCII digits in the verified `TransferDetails::valid(waiting)` contract; maintain session consent and action gates unchanged.

Implement a **pure internal GTK presentation module**, `native_desktop/src/i18n/dynamic.rs`, invoked by `i18n.rs` helpers. No new external dependency, persistent preference, D-Bus/public API, network protocol, trust grouping or package metadata. English fallback preserves a readable user experience for C/unknown locales. The existing `active()` resolves system locale, and explicit-locale functions enable deterministic tests.

- `file_count_for(locale, usize)` chooses integer cardinal categories (0/1/2/3/4/5/11/12/21/22/101 etc). The independent offline Babel 2.18 locale data on 2026-10-27 gives the relevant integer cases: Russian/Polish `one/few/many`; Arabic `zero/one/two/few/many/other`; French/Hindi/Bengali `one` for zero and one; other supported locales use one/other or invariant forms. Keep numeric count exact, unrounded. Texts are model-assisted draft, not audited native-speaker translations. No claim of full ICU/CLDR decimal/exponent support (API accepts only nonnegative integers).
- `bytes_for(u64)` uses exact decimal byte count plus unambiguous SI/IEC unit symbol `B` (one byte), without converting to KiB or changing units, and without grammatical pluralization of a translated unit noun.
- `code_for(locale, pin)` uses localized `Code` prefix but returns an **empty** status for empty input and isolates each nonempty 4-digit ASCII token using Unicode LRI/PDI (U+2066/U+2069) to keep digits in the original order in RTL. Never use this helper to authorize or decide consent; it is presentation-only. Any malformed PIN is not formatted as a verified code.
- `isolate_untrusted(s)` wraps peer-controlled filename/device/kind text in FSI/PDI (U+2068/U+2069) for concatenated labels, and removes Unicode bidi override/isolate controls from the **display-only projection** so attackers cannot prematurely close the outer isolate or inject direction override. Original domain objects remain untouched. Do not put untrusted strings into GTK markup; no URI/clipboard/file/consent side effects.

## Scope

1. Introduce module + internal locale/counter/isolator helpers and deterministic positive/negative tests. Add no UI widgets; use existing `gtk::Label::set_text`, not markup. Arabic/Indic/CJK linguistic quality still needs human review.
2. Replace the English `{count} files` branch while retaining the missing/unsupported metadata guard and single-file extension whitelist. Render journal and summary byte counts as `{n} B`. Localize and LTR-isolate only *display* PIN in the two rows callsites, and FSI-isolate untrusted peer/file/kind in the existing labels/tooltips, never by deriving peer trust.
3. Preserve each original backend status/phase/consent/user-control condition, payload amount, clipboard path, direction and journal raw kind. No fall-through that exposes `0 files` when metadata unavailable.
4. Add focused regression tests against known integer cardinal boundary oracles, all 20 locale nonempty outputs, exact byte values including 0/u64::MAX, LRI wrap for 4-digit PIN and invalid/missing code, injection-resistant outer bidi isolation, no transport/recipient changes. Extend existing UI fixture count oracles as necessary; do not mock the helper being tested.
5. Final source-exact Native GTK rustfmt/strict Clippy/widget tests + Preflight only **after the last change**. If formatter/lint reports a defect, diagnose exact source and fix, then re-check. Stable/flaky and Mode B lifecycle policy unchanged; physical KDE/Arabic RTL/20-language native review remain OPEN.

## Risk and rollback

Primary risks: incorrect plural category/caption, untrusted direction-control spoofing, confusing a PIN display with authorization, altering byte count or passing content through markup, or accidentally changing file-metadata readiness. Keep helpers pure and presentation-only, with no persistence. Code/test commit separate from this documentation and subsequent checkpoint commit. Roll back independently with inverse commits. Do not touch `master`, release tags, installed package, R2, Windows, or live network/device sessions.

State before code: **NOT IMPLEMENTED / NOT TESTED**. Issue #17 remains PARTIAL.

## Implementation checkpoint (2026-10-27)

Executable `7b7ac3255ccf29f42ffd2d669100dfd2feafc19f` adds `native_desktop/src/i18n/dynamic.rs`, a pure module for integer cardinal forms, exact `B` byte units, LRI/PDI-wrapped validated four-digit PIN and FSI/PDI-wrapped external display text with internal direction controls filtered. It is wired into the history metadata rows, session summary, nearby device code and filenames in a non-markup tooltip; untrusted domain objects, actions, backends and routes are unchanged.

Five focused tests directly exercise the formatters (all locales, Russian/Polish/Arabic integer categories, byte extremes, valid/invalid PIN and injected Unicode controls); existing `rows.rs` payload tests continue to test the actual main-card projection and are not mocked. Independent Babel 2.18 integer cardinal samples checked for ru/pl/ar/fr/hi/bn as a diagnostic; decimal CLDR support and professional linguistic signoff are **not** claimed. The Arabic compact noun string, 20-locale typography and physical RTL input still require review.

The code-only commit triggers source-exact Preflight `40509466138` and Native GTK UI `40509466348`. Both were **IN PROGRESS** at plan checkpoint; no final pass claimed. No user/package/Pixel or release changes.

### Deferred follow-up

The static incoming code-verification sentence, other consent/action labels and complete native UI translation remain separate #17 tasks. History `record.kind` and sender names still display their original (not localized or trusted) source values, now visually isolated only. Counts must never be used as proof of completed file/transport delivery. RTL widget layout and native speaker review remain physical acceptance gates.

## Final source-exact validation of typed dynamic values

Executable `b8064a09076c698c62a4794ec545babd1d74bd96` (exact formatter followup of `7b7ac325`) was confirmed by **Preflight 40509819442 SUCCESS** and **Native GTK 40509819419 SUCCESS**; all five Preflight jobs terminal SUCCESS, including Core tests and Tauri Clippy. This confirms the *source code* tests but not physical RTL layout or linguistic signoff. The later image-dialog slice is tracked separately and does not invalidate these source-exact receipts.

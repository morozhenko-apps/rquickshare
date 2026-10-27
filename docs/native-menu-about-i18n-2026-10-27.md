# Native GTK menu and About remaining captions (#17)

Date: 2026-10-27. MODE A, documentation before executable changes. Direct `dev` only. Starting source `d10d1a2342591928bc5d2fba606f74b08ee33958`, subsequent documentation only. Source-exact Native GTK `40581099630` SUCCESS; matching Preflight `40581099519` has not been accepted here until a terminal success verdict. No hardware, release, or final Mode B gate.

## Exact UI source inventory

`native_desktop/src/panel/menu.rs` calls `crate::i18n::label` for menu action captions, but these four source keys are still absent from every current native catalog:

- `Hide card` (menu action `panel.hide`, distinct from generic `Hide`).
- `Files…` (send menu action `panel.files`, distinct from `Add files…`).
- `Text or link…` (send menu action `panel.send-text`).
- `Project website` (About dialog `website_label`, currently not routed through `i18n::label`).

`Visible for one minute` and `About rQuickShare` already exist in the Settings catalog; reuse their existing translations. `Clipboard` already exists in transfer rows. Do not duplicate keys, translate URI/action names, or replace fork-attribution/legal strings.

## Implementation scope and invariants

1. Create isolated `native_desktop/src/i18n/menu_extra.rs` with exactly four English source keys and 19 additional non-English locale entries per key (76 draft strings). Use the established English fallback; no automatic pt-BR or zh-Hans mapping. Route it through `i18n.rs` and extend the catalog collision/locale completeness test.
2. The existing three `menu.rs` action captions already resolve through `i18n::label`; preserve their action IDs/permissions and only change display output. Wrap the About website caption through the same catalog without modifying URL, link opening, GNU GPL/source-attribution disclosures or accessibility behavior.
3. Unit tests for all 20 locale/key combinations; source-key uniqueness and cross-catalog disjointness; strict distinction `Hide card` vs `Hide`, `Files…` vs `Add files…`, sender text/link vs clipboard; PT-PT/RU/Arabic oracles and unsupported locale English fallback. Existing GTK menu integration tests continue to be authoritative for action wiring, capability and user gesture gates.
4. Final source-exact Native GTK nightly rustfmt + strict Clippy + full widget suite, plus matching Preflight. No per-edit suite runs. Update canonical `docs/active-backlog.md` with exact SHA, CI outcome and follow-up after the gate.

## Risks, rollback and remaining acceptance

Primary risks are translating a menu action rather than its caption, aliasing two different commands, accidental About website URL changes, localization-regressed RTL layout and brand/attribution confusion. This stage must be presentation-only. Single code-and-test atomic commit; docs commits independent. Rollback through inverse source commit; no settings/data transport changes. #17 remains OPEN until all GTK/notification/package surfaces, 20-language semantic/Arabic RTL and device acceptance; #16 Google-certified provider remains blocked and #8 BWU compatible-A2.2 exception independent. No Mode B release sign-off or publication.

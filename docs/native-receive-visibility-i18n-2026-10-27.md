# Native GTK Receive visibility countdown localization (#17)

Date: 2026-10-27. Primary MODE A. Direct `dev` workflow; no feature branch, PR, release, package, or Mode B execution. Starting executable source `6deb36452947e62d54938054011e6b93fb1a0204`; its Preflight `40579092072` and Native GTK `40579091917` were in progress at planning time. Do not cancel or reclassify these jobs to claim acceptance.

## Repository scan and scope

- `native_desktop/src/panel/countdown.rs` calculates remaining time from a checked daemon snapshot and a monotonic sampled instant. GTK only displays the timer; it **must not** renew or expire the server visibility lease.
- The sole remaining visible English timer sources in this module are `Visible to nearby devices · M:SS` and `Updating visibility…`. The 15-second automatic card idle timer in `panel/idle.rs` has no UI text and is out of scope.
- The existing `i18n.rs` system-locale resolver allows exactly twenty canonical locales: en, ru, es, pt, de, fr, it, pl, hi, zh-Hant, ja, nl, sv, tr, ko, bn, th, id, fil, ar. Explicit pt-BR and zh-Hans requests correctly fall back to English and must continue to do so.

## Implementation contract

1. Add one isolated `native_desktop/src/i18n/visibility.rs` pure presentation catalog with exactly two canonical English source keys: `Visible to nearby devices` and `Updating visibility…`. Supply 19 draft non-English translations per key (38 entries), English fallback, and a typed countdown formatter. Never parse/unescape an arbitrary template or use peer-supplied text.
2. Format the checked remaining `u64` seconds as `M:SS` without rounding or modifying the value; surround the ASCII time in LRI/PDI for Arabic/mixed-direction readability. Caption and timer remain separate from authenticated peer identity and visibility authorization. Preserve the existing visible English wording.
3. Use the localized formatting only in `Panel::refresh_countdown`, retaining `remaining_seconds`, `temporary_receiving`, view epoch match, `remaining_ms <= 60000`, timeout lifecycle, `receive-indicator` visibility, and `sync_countdown` implementation byte-for-byte apart from display selection.
4. Deterministic tests: all 20 locales return nonempty/disjoint keys; en/ru/pt/ar captions; fallback for unsupported pt-BR/zh-Hans; time 0/1/9/10/59/60/61/3599 seconds and high `u64` input without arithmetic overflow; exact ASCII `M:SS`; one LRI/PDI pair and no hidden status upgrade; unchanged existing GTK 60s, stale epoch, absent sample, hidden panel and cancellation tests.
5. Validate final executable source with Native GTK nightly rustfmt, strict Clippy and widget suite plus matching Preflight after all planned edits; normal tests are deferred to final CI gate. Source success does not count as installed Pixel/KDE, native Arabic rendering or user-language semantic sign-off.

## Risks, rollback and acceptance

The risks are status confusion between a verified visible lease and an updating/unknown snapshot, incorrect 60s boundary, inverted RTL digit order, locale fallback drift and unintended timer lifecycle changes. The locale helper is presentation-only; never expose or mutate daemon visibility/transfer state through it. Revert the isolated catalog/wiring commit to restore original English display; keep documentation and production commits separate. Issue #17 remains OPEN pending all native UI surfaces, human 20-language review and hardware acceptance. #16 Google-compatible certificate provider remains blocked; #8 remains the sole specifically recorded A2.2 exception. Do not publish 1.0.0.

## Implementation and source-exact gate checkpoint

- Docs-first `7b321271708126f7c8bdd97e091a95942c84e558`; executable `7a69310649f8767e682a5ad73862c1352c900dc9` adds `i18n/visibility.rs`, typed `visibility_remaining`, 38 non-English draft rows, two deterministic source tests and exact presentation-only `countdown.rs` UI wiring. Comparison with previous `countdown.rs` confirms timer prerequisites, expiry and timeout tail unchanged.
- Initial `7a693106`: [Preflight 40580043242](https://github.com/morozhenko-apps/rquickshare/actions/runs/40580043242) SUCCESS, but [Native GTK 40580043258](https://github.com/morozhenko-apps/rquickshare/actions/runs/40580043258) FAILED in strict rustfmt only (six exact layout edit blocks); Clippy/widgets not run.
- Style-only `d4e1dca54de5804cf809aa4c8295343660822530` applied precisely the formatter diff without changing assertions or translations. [Native GTK 40580683643](https://github.com/morozhenko-apps/rquickshare/actions/runs/40580683643) passed rustfmt but FAILED Clippy compilation with `E0433` at `panel/countdown.rs:39,40`, because `i18n::` was not in the nested module's scope; this was a module-path defect, not a test/flaky finding.
- Fix-only `d10d1a2342591928bc5d2fba606f74b08ee33958` qualifies both accesses with `crate::i18n::`. Final source-exact [Native GTK 40581099630](https://github.com/morozhenko-apps/rquickshare/actions/runs/40581099630) and [Preflight 40581099519](https://github.com/morozhenko-apps/rquickshare/actions/runs/40581099519) started; **PENDING** at this checkpoint. Do not promote the previous non-source-exact successful Preflight to release acceptance.
- Full Mode B, installed/physical Pixel/KDE, human 20-language/Arabic RTL, 1.0.0 release gates **NOT RUN**. No source or CI-success claims for those gates.

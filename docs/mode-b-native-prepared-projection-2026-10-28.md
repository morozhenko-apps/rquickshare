# MODE B: Native prepared-source captions and live card-state projection

## Scope, evidence and limitations

- Primary mode: **B (testing)**. Exact audited starting source: `1041d76c81a27c1f4c19009d5f7c6588681bf5e2`; current docs-only tip: `81dd3fc510efd59ffdb338808316ae0ff1d25820`.
- The last source-exact [Preflight 40754835823](https://github.com/morozhenko-apps/rquickshare/actions/runs/40754835823) and [Native GTK 40754835906](https://github.com/morozhenko-apps/rquickshare/actions/runs/40754835906) both **passed**. This is baseline evidence, not a new coverage measurement.
- This is one **fully enumerated scoped slice** of the outstanding repository-wide MODE B, not a claim of global Stage 12 completion. Global code spans `core_lib`, legacy Tauri/Vue, native GTK, and Python/package scripts. The latest per-file instrumented baseline is `docs/mode-b-coverage-baseline-2026-10-19.md`, on an older SHA. Rust branch coverage has **never been inferred** from its 0/0 branch counters.
- Owned production files: `native_desktop/src/i18n/source_preparation.rs`, `native_desktop/src/i18n/row_empty.rs`, `native_desktop/src/i18n.rs` (projection wrappers), `native_desktop/src/panel.rs` (source summary and live control captions). Owned test-only extension: `native_desktop/src/panel/tests.rs`. Nearby `panel/rows.rs` uses the already tested empty-caption lookup; no renderer or transfer semantics change.
- No new production APIs, locale preferences, UI behavior, daemon/DBus requests, consent, timing, tray action IDs, signing or release action.

## Stage 1: Framework and execution inventory

| Area | Actual framework | Final stable verification |
| --- | --- | --- |
| Native GTK | Rust `#[test]`, actual GTK4 widget builder under isolated Xvfb and D-Bus, Rustfmt and Clippy | `.github/workflows/native-gtk-ui.yml`, automatic dev source push |
| Core + desktop + frontend + scripts | Core Cargo stable/stress partition; legacy Tauri Cargo; Vitest; Python unittest | `Preflight` dev source push |
| Project-wide final gate | Core/Tauri/Vue/Python plus Native GTK with strict separate jobs and coverage | `.github/workflows/mode-b.yml`, **workflow_call only**, protected release chain; not a dev/pull-request job |
| Installable candidate | Ubuntu 26.04 native GTK .deb with owner-readable Actions artifact; *not* a published release | `.github/workflows/native-package-smoke.yml`, workflow_dispatch only |

No local Cargo, GTK/Xvfb test environment or GitHub clone exists in this ChatGPT container. Workflow job evidence, not a static source review, determines PASS. This mode does not authorize altering the release-only `mode-b.yml` trigger.

## Stage 2: Risk and invariants

**P0:** A stale incoming state must never acquire consent or file authority; no caption can assert that locally prepared content was sent or accepted. `panel.discard` remains destructive, and no source text may rename it as a navigation-only action. **P1:** localized source counts and byte quantities remain exact; zero/max cannot wrap, truncate or round, URL cannot be shown as Text; unknown locales fall back to canonical English, `pt-BR` and `zh-Hans` do not alias `pt`/`zh-Hant`. **P1:** drawer open/closed tooltips follow the actual GTK state, not an older render; receiving-disabled hint stays coherent. **P1:** Arabic saved/total digits are isolated with LRI/PDI without changing their values. **P2:** all 20 canonical locales have nonblank catalog rows and stable source-key meaning.

## Stage 3: complete testable-artifact inventory *for this slice*

| Artifact and function (all executable within this bounded slice) | Risk | Test level | Expected distinct new scenarios | Positive/negative applicability |
| --- | --- | --- | --- | --- |
| `source_preparation::translated(locale,key)` | P1 | Unit | 2 locale/key/fallback matrices (existing), 1 independent golden | Positive N1 N6 N8 |
| `source_preparation::files_for(locale,u32)` | P1 | Unit | 1 existing bounds matrix, 1 independent golden/negative | Positive N1 N2 N8 |
| `source_preparation::content_for(locale,bool,u64)` | P1 | Unit | 1 existing 20×2×5 matrix, 1 independent kind/limit golden | Positive N1 N2 N8 |
| `row_empty::translated(locale,key)` | P1 | Unit | 1 existing 20×3 matrix, 1 additional locale-bound row | Positive N1 N6 N8 |
| `row_empty::saved_files_for(locale,usize,usize)` | P1 | Unit | 1 existing limits matrix, 1 independent RTL/overrun golden | Positive N1 N2 N8 |
| `i18n::{prepared_files,prepared_content,saved_files_tally,label}` | P1 | Unit + integration | Existing wrapper equivalence + golden fallback tests | Positive N1 N2 N8 |
| `panel::source_summary(kind,file_count,byte_count)` | P1 | Unit | Existing 3 variants and min/max matrix; independent negative cross-kind assertions | Positive N1 N2 N10 |
| `Panel::render` drawer tooltip/discard/receive action projection | P1 | Actual GTK widget | 1 additional closed→open→closed interaction grid in existing widget smoke | Positive N4 N5 N9 |
| `row_empty` empty-row text at `panel/rows::render_rows` | P2 | Actual GTK/Unit | Existing empty selection and 20-locale lookup; no renderer changes | Positive N6 N8 |

External dependencies: Gtk4 display/D-Bus are isolated by the existing test runner; no real Bluetooth, network, filesystem write or clock sleeps are needed for this scope.

## Stage 4: exhaustive local branch map

- `translated` (both catalogs): key present vs missing; locale in each of 20 exact IDs vs unsupported fallback; duplicate key disallowed. Existing test matrices cover all exact keys and locales, with unknown/fallback cases. New independent goldens catch swapped/aliased key meanings.
- `files_for`: counts 0, 1, 2, 5, `u16::MAX`, `u32::MAX`; language plural branches are delegated to preexisting `dynamic::cardinal` tests. Append only the exact selected count, not other source metadata.
- `content_for`: Boolean Link vs Text, bytes 0/1/2/1024/`u64::MAX`, 20 locales plus unsupported. Independent golden examples catch a wrong boolean branch and fake display unit.
- `saved_files_for`: saved<total, saved=total, saved>total, zero/zero and max/max; locale English/translated/unsupported; exact embedded digits enclosed in one LRI/PDI pair.
- `source_summary`: three `PayloadKind` match arms, integer boundaries and irrelevant-argument independence; existing tests count all variants, added assertions check absence of misleading kind aliases.
- `Panel::render`: drawer Closed vs Open; tooltip must change in both directions, with action identity unchanged. Preview/no-daemon/disabled state does not grant file authority; the existing widget smoke already checks disabled controls, reveal gating and operation non-admission.
- `Panel::render` receives temporary vs not temporary status; existing daemon-state/countdown checks are separate. Only the baseline inactive tooltip is asserted by this bounded change; **the live temporary tooltip is a planned separate Native GTK subgate** if the existing fixture cannot represent a verified visibility snapshot without changing production.

## Stage 5: Positive / N1–N12 matrix

`C` = covered by existing source tests; `P` = planned in this slice; `✖` = not applicable to a pure projection/isolated widget (reasons below). No empty cells.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Source fixed caption catalog | C+P | C | ✖ | ✖ | ✖ | ✖ | C | ✖ | C+P | ✖ | ✖ | ✖ | ✖ |
| Prepared Files/Text/Link summaries | C+P | C | C+P | ✖ | ✖ | ✖ | C | ✖ | C+P | ✖ | C+P | ✖ | ✖ |
| Empty states and saved/total tally | C+P | C | C+P | ✖ | ✖ | ✖ | C | ✖ | C+P | ✖ | C | ✖ | ✖ |
| Actual GTK drawer/receive/discard labels | C+P | ✖ | ✖ | ✖ | C | C+P | C | C | C | C+P | C | ✖ | ✖ |

N1 = invalid/unknown source key or locale; N2 = numeric bounds; N4/N5 = repeated UI transitions; N6 = unsupported locale state; N8 = Unicode/RTL. N10 is a **presentation-only information-leak check**, not a test for real disk/network PII. N3 network outages, N7 authorization attacks, N11 payments, N12 failed disk writes are **not properties of these pure caption functions** and remain required in their owning Core/GTK/packaging suites. No spurious test stubs or TODO-skips.

## Stage 6: meaningful interaction matrix

1. `locale×known key` = 20×8 for prepared captions, 20×3 for empty captions; unsupported locale×all keys and missing-key×locale.
2. `locale×files count` = 20×(0/1/2/5/65535/4294967295); `locale×kind×bytes` = 20×2×(0/1/2/1024/max). English and independent non-English goldens detect shared-formatter regressions.
3. `locale×saved,total` = 20×(0/0, 0/2, 1/2, 2/2, 3/2, max/max), plus independent Arabic/English malformed-direction regression.
4. Actual widget drawer Closed→Open→Closed and no-daemon action identity; no sleep/timer or mutable global `LC_ALL` in parallel tests.

## Stage 7: test-count estimate

Existing test methods: 2 in `source_preparation.rs`, 2 in `row_empty.rs`, 1 focused `source_summary` in `panel/tests.rs`, 1 compose summary test, and 1 large isolated GTK builder smoke. Planned new independent **2 pure Rust test methods** plus stricter assertions inside the existing GTK builder smoke (no separate trivial extra E2E). The matrices above expand into hundreds of table-driven assertions; do not equate test-method count with branch count.

## Stage 8: per-production-file coverage map (source review, NOT a new instrumented percentage)

| Production file and functions | Executable local branches | Existing test mapping | Missing/new work | Evidence limit |
| --- | --- | --- | --- | --- |
| `i18n/source_preparation.rs` — translated/files_for/content_for | locale lookup, fallback, files vs content, Link vs Text | 2 source tests + panel/compose | 1 independent golden matrix; ensure aliases fail | Instrumented current-SHA coverage unavailable |
| `i18n/row_empty.rs` — translated/saved_files_for | locale lookup, fallback, 0/partial/full/overrun | 2 source tests + reveal smoke | 1 independent RTL numeric golden | Same |
| `i18n.rs` — resolve/active/label/wrappers | env priority, exact locale, unknown fallback | existing locale catalog/resolve, panel widgets | No environment mutation; use direct locale injection in unit tests | Same |
| `panel.rs` — source_summary/Panel::render | 3 source kinds; 2 drawer states; visibility state | panel/compose tests and one GTK builder smoke | assert actual Closed/Open/Closed tooltip and fixed `panel.discard` identity | Same |
| `panel/rows.rs` — render_rows empty captions | three page empties, selection/no selection | existing `verify_keyed_layout`, catalog tests | source-only localization already exercised; no production edits | Same |

## Stage 9/10 planned implementation and mutation review

- Add independent locale-specific expectations, not just self-comparisons to `i18n` helpers. A wrong locale key, Text→Link inversion, count rounding, `bytes` instead of `B`, or fallback alias must fail at least one assertion.
- Check long/max saved/total and Arabic bidi LRI/PDI are **exact**, not a substring-only check. Dropping either mark, swapping numerator/denominator, coercing saved>total or rounding values must fail.
- Strengthen the real GTK widget test for both visible drawer hints and its `panel.discard` action name across open/closed transitions. A stale tooltip or miswired action fails.
- Neither change is allowed to modify production code, weaken an assertion, sleep or mock domain logic.

## Stage 11/12 gates

Implement the scoped tests, then rely on a **single final source-triggered** Preflight + Native GTK CI. These remain *source* gates and do not supply installed KDE/Android/Windows acceptance or refreshed instrumented line/branch coverage. A strict source-current repository-wide MODE B remains **NO** until the full branch map and per-file evidence are reconciled, release-only workflow gate is properly admitted and device acceptance is done. No `.deb` package smoke is dispatched by documentation changes. Stop after the initial verification of any source-triggered workflows; do not poll an in-progress CI.

# MODE B: Native 0.9.0-3 locale, Settings/About, R2 readback

2026-10-29. Selected mode: MODE B testing. This is a scoped delta audit and NOT a global repository-wide Mode B completion. Baseline: c0b2b331457cd80e381907b9911f47dccdec2780. Documentation-before-test rule: no tests are changed in this commit.

## Stage 1. Repository scan

Stack: Rust unit/GTK widget tests (Xvfb, D-Bus), nightly rustfmt, stable strict Clippy; Python unittest (APT, native release scripts), CI Preflight and Native GTK. Existing tests: five ui_language.rs tests; i18n catalog and semantic tests; panel/settings.rs GTK smoke and status/port tables; panel/tests.rs menu/About/Settings smoke; tray/menu/tests.rs D-Bus contract and revision tests; test_native_r2.py HTTP status/hash/publisher; test_native_deb.py revision order.

Production files changed from installed 0.9.0-2: native_desktop/src/ui_language.rs, i18n.rs, main.rs, panel.rs, panel/menu.rs, panel/settings.rs, tray/menu.rs, scripts/native_r2.py, native_desktop/packaging/release.toml. Previous native 0.9.0-2 physical acceptance was 13 PASS, 1 UNTESTED, and is not proof for this source.

Exact source c0b2b331 Preflight run 40857519623 SUCCESS. Native GTK run 40857519512 FAIL: strict Clippy field_reassign_with_default at src/tray/menu/tests.rs line 379; actual GTK widget tests skipped. Source/branch coverage metrics are NOT measured at this revision. Historical 2026-10-19 measurements (Core 90.42% lines, GTK 70.75%) are not current-source evidence.

## Stage 2. Risk and invariants

L1: System default + exactly 20 canonical explicit locales; invalid, unsupported and corrupt preferences cannot activate an invented locale.
L2: editing dropdown cannot persist, alter live language or write daemon state; Cancel/close discards, OK commits atomically and retains prior state on error.
L3: locale-only change works without daemon and does not affect start-at-login, consent, recipients, active transfers or stored receive folder.
L4: live GTK card/context menus and exported DBusMenu must repaint, revision must advance, authorization/events must not change. Open secondary windows and full 20-locale semantic/RTL audit remain unverified and require separate acceptance.
L5: remove duplicate Settings in-body branding and About tab, hide only confirmed-success footer while preserving all warnings; separate About must credit upstream/GPL and maintainer.
R1: public R2 readback sends explicit User-Agent and no authorization, verifies exactly HTTP 200, body size and SHA256, never weakens storage/signing/receipt gates.
P1: version 0.9.0-3 must upgrade 0.9.0-2 without artifact identity reuse.

Global remaining P0 risks (#8 excepted only as tracked compatibility risk; #16, #25, #26, plus 20-language #17) mean repository-wide Stage 12 is NOT satisfied by this delta.

## Stage 3. Exhaustive test inventory for this changed surface

A1: ui_language.rs translated/choices/selection/explicit_locale and mapping all 21 indices (unit; locale risk; 5 scenarios).
A2: ui_language.rs parse/load_at/config_file/initialize, bytes, environment precedence, corrupt files, symlinks, size boundaries (unit IO; security; 7 scenarios).
A3: ui_language.rs prepare_directory/save_at/save, atomicity, conflicting/invalid files, permission and temp-collision rollback, repeated saves (unit IO; high; 7 scenarios).
A4: i18n.rs resolve/active/label/localize_panel and new explicit language selection, catalog, fallback, dynamic data (unit/GTK; medium; 5 scenarios).
A5: panel/settings.rs open/reopen/language_changed/needs_native_save/confirm/finish_confirmation/cancel_draft/load/refresh/status (GTK; critical; 8 scenarios).
A6: panel/menu.rs refresh_localized_menus/show_settings/show_about, panel.rs retranslate/render, About and menu state preservation (GTK; high; 6 scenarios).
A7: tray/menu.rs refresh_labels/publish/DBus layout/properties/click, disabled action and revision overflow (unit/integration; high; 6 scenarios).
A8: main.rs initialize before GTK activate and preview/options (integration; medium; 3 scenarios).
A9: scripts/native_r2.py public_readback and publish_release caller, URL/user agent/status/body/header privacy and no false receipt (Python; critical; 7 scenarios).
A10: native_desktop/packaging/release.toml, scripts/native_deb.py, test_native_deb.py version ordering and package metadata (Python; high; 3 scenarios).
Counts are scenario groups, not asserted executed test count. Existing domain, Core, Windows and frontend invariants are not omitted from the overall project; only unrelated to this delta.

## Stage 4. Branch map for delta

A1: known/unknown label, system/20 locales, first/last/in-range/invalid index. A2: utf8/invalid encoding; newline/no newline/repeated; exact/unknown/empty; missing/file/dir/symlink; size 0/1/32/33+; absolute/relative XDG/HOME/unset. A3: valid/invalid selection; unchanged/changed; new/existing/symlink parent; write success/failed; temp collision; atomic replacement and active state. A4: explicit locale vs environment; unsupported territory, unknown source, fixed widget found/absent. A5: clean/dirty native; clean/dirty login; language draft vs no draft; save success/failure/uncertain; daemon online/offline, cancel/close/reopen, live status. A6: menu/card open/closed, about present/absent, transfer active/absent, locale A/B/system; no action changes. A7: revision normal/MAX/exhausted; equal/different projection; allowed/blocked action; authenticated/foreign call. A8: preview/normal; preference readable/corrupt; repeated activation. A9: default/injected opener; 200 vs 206/301/403/404/429/5xx; body short/exact/extra/wrong hash; network errors; receipt fail closed. A10: versions older/equal/newer, invalid package fields.

## Stage 5. Positive + N1-N12 matrix

Legend: C = currently authored test/fixture, pending final exact-source run; P = planned additional test; X(reason) = not applicable. No empty cells. Negative classes: N1 invalid, N2 bounds, N3 network, N4 retry, N5 race, N6 corrupt, N7 security, N8 locale/time, N9 interrupted, N10 privacy, N11 billing, N12 storage.

| Artifact | Pos | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A1 | C | C | P | X(local) | X(pure) | P | C | P | C | X(pure) | C | X(no billing) | X(pure) |
| A2 | C | C | P | X(local) | P | P | C | C | C | P | P | X(no billing) | P |
| A3 | C | C | P | X(local) | P | P | P | C | C | P | C | X(no billing) | P |
| A4 | C | C | C | X(local) | P | P | C | P | C | P | C | X(no billing) | X(read-only) |
| A5 | C | P | C | P | P | P | C | C | P | C | P | X(no billing) | P |
| A6 | C | P | P | X(local) | P | P | X(no data) | C | P | C | P | X(no billing) | X(read-only) |
| A7 | C | C | C | P | C | P | P | C | P | C | C | X(no billing) | X(no storage) |
| A8 | C | C | P | X(local) | C | P | C | C | C | C | C | X(no billing) | P |
| A9 | C | C | C | C | C | C | C | C | X(no time) | C | P | X(no billing) | C |
| A10 | C | C | C | X(local) | C | P | C | C | X(no locale) | C | C | X(no billing) | X(no storage) |

## Stage 6. Interaction matrix

I1: language {system,ru,pt,ar} x daemon {online,offline} x native settings {clean,dirty} x login {clean,dirty} x action {Cancel,OK}. I2: saved preference {missing,valid first/last,malformed,oversized,symlink,inaccessible} x environment {LC_ALL,LC_MESSAGES,LANGUAGE,LANG,none}. I3: menu revision {normal,MAX} x projection {equal,different} x menu event {read,click}. I4: R2 status {200,206,301,403,404,429,500,503,network error} x body {exact,short,extra,same length wrong hash}. I5: Debian {0.9.0-1.debug,0.9.0-2,0.9.0-2.debug,0.9.0-3,0.9.0-3.debug,0.9.1}.

## Stage 7. Coverage estimation

Plan to add at least 14 independent branch and interaction-focused checks across artifacts A1-A10; reuse documented existing fixtures and record exclusions rather than sampling trivial cases. Exact tests/counts and instrumented coverage shall be read from the final CI logs. Branch instrumentation for Rust is unavailable in the older 2026-10-19 report, not implicitly 100%.

## Stage 8. Per-production-file coverage map

| Source file | New production boundary | Already authored | Remaining test/evidence |
|---|---|---|---|
| ui_language.rs | 11 functions: mapping, parser, config IO, atomic save | five pure/IO unit tests | boundaries, permission/rollback, idempotency, env |
| i18n.rs | explicit active override + existing functions | catalog unit tests | explicit vs system, GTK localization |
| main.rs | initialization before GUI | option parser test | injected invalid stored preference, startup |
| panel/settings.rs | selection, save, Cancel, stripped About/status | widget smoke + status tables | offline/save error/combined edits |
| panel/menu.rs | context menu refresh, About | navigation + panel About smoke | all menu labels/actions after switch |
| panel.rs | live retranslate | panel GTK test fixture | preserved controls, opened secondary windows |
| tray/menu.rs | refresh labels, revision and bus | two tests + D-Bus tests | MAX, stale state and idle refresh |
| scripts/native_r2.py | public GET Request and strict readback | real response matrix, publish tests | UA/no-secret, exception and HTTP fail-closed |
| packaging/release.toml | revision=3 | dpkg version test | future artifact identification |
| scripts/native_deb.py | package identity downstream | packaging tests | built .deb, APT simulation only later |

## Stages 9–12 criteria

Stage 9 follows ONLY after this inventory. Fix confirmed test-only Clippy failure, add the planned test cases without relaxing product contracts. Stage 10: mutations must be killed for active-language commit ordering, invalid index, 32/33-byte boundary, no false daemon save, DBus revision exhaustion and R2 HTTP 200/size/hash/UA. Stage 11: run complete applicable STABLE suite once at final source gate and tracked FLAKY once separately; no repeated polling. Stage 12 requires all changed public paths/branches/rules/boundaries/applicable N1-N12 and interactions, else record NO and exact gaps. Even successful scoped Mode B cannot close global Mode B or skip owner-installed T14. No master, tags, signing or R2 writes.

## Stage 9. Source test implementation checkpoint

Test-only commit: 1d0d32afdbf6993ab42b16da8e0a2ae4931eb457. No Core/daemon, R2 upload, Settings domain, release, signing, tag or master modifications.

- Native GTK tests: replaced Default-then-field-reassign with complete State initializer, fixing strict Clippy without disabling a lint; added two DBusMenu tests for exhausted and repeated revision. Existing tests retain normal revision and MAX transition.
- UI language tests: added six independent tests for 21-option index/catalog parity, malformed bytes and 32/33 maximum, valid atomic replacement and old-value retention, private file/directory modes and symlink rejection, failure with non-directory parent, and multiple full-value replacements.
- Native i18n tests: added a table of supported system territory mappings (en/ru/pt/zh-Hant) and explicit rejection of pt-BR, zh-Hans, unknown locales.
- Existing GTK Settings smoke now asserts language-only selection is confirmable offline, draft is not native/login dirty, Cancel restores original language, three remaining page navigation works; deleted obsolete navigation to About page, consistent with owner-approved removal.
- Python R2 tests: added injected opener transport error and streaming read failure with context-manager close and non-secret explicit User-Agent; existing 200/206/301/403/404/429/500/503 and short/long/hash matrix remains strict.

This is an authored-test checkpoint, **not evidence that any of these tests passed**. Mode B stages 11/12 are pending CI.

## Stage 10. Mutation review

- Change 21 choices to 20, reorder the locale table or translate unknown territory: new exact index and i18n variants fail.
- Replace reject-invalid with accept-invalid, weaken 32-byte bound or treat directory as a file: corruption/boundary table fails.
- Remove atomic replace, corrupt stored previous value, accept symlink, drop private file mode: overwrite/private mode tests fail.
- Make a language-only change silently set native-settings dirty, disable offline confirmation or let Cancel keep the draft: GTK editor fixture fails. Actual confirmed file-write failure while an open GTK editor remains *hardware/UI-integration evidence*, not fully proven by fixture.
- Change DBus menu revision increment to no-op or overflow wrap; publish events despite exhaustion: new tests fail. Actual KDE shell's refresh behavior is still hardware evidence.
- Drop User-Agent, leak auth into public Request, accept status other than 200, relax exact byte count/hash or fail to close a broken stream: Python tests fail.
- Revert package revision to 2 or reuse installed candidate identity: Python dpkg ordering test fails.
- Remove unauthenticated receiver authorization or enable auto-consent: none of these delta tests is a substitute for the independent Core/P0 security tests or manual Pixel acceptance. They remain open and are **not** presented as killed mutations.

## Stage 11. Final execution pending

Push of code commit 1d0d32af triggers the configured Preflight and Native GTK workflows automatically. Current session has no local Rust compiler/GTK host and cannot claim local GTK execution. The strict CI result is UNKNOWN until a terminal completion event or user-supplied result; do not poll running workflows, trigger duplicate reruns or infer green from previous source c0b2b331. The known-FLAKY suite (if any is applicable to the scope) needs independent evidence. This document-only commit must not dispatch CI.

## Stage 12. Completion Gate

Source-only review status: inventory YES; branch map YES for changed delta; Positive/N1-N12 plan YES; interaction matrix YES; test implementation PARTIAL; mutation review PARTIAL; execution NO; complete branch/line measured coverage NO; KDE/Android physical T14 NO. Therefore **scoped Stage 12 = NO, repository-wide Stage 12 = NO, installed-package acceptance = NO**. Do not publish, merge master or create tag.

Next checkpoint: receive terminal CI for test commit; fix only supported test/source defects using explicit MODE selection, rerun final gate if needed; then create native 0.9.0-3 .deb smoke artifact only after exact-source gates are green; verify hashes/metadata and hand operator T14 physical checklist. This does not override unrelated issue P0/feature readiness.

## Localization audit scope, owner decision and latest gate

Owner decision (2026-10-29): no human walkthrough for every one of the 20 languages. Agent owns completeness/semantic review with deterministic Rust/Python tests; physical acceptance targets language behavior and display, not 20 separate sessions. The previous wording requiring 20/20 *human* language reviews is superseded. Static source review: 19 table-based locale catalogs, 206 unique keys, 19 non-English arms each; exact key/table lengths and no empty translations. Dynamic formatting, RTL isolation and source-to-catalog coverage need independent checks; table presence alone is not semantic acceptance.

New exact-source CI: Preflight 40877420909 SUCCESS on 1d0d32af; Native GTK 40877421291 FAIL **nightly rustfmt** on ui_language.rs newly added test rows; GTK tests skipped, not failed. Apply exact formatter diagnostics without altering expected behavior before next final gate. Current physical acceptance still historical 0.9.0-2 (13/14), not 0.9.0-3.

## Additional MODE A defect remediation and MODE B regression inventory

Audited a functionality gap beyond structural catalogs: changing language left already-open secondary GTK dialogs in the old locale. The cause was construction-time translated text with no stable retranslation path. Owner-approved live-UI client-only contract already required an in-place update. The separate MODE A design and rollback are recorded in docs/native-ui-language-preference.md before the production edits.

Production files for current candidate: panel/transfers.rs (History title/Refresh/row regeneration), panel/text.rs (Received Text headings/status source), panel/compose.rs (Send Text draft status/headings and action labels), panel/image.rs (image metadata/status/key and controls), panel.rs (explicit dispatch). No new daemon/settings/transport API. Tests in panel/tests.rs, panel/compose/tests.rs, panel/image/tests.rs, panel/text/tests.rs exercise in-place repaint without resetting prepared data/clipboard/consent. Source inventory in scripts/tests/test_native_i18n_catalog.py adds a 19-by-19 table/completeness check, source-to-catalog mapping and bidi override detection. The Rust i18n module now tests 20-locale status/action contrasts.

Mutation targets to kill: omit History Refresh update; erase text draft on repaint; clear image captured bytes; corrupt received content; collapse Accept/Decline or Completed/Failed translation meanings; introduce untranslated fixed i18n label; allow bidi override controls; regress explicit User-Agent/readback hash; reassign DBusMenu revision after exhaustion. Existing unit/smoke tests and new tests cover the first eight by construction but are not counted as PASS before CI. Audit limit: static language catalogs are structurally complete in inspected 19 modules (206 unique keys, no blanks/missing arm); dynamic modules and physical GTK RTL layout require their own tests. No human 20-language walkthrough required.

Latest source-style checkpoint 47a60cda9d12a642f0ddb2e9bb13875040f9f1a3. Native GTK first exact-source red before these fixes: 40877421291 (rustfmt only); Preflight 40877420909 success at previous 1d0d32af. Both are historical, not CI evidence for 47a60cda. Stage 11 stable and flaky final results remain unverified, Stage 12 NO. Package 0.9.0-3 has not been built, installed or published. No master promotion, tag, R2 upload or signing authorized.

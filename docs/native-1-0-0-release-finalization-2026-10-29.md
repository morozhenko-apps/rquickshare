# Native 1.0.0 finalization — owner 0.9.0-4 accepted, follow-up polish and release gate

Date: 2026-10-29. Primary mode: MODE A (user-visible GTK and release preparation). Switch to a separate MODE B stage for test matrix and final validation after the last source change.

## Current source and physical baseline

Owner installed `0.9.0-4`, reported 5/5 PASS: RepoTicker-style About/attribution/license/link, KDE launcher `rQuickShare Native`, localized empty Nearby, live RU/EN/PT UI+About, bilateral Pixel/KDE text/images/Decline/Cancel. This acceptance proves those observed cases, not first-try BLE, a cancellation concurrent with committed BWU, slow throughput or Windows.

Follow-up screenshot shows two explicit remaining native GTK defects: the KDE titlebar has an unwanted minimize action in both **Settings** and **About**, and most About secondary text appears low-contrast because `ui/panel.css` uses `alpha(@theme_fg_color, 0.56)`. Owner asks shorter tray actions **О программе** and **Выход** (English `About`/`Quit`). Requested no more intermediate physical acceptance before final product release. Prior frozen #8 and #16 must remain *untouched*, not silently reclassified as fixed.

## MODE A plan and decisions (before code)

1. Use GTK4 native `gtk::HeaderBar` as window titlebar with `decoration_layout(":close")` and native theme button styling on **only** Settings and About. Do not use `set_decorated(false)`, which would remove the necessary close control and keyboard affordance; do not mutate the main tray card or change daemon lifecycle. Preserve actual window title, close/cancel behavior and re-open reuse. Verify no minimize/maximize ornaments through GTK titlebar property and later installed KDE.
2. Remove the `0.56` alpha from About metadata text and use the current GTK theme's full foreground. Preserve hierarchy by typography (existing 12 px body vs 19 px heading), not by inaccessible text contrast. Do not hardcode white/black or break dark mode.
3. Replace only two menu caption source IDs with compact `About` and `Quit`, preserving their existing `panel.about` and `panel.quit` action IDs and authorization. Add their canonical English + 19 locale translations to the source-backed menu catalog with negative/locale tests. This is not a rename of actual action IDs.
4. Reconcile remaining open issue tickets with existing exact-source code/test and owner physical evidence. Close **only** proven issues; classify others with precise remaining reproduction/acceptance. #8/#16 untouched. Windows #27 is an independent unsupported platform and does not become silently functional in a Linux-only release.
5. Product version `1.0.0` is a distinct source/package identifier from 0.9.0-4. Before changing any version or tag, inspect exact source version and release tooling/tests, signed R2 configuration, immutable release/tag gates and owner approval. No arbitrary untested package, early release or old version reuse.
6. Only after last implementation change, run final Mode B: complete changed-file branch/negative matrix (including N1-N12), mutation review, strict formatter/Clippy, 20-locale source coverage, automatic stable and isolated native widget suites; separate flaky/race suite once and package verify. No busy-poll. Once a CI gate is in progress, state the precise workflow `created_at` (Europe/Lisbon), SHA, URL and status and stop until new user turn.
7. No intermediate physical user test requested; however CI success is not real Pixel/Windows evidence. Do not mark #23/#24/#25/#26 resolved without a scoped check. Release publishing remains fail-closed if required security, protected ref or signing prerequisites lack confirmation.

## Invariants, risk and rollback

- Gtk close must still work from titlebar and Escape/Cancel paths; showing About and Settings must not trigger Core transitions.
- Accessibility respects high contrast/dark mode; two actions have correct semantics in 20 locales; titlebar only affects these two secondary windows.
- Do not weaken frozen #8 authentication gap or #16 verified identity scope; no peer regrouping, new privilege, hidden flags or signoff by assumption.
- Roll back single-purpose UI, translation and version changes independently; preserve known accepted `0.9.0-4` binaries/artifacts; don't overwrite existing installer bytes.
- Production publication is a separate irreversible trust boundary: tag `native/v1.0.0`, protected `master`, isolated signer `apt-release`, R2 CAS index, public archive verification. If required protected/security facts aren't demonstrated, stop the *publication*, not the independent safe UI work.

## Status

Initial MODE A analysis DONE; code IMPLEMENTATION PENDING; Mode B PENDING; final source CI PENDING; 1.0.0 package/release NOT PUBLISHED. Recording this is not claiming permission to bypass unverified security or tests.

## Product version stage and operator risk boundary

The owner requested direct preparation for `1.0.0` without an intermediate installed acceptance after the accepted `0.9.0-4`. The version source is `native_desktop/Cargo.toml` and its locked `rquickshare-desktop` package entry; Debian `packaging/release.toml` must start at revision **1** for new product SemVer. The corresponding native metadata/ordering contract must require `1.0.0-1` > the already accepted `0.9.0-4` and reject same-version mutation. No change to Core crate version 0.11.5 or legacy Tauri release identity.

This prepares a source-level **1.0.0 candidate**, not a guaranteed public release. Issue #8 remains a documented pending route-consumption availability/DoS risk and #16 remains missing authenticated peer-route grouping. Both are explicitly frozen; do not change either source/issue, claim resolved, or silently waive them. Current product strategy says #16 is a 1.0.0 feature. Consequently a public 1.0.0 release must fail closed if the owner has not separately decided the source/product/security contract and the real signing/protected-ref gates do not pass. Existing Github connector cannot independently dispatch workflow or write protected annotated release tags.

Windows #27 is a separate native Windows platform adapter and has no Windows build/physical evidence; do not advertise it in the Linux 1.0.0 package. The Linux-only artifact is scoped to Ubuntu 26.04 amd64. Previously passed ordinary Pixel transfer and Cancel tests do not establish committed BWU Cancel race, initial BLE-first success or bytes/sec benchmarks.

This is a separate STOP FACTOR for public distribution, not for safe, isolated GTK and version-source preparations. All publication and secrets must remain untouched unless complete release prerequisites are demonstrated.

## Implementation checkpoint and scoped Mode B matrix (2026-10-29)

MODE A implementation on dev: bc4864e5 creates the shared, close-only GTK secondary titlebar and applies it exclusively to Settings and About, replaces the 0.56-alpha dim About font with the full theme foreground, and shortens tray captions to About/Quit with 20-locale source translations while retaining original panel.about/panel.quit action IDs. 6119940c adds GTK assertions that both windows own a HeaderBar with decoration layout :close. 736c967c changes only native product SemVer to 1.0.0, its Cargo.lock entry and Debian revision to 1; Python package tests require 1.0.0-1 and actual dpkg upgrade ordering from accepted 0.9.0-4. 0419fd8f expands five one-line Unicode locale arms for nightly rustfmt. 5c511078 adds a Python regression checking solid GTK About metadata foreground and rejecting opacity/alpha. These commits are not an already published product.

MODE B changed-file inventory / branch map / coverage expectations:
- panel/chrome.rs: close-only GTK HeaderBar creation and preservation; two actual widget assertions in panel/tests.rs cover Settings and About, reuse, close/title. Mutation :close -> minimize/close must fail. N1 fixed literals; N2 narrow windows; N3 independent of Core; N4 repeated About; N5 simultaneous windows; N8 GTK theme; N9 close; N10 no peer input; N11 billing N/A; N12 no storage changes.
- panel/settings.rs and panel/about.rs: existing GUI tests exercise lifecycle, editable settings/Cancel, existing About trusted author/upstream/version/link, locale repaint, reused child and titlebar. N6 existing settings corruption tests unchanged; N7 no new access; N9 interruption; N10 static attribution.
- panel/menu.rs and i18n/menu_extra.rs: two *labels* change, not action routes. Six keys times 20 locales audited with exact RU/EN/PT output, every nonempty value, no placeholder injection, unknown-ptBR-zhHans fallback and About-vs-Quit distinction. Mutation swapped/blank locale or changed action ID is observable. N1 invalid locale; N2 short captions; N4 repeated label retrieval; N8 all supported languages; N9 real quit action existing tray/lock test; N10 no personal peer strings; N3/N6/N7/N11/N12 not independently introduced.
- ui/panel.css: Python source check requires About metadata color to equal theme foreground and excludes alpha/opacity; GTK render suite exercises loaded theme CSS. Mutation to low contrast fails. Light/dark visual contrast requires real system verification not available in code tools.
- native_desktop/Cargo.toml and Cargo.lock, packaging/release.toml: 1.0.0 / revision 1 / locked self-version contract in test_native_deb.py, all historical 0.9.x upgrade comparisons, higher revision/debug comparisons, invalid-version and package artifact negative test families. Mutation back to 0.9.0 or inconsistent lock fails. N1 malformed values, N6 outdated manifest, N12 file metadata/permission tests; other categories N/A to immutable version strings.
- scripts/tests/test_native_i18n_catalog.py and test_native_deb.py are test-oracle changes, not production state; no new network or auth paths.
Interaction matrix: secondary window × close/hide/reopen × active locale, About + Settings simultaneously, tray About/Quit × 20 languages × unlocked/locked state, 1.0.0-1 vs 0.9.0-1 through 0.9.0-4 variants. N5 overlap is owned GTK UI state; frozen transfer routing and consent are unchanged. N11 N/A throughout because there is no billing.

Stable execution and separately identified race/flaky suites remain PENDING on final changed SHA; no full code coverage percentage or executed mutation tool is claimed. The prior 0.9.0-4 5/5 owner physical PASS is historical, not acceptance of the newly changed chrome/contrast.

Release blocker: Frozen #8 availability/DoS path and #16 authenticated route grouping remain unresolved; #23 initial BLE stall, #24 throughput, #26 special BWU-cancel race and #27 Windows also lack exact current physical acceptance. Public 1.0.0 cannot claim those gates met. Required signer/protected tag environment is not verified by connected tools; no protected master/tag/R2 production action has occurred. Direct 1.0.0 source candidate can proceed without an extra human acceptance, but irreversible publication is fail-closed.

## Superseding owner release authorization — 2026-10-29

The owner explicitly clarified: **SHIP NATIVE 1.0.0 WITH ISSUES #8 AND #16 OPEN**. They are known retained limitations and are **not** release blockers. This decision supersedes earlier paragraphs in this document treating their unresolved state as a 1.0.0 release STOP FACTOR or demanding another product-scope waiver. Do not change source code or issue state for #8/#16; do not claim they were solved. Preserve compatibility, independent unverified peer routes and consent; no "authenticated route equivalence" claim. The owner also chose no intermediate human 1.0.0 acceptance after 0.9.0-4 5/5 PASS. No evidence that Google resolved or failed to resolve a corresponding defect is asserted by this agent.

Remaining release *integrity* gates are non-negotiable engineering checks (not a new request to accept known issues): terminal successful latest-source Preflight + native GTK, exact-source 1.0.0-1 package smoke and hash/dependency validation, correct protected master and immutable annotated native/v1.0.0 tag, validated signer-only environment, signed R2/APT metadata and public HTTPS verification. If a gate fails, fix the concrete error; no retries in a polling loop or silent bypass.

Terminal CI failures inspected at 19:35:25 and 19:36:05 Lisbon (UTC+0):
- GTK #40941463331 at 0419fd8f: 171/172 passed; the i18n catalog invariant found "About" duplicated between Settings and menu_extra.
- Preflight #40941562152 at 5c511078: 90/91 Python methods passed; a newly authored CSS selector test erroneously used double-escaped regex.
Root fixes in executable commit 1e0825c6: unique menu source identifiers "Tray About"/"Tray Quit", separate English user-visible fallback "About"/"Quit" (20-language localized entries and panel.about/panel.quit actions unchanged); correct CSS regex without weakening the required solid theme-foreground check. No changes to Core, identity, BWU or #8/#16. New terminal CI is **not yet established**, and no protected release has been published.

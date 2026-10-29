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

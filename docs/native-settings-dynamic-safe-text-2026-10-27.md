# Native settings dynamic name/path display: typed locale and bidi isolation

Date: 2026-10-27. MODE A, slice of issue #17, based on `ca24939729592f33edca5510260909fdf98e0fd1`. This document precedes executable code. Prior Settings warning source `ac152f69` passed Native GTK `40548418258`; full matching Preflight `40548418257` must be read independently and not inferred as green. This work is only display localization, not final language approval or Mode B.

## Current source/risks

`native_desktop/src/panel/settings.rs::Editor::load`:
- Copies authoritative persisted editable `view.device_name` into the actual GTK text entry. Must remain byte-identical; this is an input, **not** a display-only projection.
- Uses unlocalized `format!("Name for new connections: {}", view.resolved_name)` for a separate read-only `name_hint` label. A caller-supplied Unicode bidi override/PDI can spoof nearby label direction in Arabic or Latin layouts, including pretending the device name is a confirmed identity.
- Displays `view.directory_display` directly in the read-only directory label or `Default Downloads folder`; a chosen folder result `display` is also copied straight to the label in the guarded picker callback. Never rewrite the retained `gio::File` capability or persisted path.
- Other static status strings remain separate planned #17 work; no change to the `status()`, `failure()`, `login_failure()`, `confirmation_applied()` and controller-action contracts.

## Design and planned tests

1. Extend **existing pure** `src/i18n/dynamic.rs` with `resolved_name_for(locale: &str, value: &str) -> String` using the 20 canonical locales from existing `LOCALES`. The 19 non-English prefixes are draft PT-PT, Traditional Chinese and Arabic, with English fallback. For nonempty values, call the existing `isolate_untrusted` (FSI/PDI, filter nested bidi controls) on the display-only value. Empty value uses the already existing static fallback instruction; do not introduce a new persistent setting.
2. Publish a minimal `i18n::resolved_name(&str) -> String` wrapper selected by the existing OS locale. No network, app permission, schema, debounce, global state or session identity changes.
3. Change exactly three GTK read-only display sites: name hint for nonempty resolved name; `directory_display` in Settings load; selected folder display when chooser succeeded. Preserve the source value `view.device_name` in editable input, `view.directory_display` in Controller state and `File` capability returned by chooser. All labels use GTK `set_text` (no Pango markup).
4. Direct tests: all 20 language prefixes nonempty, unknown locale English fallback, RU/PT/Arabic expected captions, malicious bidi override/PDI removed inside outer FSI/PDI while original source string unchanged, fake markup kept literal, empty result never exposes a fabricated connected name. Existing actual Settings smoke test verifies the user-input value remains unchanged and no write is admitted disconnected.
5. Source review before commit, one executable code+test commit; documentation update in standalone `docs:` commit after exact source-exact Native GTK strict rustfmt/Clippy/widget tests plus Preflight trigger. No normal Mode B suite at this stage. If CI blocks, diagnose *exact source* and correct only root cause, never weaken or skip tests.

## Risk/rollback

Critical boundaries: a path/name is **not authenticated peer identity**, directional controls only filtered in the display projection; PIN and consent unchanged; user editable value must be preserved; storage save/Apply/Cancel and FileDialog semantics remain exactly unchanged. Revert only this new module/helper plus three GTK projections to return to previous display text. No `master`, tags, signed R2, .deb install or release changes.

State before production modification: **NOT IMPLEMENTED**. Issue #17 remains OPEN/PARTIAL and still has other GTK/tray, semantic language/RTL/physical gates. No work on #8's agreed compatibility exception.

## 2026-10-27 executable implementation and gate

- Source-only `378834ac270517c90b9823c2448b8f1150ce3e8c` adds `resolved_name_for(locale, name)` with **20 canonical locale prefixes**, display-only FSI/PDI isolation, unknown-locale fallback and empty-value protection. `i18n::resolved_name` is the sole OS-locale wrapper.
- `Editor::load` projects `resolved_name` into its hint and `directory_display` into a read-only label; the existing folder-chooser success callback isolates the displayed `display`. `name.set_text(&view.device_name)`, retained `gio::File`, controller save/retry/recover and phase matching remain unmodified.
- Source audit PASS: all 19 non-English locale branches; exact three GTK display sites; original editable-name and save-action code still present. New direct tests include RU/PT/Arabic labels, unknown locale fallback, Unicode bidi attack controls, literal angle-bracket text and empty value. This is **source-only**, not human/native UI proof.
- On this exact source `378834ac`, Preflight `40561114184` and Native GTK `40561114130` were **IN PROGRESS** at checkpoint. Do not claim PASS until the full terminal jobs are observed; if Native GTK rustfmt or Clippy fails, use its exact diff. No Mode B, installed KDE binary, Windows or physical Pixel hardware test.

## Exact-source validation result and rustfmt correction (2026-10-27)

Original source `378834ac270517c90b9823c2448b8f1150ce3e8c`: Preflight `40561114184` terminal **SUCCESS, five required jobs**, including Frontend, Rust formatting, Core Rust tests, Core Clippy and Tauri Clippy. Native GTK `40561114130` terminal **FAIL** in the very first native Rustfmt gate, no GTK widget runtime tests were executed. Exact runner diagnostics listed **three layout-only deviations**: one empty line before the new test, multiline `assert!(contains(...))` and chained `directory.set_text` formatting. The source's functionality, tests and data projections did not fail or get loosened.

Follow-up code-only `84bcff96dbd56d93c2693871ca32fd71c35a560a` applies those three exact formatting changes. Final source-exact Preflight `40563016992` and Native GTK `40563016773` **QUEUED / IN PROGRESS** at checkpoint; terminal verdict must be read from those IDs. Distinct docs checkpoint does not alter executable code. No hardware, Windows or MODE B run.

## 2026-10-27 source-exact native GTK PASS, Preflight still partial

Final corrected executable `84bcff96dbd56d93c2693871ca32fd71c35a560a`: **Native GTK `40563016773` terminal SUCCESS**, strict native lint plus widget/controller suite; source-boundary registration and GTK dynamic value formatting are verified at that gate. **Preflight `40563016992` still IN PROGRESS at last read** with completed-success Frontend, Rust format, Core Rust tests and Core Clippy (4/5); Tauri Clippy unfinished. Do not claim a fully green combined 6-job source gate until the fifth job completes. Subsequent documentation updates do not change executable source. Separate owner KDE/Arabic visual verification of RTL, long paths and text copying remains required.

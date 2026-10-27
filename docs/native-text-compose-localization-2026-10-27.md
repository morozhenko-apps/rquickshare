# Native GTK text composer localization: bounded feature slice #17

Date 2026-10-27. Mode A (feature); source baseline `e1abea6db124a4bb3117a2a18bd23fb9c1c1711f` on integration branch `dev`. PR #28 received-text review localization was merged with source-exact Preflight and Native GTK success; #17 remains open because the other native GTK UI surfaces, RTL and professional linguistic/physical audit remain incomplete.

## Source inventory

`native_desktop/src/panel/compose.rs` has eleven static, new catalog messages: `Send text`, `Write or paste text, then prepare it and choose recipients.`, `Clipboard is read only when you paste. Nothing is sent yet.`, `Paste clipboard`, `Prepare text`, `Text must be UTF-8 without NUL and at most 128 KiB.`, `Text captured. Review it, then prepare and choose recipients.`, `No supported text was captured. Use 1–128 KiB of UTF-8 text without NUL.`, `Enter 1–128 KiB of text without NUL.`, `Preparation was not queued. Review the current source and connection.`, and `Write or paste text, then send it to the selected device.`. The existing `Cancel` and `Send` keys MUST be reused, not duplicated.

## Contract and implementation plan

1. Add a separate `COMPOSE_KEYS: [&str; 11]` with 19 non-English parallel translation rows to existing 20-code startup-locale catalog; do not disturb the 17 menu and eight review keys, English fallback, locale/setting choices or DBus/IPC.
2. Change only static GTK labels in `panel/compose.rs` to `crate::i18n::label` calls. Dynamic editor and clipboard text remain raw, no markup. The `Send` button must only translate its caption, not change the distinct `direct`/prepare paths.
3. Preserve actual permissions and event handlers: clipboard read occurs only on explicit paste, PRIMARY middle-click remains intercepted, source validation remains UTF-8/without NUL/1–128 KiB, no automatic acceptance, send, recipient selection or hidden clipboard side effect. No changes to `NativeText`, `PreparationPermit` or `ControllerError` logic.
4. Source regressions: unique keys across sections; all 20 locale rows and no empty translations; English unchanged; selected RU/PT and privacy text accuracy; positive vs blocked/failed outcomes and paste vs prepare vs send remain distinct for every locale; technical literals `UTF-8`, `NUL`, `128 KiB`, `1–128 KiB` preserved where applicable.
5. Final validation **only after the last code edit**: source exact native GTK Rustfmt/Clippy/Xvfb widget tests and Preflight stable suite; known-flaky tests separately if applicable and never silently classified as stable. Do not run a full 10× release Mode B on this incremental feature. Physical KDE/RTL/font/context review remains open.

## Risk / rollback / progress

- Most important risks: semantic translation falsely claiming sent when only pasted/captured; mistranslated copy/paste privacy disclosure; changing byte units in limits; translation of untrusted text; unintended changes to explicit clipboard/recipient actions.
- Arabic bidi and font/width behavior are separate visual gates not established by catalog cardinality. Machine-assisted strings require human linguistic review.
- Keep code and documentation in separate commits. Use optimistic expected-SHA update on shared dev; no release/master, package, Pixel, Google identity, signer or R2 changes. Roll back code commit separately from docs.
- **State before code:** planned, NOT IMPLEMENTED, NOT TESTED; #17 stays PARTIAL until full surfaces and release gates are met.

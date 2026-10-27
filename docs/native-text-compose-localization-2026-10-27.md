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

## Implementation checkpoint

- Production commit `120ca2f18f0481636687c18192778f77a6c37030` adds 11 distinct text-composer messages across all 19 non-English rows (209 additional draft translations), with English fallback and tests for exact `UTF-8`, `NUL`, `128 KiB`, 20-locale completeness, key uniqueness, error-vs-captured and manual-paste-vs-preparation distinctions.
- All thirteen static GTK callsites in `native_desktop/src/panel/compose.rs` now use the catalog; the existing `Send`/`Cancel` keys are shared. The inserted source calls do **not** change actual clipboard, selection, byte validation or authorization logic.
- Independent source structural check **PASS**: 19 catalog rows × 11 values, 11 canonical keys, eleven declared messages present in the composer, original explicit paste/prepare callbacks and middle-click suppression intact. This is **not** a compiler, test-suite or KDE/RTL/human language acceptance result.
- Exact-source Preflight and Native GTK source CI are still **pending**, and the final source-exact outcome must be added after terminal status. #17 remains **PARTIAL** and existing 0.9.0/1.0.0 release blockers are unchanged.

## Formatter follow-up / final source gate

- Exact earlier Native GTK workflow `40470209556` on `120ca2f1` ended **FAIL** in strict formatting, at seven diagnostic locations in `i18n.rs` and `panel/compose.rs`; no source evidence of a runtime, security or translation-test failure from that workflow.
- Format-only source follow-up `9cfe63facfc409525520358d4200fab51966b559` applied every reported multiline layout change without altering any message, validation branch or consent/clipboard behavior. The independent source catalog/wiring audit remains PASS.
- New exact-source Preflight `40470625873` and Native GTK `40470625837` were **IN PROGRESS** at this checkpoint. The task must not be marked complete until their terminal jobs and outcomes have been read. Historical cancelled/failed runs remain recorded rather than reclassified as success.

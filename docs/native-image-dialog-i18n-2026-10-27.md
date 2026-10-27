# Native clipboard image dialog, 20-locale static localization (#17)

Date: 2026-10-27. MODE A feature, planned after the dynamic amount/PIN/bidi code `b8064a09076c698c62a4794ec545babd1d74bd96` passed Native GTK `40509819419`, with its exact-source Preflight `40509819442` still awaiting terminal result at planning time. No current other PR. This is a bounded follow-on to #17, not a 1.0.0 release signoff.

## Inventory / plan

`native_desktop/src/panel/image.rs` is the single manually initiated clipboard-image composition window. It displays ten **new** static English safety/operation messages:

1. `Send image`
2. `Capture a clipboard image, review it, then choose recipients.`
3. `Clipboard is read only when you capture. Nothing is sent yet.`
4. `Capture clipboard image`
5. `Prepare image`
6. `No supported PNG was captured. Use a static image up to 32 MiB and 64 million pixels.`
7. `Image capture failed validation or exceeded resource limits. The previous image is kept.`
8. `Image source could not be prepared. Capture it again.`
9. `Review the image, then prepare and choose recipients.`
10. `Image preparation did not complete. Review Recover or discard the pending preparation.`

Existing `Cancel` (menu) and `Preparation was not queued. Review the current source and connection.` (composer) must reuse their original translations.

Implement another scoped internal catalog `native_desktop/src/i18n/image.rs` with 19 translated rows ×10 new keys, English fallback and no runtime schema. Use `i18n::label` for static source strings only. Replace the assembled image-size status with exact `width × height px · source-byte-count B · Clipboard.png` plus a separately translated instruction. `px` and `B` are unambiguous technical unit symbols; neither dimension nor source byte count may be rounded or transformed. `Clipboard.png` remains a constant local file title, not a user/peer path, and is not localized.

## Critical boundaries / test plan

- Image clipboard is read **only on explicit Capture**. Do not move, bypass, alter or duplicate `clipboard_png`, MIME whitelist, bounded PNG streaming, image validation (32 MiB / 64 million pixels), `PreparationPermit`, epoch/guard checks, task cancellation, worker dispatch, preview replacement, session selection or user action callbacks.
- Review / captured / prepared / sent are **distinct phases**. Translation may never claim the image has been sent before the controller has completed recipient selection/consent.
- Resource/negative/error text: `PNG`, `32 MiB`, `64` remain present for all supported locales. `previous image is kept` must not translate as cleared, replaced, transmitted or automatically copied. `nothing sent yet` must remain explicit in every locale; native-speaker visual confirmation is still pending.
- Tests: unique full source-key matrix across MENU/REVIEW/COMPOSE/HISTORY/IMAGE for 20 canonical locales, nonempty translations with exact unit constants, RU/PT semantic examples, distinguish capture and prepare actions, fail/not-sent/captured warning collision, correct fallback for unsupported locale, source-level exact capture behavior verification. Mutation review should fail for missing keys, wrong limits or success/failure caption collisions. Test actual `i18n::label` and existing GTK widget path; never mock clipboard subject.
- Source-exact final Preflight stable and Native GTK Xvfb/strict Clippy/test workflow only **after last implementation edit**. Treat any network/formatter failure separately from regressions. No full 10× Mode B, hardware/RTL font/native-speaker review or signed release is implied.
- Documentation in a separate preceding commit, source+tests in one atomic executable commit, status/evidence documentation afterward. Do not touch `master`, signing, package publication, existing user settings or other #16/#8 work.

Before executable changes: NOT IMPLEMENTED, NOT TESTED; #17 remains open.

## Implementation checkpoint (2026-10-27)

- Executable `b849003e6f74d9dba71572c18a1fb5cb29cdd52d`: `src/i18n/image.rs` includes 10 new static keys ×19 translated rows = 190 draft translations, English fallback, limit/semantic-collision checks and RU/PT positive oracles. `src/i18n.rs` routes only static keys through the existing 20-locale lookup and validates no collisions with menu, text review, composer and history catalogs.
- The GTK capture dialog, source validation, image preview, stale epoch, resource and cancel paths retain their existing authoritative callback logic. Labels/status/notice use `i18n::label`; `Cancel` and `Preparation was not queued` share existing translations. Capture preview reports exact `width × height px · bytes B · Clipboard.png`, with its separately translated review instruction.
- Static source audit PASS: 19 ×10 catalog rows, all key definitions, resource literals `PNG` / `32 MiB` / `64` present, explicit capture callback, PNG MIME whitelist and cancellation path preserved. **This is not a Rust build/test or native-speaker proof.**
- Exact-source Preflight `40511038788` and Native GTK `40511038773` for this executable SHA **IN PROGRESS** at checkpoint. Must read terminal job results; no 6/6 green claimed.
- Human acceptance remains: all 20 language semantics, Arabic RTL and font metrics, long captions, unchanged privacy statement and physical desktop/Pixel. No master, release, tag, R2 or installed binary was touched.

## Formatter diagnosis and final source gate (2026-10-27)

Source `b849003e` Native GTK `40511038773` completed **FAIL**, with exact `rustfmt` suggestions for three long assertion invocations in `i18n/image.rs` and one `set_text` call in `panel/image.rs`. No native app compilation or test failure was observed; the formatter stopped the job first. Plausible contributing causes considered: long translated assertion expressions, GTK chained-call layout, inconsistent formatter/toolchain settings, whitespace/line endings, stale runner checkout, network/dependency errors and separate source-level compilation issues. Exact changed-line diff supports the first two and falsifies the others for this failure; no temporary runtime logging is warranted for deterministic source formatting.

Code-only follow-up `9e6251d340dd0a40a5efb09eb7db0979d75e8aff` adopts those four exact formatting changes without changing a translation or runtime behavior. Final source-exact Native GTK `40511449070` and Preflight `40511449089` were **IN PROGRESS / QUEUED** on last read. No final CI success claimed yet. Previous dynamic transfer formatter remains separately confirmed SUCCESS on `b8064a09`.

The next #17 area is native Settings. The source spans 1,235 lines with at least 40 user-visible static/dynamic message candidates and validation/save/login/recovery states, so it needs a separate risk inventory and catalog rather than treating untrusted values or error codes as static keys. No settings behavior has been changed in this slice.

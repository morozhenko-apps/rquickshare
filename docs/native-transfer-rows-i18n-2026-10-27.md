# Native GTK transfer-row content, consent and accessibility localization

Date: 2026-10-27. Primary MODE A feature slice following #17 Settings residual. Documentation-only plan **before source changes**. `dev` `dc1da6a700087f8106b8748a05be9150098b0cd2` has native Settings residual code `723e7c03`, strict formatter followup `dc1da6a7`. Its exact-source Preflight `40572346805` and GTK `40572346978` were PENDING during this scan; do not initiate another executable update until a credible source result is checked. #8 remains the sole owner-excepted BWU risk; #16 Google-certificate A stays independently OPEN.

## Source inventory

`native_desktop/src/panel/rows.rs` (approx. 1200 lines) is the main source of user-visible incoming transfer affordances, file metadata, PIN and consent. Current labels contain 15 distinct **static** source strings with special correctness requirements:

`Waiting for details`, `Unsupported content`, `File`, `Text`, `Link`, `Connecting…`, `Incoming request`, `Transfer`, `Check this code on the sending device before accepting.`, `Show in folder`, `Saved files`, `Nearby advertised device`, `Unavailable`, `Folder`, `Clipboard`.

`Send` already exists in the 17-key MENU catalog: reuse, do not duplicate. Dynamic peer names, filenames, four-digit PIN, count and byte values MUST remain display-only and use existing `i18n::isolate_untrusted`, `i18n::pin_code`, `i18n::file_count`, `i18n::byte_count`; do not interpolate untrusted text into a static key. A row for one file currently displays only a whitelisted ASCII extension when present (otherwise `File`), which should remain unchanged. A missing payload source is `Waiting for details`, **never `0 files`**, and an unsupported payload is not silently labeled Files.

## Implementation design / invariants

1. New `native_desktop/src/i18n/transfer_rows.rs` with 15 canonical keys, 19 model-assisted non-English translated rows, English fallback (including pt_BR / zh_Hans). All locale catalog keys must be distinct from current module MENU, Settings, History and Composer keys.
2. Attach source-key translations at precise UI presentation points: `payload_label` status strings and categories; `peer_title` anonymous Connecting; `transfer_row` incoming/transfer title; tooltip + accessible description around PIN; saved file action and menu; nearby device aria/tooltip; File/Folder/Text/Clipboard choice tooltips; Send reuse. Do not translate Pango icon names, stable GTK widget identifiers, route IDs, `Phase` variant names, file extension payload, timestamps or peer-supplied text.
3. Fix a **display-only spoofing gap** in completed file list: `label(&file.name)` currently inserts raw peer-controlled text into a label; use the existing `i18n::isolate_untrusted` to filter bidi override/isolate controls only in the shown string. Original completed-file pathname/capability/reveal target remains unchanged.
4. Preserve `ready`, pending-control, `AwaitingConsent`, `CancelRequested`, `reveal_revision`, `consent_revision` and exact endpoint binding predicates byte-for-byte. Nonauthenticated `Nearby advertised device` must NEVER be translated into `Verified` or imply account trust. PIN display remains user-reviewed; no auto accept or clipboard read/write.
5. Focused source tests: 20/20 complete rows, unique/disjoint keys, source literal RU/PT/Arabic expected captions, no two errors rendered as `Complete`, explicit consent string distinct from automatic send, stale metadata `Waiting` vs unsupported, English fallback, GPL/privacy invariants, malicious filename with embedded U+202E/U+2069 remains unchanged in original domain object and safely projected. Existing `rows.rs` tests still exercise actual `payload_label`, `peer_title`, focus transfer and progress.
6. Exact-source **Preflight**, **Native GTK UI strict rustfmt/Clippy/Xvfb** on final last source SHA; if formatter fails, repair only exact diff and recheck. No full Mode B, physical KDE/Pixel or human native-speaker/RTL signoff is inferred from static source tests.

## Risk, rollback and later work

Risk: translation of `Accept/Decline` source labels may confuse consent, but those keys are elsewhere, not to be moved in this slice. Dynamic RTL long names and file counts may exceed row width; GTK narrow-screen acceptance remains separate. Revert isolated catalog and row label changes independently, no daemon/session/protocol/persistent config effect. After this bounded slice, continue `reveal.rs`, notifications, countdown, outbound, tray and packaging/accessibility audit. Do not close #17 until all intended surfaces and hardware/semantic gates are proven.

**Before implementation:** planning only, 15 new keys and source-level secure projection are NOT YET IMPLEMENTED.

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

## Implementation checkpoint (2026-10-27)

Production `3fceee03cd2d13bcf6955eb21b874af630563a9a`: added `src/i18n/transfer_rows.rs` with **15 exact static keys ×19 non-English rows = 285 draft translations**, English fallback and 20-locale completeness/error/consent distinction tests. Wires `panel/rows.rs` metadata states, safe file kind/fallback, anonymous connection text, incoming title, PIN-review accessible description/tooltip, Save-in-folder, Saved files menu, nearby *advertised* (not authenticated) status, file/folder/text/clipboard send choice tooltip+accessibility labels. Existing `Receive`/`Send` use canonical menu translations, without duplicate keys.

Security and source testing: `label(&file.name)` in the completed-file action popover now uses `i18n::isolate_untrusted(&file.name)`, which strips Unicode bidi controls only from the displayed copy; original file record, path, chooser, reveal permissions and file bytes are unchanged. Single-file extension remains ASCII whitelist and content-type/bounded-uppercase identifier; focused regression tests now assert whitelisted remote-derived extensions remain ASCII while **localized** fallback `File` may contain Unicode (a correct distinction). Metadata statuses continue to distinguish Waiting/Unsupported/Ready and do not manufacture `0 files`; count remains exact and localized. Existing keyed Accept/Decline/Cancel, `reveal_revision` and `consent_revision` predicates are not modified. No trust badge, device alias merge, auto-consent or clipboard side effect added.

Static-source audit **PASS**: all 19×15 rows, module route, catalog registration, direct GTK bindings, original consent gates and user-file bidi projection. **Source-exact Preflight `40573609635` and Native GTK `40573609686` were IN PROGRESS**, not terminal green. Do not claim visual Arabic/RTL/native-speaker acceptance from static tests. Remaining #17: `reveal.rs`, result and countdown, outbound/dismissal and tray/notifications, packaging accessibility, and 20/20 visual semantics. #16 Google-compatible certificates OPEN, #8 sole approved exception; no Mode B.

## 2026-10-27 GTK formatter-only correction and final gate

Initial `3fceee03` Native GTK `40573609686` completed **FAIL** on five source-format-only diffs: multiline duplicate-key and PT consent assertions; compact table formatting; multiline File/Folder/Text chooser tooltip tuple expressions; concise metadata label assertion. No GTK Rust compilation or widget test executed in that failed run. Formatter-only `1a422353a45aa337e5150e6f8ba7459a723d357a` applies the runner's exact diff, preserving all translated bytes, semantic assertions, controlled payload classifier, consent/Cancel/reveal gates, and untrusted filename display projection.

Exact-source Native GTK `40574150162` and Preflight `40574149980` were **IN PROGRESS/PENDING** at this checkpoint. No end-to-end hardware, human RTL/20-language, final Mode B or installed-package validation is inferred. The first-source Preflight `40573609635` does not establish this final corrected SHA. If a strict gate fails, inspect exact failing job log rather than weakening test conditions or repeatedly rerunning.

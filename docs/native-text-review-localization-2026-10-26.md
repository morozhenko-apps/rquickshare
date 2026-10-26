# Native GTK received-text review localization, #17

Date: 2026-10-26. Mode A feature slice. Branch: `feature/native-text-review-i18n-20261026`; based on `dev` at `612fdb78722eaed68d0f009b7b4dd24a3e337cda`. The #16 authenticated-route work is concurrent on `dev`; do not touch its production files or the protected `master`/release tracks.

## Scope and source inventory

The received-text review window in `native_desktop/src/panel/text.rs` currently displays eight English messages: `Received text`, `Received link · plain text`, `Copy`, `Close`, the untouched-clipboard disclosure, copy-unavailable warning, copy-failure warning, and copy-success confirmation. These are static application-owned strings; the received text itself is untrusted user content and must never go through localization or markup.

The existing `native_desktop/src/i18n.rs` exposes a 20-code catalog for the tray/quick actions and `label(&'static str)` with system-locale selection. Extend this catalog with an *independent typed-size review section* without altering earlier 17 key positions or locale mapping. The canonical codes are `en,ru,es,pt,de,fr,it,pl,hi,zh-Hant,ja,nl,sv,tr,ko,bn,th,id,fil,ar`; `pt` means European Portuguese, `zh-Hant` means Traditional Chinese, and unsupported `pt_BR`/`zh_Hans` fall back to English. No preference, D-Bus, storage or protocol schema changes.

## Implementation and risk boundaries

1. Add the eight complete native review translations per canonical locale. Preserve the exact source-key inventory and explicit English fallback for unknown keys.
2. Route only the eight static labels in `panel/text.rs` through `i18n::label`. Preserve the copy button's explicit, capability-guarded event handler and clipboard write; no clipboard read, auto-copy, auto-open, content markup, or changed success state. Do not localize `body.text()`.
3. Keep consent/PIN, settings, history and other dynamic surfaces separate; this is a bounded #17 contribution, not full GTK localization.
4. Add deterministic per-locale catalog invariants, expected RU/PT security wording, unsupported-locale fallbacks and all review-key coverage to existing source tests. Check source exactness before GitHub CI.

## Risk, validation, rollback

- Security invariant: `Nothing has been copied` must never imply an automatic clipboard write; failed/blocked copies must not be translated as success. All strings are static and untrusted peer content stays escaped by GTK Label markup-disabled handling.
- Language risk: draft translations require human native-speaker contextual review. Automated cardinality and key tests do not establish linguistic quality, bidi visual correctness or font availability.
- Planned final gate on the feature branch: native GTK verification and full applicable preflight, with STABLE vs known-FLAKY results reported separately using actual source-exact CI receipts. No local Rust/GTK toolchain is assumed in the current ChatGPT environment; no tests are claimed passed without execution evidence.
- Manual acceptance: KDE screen-width/font fallback, Arabic RTL and mixed-script untrusted text; copy disabled state, successful Copy and clipboard-failure warning, switching locale by environment before launch.
- Rollback: revert the documentation-only commit independently from executable commits, then revert the i18n and review-window edits. No persistent migration, package, signer, Pixel or network changes.

**State on plan creation:** NOT IMPLEMENTED, NOT TESTED. Issue #17 remains open. #16 and release/security gates remain unchanged.

## Implementation checkpoint (2026-10-26)

- `e0238edd` adds all eight static review strings for the 19 non-English catalog rows (152 translated entries; English is canonical fallback), eight-key tests and PT/RU action/status oracles.
- `32dc2bb` applies those translations only to the received text/link review labels, without changing the guarded explicit copy callback, untrusted content path or markup-disabled body.
- `6a0bfb27` reformats all 19 new match arms into the project's Rust per-string row style. No protocol/identity/consent changes.
- Independent source structural audit **PASS**: 19 × 8 row cardinality, eight key definitions, conditional link-vs-text heading, six remaining label bindings, preserved `label(body.text())`, `set_use_markup(false)` and `copy_explicitly()` invocation. This **is not** a Rust compile, test execution, visual acceptance or a professional translation review.
- [Draft PR #28](https://github.com/morozhenko-apps/rquickshare/pull/28) targets `dev` while the main branch is concurrently being modified for #16. CI source-exact results pending and must be recorded before merge. **#17 remains PARTIAL** until the other native surfaces and locale layout/context checks pass.

Do not auto-merge this branch, close #17 or publish any `master`, `native/v*`, APT or R2 asset on the basis of this limited static check.

### Extended source guard (after review)

- `8909e652`: test every locale for distinct Copy/Close actions, positive Copy vs failed Copy, positive Copy vs blocked Copy, and text vs link heading; no test mocks the translation subject. These are structural/semantic-collision oracles, not full human translation review.
- `85eea37a`: wrap disclosure and error labels to conform to Rust source style.
- Independent read-only source checks **PASS** on these two commits: 19 complete catalog rows × 8 literals, eight canonical source keys, all four per-locale collision oracles present, unchanged manual Copy callback, raw received-body label without markup, and no newly overlong i18n UI calls. **No Rust/GTK tests or formatter executed locally.**
- Remaining merge gate: exact-current-HEAD Preflight **and** Native GTK build/clippy/isolated widget tests. The Native GTK workflow currently handles `push: dev` and manual `workflow_dispatch`, not feature-branch pull requests. A green feature PR Preflight alone is insufficient to claim its GTK gate passed; do not auto-merge or silently trigger the protected release workflow.

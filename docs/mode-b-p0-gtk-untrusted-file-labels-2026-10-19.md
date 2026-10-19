# MODE B P0: Native GTK untrusted file summary labels — 2026-10-19

## Stages 1–2: scan and invariants
Source reviewed: `native_desktop/src/panel/rows.rs::payload_label`, existing `rows.rs` tests, `rqs_lib::service::client::state::{TransferDetails,FileDetail}`, and exact-SHA [Native GTK coverage 39209632695](https://github.com/morozhenko-apps/rquickshare/actions/runs/39209632695). Baseline `panel/rows.rs`: 62.99% lines (862 total, 319 missed). This is test-only MODE B on `dev`. Untrusted remote filenames may project an ASCII alphanumeric 1–12 byte *extension*, uppercased, without path, markup, control or non-ASCII characters. Unsafe or missing extension maps to generic `File`; other payload types and multi-file manifests must use fixed kind or cardinality labels. Never treat a label as authorization for file access.

## Stages 3–4: test inventory and branch map
Exactly **two named tests**, added only within existing `rows.rs` test module. Method `payload_label` has branches: (1) Files with exactly one file; `rsplit_once` None vs Some, empty extension, length 1/12/13, ascii alphanumeric vs Unicode, punctuation, uppercase; (2) Files 0/2/3 manifest count; (3) Text, Url ignore file metadata. Existing `arrange`/row-rendering tests cover other UI behavior; portal/reveal lifecycle requires real panel/client, is **NOT** closed by these pure tests.

## Stages 5–6: Positive/N1–N12 and interactions
Positive: ASCII extension and file count, fixed Text/Url. N1 malformed/no period, punctuation, HTML-like input; N2 1/12/13-byte boundary; N3 no transport (N/A); N4 deterministic repeat; N5 pure single-threaded (N/A); N6 no persisted data (N/A); N7 never convert filename into an authority; N8 Unicode extension rejected as generic; N9 no UI event (N/A); N10 no raw path/markup echo; N11 no entitlement (N/A); N12 no filesystem (N/A). Matrix: basename shape × extension alphabet × byte length × count × kind, with explicit equality assertions. This is *scoped* category coverage only, not all repository N1–N12.

## Stages 7–10: coverage map and mutation review
Target production method `native_desktop/src/panel/rows.rs::payload_label`. Test file same module. Mutation candidates: accepting 13-byte or Unicode/punctuation extension; leaking original raw path rather than `File`; wrong uppercase or manifest cardinality; selecting file extension for Text/Url. Mutation testing is proposed, not executed. No product/architecture changes.

## Stages 11–12: verification and remaining gates
Run Native GTK UI verification at exact test SHA (and applicable Preflight if triggered). Source-matched line coverage baseline is **BEFORE** these tests, so do not claim improved numeric coverage without rerunning llvm-cov. `panel/reveal.rs` (10.87%), `preview.rs` (23.33%) and hardware BlueZ/mDNS remain separate real UI/OS integration gaps. Strict full 10× suite, global per-file branch analysis, hardware acceptance and Mode B Stage 12 remain OPEN.

# Native GTK preview-test executable fixture: ETXTBSY incident

Date: 2026-10-27. MODE A defect correction, not a product or protocol change. Source baseline `cb572c9ff4309c92d8d8adba4e2083b93b042545` on `dev`. The image preview sandbox/test code predates this localization slice; do not classify the first observed failure as an i18n regression or silently reclassify it as flaky.

## Reproduction / evidence

Exact-source Native GTK `40537441309` (source `e9b4e4b434700933da61a585d62794d53d39f775`) passed strict native GTK source format and Clippy; test step `Native GTK isolated widget and controller suite` compiled `rquickshare-desktop` and ran 10 preview tests: **9 pass, 1 fail**. The failing `preview::tests::given_external_process_failures_when_cancelled_or_malformed_then_owned_children_are_reaped` panicked at `native_desktop/src/preview/tests.rs:283` while probing a newly generated script: `Os { code: 26, kind: ExecutableFileBusy, message: "Text file busy" }` (Linux `ETXTBSY`). The code creates executable Python text at `tempfile::tempdir()/fixture` via a path write followed by chmod; it then launches the script via the real `Decoder` and only on unexpected result probes the exact executable via `std::process::Command`. This direct probe exception obscured the Decoder-vs-expected category mismatch, but proves executable publication/launch was involved. A prior native GTK `40511449070` completed SUCCESS with the same unchanged preview test source, so this is an intermittent test/environment interaction until root cause is confirmed.

Expected: fixtures are fully written, closed and executable before the **first** decoder attempt; the real decoder must classify malformed/truncated/oversized/crashing child output safely, and cancellation/deadline must reap owned child processes. No logic, expected classification, direct sandbox, child ownership or process lifecycle guarantee may be weakened to make the test green.

## Plausible causes and minimal diagnostic reasoning

1. A writable file descriptor overlaps the executable launch, causing `ETXTBSY` (direct kernel errno on probe; highest priority).
2. Publication of a newly-written executable under a busy/mounted CI target temp dir before staging/finalization; filesystem interactions may amplify race (possible contributor).
3. Concurrent fixture modification by another test or external process (unlikely: unique `TempDir` and unique script path).
4. `/usr/bin/python3` itself being replaced while a script is started (possible but no direct evidence; absolute shebang reduces PATH interpretation).
5. Missing Python or invalid shebang (not supported by `ETXTBSY`; would normally yield `ENOENT` / exit error).
6. Decoder stdout/parser/timeout classification defect independently of executable launch (possible secondary issue; first recover original `actual` mismatch without unconditional probe `expect`).
7. Service or cancellation behavior (not in the failing first malformed-output fixture startup path; fails in direct probe without daemon).

## Bounded implementation plan

- Write the script to a **distinct staging filename** inside the unique temp directory; complete and `sync_all()` its file, close its writable handle, chmod the staging inode, then atomically rename the complete fixture to the executable path. The executable path never names a file while the test holds a writable handle. This is a deterministic fixture-publication invariant, not a retry/sleep.
- Keep the real `Decoder::decode` invocation **first**. Only on unexpected result, report whether the independent disposable probe spawned, retaining the original Decoder actual/expected error and errno. Do not blindly `expect` an OS spawn before reporting the main test mismatch.
- Preserve the exact existing test oracles for malformed magic, excessive output, child exit, cancellation, deadline and child reaping. No mock subject under test, no arbitrary retry, timeout or excluded test.
- Code/test-only edit in `native_desktop/src/preview/tests.rs` after this standalone documentation commit. Source-exact Native GTK rustfmt, strict Clippy and actual widget suite plus Preflight once on final source; do not infer green from a local static check. If ETXTBSY persists after atomic publication, next diagnostics should capture process ownership and write-open descriptors on that exact runner without changing product decoder behavior.
- Roll back test-only commit independently. No release/master, protocol, clipboard, peer identity, installed app or R2 changes. This addresses a CI test infrastructure fixture defect, not application image-preview security.

**State before source change:** root cause candidate documented; test fix NOT YET IMPLEMENTED or VALIDATED.

## Implementation and final verification checkpoint

Test-only source `1fe649ed8b3947e4d61ffdc11d3050adee444f70` adopts the deterministic fixture hand-off: unique staging path, explicit `write_all`, `sync_all`, closure of writable `File` scope, chmod and atomic rename into `fixture` only after no test-owned writable handle remains. The diagnostic probe still runs ONLY after actual Decoder classification differs from the expected error. If the probe itself cannot exec, its errno is reported alongside `actual` and `expected` rather than swallowed by a generic `expect`. No production module or test classification changed.

Static source audit confirms the writer is scoped/dropped before renaming, that real `Decoder::decode` remains the first launch, `assert_eq!(actual, expected)` and cancellation/deadline/child-reaping assertions remain present. This is **not** proof that `ETXTBSY` is eliminated until real-runner CI completes. Native GTK workflow `40539064009` and Preflight `40539063998` were **IN PROGRESS/QUEUED** for exact source `1fe649ed` at checkpoint; do not claim green. Older `e9b4e4b4` Preflight `40537441279` was terminal **SUCCESS** across all required jobs, and its native GTK `40537441309` failed the single preview fixture test after strict formatting/Clippy PASSED. Source code coverage of settings errors was not the failing test path.

Next continuation: read terminal `40539064009` and `40539063998` jobs; if failing, inspect the exact step/log before changing code. Any repeated `ETXTBSY` should be diagnosed with open writable descriptor evidence rather than arbitrary wait/retry, and the test must stay required.

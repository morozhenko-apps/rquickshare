# MODE B P0: Tauri received-file path containment and input boundaries — 2026-10-19

## Stage 1. Repository scan
Branch `dev`, baseline `981e08b252bb8efba88ba2c6f05032ebf47bd68a`; test-source last run `ea47d49d` [Preflight 39282270240](https://github.com/morozhenko-apps/rquickshare/actions/runs/39282270240) and [GTK 39282270300](https://github.com/morozhenko-apps/rquickshare/actions/runs/39282270300) both **SUCCESS**. Read `agent.md` SSOT, `app/main/src-tauri/src/cmds/{open_download_destination,discovery,change_download_path,send_to_rs}.rs`, `app/main/src-tauri/src/store.rs`, Tauri Cargo dependencies and the [measured MODE B baseline](mode-b-coverage-baseline-2026-10-19.md). Prior Tauri instrumented coverage 23.86% lines, including several lifecycle command entrypoints with 0% coverage.

Primary target: **real production functions** `validate_received_relative_path` and `resolve_received_item` in `app/main/src-tauri/src/cmds/open_download_destination.rs`. Existing tests cover eight traversal/string variants and ordinary nested paths, but **not symlink escapes, same-prefix sibling escape, symlink-to-inside, lexical 4096-byte boundary or maximum depth/segment size**. Do not fake filesystem or call external file manager; use an isolated real temporary directory tree and the existing private functions. No public API, implementation, policy, packaging, `master` or production changes.

## Stage 2. Risks and invariants
P0: untrusted filename/path input from the frontend must never cause the file manager to reveal a target outside the canonicalized download directory. `resolve_received_item` must reject a symlink pointing outside, even if its lexical path lies inside the directory; must reject a symlinked directory escaping to a sibling whose *string prefix* matches the legitimate path; may resolve legitimate symlinks strictly within the destination, and must leave missing paths as `Ok(None)` (not open them).

P0/P1 parser boundaries: full input 1..=4096 **bytes**, <=64 path components, each nonempty normal component <=255 UTF-8 **bytes**, no '.'/'..', absolute path, backslash, NUL or Unicode control character. Unicode visible letters are allowed by current contract; do not introduce broader Unicode-visual confusables rejection under MODE B. Native slash remains the separator. No log/content/payload side effects.

## Stage 3. Scoped exhaustive test inventory
| Production method / condition | Level | Risk | Named test |
| --- | --- | --- | --- |
| `resolve_received_item`: final symlink outside, symlink directory outside, prefix-collision sibling, valid symlink inside, missing paths | real filesystem integration | P0 | 1 |
| `validate_received_relative_path`: 0/1/4096/4097 input, 64/65 segments, 255/256-byte components, dot/absolute/backslash/NUL/control, unicode valid | pure unit | P0 | 1 |
| `reveal_download_item`: live Tauri AppHandle, OS file manager `ShowItems` and fallback | desktop integration | P0 | OPEN; not claimed by these tests |
| Symlink mutation between check and external file manager | TOCTOU/security boundary | P0 | OPEN; requires separate security contract decision / MODE A, no code changes here |

Exactly **2** new named tests added to the existing `#[cfg(test)]` module in `open_download_destination.rs`, with a platform `#[cfg(unix)]` guard for the symlink integration case. No new dependencies.

## Stage 4. Branch map
`validate_received_relative_path` rejects 0 bytes or >4096 bytes; >64 components; empty, `.`, `..`, >255-byte component; any backslash, NUL or control; any non-`Normal` filesystem component. Accepted unicode/long but bounded safe components preserved verbatim. `resolve_received_item` calls validation first, joins segments, returns None on absent leaf, canonicalizes existing candidate, rejects canonical target failing `starts_with(destination)` by actual *components* (not string prefix), returns full canonical target for legitimate contained item. Real symlink chains are exercised; no mock bypass.

## Stages 5–6. Positive/N1–N12 and interaction matrix
| Scenario | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Relative path syntax | bounded nested Unicode | NUL, traversal, slash, controls | 255/256, 4096/4097, 64/65 | N/A pure parser | stable multiple calls | N/A no race | N/A no persisted versions | reject absolute/outside | valid UTF-8 letters, no normalization changes | N/A | no path echo success | N/A | N/A parser no FS |
| Canonical containment | symlink within root | symlink to outside file / folder | sibling with same prefix | real local FS | missing/broken link None | TOCTOU OPEN, not simulated | N/A no persisted versions | only in-root canonical | Unicode safe | N/A no UI | no outside path accepted | N/A | real filesystem symlink and regular files |

Interactions: request {ordinary, symlink leaf, symlink ancestor} × destination {inside, outside, prefix-sibling, missing} × parser {lexically valid, invalid} × boundary lengths. Test assertions use exact resolved canonical paths and exact *error-vs-None* behavior, not just `is_err()` on arbitrary exceptions.

## Stages 7–10: coverage/implementation/mutation
Source file: `app/main/src-tauri/src/cmds/open_download_destination.rs`, pure validation and path resolution branch coverage. Two `#[test]` functions; the symlink test creates its own PID/thread-named isolated tree under `std::env::temp_dir()` (existing pattern in this file), performs real fs/symlink operations, removes the tree after assertions; avoids global app configuration and network. Mutation targets: removing `canonicalize()`, replacing component-aware `starts_with` with string-prefix test, allowing parent segments, allowing 4097-byte paths, accepting 65 segments, or permitting 256-byte segment. No mutation engine execution claim.

## Stages 11–12. Execution and gate
Use exact-SHA Preflight `Clippy (app/main/src-tauri)` and Tauri desktop tests, Rust format, Core, frontend; GTK if triggered. The old 23.86% Tauri percentage is *pre-suite baseline* and must not be treated as updated. The strict entire-suite 10× `mode-b.yml`, current-commit llvm-cov, native file manager/Android/BlueZ/mDNS hardware and whole-repository Stage 12 remain OPEN. Rollback: remove 2 tests and this documentation. Any uncovered TOCTOU architecture decision belongs to MODE A and needs a separate approval.

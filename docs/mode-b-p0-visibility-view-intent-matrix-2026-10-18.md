# MODE B P0: visibility wire-view and intent validation matrix — 2026-10-18

## Stages 1–2: repository scan, risks and invariants
Source scanned: `core_lib/src/service/visibility_service.rs` (projection, `VisibilityView::valid`, `VisibilityIntent::valid/base` and two existing tests), `core_lib/src/service/settings_service.rs` (`valid_hex` canonical token grammar) and `core_lib/src/service/client/visibility.rs` (client refuses invalid daemon views/intents). Test-only scope: `core_lib/src/service/visibility_service.rs`. No daemon, bus, network or UI behavior changes.

P0 invariants: a wire view with the wrong daemon epoch, out-of-range mode/remaining, temporary countdown without a running temporary mode, or unavailable-but-running/nonzero-revision state must not be trusted. A submitted visibility intent must have the exact current epoch, bounded mode and **canonical lowercase hexadecimal** 64-byte token / 32-byte request. `base()` must preserve the submitted revision and token unchanged, not silently generate a new generation. Existing tests touch individual malformed view fields and one valid intent, but do not cross the combinations or probe token/request grammar boundaries.

## Stage 3: exhaustive in-scope inventory and estimation
| Production method / branch | Risk | Test class | New named test count |
| --- | --- | --- | ---: |
| `VisibilityView::valid` and availability/running/mode/remaining/revision constraints | P0 | Cartesian table, independent predicate | 1 |
| `VisibilityIntent::valid`, `::base` and `settings_service::valid_hex` transitively | P0 | Input partition matrix, exact field preservation | 1 |
| `VisibilityView::project`, actual temporary deadline renewal/expiry | P0 | Existing deterministic `start_paused` tests | 0 |
| Client D-Bus `read_visibility/set_visibility/retry_visibility` | P0 | Separate bus integration | 0, remains OPEN beyond this scoped suite |

## Stage 4: source branch map
View valid conjunction: exact lowercase padded 32-character epoch; `mode <= 2`; `remaining_ms <= 60_000`; countdown only with `running && mode == 2`; stopped implies `mode == 1`; unavailable implies `!running && revision == 0`. Intent conjunction: exact epoch; `mode <=2`; token exactly 64 lowercase ASCII hexadecimal bytes; request exactly 32 lowercase ASCII hexadecimal bytes. `base()` clones exact revision and token.

## Stage 5: Positive/N1–N12
| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| View validation | Available+running/stopped legal states | Wrong epoch, mode | remaining 0/1/60000/60001/MAX, rev 0/1 | N/A no transport | All state cross products | N/A pure function | N/A no persisted snapshot | Unavailable cannot assert running | N/A locale | N/A retries | Reject forged state | N/A billing | N/A FS |
| Intent/base | Exact canonical tokens, modes 0..2 | Invalid token/request, epoch | length 0/31/32/33/63/64/65, Unicode | N/A no transport | repeated validation | N/A pure function | revision `0`, `u64::MAX` retained | N/A | N/A | N/A | Reject noncanonical IDs | N/A | N/A |

## Stages 6–8: interactions, file coverage map and expected count
View: 2 epoch validity x 2 available x 2 running x 4 mode x 5 remaining x 3 revision = 480 cross-product conditions; expected value computed from independent contract conditions (not `valid()`). Intent: 3 mode options + out-of-range modes and token/request edge cases, epoch mismatch and base equality. Total: 2 new named tests, >= 480 concrete validation combinations. Mutation candidates: invert any conjunction; accept countdown when stopped or mode != temporary; treat unavailable as available; allow wrong epoch, uppercase token, bad request length, or rewrite revision in `base()`. Manual mutation analysis, **not measured kills**.

Covered production file/method map: `core_lib/src/service/visibility_service.rs` `VisibilityView::valid`, `VisibilityIntent::{valid,base}`; transitively canonical `valid_hex` in settings service. Out-of-scope: client D-Bus protocol resilience, actual mDNS/GATT on-air visibility, hardware.

## Stages 9–12: execution and strict gate
Implement tests only after this plan, run exact-source Rust format/Core tests/Clippy/Native GTK/Preflight, repair diagnostics without weakening contracts. Exact-source 10 consecutive **full** CI suite runs, independent current-commit instrumented per-file coverage, mutation execution, and global Stage 12 still OPEN. No merges or release. Rollback remove two tests and this document.

## Stage 9 implementation checkpoint

- Documentation-first plan committed in `d5d696ee`.
- Two scoped unit methods added in `core_lib/src/service/visibility_service.rs` at `6754ab0e`: **480** view-state cross-product evaluations (epoch x available x running x mode x countdown x revision), plus canonical token/request/mode/epoch/base preservation matrices. These call real production validators.
- Initial `rustfmt` job found one whitespace-only line-wrap change in a matrix predicate. Corrected at `0d4f3ad5`, without relaxing assertions or changing tested production semantics.
- Exact-source jobs: [Preflight 39167407105](https://github.com/morozhenko-apps/rquickshare/actions/runs/39167407105) and [Native GTK 39167407102](https://github.com/morozhenko-apps/rquickshare/actions/runs/39167407102), **both running at the checkpoint, neither yet a confirmed pass**. As with other scoped suites, current-source full ten-runs and Stage 12 remain OPEN.

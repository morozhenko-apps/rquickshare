# MODE B P1 — native tray/card placement and drag coordinate boundaries

Date: 2026-10-18. Primary MODE B Testing, branch dev.
Only `#[cfg(test)]` native Rust changes; production
positioning, GUI, master, packages and release untouched.

## Stages 1–2 — scan, risks and invariants

Read user's agent.md MODE B SSOT, global current state
and completion audit, `native_desktop/src/placement.rs`,
`native_desktop/src/panel/movement.rs`, and Native GTK CI.
Prior verified test source `66ddf1fc`:
Native GTK **64/64**, Core **744** stable,
desktop 10; stage 12 global NO. Existing tests cover
four tray edges, negative monitor origins, valid moves,
a few invalid sizes and bottom-anchor resizes.
Missing interaction coverage: a grid of unequal panel
sizes, nonzero signed display origins, near-i32
limits, independently reconstructed margins and
tie-breaking. Screen placement must **never**
return an off-monitor card, overflow, guess for
nonpositive geometry, or unexpectedly reorder
edge tie priority. Bottom-pinning must use exact
checked subtraction and reject impossible heights.

## Stage 3 — full bounded test inventory

| Production function | Risk / branch group | Planned new methods |
|---|---|---:|
| `placement::place` positive monitor corners/centers with independently reconstructed margins | P1 geometry | 1 |
| `placement::place` equal-distance ties and extreme/invalid logical coordinates | P1 visual origin | 1 |
| `panel::movement::clamp_position` broad limits and exact independent clamp result | P1 drag bounds | 1 |
| `panel::movement::bottom_margin` checked inverse and negative/oversized geometry | P1 bottom anchor | 1 |

Exactly four independent Given_When_Then `#[test]`
methods in two existing test modules. No synthetic
GTK controller or production seam.

## Stage 4 — branch map

1. `place` with monitors {small, 360×250, 1920×1080,
   nonzero signed origin}, panels {1×1, 20×30,
   120×100, monitor exact-fit}; offsets at corners,
   middles and edge-adjacent positions: reconstruct
   logical placement from edge/margins, assert within
   monitor and nearest-edge pin relation.
2. `place` ties (left/top vs right/top vs
   left/bottom vs right/bottom), extreme signed
   origins where relative coordinates must not wrap;
   off-monitor coords and oversized panel rejected.
3. `clamp_position` table of monitor/card sizes,
   requested x/y including negative, maximum,
   beyond-edge and i32 extrema; compare independently
   clamped x/y; oversized/zero/negative monitors reject.
4. `bottom_margin` valid top and height summing
   <= monitor, exact inverse `monitor-height-top`,
   invalid top/height/monitor and i32 extreme
   inputs rejected without overflow.

## Stage 5 — Positive/N1–N12 matrix

| Artifact | Pos | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `place` | Planned valid | invalid geometry | signed boundary | N/A no network | deterministic repeats | N/A pure | overflow rejection | visual containment | N/A | N/A | no offscreen | N/A | N/A |
| `clamp_position` | Planned valid | invalid size | i32 boundary | N/A | repeat | N/A pure | checked/clamp | visual containment | N/A | N/A | no offscreen | N/A | N/A |
| `bottom_margin` | Planned valid | negative input | exact fit and +/-1 | N/A | repeat | N/A pure | checked subtraction | bottom-anchor containment | N/A | N/A | no offscreen | N/A | N/A |

Other categories N3/N5/N8/N9/N11/N12 do not
apply to these pure layout functions; no peer
or storage claims.

## Stage 6 — interaction map

Monitor {origin negative/zero/near i32 extreme}
× dimensions {tiny/normal/large}
× panel {tiny/exact fit/oversized}
× activation {corners/edges/center/off-monitor}
× drag {negative/valid/excessive/i32 extremes}
× bottom anchored {zero/valid/too-tall}.

## Stage 7 — planned counts

4 original methods; Positive 4; N1 4;
N2 4; N3 0 N/A; N4 4; N5 0 N/A;
N6 4; N7 4; N8 0 N/A; N9 0 N/A;
N10 4; N11 0 N/A; N12 0 N/A.
These labels overlap and are not global
coverage percentages.

## Stage 8 — production file coverage map

`native_desktop/src/placement.rs`: `place`
geometry validation, nearest edge, clamping
and margin reconstruction. GLib X11/Wayland
monitor detection and actual pointer coordinate
translation still need physical acceptance.
`native_desktop/src/panel/movement.rs`:
`clamp_position`, `bottom_margin`.
GTK gesture callbacks, actual layer-shell
anchors/monitor hot unplug remain unverified.

## Stage 10 — mutation sensitivity

Relax a <= size rejection, confuse monitor vs
card origin, add widths in i32 without checked
intermediate, select a different nearest edge
on tie, allow negative clamp, drop inverse
bottom subtraction: each contradicts explicit
independently calculated assertions. Manual
review, not executed mutator.

## Milestones

1. [x] Repository scan, risk, exhaustive bounded
   inventory, branch/negative/interaction/mutation
   and per-file map written before tests.
2. [x] Add all 4 independent methods.
3. [x] Run full Native GTK and quick Preflight
   on final test SHA, inspect actual method lines.
4. [x] Record D1–D5 and global index.
5. [ ] Full strict 10x and Stage 12 global remain NO.

Rollback test and docs only.

## Source checkpoint

Four original `#[test]` methods added in
`native_desktop/src/placement.rs` (test-only
commit `f936aab3`) and
`native_desktop/src/panel/movement.rs`
(`5a3056bf`). The first two assert
reconstructed card margins inside three
monitor dimension sets × three signed
monitor origins × four card sizes ×
25 activation locations (900 cases), plus
eight explicit nearest-edge tie cases,
i32 extreme origin and rejection cases.
The drag test checks 4 monitor dimension
sets × 7 panel sizes × 7 signed drag
offsets (196 cases), comparing with
independent i64 widened bounds. The
bottom-anchor test runs 7 monitor heights
× 6 top margins × 6 card heights (252
cases), comparing an independent i64
inversion and invalid geometry.

Counts are *table rows*, not new Rust
methods; 4 independently named methods
are planned. Real GTK/Wayland monitor
hotplug remains OPEN. Tests compile
and execute only after actual CI
evidence on source `5a3056bf`
(or a later format-only SHA).
No production changes.

## First CI red-state triage

Initial exact code `5a3056bf`,
[Native GTK 39134805749](https://github.com/morozhenko-apps/rquickshare/actions/runs/39134805749)
failed at **native rustfmt before test execution**.
The actual job log lists six layout-only differences:
three in the movement test and three in the
placement test. Test-only formatting commits
`721ebb89` and `00a0a76b` apply the
expected layouts without changing assertions,
input grids or runtime code. The intermediate
Preflight may have been cancelled by branch
concurrency and cannot count as PASS.

A complete Native GTK + Preflight outcome at
`00a0a76b` must be verified by actual job
logs before any new four methods are marked
passing or 68 native methods asserted.
Global 10× and hardware are unaffected.

## D1–D5 verified execution and residual

**D1 — actual native test verification.** Final test
source SHA `00a0a76bc53bf22c7d2a4312099de559b06bd362`.
[Native GTK 39134995889](https://github.com/morozhenko-apps/rquickshare/actions/runs/39134995889)
completed **SUCCESS**. Inspected entire native job
`117697601520`: `68 passed; 0 failed; 0 ignored;
0 filtered`, a net **+4 independent test methods**
compared with preceding verified native baseline
64. All four original method names occur
**exactly once** in the actual job log as `... ok`.
Strict native fmt and Clippy passed.
Historical red `39134805749` stopped at
six formatter differences before executing
new tests; the precise pure-test layout
fixes `721ebb89` and `00a0a76b`
did not change assertions.

**D2 — actual overlapping category evidence**
(four independently named test methods;
table rows not independent test methods):

| Category | Methods | Verified bounded evidence |
|---|---:|---|
| Positive | 4 | PASS: 900 reconstructed placements, 196 clamped drag positions, 252 bottom-margin combinations, explicit ties |
| N1 invalid geometry | 4 | PASS: nonpositive/oversized rectangles and offscreen positions rejected |
| N2 boundaries | 4 | PASS: exact panel fit, signed near-i32 origin, ±1 activation and max/min draggable values |
| N3 external failure | 0 | N/A pure coordinate functions |
| N4 deterministic repeat | 4 | PASS: stable edge tie comparisons and table-invariant expectations |
| N5 concurrency | 0 | N/A pure functions; real gestures remain external |
| N6 overflow | 4 | PASS: i64 independent arithmetic reference and checked inverse |
| N7 visual containment | 4 | PASS: no card outside monitor and valid bottom anchor |
| N8 locale/time | 0 | N/A |
| N9 cancellation | 0 | N/A |
| N10 UI isolation | 4 | PASS: valid margins within selected logical monitor |
| N11 billing | 0 | N/A |
| N12 storage | 0 | N/A |

**D3 — changed source.** Added four new
`#[test]` methods under existing `#[cfg(test)]`
in `native_desktop/src/placement.rs` and
`native_desktop/src/panel/movement.rs`.
No application behavior/public API/installed
desktop changes; only test layout and
documentation commits.

**D4 — testability.** Pure geometry can
be exhaustively crossed in bounded grids
without a compositor adapter. Actual
Wayland monitor enumeration, screen
scaling, pointer event offsets, hotplug and
layer-shell anchor application require
physical operating-system integration and
are NOT covered by these methods.

**D5 — manual mutation sensitivity.**
An unchecked signed delta, offscreen
rectangle, wrong nearest edge on ties,
non-clamped drag, accepted negative
card height or missing bottom margin
subtraction violates explicit assertions.
This is not a run of a mutation engine,
nor complete UI public-method coverage.
Global 10x and Stage12 **NO**.

**Same-source general CI checkpoint.**
[Preflight 39134995892](https://github.com/morozhenko-apps/rquickshare/actions/runs/39134995892)
stable Core, formatter, frontend and Core
Clippy are SUCCESS. Desktop Tauri completed
**10/10 PASS**, independently verified from
job log `117697629563`, 0 failed/ignored.
The same-source quick Preflight has
**5/5 jobs SUCCESS** at `00a0a76b`.
This is NOT the strict ten-consecutive
full MODE B gate.

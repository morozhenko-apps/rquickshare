# Mode B test plan and coverage map

_Last updated: 2026-10-06_

## Status

**Mode B is not complete.**

The repository has measured line/function/branch coverage, regression tests, a targeted mutation audit, and a pre-smoke stability run. Those artifacts are useful evidence, but they do not yet satisfy the full Mode B SSOT because the mandatory test inventory, branch map, Positive/N1-N12 matrix, interaction matrix, per-production-file coverage map, and literal completion gate are not fully recorded.

No new tests may be added until Stages 3-8 below are complete.

## Scope

Primary scope is draft PR #2 (`feat/pixel-ble-receiver -> dev`) plus the maintenance-pass regressions that protect behavior changed by the PR.

The scope includes all changed production artifacts in:

- `core_lib/src/hdl/blea.rs`
- `core_lib/src/hdl/bwu.rs`
- `core_lib/src/hdl/gatt.rs`
- `core_lib/src/hdl/inbound.rs`
- `core_lib/src/hdl/migratable.rs`
- `core_lib/src/hdl/mod.rs`
- `core_lib/src/hdl/outbound.rs`
- `core_lib/src/lib.rs`
- `core_lib/src/manager.rs`
- `core_lib/src/protocol.rs`
- `core_lib/src/utils.rs`
- `app/main/src/vue_lib/network.ts`

Existing frontend component tests are also in scope where they validate maintenance-pass behavior in `ContentStatus.vue`, `Heading.vue`, `SettingsModal.vue`, and `SideMenu.vue`.

Generated bindings, DTO-only generated protobuf code, package lockfiles, and purely declarative metadata are excluded from branch inventory unless they contain executable product behavior.

## Risk priorities

P0:
- UKEY2 / D2D secure-session state and sequence continuity.
- BLE/GATT bootstrap and BLE -> Wi-Fi-LAN bandwidth upgrade.
- Peer-controlled frame and payload lengths.
- Filesystem path traversal, destination collision, and partial-state handling.
- Peer public-key validation and panic-free malformed input handling.

P1:
- BWU socket routing, cancellation, duplicate registration, and pending-route lifecycle.
- GATT framing/reassembly, queue bounds, malformed connection requests, and disconnect handling.
- LAN interface selection when VPN/Docker/tunnel interfaces coexist.
- Outbound cancellation/consent/file preparation.
- Receive visibility versus outbound discovery lifecycle.

P2:
- Frontend port validation/settings persistence.
- Clipboard fallback and UI state/event regressions.
- Desktop/window/tray behavior that is only representative in packaged smoke.

## Invariants

1. Modern Android discovery may bootstrap over FEF3 BLE/GATT without requiring Wi-Fi discovery first.
2. The full Nearby receiver advertisement is served over GATT while the on-air advertisement remains legacy-sized.
3. Receiver endpoint-info length never exceeds the one-byte protocol limit and UTF-8 truncation remains valid.
4. GATT write buffering is bounded and malformed/oversized framing is rejected.
5. A BWU route can be registered once, cancelled, or consumed exactly once without leaking a pending route.
6. The TCP BWU socket must be routed without consuming bytes required by the inbound state machine.
7. CLIENT_INTRODUCTION must match the expected endpoint and BWU introduction semantics.
8. BLE -> TCP migration preserves the established secure session, keys, and sequence counters.
9. LAST_WRITE / SAFE_TO_CLOSE ordering is preserved during prior-channel handoff.
10. Peer-controlled frame/payload lengths must not cause negative allocation, oversized allocation, overflow, or truncated reads.
11. Inbound filenames must remain one safe path component and duplicate names must reserve unique destinations.
12. A malformed inbound introduction must not partially mutate transfer state.
13. Outbound file preparation must classify supported files deterministically and skip invalid/missing sources without corrupting the transfer plan.
14. Invalid P-256 peer coordinates must return errors rather than panic.
15. LAN selection must reject known virtual/tunnel interfaces and loopback/link-local addresses.
16. No production `unwrap`/`expect`/panic path may be reachable from peer-controlled input in the changed Rust modules.
17. Packaged Ubuntu/KDE behavior remains a required smoke gate for BlueZ coexistence, Wayland UI lifecycle, CSP/freezePrototype, and real Pixel interoperability.

## Current evidence

- Rust stable suite: 50 tests.
- Frontend stable suite: 25 tests across 6 files.
- Rust core coverage: 30.37% lines, 29.75% functions, 31.19% regions.
- Frontend coverage: 46.18% lines/statements, 82.71% branches, 53.33% functions.
- High-value file coverage currently recorded:
  - `hdl/bwu.rs`: 95.83% lines.
  - `hdl/migratable.rs`: 81.40% lines.
  - `hdl/inbound.rs`: 31.67% lines.
  - `hdl/outbound.rs`: 30.67% lines.
  - `utils.rs`: 61.34% lines.
- Targeted mutation audit: 41 mutants, 22 caught, 18 unviable, 1 equivalent/unobservable survivor, 0 timeouts after regression hardening.
- Pre-smoke Mode B run: complete stable suite once plus three selected async transport/BWU tests ten times.

## SSOT compliance gap

The current evidence does **not** yet prove complete Mode B compliance.

Missing mandatory artifacts:
- Stage 3: exhaustive test inventory with risk, level, rationale, expected count, and Positive/N1-N12 applicability.
- Stage 4: executable branch map for every scoped public/business-rule method.
- Stage 5: complete Positive/N1-N12 matrix with no empty cells.
- Stage 6: meaningful interaction combinations.
- Stage 7: expected test-count estimate per artifact/category.
- Stage 8: per-production-file function/branch coverage map with remaining reasons.
- Stage 11: literal SSOT requires ten consecutive successful complete-suite runs; the current workflow repeats only selected stability-sensitive tests.
- Stage 12: explicit YES/NO completion gate for public methods, branches, business rules, invariants, boundaries, N1-N12, and interactions.
- Deliverable D2: test counts broken down by Positive and each N-category.
- Deliverable D4: explicit testability refactors with risk and compatibility assessment.

## Execution plan

### Milestone 1 - Inventory and branch map
Status: IN PROGRESS

- Enumerate every executable artifact in the scoped production files.
- Map current tests to artifacts.
- Enumerate branches and state transitions for protocol/security/filesystem behavior.
- Mark OS-bound branches that require package/live smoke instead of pretending unit coverage is representative.

Rollback: documentation-only commit.

### Milestone 2 - Test matrix and interaction matrix
Status: NOT STARTED

- Build Positive/N1-N12 matrix with explicit N/A reasons.
- Enumerate interaction pairs/triples that can change outcomes, especially transport x visibility x route state, filename x collision x filesystem state, frame length x transport state, and cancellation x consent x transfer state.
- Estimate required test count before implementation.

Rollback: documentation-only commit.

### Milestone 3 - Implement missing deterministic tests
Status: NOT STARTED

- Add only tests justified by the completed matrix.
- Prefer unit/domain and integration/contract tests.
- Do not add sleeps, real network dependencies, or weak assertions.
- If an important branch is untestable, record the testability defect and apply only a minimal reversible seam.

Rollback: atomic `test:` or `refactor:` commits per logical change.

### Milestone 4 - Mutation and coverage review
Status: NOT STARTED

- Re-run targeted mutation analysis for changed high-risk logic.
- Re-measure Rust/frontend coverage.
- Treat mutation survival in a meaningful branch as missing coverage, not as a percentage problem.

Rollback: tests/refactors remain isolated in atomic commits.

### Milestone 5 - Ten-run automated stability gate
Status: NOT STARTED

The SSOT supplied on 2026-10-06 is authoritative for this pass. The complete automated suite must pass ten consecutive runs for the final Mode B gate. This is stricter than the earlier repository policy of stable 1x plus selected flaky-sensitive tests 10x.

The final hardware/package smoke remains separate and cannot be replaced by CI repetition.

### Milestone 6 - Hardware/package smoke and completion gate
Status: BLOCKED ON TARGET HARDWARE

Required live checks:
- install/start on target Ubuntu/KDE/Wayland;
- window/tray lifecycle;
- CSP/freezePrototype;
- Pixel -> Linux discovery and transfer through BLE/GATT -> Wi-Fi-LAN;
- repeat receive without restarting;
- Linux -> Android send;
- FE2C/FEF3 BlueZ coexistence.

After smoke, re-run the final automated Mode B gate and record Stage 12 answers.

## Completion gate

Current state:

- all public methods tested? **NO - inventory incomplete**
- all branches tested? **NO - branch map incomplete**
- all business rules tested? **NO - matrix incomplete**
- all invariants tested? **NO - hardware-bound invariants remain**
- all boundary values tested? **NO - full boundary map incomplete**
- all applicable N1-N12 tested? **NO - matrix incomplete**
- all interaction combinations tested? **NO - interaction matrix incomplete**

Therefore Mode B is **NOT COMPLETE**.

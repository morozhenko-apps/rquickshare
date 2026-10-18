# MODE B P0: BWU concurrent ownership and routing isolation — 2026-10-18

## Scope and provenance (Stages 1–2)
Mode B, tests only. Read `core_lib/src/hdl/bwu.rs`, its 15 existing tests, `docs/mode-b-completion-audit-2026-10-18.md`, `core_lib/src/hdl/gatt.rs` and `core_lib/src/hdl/migratable.rs`. The BWU router hands real Wi-Fi TCP sockets to precisely one pending BLE owner. Existing tests cover single owner/alias, exact precedence, dropped waiter cleanup, duplicate registration and cancellation. They do **not** demonstrate a multi-socket race for one token or concurrent routing to many independent tokens. Both are P0 owner-isolation / privacy invariants.

No production changes, external-device tests, security-policy decisions, releases, master merges, or hardware modifications.

## Stage 3: exhaustive in-scope artifact inventory
| Public method / rule | Risk | Level | New tests | Rationale |
| --- | --- | --- | ---: | --- |
| `BwuRouter::route_with_alias`: one-shot route under concurrent arrivals | P0 | Contract / loopback TCP | 1 | Exactly one of eight competing sockets may acquire one route; all rejected sockets remain owned by caller and usable |
| `BwuRouter::{register, route_with_alias, has_pending}`: independent owners | P0 | Contract / loopback TCP | 1 | Eight independent pending sessions must receive only their respective bytes, with all pending tokens consumed |
| `BwuRouter::cancel` | P0 | Already covered | 0 | Existing tests prove cancellation/late denial, including reused key; do not duplicate |
| `BwuRouter::new` and cloned router sharing | P1 | Already covered; exercised in new tests | 0 | Existing shared ownership and one-shot checks |

## Stage 4: branch map
`register`: empty ID error; reap closed senders; duplicate error; success. `route_with_alias`: reap closed; prefer exact ID; choose admitted alias if exact missing; reject without match; remove ownership; send succeeds or fails if receiver dropped. `has_pending`: purge closed and check emptiness. Test #1 repeatedly contends for one key, observing success/rejected cases and source-byte ownership; test #2 establishes eight disjoint keys and validates exact routing even when a different alias is present. Existing tests already cover the other named branches.

## Stage 5: Positive/N1–N12 matrix
C = existing coverage; P = planned; X = not applicable with reason.
| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| One-owner contention | C+P | C(empty ID) | P(8 sockets, 1 token) | X(loopback TCP healthy) | P(one shot/no replay) | P(simultaneous route futures) | X(no persisted data) | P(deny extra owners) | X(no locale/clock) | C(cancel) | P(no cross-peer bytes) | X(no entitlements) | X(no filesystem) |
| Eight distinct sessions | C+P | C(unknown IDs) | P(8 distinct IDs) | X(loopback TCP healthy) | C+P(all tokens consumed) | P(overlapping route futures) | X(no versioned data) | P(exact identity isolation) | X(no clock) | C(cancellation) | P(no cross-peer bytes) | X(no entitlements) | X(no storage) |

## Stages 6–8: interactions, estimates, file map
Eight candidate endpoints × exact/alias attempt × accepted/denied socket outcomes; eight independent endpoints × route order × distinct short wire payload. Exactly two new named async test methods, hundreds of assertions on real loopback sockets. No wall-clock sleeps, timeouts, fake router, BlueZ or Wi-Fi dependency. Production file coverage map: `core_lib/src/hdl/bwu.rs`, method `route_with_alias` race/ownership and `register/has_pending` independent session lifecycle. Actual multi-adapter network loss, interrupted encrypted BWU migration and BlueZ integration remain OPEN.

## Stages 9–12
Implementation: append both documented tests in `bwu.rs` test module. Mutation review: duplicating successful deliveries, allowing unknown socket admission, choosing alias despite live exact, dropping rejected sockets, overwriting active tokens or routing wrong peer must fail assertions. This is a manual mutation *design*, not measured kills. Run source-matched Core/Preflight and native GTK; final strict full ten consecutive runs and Stage 12 global inventory still OPEN. Rollback limited to tests and this document.

Status: documentation authored before test code.

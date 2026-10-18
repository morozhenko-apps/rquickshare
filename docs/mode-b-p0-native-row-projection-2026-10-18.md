# MODE B: Native transfer row projection (2026-10-18)

## Stage 1-2: scan, risks, invariants
Pure helpers in `native_desktop/src/panel/rows.rs` drive whether transfer rows appear in the nearby panel and whether progress is shown. Existing tests cover a subset of directions, phases, endpoint membership, and byte counts. We must ensure active outbound transfers linked to a current endpoint are not duplicated, missing endpoints do not hide operations, inbound transfers stay visible, terminal transfers never appear as active cards, and progress is finite, clamped, monotonic and zero for unknown size.

## Stage 3: scoped exhaustive inventory
| Artifact | Level / risk | New methods | Reason |
| --- | --- | ---: | --- |
| `bound_current_endpoint`, `card_transfer_visible` | P0 unit | 1 | Full direction × phase × associated endpoint × discovered-set membership cross-product with independent oracle |
| `progress_fraction` | P1 unit | 1 | Boundary, overrun, monotonicity and exact fraction invariants with independent widened-arithmetic oracle |

## Stages 4-5: branch and negative matrix
`bound_current_endpoint`: inbound reject; outbound no associated endpoint; outbound associated peer absent/present in discovered set. `card_transfer_visible`: terminal reject; live outbound with associated current reject; otherwise show. `progress_fraction`: zero total; done <= total; done > total; u64 maximum and overrun clamped.

| Artifact | Pos | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| peer projection | planned | typed NA | phase×peer cross product | NA pure | duplicate membership | NA pure | stale associated peer | no false peer ownership | NA | terminal interruption | no unverified peer association | NA | NA |
| progress | planned | typed NA | 0/1/max/overrun | NA | deterministic repeat | NA pure | inconsistent counters | no over-100% display | NA | zero/partial | no byte value leak | NA | NA |

## Stages 6-8: interaction and file map
Direction (2) × phase (11) × association (3: none/current/stale) × discovered membership (3: empty/current/stale), 198 table cases. Progress totals (0,1,2,3,255,65535,u64::MAX) × completed bytes (0,1,2,3,254,255,65535,u64::MAX), 56 table cases plus monotonic property. Planned new original methods: exactly two. File map: `native_desktop/src/panel/rows.rs`; no changes to GTK rendering or production domain.

## Stages 9-12
Implement two parameterized unit methods in the existing `device_contract_tests` test module. Compare to independent contract oracle, not the production helper itself. Mutation review: terminal-status inversion, associated stale treated as live, reversed Direction and progress over-100% must break assertions. No production refactor. Exact-source GTK and Preflight quick CI required; strict 10×, physical device flows and repo-wide Stage 12 remain open. Rollback test/docs only.

Status: documented before code changes.

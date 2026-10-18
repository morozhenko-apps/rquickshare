# MODE B: Native compact card arbitration (2026-10-18)

## Scope and guardrails
Testing-only continuation on `dev`. No product change, no release, no master update. The pure `native_desktop/src/card_view.rs` selector chooses one compact transfer view from an ordered snapshot; changing priority could conceal an inbound consent request or show a terminal transfer instead of a live one.

## Stages 1-2: scan, risks and invariants
Production artifacts: `Drawer::parse`, `compact_index`. The existing single test checks single transfers and small examples, but not exhaustive ordered state combinations. Preserve:
1. First inbound `AwaitingConsent` wins over any active transfer, regardless of its index.
2. Without incoming consent, first nonterminal transfer wins.
3. Terminal-only and empty snapshots yield `None`.
4. Duplicate and repeated entries do not change ordering priority.
5. Parsing is case-sensitive; only `compact` and `full` are accepted.

## Stage 3: scoped inventory
| Artifact | Risk | Level | Planned original methods | Rationale |
| --- | --- | --- | ---: | --- |
| compact_index | P0: consent priority | unit | 1 | Cross-product of all Direction/Phase pairs in ordered triples, with independent expected-index oracle |
| compact_index | P1: long ordered snapshot | unit | 1 | Delayed inbound consent, terminal prefixes, repeated active and missing consent |
| Drawer::parse | P2: invalid external presentation option | unit | 1 | Exact literals, near-misses, Unicode and NUL |

## Stages 4-5: branch map and Positive/N1-N12 matrix
Branches: iteration with no entries; inbound AwaitingConsent early return; first nonterminal `active.is_none()`; subsequent nonterminals ignored; terminal ignored; result `None` or first active. Parse: compact/full/unknown.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| compact_index | planned | NA typed inputs | planned length/order | NA pure | planned duplicates | NA no concurrency | NA typed enum | planned consent priority | NA time | planned terminal interruption | planned selection privacy | NA billing | NA storage |
| Drawer::parse | planned | planned invalid | planned Unicode/NUL/length | NA pure | planned deterministic | NA pure | planned unknown | NA presentation-only | planned unicode | NA pure | NA no PII | NA billing | NA storage |

## Stages 6-8: interactions, estimate, file map
Ordered triples span 22 distinct (direction, phase) states cubed = 10,648 sequences, plus empty and six-item schedules. Pairwise dimensions are direction × phase × position × prior active × later incoming consent. Estimate: three independent new test methods (thousands of table-driven assertions). Per-file map: `card_view.rs` functions `Drawer::parse` and `compact_index`; branch/enum cases as above. The actual panel binding, GTK rendering, and physical user actions are outside this pure module and remain open.

## Stage 9-12 execution and acceptance
Implement three independent tests using an oracle that separately locates inbound consent before locating first nonterminal transfer; assert exact index (not merely Some). No network, sleep, production mock or real clock. Mutation review must reject swapped priority, wrong phase/direction, last-active-wins, and terminal inclusion. CI evidence must be tied to the final commit: Native GTK and Preflight quick workflow; full strict 10x and global Stage 12 remain NO until separately completed.

Status: documentation recorded before implementation. Rollback: remove this plan and new test methods; no runtime changes.

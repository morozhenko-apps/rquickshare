# MODE B: Native settings acknowledgement matrix (2026-10-18)

## Stage 1 and 2
Scope: pure presentation and acknowledgement gating in `native_desktop/src/panel/settings.rs`, without changing GTK/editor behavior. Incorrect success messages or permission to close confirmation could mislead users after failed storage writes, lost daemon acknowledgement, or stale discovery names.

## Stage 3 inventory
- `confirmation_applied`: unit P0, one exhaustive boolean/enum cross-product method. Exercise available, source, revision, visibility recovery, configured, running, error, applied revision, restart-required and name status.
- `status`: unit P0, one branch-precedence method. Exercise every primary error, discovery failure variants, invalid recovered visibility, source migration/default, service stopped, pending name, live revision mismatch, restart.
- `parse_port`: unit P1, one boundary/whitespace/Unicode/overflow method beyond existing scalar cases.

## Stage 4 branch map
Confirm applied iff all nine conditions pass, including name statuses excluding 1 and 3. Status first checks unavailable, then error 1-4, recovered, sources 0-1, stopped, name 1/3, revision, restart, success. Port trims ASCII/Unicode whitespace; empty=>automatic; digits only and valid u16.

## Stages 5-7 negative/interaction matrix
| Artifact | Pos | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| confirmation | planned | typed NA | cross-product | external errors through state | deterministic | pure NA | corrupt source | never claim unacknowledged | NA | daemon stopped | no false success | NA | uncertain write |
| status | planned | typed NA | all status enum values | disconnected | repeat | pure NA | error precedence | never false applied | NA | restart | truthful failure | NA | storage failures |
| parse_port | planned | invalid strings | 0/65535/65536, long input | NA | repeat | NA | malformed | no invalid port accepted | Unicode whitespace | NA | NA | NA | NA |

Cross boolean gates with independently reconstructed reference predicate; error precedence with contradictory secondary fields; port min/max/excess. Estimated three new named tests. Runtime source and public contracts unchanged.

## Stage 8-12
File coverage map: `native_desktop/src/panel/settings.rs`; GTK dialogs, login integration, port binding and daemon persistence remain outside this pure batch. Manual mutation targets: inverted flags/zero revision/stale applied revision/wrong name status; masked errors; invalid port overflow. Implement tests only; require exact-source CI success, and retain repo global ten-run and Stage 12 as OPEN. Rollback: remove this doc and three new methods.

Status: plan written before implementation.

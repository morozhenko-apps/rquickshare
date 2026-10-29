# MODE B: coherent GATT slot epoch isolation (2026-10-18)

## Stages 1-2: repository scan, risk and invariants
Scope: `core_lib/src/hdl/gatt/slot.rs` only, test-only changes. Coherent reads must pin bytes by peer on offset zero, never splice a subsequent advertisement epoch, preserve an owner's snapshot after an invalid overrun, and evict the oldest admitted owner when 32 leases are full. Legacy noncoherent reads must observe the current bytes and require no lease.

## Stage 3 inventory and estimates
- `SlotValue::read` coherent invalid offset and resume across `install` (P0, unit): one parameterized test, check 1..len-1 and len+1/usize::MAX, exact/overrun semantics.
- `SlotValue::read` peer restarts while capacity is full and advertisement rotates (P0, unit): one test, 32 pinned peers, verify refresh does not evict unrelated peers, count bounded.
- `SlotValue::install` state transition is exercised by both. `SlotValue::new` coherent and legacy covered by existing methods.

## Stages 4-5: branches and Positive/N1-N12
Branches: coherent/noncoherent; initial offset zero vs unprimed nonzero; matching peer vs unknown; valid/in-range, exact-end and past-end offsets; refresh lease; queue full vs not full; installed epoch changes without lease rewrite.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| read/install | planned | typed inputs NA | offsets 0/1/len/len+1/MAX, capacity 32/33 | NA pure in-memory | restart/repeated read | NA synchronous | stale epoch and invalid offset | peer isolation | NA | interrupted continuation | no foreign snapshot | NA | NA |

## Stages 6-8 interactions, coverage map
Peer A/B × epoch old/new × offset valid/exact/invalid × coherent flag × capacity under/full × restart. Two new independently named tests using concrete byte expectations, no mocks or sleeps. Covered source file: `core_lib/src/hdl/gatt/slot.rs`; live BlueZ GATT callbacks remain out of scope and unverified.

## Stages 9-12
Manual mutation review: evicting on existing owner refresh, returning current instead of pinned bytes, consuming a snapshot after invalid offset, allowing a foreign peer to continue, over-capacity queue would fail test assertions. Exact-source Core CI and ten-run strict gate remain separate. Product behavior unchanged; rollback by removing two tests and this file.

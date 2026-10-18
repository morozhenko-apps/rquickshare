# MODE B: fragmented frame header read/recovery (2026-10-18)

## Scan, risk and invariants
`core_lib/src/hdl/frame_header.rs` holds a four-byte header across cancellation. Already covered: all single split boundaries, repeated byte interruptions, consecutive coalesced frames, truncated EOF and forwarded I/O error. Remaining P0 edge: a *completed* prefix must reset state before reading the next frame, even if that next frame is partial or fails; distinct back-to-back fragmented headers must never share bytes. On failure after a partial prefix, the code retains progress by design; the subsequent reader must continue the remaining bytes without silently resetting.

## Inventory, branch and interaction matrix
Two targeted asynchronous methods: (1) all 3×3 initial/next nonempty splits in sequential frames, including cancellation between both frames; (2) a recoverable I/O failure after a consumed partial prefix, followed by continuation through a separate reader. The latter verifies retained progress, no foreign bytes, and correct follow-on frame.

Positive: full 4 bytes; N1 invalid inputs typed NA; N2 splits 1-3 and multiple frames; N3 connection failure and retry; N4 repeated interruption; N5 concurrency not applicable to exclusively borrowed header; N6 malformed/truncated frames already covered; N7 no byte splicing; N8 persistent storage NA; N9 retry after read failure; N10 data leakage prevention; N11 clock NA; N12 filesystem NA.

Coverage map: only `core_lib/src/hdl/frame_header.rs`. Test-only, no transport refactor or production changes. Mutation targets: resetting filled on pending/error, failing to reset after a completed header, or reusing stale header bytes. Exact-source Core CI required; strict global ten-run / Stage 12 remains OPEN.

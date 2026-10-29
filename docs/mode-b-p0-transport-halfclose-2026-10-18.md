# MODE B: Transport half-close invariants (2026-10-18)

## Scope and repository scan
Production file `core_lib/src/hdl/migratable.rs`. The stream abstracts BLE `DuplexStream` and LAN `TcpStream` and dispatches `AsyncRead`, `AsyncWrite`, `flush`, and `shutdown` to the underlying transport. Existing tests cover bidirectional round-trips and write-side shutdown reaching each peer. They do **not** verify that local write shutdown leaves the read side usable, which is important while a receiver finishes sending final control bytes after our last write.

## Risk and invariants
A half-close must (1) preserve exactly the bytes written before shutdown, (2) expose EOF to the peer after they drain those bytes, (3) continue to receive inbound bytes after the local write side closes, and (4) keep BLE/TCP semantics consistent without waiting on real Bluetooth hardware. No new migration logic or production refactor is authorised.

## Test inventory and branch map
| File | Mode | Planned tests | Covered branches |
| --- | --- | --- | --- |
| `core_lib/src/hdl/migratable.rs` | P0 protocol adapter | Two async tests (one BLE, one TCP) | `MigratableStream::Ble` and `::Tcp` write, flush, shutdown and post-shutdown read |

## Positive, N1–N12 applicability
Positive: peer drains a sent control message, observes local EOF, and sends a distinct final response successfully. N1 invalid input typed NA; N2 small packet and EOF boundary; N3 socket failure not injected, existing shutdown tests cover basic EOF; N4 repeated stream transitions covered via read/write/shutdown order; N5 concurrency NA (single borrowed stream); N6 malformed payload handled at higher protocol layer; N7 no trailing-data loss; N8 persistence NA; N9 retry NA; N10 inbound data never confused with outgoing; N11 clocks NA; N12 filesystem NA.

## Interactions / mutation review
Two transport types × two directions × shutdown-before-inbound-control-response. Mutation targets: forward shutdown to read side, drop buffered outbound bytes, block read after write shutdown, or leak a truncated control frame. Test plan uses bounded in-memory messages, existing loopback TCP test fixture, no sleeps or real adapters.

## Completion gate
Source-matched Core Rust tests, Clippy and formatting must succeed; global MODE B Stage 12 and ten consecutive runs remain OPEN. Rollback: remove these two tests and this test plan only. Status: plan recorded before implementation.


## Stage 9 implementation checkpoint

- Documentation-first plan: `3e3031ce`.
- Added two test-only async half-close cases to `core_lib/src/hdl/migratable.rs`: `b59bd73b`. The methods cover BLE duplex and TCP loopback separately; both verify buffered outbound data, peer EOF, and an inbound acknowledgement after local writer shutdown.
- Exact-source [Native GTK run 39158983654](https://github.com/morozhenko-apps/rquickshare/actions/runs/39158983654): **SUCCESS**, 80/80 passed.
- Exact-source [Preflight run 39158983660](https://github.com/morozhenko-apps/rquickshare/actions/runs/39158983660): Core Rust **750 passed, 8 filtered**; format, frontend and Core Clippy succeeded. Tauri desktop tests were still executing at the documentation checkpoint. Treat complete Preflight as pending until the workflow finishes.
- The previous frame-header formatting fix at `7652ad26` was included in the same source revision and passed the new Core test run.
- Mutation review is reasoning-based only, not a mutation-engine report; mutation kills have not been measured.
- Global strict 10× / Stage 12 is still **OPEN**. No hardware acceptance or product code changes are claimed.

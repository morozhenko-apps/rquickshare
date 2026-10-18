# MODE B P0: mDNS refresh channel lifetime and TXT UTF-8 boundaries — 2026-10-18

## Stages 1-2 — scan, risks and invariants
Scope: `core_lib/src/hdl/mdns.rs` test module and in-memory `tokio::sync::broadcast` channels plus `mdns_sd::ServiceInfo` metadata builder. Existing tests cover direct mapping of Ok/Lagged/Closed, basic visible/invisible identity transitions, service name/type/port, and renames through valid identity names of up to 237 bytes. No existing test drives actual overrun/closure of a broadcast receiver or the TXT UTF-8 truncation boundary through the *actual* ServiceInfo builder.

Primary risks: a burst of slot0 refresh requests silently disables resending, closure causes endless refresh retries, or an overlong/partial multibyte device name is published with malformed UTF-8. No live mDNS daemon, BlueZ, sockets, or installed system is used.

## Stage 3 inventory
| Production method | Risk | Added tests | Category |
| --- | --- | ---: | --- |
| `receiver_refresh_action` | P0 liveness | 1 | Unit + real broadcast lag/close |
| `MDnsServer::{named_service,renamed_service}` | P0 wire identity | 1 | ServiceInfo SDK + UTF-8 boundary |
| `MDnsServer::run_inner` ServiceDaemon event handling | P0 integration | 0 | OPEN: needs deterministic event/daemon seam plus physical verification; no false coverage claim |
| `MDnsDiscovery::run` event cache and TCP reachability | P0 integration | 0 | OPEN: hardware-bound integration with current source; requires explicit design/approval for production changes |

## Stages 4-6 branch, negative, interaction matrix
`receiver_refresh_action`: real receiver reports `Lagged(3)` after 5 sends to a 2-event queue -> Resend; drains 2 successful events -> Resend; when all senders drop receiver returns Closed -> Disable; repeated closure must stay Disable. `named_service`: UTF-8 lengths 254, 255, 256, and split multibyte codepoint; verify exact expected longest complete prefix, port and full DNS name invariance, original TXT untouched.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Refresh channel | Planned | NA typed | Planned bounded channel 2/5 | NA no network | Planned lag/close | Planned ordered overrun | NA no persisted data | NA no authority | NA no clock | Planned sender closure | NA no user data | NA no subscriptions | NA no files |
| TXT identity | Planned | Planned invalid length | Planned bytes 254/255/256; 2/4 byte Unicode | NA pure metadata | Planned multiple renames | NA no concurrent writers | Planned codepoint split | Planned immutable endpoint | Planned Unicode | NA no user interaction | Planned no partial UTF-8 publication | NA | NA |

Two methods, with table-driven assertions and actual Tokio/ServiceInfo values. Interaction: burst/full/drain/closed; name length × byte width × original/renamed ServiceInfo; stable endpoint ID & port across renames. No sleeps or generated random-value equality expectations.

## Stages 7–10 counts / coverage / mutation
Planned 2 named methods, plus existing 6 methods in `mdns.rs`. Coverage map: `core_lib/src/hdl/mdns.rs` refresh action and service metadata only, not native `mdns_discovery.rs` event logic. Mutation review: treating lag as Disable, treating Closed as Resend, truncating mid-codepoint, changing endpoint/port/hostname on rename, or mutating original record should break explicit assertions. This describes potential kills; no automated mutant execution is claimed.

## Stage 11–12 completion criteria
Use exact-SHA Rust formatting, Core tests, Clippy, GTK via current quick CI. Global full 10-run suite, current-source instrumentation, live BlueZ/Android/mDNS interactions, and entire MODE B Stage 12 remain OPEN. Code modifications restricted to tests and documentation, rollback only these files.

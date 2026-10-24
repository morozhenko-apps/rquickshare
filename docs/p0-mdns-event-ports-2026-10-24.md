# P0 #9 — deterministic mDNS SDK event and TCP reachability ports

Date: 2026-10-24. **MODE A** minimal internal port extraction under the requested next-work order #11 → #9, with production SDK behavior preserved. Source baseline current `dev` after #11. [Active backlog](active-backlog.md); [issue #9](https://github.com/morozhenko-apps/rquickshare/issues/9); [closed browser fault](p0-mdns-browser-closure-2026-10-24.md); [Mode B prior scan](mode-b-p0-mdns-service-txt-2026-10-18.md).

## Current architecture and actual gap

`MDnsDiscovery::new` creates `mdns_sd::ServiceDaemon`. `run()` owns a fullname-keyed mutable cache and always drains tombstones before awaiting the SDK shutdown response. `run_inner` browses `_FC9F5ED42C8A._tcp.local.`, waits on `receiver.recv_async()` or cancellation, handles SDK `ServiceResolved/ServiceRemoved` directly, parses `n` TXT, filters IPv4/self, awaits live `TcpStream::connect` or cancellation, then updates the cache and broadcasts `EndpointInfo`. SDK browser close exits terminally (recent #12 correction), not by busy-spinning. No injected event source or reachability exists; pure TXT tests, Core worker cancellation tests and published tombstones do not test the real resolve/remove/probe state transitions. Need to preserve event-loop ownership without duplicating domain behavior.

**Minimal option A design:** extract the *existing* event loop into a private generic function taking (a) a callable async event-source port that returns real `ServiceEvent`, (b) a callable async TCP reachability port that returns `Result<(), std::io::Error>`, plus unchanged sender, cache and CancellationToken. `run_inner` retains SDK browse, trace, runtime logging and calls the same loop with closures wrapping the real SDK receiver and `TcpStream::connect`. Unit tests call the **actual production event loop** with genuine `ServiceEvent`/`ServiceInfo` instances and controlled external event/probe outcomes; no daemon, external multicast or socket required to fake *external inputs*. Do not mock the event loop or cache subject. No new persistent model, public API, wire/schema, package, secret or runtime flags. No spawned detached tasks or extra cancellation owner. Canceled TCP reachability must not publish a peer; external receiver close must withdraw cached endpoints via the existing outer `run` finalizer (test its owned cache semantics separately). The current service lifecycle shutdown remains SDK-owned.

## Risk/invariant analysis

I1. Invalid/no IPv4, self IP, missing/bad TXT or unreachable TCP never yields present route. I2. Valid reachability yields exact fullname/id/name/IP/port/type/present `EndpointInfo`; verified reachability is *not authentication*. I3. Removing an unknown fullname is no-op; removing a known fullname emits only a redacted `present=false` tombstone, once. I4. A canceled/panicked/failing or closed browser does not start another worker or spin. I5. Starting a new discovery generation always withdraws old cache and joins prior SDK before fresh advertisements. I6. No leakage of secrets or real peer identifiers from test fixtures. I7. Preserve current semantics on duplicate resolves and reachability rejection; do not introduce TTL, dedup, retry, aggressive timeouts, parsing policy, alias merge or different network selection under a testability refactor.

Risks: Rust async generic borrow/lifetime ergonomics; SDK clone receiver semantics; async probe cancellation priority; fallible `ServiceInfo` fixtures; accidental reordering of cache retirement and SDK shutdown. Mitigate by moving unchanged lines rather than redesigning event handling and comparing exact source snippets; execution at final CI gate. Revert one atomic production+test commit independently of docs.

## Branch map and planned tests

| Production branch | Fixture / expected outcome | New test |
| --- | --- | --- |
| `SearchStarted/SearchStopped` | ignored, no published endpoint | discovery event driver |
| Valid `ServiceResolved` | parse original real `ServiceInfo`, only publish on successful probe | resolves and publishes |
| `Resolved` missing IPv4 or invalid/missing TXT | no false discovery, probe not run | malformed candidates |
| Probe fail / cancellation during pending probe | never emit a false present route, no uncanceled pending owner | reachability/cancel |
| `ServiceRemoved` unknown/known/repeated | one exact redacted tombstone, no cross-peer entry | removal sequence |
| Closed source `Err` | terminal `Err`, no immediate re-read/spin | close/fault |
| Canceled before event or during blocked event | no callback and no traffic after stop | cancel |
| Same fullname repeated/moved | retain current event ordering/republication behavior, do not infer verified identity | ordering |
| Outer `run` shutdown + withdraw | existing source behavior and Core tests | existing + future physical |

Positive: known valid resolved probe routes then removed; N1 malformed/missing TXT/no IPv4; N2 zero/one/multiple events; N3 unreachable endpoint; N4 repeated resolved/removals/closed; N5 cancellation vs probe; N6 old cache generation; N7 no spoof-based identity elevation; N8 Unicode TXT parsed by original decoder; N9 reconnect/remove; N10 only synthetic addresses; N11 unrelated; N12 unrelated. Interactions: valid vs malformed × probe success/failure; resolved vs removed × same/other fullname; cancellation × event-ready vs pending probe; backend closed × nonempty cache. Unknown/reordered IPv4 HashSet selection is unchanged and not claimed deterministic IP tie-breaking.

**Coverage estimation:** new tests should run previously untested live event-loop branches; no coverage percentages until measured. Per-source map: `hdl/mdns_discovery.rs` is the primary domain subject; `utils.rs::parse_mdns_endpoint_info` and `is_not_self_ip` are existing external validation, `hdl/mdns.rs` service builder remains unchanged, `lib.rs` start/stop owner remains unchanged. Mutation targets: remove IPv4/self check; admit bad TXT; invert probe result; skip tombstone; accept after cancellation; retry closed browser; change published ID/fullname; new tests or existing tests must detect each applicable mutant. This is reasoning, not executed mutation engine.

## Implementation / completion gates

1. [x] Source/issue scan, minimal DI architecture, risk and scenarios written **before** implementation.
2. [ ] Extract production event loop to private generic event+probe ports, preserve SDK `browse` and outer shutdown, keep same run ordering.
3. [ ] Add original end-to-end event-loop regression methods with real `ServiceInfo` but controlled external event/reachability, including fault and cancellation; assess test fixture generated TXT and ordinary Android naming correctness.
4. [ ] Final strict fmt, Core stable once, Core+Tauri Clippy, frontend and Native GTK exact-source CI; separate sensitive-race checks only at mandated stage, no redundant heavy 10× loop.
5. [ ] Record executed test names, outcomes/coverage evidence, code SHA, rollback; issue update. Physical mDNS SDK/Pixel/Windows remains OPEN; no closed claim without installed acceptance.

Do not use green unit fixtures as proof of real SDK multicast timing or Google peer compatibility. The selected architecture is a testability boundary, not a security change.

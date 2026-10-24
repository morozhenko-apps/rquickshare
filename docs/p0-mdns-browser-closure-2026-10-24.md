# P0 mDNS browser receiver closure: terminal exit, SDK join and restart safety

Date: 2026-10-24. Scope: the already approved cancel-and-join discovery lifecycle (issue #12) and the bounded closed-browser-path defect within #9. No approval to install a full SDK event/reachability test seam; #9 Option A/B/C architectural decision remains open.

## Source scan and root-cause evidence

Source: `core_lib/src/hdl/mdns_discovery.rs::MDnsDiscovery::run_inner` loops on `receiver.recv_async()`. On `Err(err)`, it currently logs `MDnsDiscovery: error` and loops immediately. A permanently closed channel returns a new error immediately, causing unbounded log/CPU spin, preventing the worker from finishing and therefore delaying `RQS::stop_discovery_and_wait` until unrelated cancellation. This is a **source-level failure mode**, not an observed production CPU incident.

`MDnsDiscovery::run` already reliably calls `withdraw_cached_endpoints` after `run_inner` exits and awaits `ServiceDaemon::shutdown` completion. `core_lib/src/lib.rs::stop_discovery_and_wait` already joins the generation and returns any worker error. Retaining these mechanisms is the correct ownership model; do not detach the SDK daemon, ignore failures, add arbitrary sleep/backoff, close the entire Core lifetime, change wire formats, or claim that an automatic new generation has started.

Related source-level hypotheses: a browser receiver can close on internal SDK fatal errors, a browse-channel shutdown or external service. No actual SDK close event was captured. TCP reachability timeout, ambiguous IP choice, duplicate resolved events and normal cancellation are independent scenarios, not solved by this patch.

## Narrow fix / invariants / rollback

On `Err(err)` returned by `recv_async`, log exactly once and return an error from `run_inner`. The enclosing `run` then publishes cache tombstones and joins the SDK shutdown. `RQS::stop_discovery_and_wait` must return the error after joining and remove old generation ownership. The existing Option B restart fence requires the previous worker to terminate before any next worker; on error it fails closed rather than silently resurrecting a defective SDK browser. Preserve successful per-event processing, consent, endpoint routing and privacy.

Rollback the executable changes together; tests remain deterministic and do not initialize actual mDNS daemon, BLE/GATT or the installed app. Docs and executable commits separated.

## MODE B bounded test inventory / risk matrix

| Production branch | Existing coverage | This slice | Remaining gap |
| --- | --- | --- | --- |
| `mdns_discovery.rs::run_inner` normal resolve/remove | actual cached tombstone shape and order tests | no behavior change | SDK events + TCP reachability (#9) |
| `run_inner` cancelled | Core worker cancellation / join fixtures | no behavior change | actual running SDK |
| `run_inner` browser `Err` | **none** | exit with error, no busy-spin | direct simulated SDK receiver failure blocked by #9 seam decision |
| `MDnsDiscovery::run` post-loop finalizer | cache retirement tests | preserve shutdown-before-return contract | SDK shutdown error/order integration |
| `RQS::stop_discovery_and_wait` failed completed worker | shutdown-failure tests after cancel | test worker error before cancellation; ensure token/handle cleared, owner resources not leaked and no automatic restart | actual SDK invocation |
| `RQS::replace_discovery_with` after unexpected worker failure | existing cancelled worker error test | assert no replacement generation starts after failure | SDK seam needed |

Positive: already completed worker joins once after graceful cancellation. N1 unexpected SDK close/error; N2 repeated stop/zero cache; N3 network/sdk unavailable; N4 repeated restart/failure; N5 outstanding cancellation and worker shutdown ordering; N6 stale advertisement cache; N7 no authorization boundary changed; N8 no display text; N9 stop/restart; N10 logs must not include peer name/address/key; N11 not applicable; N12 no file writes. Interactions: early worker failure × stop/replace, independent Core tracker × failed discovery, cancelled SDK worker × existing cache tombstones. Never count a controlled Core future as actual SDK-event injection.

Coverage estimate: the new lifecycle test can prove the real Core ownership/join behavior after a worker terminates with error; it **cannot** directly simulate `ServiceDaemon::browse().recv_async()` closure with the current production design. That branch is an explicit runtime hot-loop fix but remains unverified at direct event-source level. The rest of issue #9 remains OPEN.

## Completion gates

- [x] Source scan and root-cause differential, plan and risk inventory
- [ ] Source correction and focused Core lifecycle regression
- [ ] Exact-source Preflight, Native GTK, strict Rust format / Clippy with logged test method
- [ ] Physical test: daemon closed-browser fault / normal cancellation and device retirement, where reproducible (not required to corrupt OS state)
- [ ] Full Mode B Stage 12, injected SDK coverage, #8/#9/#11/#16 security + lifecycle hardening, 20 locales, release 1.0: still OPEN

This task affects only failure cleanup and prevents a potentially unbounded loop. A green preflight alone never constitutes physical acceptance.

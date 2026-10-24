# Native cold discovery timeline: bounded opt-in phase diagnostics

Date: 2026-10-24. MODE A (diagnostic feature, issue #18). Primary integration branch `dev`. This is a scoped diagnostic to gather reproducible timing evidence **before** any startup or discovery performance optimization. No change to consent, advertisements, Bluetooth/Wi-Fi, authentication, route selection, persistence, public API, installed packages or releases.

## Repository evidence and hypotheses

The current physical report is subjective slow *first* discovery on KDE/Wayland and Pixel 6a, with no measured timestamps. Distinguish different observable milestones: startup of Core transports, admission of a discovery worker, mDNS browse activation, first validated IPv4/TCP route admitted by the Core runtime, BlueZ receiver advertisement, GTK presentation and actual authenticated recipient readiness. A mDNS peer is **not** a verified cryptographic device or necessarily the Pixel.

Plausible causes (not all confirmed): (1) settings/process/daemon activation delay; (2) Core startup awaiting BlueZ initialization before mDNS; (3) delayed mDNS browse/service resolution; (4) sequential TCP reachability checks blocking browse event processing; (5) Bluetooth advertiser/GATT registration latency; (6) GTK/controller snapshot or screen presentation delay. The most directly source-supported candidates to measure first are Core startup's awaited BleListener initialization and mDNS browse/reachability latency. Neither is established as the root cause without an actual opted-in host trace.

Core `RQS::start_transports` starts TCP, awaits optional BLE initialization, then creates/spawns the mDNS server. `NativeRuntime::start` awaits `core.start` before `core.discovery`; `MDnsDiscovery::run_inner` calls `browse` before responding to resolved services, and Core `NativeRuntime::discovered` admits only parsed, live routes. Existing opt-in `core_lib/src/receiver_trace.rs` writes only fixed `rqs-receiver event=... elapsed_ms=N` lines, using a process-monotonic clock when `RQUICKSHARE_RECEIVER_TRACE=1`. No new tracer, unfiltered PII logging or new global process clock.

## Planned source changes (before coding)

1. Expand that existing allowlisted Event enum and deterministic format test with `runtime_start_requested`, `runtime_core_ready`, `runtime_discovery_worker_ready`, `mdns_browse_started` and `first_mdns_route_admitted`. All values are fixed strings. The flag remains opt-in, exact `1` only.
2. Emit the three runtime events in `NativeRuntime::start` before attempting work, after successful Core startup, and after successful discovery-worker admission, respectively. Failed stages must never emit their success marker.
3. Emit mDNS browse marker only after `ServiceDaemon::browse` succeeds.
4. Emit first route marker *once per runtime lifetime* after successful validation, coordinator admission and `routes.insert`, exactly when the first ever route receives sequence 1. A refresh/duplicate, malformed endpoint, expiry/removal or presentation-only row must not create a new first route event. Use existing `endpoint_sequence` invariant; do not add persistent device identity.
5. Update the existing fixed-line output contract tests for each marker. Defer normal test-suite execution until the final gate. Source read-back verifies one named event per intended phase; existing Core runtime tests cover endpoint admission invariants. Required CI is exact-source Preflight (stable Core/Clippy/format/frontend/Tauri) and Native GTK as applicable; separately tracked race-sensitive suites are not silently claimed green.

## Security / compatibility / rollback

No names, endpoint IDs, addresses, files, PINs or pairing secrets in emitted diagnostic events. The trace is fully inert unless specifically opted in; no telemetry or network upload. A process-relative elapsed time does not, by itself, measure GUI startup, cold Bluetooth discovery on the Pixel or ready-to-send status. Failed Core startup, browse failure and lack of a first admitted mDNS route are identified by **missing markers**, not invented durations. Never disable authentication, manipulate probe delays or use sleeps/retries to improve a benchmark. Revert the four scoped source files as one diagnostic slice if necessary; no schema/installation migration.

For a voluntary physical trial, capture **only** the allowlisted `rqs-receiver event=...` lines from the daemon journal with local monotonic ordering after an idle, graceful restart of the owned service. Do not upload full journald or enable all `RUST_LOG`; cold and warm sessions should retain the same radio, visibility and network conditions. Existing installed 0.9.0-1 does not contain these new markers until a separately approved package install. User acceptance and issue #18 remain OPEN.

## Evidence checkpoint

- [x] Architecture/repo scan and root-cause hypothesis inventory.
- [x] Scoped plan, event contracts, privacy guard and rollback documented ahead of coding.
- [ ] Source instrumentation and output-shape tests on `dev`.
- [ ] Final exact-source stable CI and independent targeted event-shape evidence.
- [ ] Real owner-host cold/warm stage measurements; then only evidence-driven performance fix.

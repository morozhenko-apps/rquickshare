# Native backlog resolution — 2026-10-29

## Owner scope and mode

MODE A (production fixes and candidate repair), followed by a separate MODE B test gate and operator acceptance. Owner explicitly prohibits touching issues **#8** and **#16**: no changes to their source contracts, authorization/identity assumptions, issue bodies/status or related implementation, including indirect workarounds. Their unresolved state is acknowledged but cannot block independent work.

The user asks to resolve other pending issues without redundant 20-human-language sessions. Use agent-led locale QA with automatic catalogs, semantic/RTL checks, and one installed-system behavior acceptance. Read current GitHub Issues and do not treat open status as proof of missing implementation. Each close requires exact code/CI or installed-device evidence in the issue; no speculative green.

## Phase 1: rescue the executable source gate

At current dev source 47a60cda, Preflight 40881389186 passed; Native GTK 40881389292 failed nightly rustfmt before Clippy/widgets. Failure is a source formatting mismatch across i18n.rs and secondary-window code/tests, not evidence of passing GTK. Apply exactly the rustfmt diagnostics, with no behavior change and a scoped style commit. Then observe new CI only through completion evidence. Don't rerun stable suites per commit or poll in a loop.

## Phase 2: reconcile implemented vs remaining

- #9, #11, #12, #19: existing testable source fixes and prior CI documented; only close after new candidate exact-source gate and any explicitly missing physical validation. Do not modify trusted peer route identity or #8 TCP route admission.
- #13, #17, #18, #21, #22, #23, #24: evaluate current UI/lifecycle, preserve observed device outcomes and known flaws; fix the smallest demonstrated defect. Do not demand human manual inspection of each locale; use automated source-wide catalog, semantic, bidi and layout review.
- #25, #26: correctness of Cancel and terminal state is high priority; a fix must preserve the crypto writer and already-approved migration boundary, cannot claim wire flush when merely deferred. If an architectural security contract change would be needed, halt **that** part for a material decision, continue other independent tasks.
- #27 Windows and #14 protected release: independent platform and publication gates, not preconditions for issuing a dev-only test candidate. Do not deploy or promote master.

## Phase 3: final MODE B and package for acceptance

After the final implementation revision: exhaustive changed-file branch/test/negative matrix, mutation review, deterministic tests for state, storage and UI; one final Preflight and GTK stable suite. A source gate must reach the GTK widget tests; mere rustfmt success is not sufficient. A new native 0.9.0-3 package is a dev-only test artifact, not signed release. Verify SHA256, Debian version ordering, package integrity and install simulation before asking operator to install. Stop before actual protected tag, signing, R2 or master changes.

## Rollback and status

All code and docs commits separate and atomic. Revert style fixes independently; do not mutate prior installed 0.9.0-2. Initial status: Phase 1 IN PROGRESS, Phase 2 PARTIAL, Mode B completion NO, new package absent. Existing physical 0.9.0-2 13/14 PASS remains historical to previous SHA. Keep an accurate evidence checkpoint here, not claims of future success.

## Checkpoint: existing fixes versus required new work

Read-only reconciliation on current dev:
- #11 unusable LAN IPv4 exclusion has existing pure tests and green prior exact-source CI 2545e051; owner acceptance still requires physical VPN/LAN cases.
- #12 approved cancel/join discovery worker and terminal browser closure are implemented with prior CI evidence at 424a3a54; SDK/hardware lifecycle remains unverified.
- #19 mDNS tombstone producer/consumer mismatch is fixed and tested with prior six-green c384e8e2; stale-entry physical H01/H15 remains pending.
- #21 manual-only Hidden startup has an owner-reported physical PASS on installed prior debug candidate and source code enforces initial Invisible; outstanding risk is stored visibility snapshot / settings reapply consistency.
- #22 premetadata row "Connecting… / Waiting for details" implemented and source tested; installed screen confirmation pending.
- #26 source is **not** in the original unfixed state: inbound.rs::send_local_cancel now returns an explicit error when deferred_upgrade_writes is present or the committed BWU writer is unavailable; gatt.rs retains committed migration writer ownership until completion or failure; inbound/bwu_cancel_tests.rs covers eight cancellation/migration scenarios. This is a source/fixture correction, **not** proof Pixel observed a cancelled UI during the committed migration.
- #25 ordinary LAN local Cancel has previously been physically observed as cancelled on Pixel; cancellation under BWU and sender/receiver terminal ordering still require scoped installed acceptance.
- #23 has source-instrumented static GATT/Weave stage markers; first BLE handshake stall lacks a new same-process trace. #24 already has first-file and 25/50/75% redacted trace markers; suspected throughput loss still lacks a controlled bytes/time measurement, so tuning would be speculative.
- #13 Devices attention code and GTK navigation tests exist; owner's 0.9.0-2 T11 marked PASS, but the reported summary alone does not establish all multi-request/focus/scroll and reduced-motion acceptance.
- #17 has agent-run source inventory and localized secondary-window fixes, not a 20-human-review requirement. #27 Windows is a separate platform adapter and card deliverable; no Windows source or physical Windows claim. #14 protected publication not approved.

No issues closed from code existence alone. No action on #8 or #16.

## Phase 1 execution evidence

Nightly rustfmt failure on Native GTK run 40881389292 (source 47a60cda): exact diagnostic changes applied to eight native GTK files with style-only commit a845d8476bebfe4b9621077c9fbfc107420a8f27. This successfully exposed the next gate: Native GTK run 40900116999 compiled sufficiently to reveal test compiler errors E0369/E0277 in compose/tests.rs line 115. Root cause was the added test comparing un-awaited Future values returned by clipboard_text; the test was intended to compare actual clipboard content. Focused test-only fix b85aeaad4af6ad2dbe0f64d5501541be90419c10 now reads actual GTK clipboard text through GLib MainContext; no production or protocol behavior changed. Preflight 40900117006 was in_progress at the checkpoint and GTK 40900116999 failure was only at test compilation. **Do not claim test suite completion or candidate readiness until subsequent terminal exact-source CI**.

Phase 1: PARTIAL; Phase 2: source audit completed / physical evidence pending, not fully closed; Mode B: NO; .deb 0.9.0-3: NOT BUILT; master/tag/R2: UNTOUCHED.


## Exact source native GTK gate — terminal result

Executable code SHA **b85aeaad4af6ad2dbe0f64d5501541be90419c10**. Native GTK UI verification [40900537836](https://github.com/morozhenko-apps/rquickshare/actions/runs/40900537836) completed SUCCESS. Steps: Ubuntu 26.04 real Bubblewrap, nightly Rust fmt, stable strict Clippy, native GTK isolated widget/controller tests. Exact log: **10/10** first test bin, **171/171** native GTK test bin, 0 failures, 0 ignored. The previous compiler test defect is fixed at this SHA. This is not the Preflight or packaged-artifact result; Preflight [40900537851](https://github.com/morozhenko-apps/rquickshare/actions/runs/40900537851) was still in progress at the source-matched checkpoint. No package exists yet and owner must not be instructed to install an unverified artifact.

Stage 1: GTK gate DONE, Preflight pending; Stage 2: verified prior fixes only; final acceptance pending. Source package still has distinct Debian revision 0.9.0-3. Do not rerun an expensive suite solely for documentation.

## Terminal paired source gate and manual package handoff

Executable source b85aeaad4af6ad2dbe0f64d5501541be90419c10: [Preflight 40900537851](https://github.com/morozhenko-apps/rquickshare/actions/runs/40900537851) completed SUCCESS, all five configured jobs terminal SUCCESS. [Native GTK 40900537836](https://github.com/morozhenko-apps/rquickshare/actions/runs/40900537836) completed SUCCESS (10+171 Rust GTK tests, strict native Clippy and nightly rustfmt). The current dev HEAD 28516441119c96a078654e7e70f5e70d060f5de2 differs from the executable source **only** in docs/native-backlog-resolution-2026-10-29.md and docs/native-0-9-0-revision3-acceptance.md. No executable drift between validated source and proposed package HEAD.

Release package smoke is deliberately workflow_dispatch only. Current connected GitHub tools do not offer workflow_dispatch; the owner's authenticated gh CLI must trigger **native-package-smoke.yml** once on dev. No action was triggered by this assistant. After terminal smoke SUCCESS, require official workflow artifact named rquickshare-native-ubuntu-26.04, sha256 -c, package metadata version 0.9.0-3 amd64 and apt install simulation; no automatic install. A run ID without terminal completion is not a package PASS. No release/tag/R2/master change.

Global MODE B remains NO until remaining independent P0/physical acceptance; this green pair is the gate for a test **candidate**, not signoff for publication.

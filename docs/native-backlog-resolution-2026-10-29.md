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

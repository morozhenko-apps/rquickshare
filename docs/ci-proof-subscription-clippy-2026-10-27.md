# Clippy exact-error assertion in private route proof subscriber

Date: 2026-10-27. MODE A defect correction limited to `core_lib/src/sessions/tests.rs`; this unblocks the baseline Core Clippy gate without altering the authenticated-route production implementation. Baseline dev `612fdb78722eaed68d0f009b7b4dd24a3e337cda`.

## Reproduction and observed evidence

GitHub Preflight run `40412736957`, Core Clippy job `121887351422`, deterministically failed with `clippy::err_expect` at `src/sessions/tests.rs:984`: the test calls `.err().expect(...)` on a `Result`; `-D warnings` makes this a compile gate failure. All other recorded dev results remain independently scoped, and the successful Native GTK run `40412737062` must not be mistaken for successful Core Clippy.

Feature PR #28, run `40414513690`, also showed Core Clippy failure, but its immediate root cause was DNS resolution of `index.crates.io` and `github.com`, not this lint (dependencies were never loaded in that run). A retry is warranted only after a network-health check and without removing the Clippy gate. Do not treat environment failures as source-level failures or vice versa.

Potential contributors evaluated: an unguarded test-only lint, stricter Clippy toolchain, stale CI cache, unintended source change, a `Result` diagnostic pattern introduced in #16, dependencies not compiling, and runner DNS. Verified source/compiler message rules in the invalid `.err().expect()` test pattern; `expect_err()` could impose a `Debug` bound on the successful `broadcast::Receiver<RouteProof>` value, while `RouteProof` is intentionally non-Debug to prevent revealing private credential material.

## Minimal plan and invariants

- Replace the unwrap-style error extraction with `assert!(matches!(hub.subscribe_route_proofs_fresh(), Err(SessionError::Closed)), "shut down hub must reject proof subscriptions")`. This asserts the **exact** fail-closed error and avoids `Ok`-value Debug requirements; no weaker assertion, Clippy suppression, new Debug impl, or production behavior change.
- Keep session shutdown, subscription, credential privacy, routing and explicit recipient invariants unchanged; no API/format/settings migration.
- Source expected result: strict Clippy must no longer flag `err_expect`, and existing shutdown assertion still fails for `Ok` or any different `Err`.
- Final evidence: source-exact Preflight stable/test/lint and separate native GTK gate; no success claimed before CI terminal checks. If DNS recurs, classify and diagnose runner environment independently.
- Code/test edit in a separate `fix:` commit after this `docs:` commit, using an expected-HEAD lease. Rollback through inverse commits.
- PR #28 remains independent, draft and unmerged until its own exact-source checks and feature-level GTK validation.

**Status:** documentation recorded before executable edit. No release, master or R2 changes.

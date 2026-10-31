# CI cadence, runner policy and failure triage (2026-10-18)

Status: investigation complete for current dev; CI changes pending validation.

## Observed failures (October 18 UTC, actual GitHub job logs)

- Native GTK job 117553112991: rustup-init fails because `HOME=/github/home` differs from the root user's actual home `/root` inside Ubuntu 26.04 container.
- Native package job 117552410527: `cargo-clippy` missing from stable Rust toolchain despite passing core tests.
- The most recent inspected Preflight job set in run 39086175208 passed all five jobs. Billing display `Gross $24.47 / Billed $0` is not proof that public standard GitHub-hosted usage consumed billed quota.
- Current dev Preflight explicitly runs race-sensitive Rust tests 10 times per push. Current `mode-b.yml` can run on a dev push modifying its own YAML, or through manual dispatch; that violates final-only stability intent.

## Required invariants

- Public repository: standard GitHub-hosted Ubuntu/macOS/Windows runs remain the default. Never silently enable untrusted PR workflows on local organization self-hosted runners.
- Routine dev/master pushes: fast deterministic tests, lint and type checks exactly once; no 10x repetitions and no automatic packaging/publishing. Preserve existing positive, negative and security coverage.
- Final full Mode B: exactly ten repetitions, invoked only by an authorized release-tag workflow after verifying the tag points to current master HEAD.
- Per-branch/PR concurrency cancels superseded checks, release tags are not canceled by branch push. Keep separate workflow groups where necessary.
- Existing `dev` contains substantially more commits than `master`; do not fast-forward unverified product code as part of CI maintenance.
- Rustup inside the Ubuntu 26.04 container needs a consistent `HOME` for the container user and explicit Rust components; never weaken lint/test checks.

## Implementation plan

1. [complete] Read source, workflows, job logs and current branch ancestry; differentiate platform quota rules from actual failing steps.
2. [pending] Limit automatic Preflight to one stable test pass; reserve race-sensitive 10x for the reusable release Mode B suite.
3. [pending] Remove dev/manual triggers from full Mode B so only the existing validated `v*` master-tag publishing workflow calls it.
4. [pending] Set container-user HOME consistently for containerized Rust jobs and explicitly install Clippy where required.
5. [pending] Add/check concurrency on any missing dev-triggered checks; preserve release-tag noncancellation.
6. [pending] Inspect dev CI outcome, fix infrastructure defects without masking test failures, and record measured evidence.
7. [pending] Fast-forward master only if all relevant gates are green and all intervening dev commits are verified, without force-pushing.

## RunnerRouter integration

The organization uses an external Cloudflare Worker for compatible private repositories, but public rquickshare is not in its current allowlist. Routing this public repo to self-hosted would change the trust boundary; leave it GitHub-hosted pending explicit security approval. This is not the cause of the confirmed Rust container failures.

## Rollback

Revert CI commits on dev, retain the protected master SHA and immutable release tags, do not change packaging semantics or disable deterministic assertions.

## 2026-10-18 execution evidence

- Completed on `dev`: normal Preflight now runs stable Core once; full Mode B is callable only by the master-HEAD tagged release workflow. Three P0 10x jobs reject dev refs and have no push triggers. Heavy native package, coverage and release syntax checks are manual. Native GTK scoped widget tests remain single-pass when native sources change. Per-ref cancellation preserves new pushes, while protected publication is non-cancelable.
- Added `scripts/ci-check-workflow-policy.sh`, invoked automatically by Preflight. The exact development Preflight SHA `aec1f1f7bb7579b88ab5980eab32ba0ec1ce78cc` passed in GitHub Actions run `39091401200`; native GTK gate also passed at preceding SHA.
- **Not yet complete:** no fresh valid release-tag 10x run, no bundled Windows/macOS acceptance, no master promotion. The very large outstanding development branch contains product changes beyond this CI scope. Public hosted runners remain deliberate for untrusted repository safety.

## Build toolchain maintenance integration

The owner approved a scoped integration of the already validated maintenance into the current release branch through a reviewable PR. This authorization supersedes ordinary temporary-branch restrictions for this integration only. Build/CI tool versions change; application behavior, public contracts, data, credentials and immutable release identities remain unchanged. Upstream Actions keep their supported internal runtime; project Node tooling uses the exact maintained LTS pin.

Plan:

- [x] Inspect current release source, owning instructions, prior maintenance evidence and deployment triggers. Start from the current release head to preserve its unique commits.
- [x] Record this plan before executable edits, with documentation and executable changes committed separately.
- [x] Apply only the reviewed Actions, Node and required compatibility/security corrections; adapt references to existing release workflows. Exclude unrelated accumulated development work.
- [x] Review complete diff and source invariants, run applicable final stable checks once, and reuse previous suite evidence only with exact source/lock equivalence. Keep native/store and release-only acceptance separate.
- [ ] Create the scoped PR; integrate only after its exact-head required checks and existing production-admission conditions pass. Preserve concurrent release changes and never bypass branch protection.

Rollback uses an ordinary revert of the scoped maintenance commits; no tag movement, resource recreation or provider/data rollback is part of this task. Existing automatic Git deployment ownership and triggers are preserved. A source/PR pass alone does not establish effective production Node or native/store acceptance.

Validation result: PASS. Source review preserves workflow events, permission/runner/release conditions and existing release-side commits. Executable source/lock bytes match the previously validated development maintenance, excluding explicitly reviewed documentation, workflow and non-executable housekeeping differences; its complete consumer-suite evidence remains applicable. New source contracts and diff checks were executed on this release-based branch. Remote PR/release results are tracked separately and are not implied by local checks.

- `python3 -m unittest discover -s scripts/tests -v` — exit0 (STABLE_SOURCE).
- `bash scripts/ci-check-workflow-policy.sh` — exit0 (STABLE_SOURCE).
- `git diff --check d3844ff9770452cf6bce557049816194d17af352` — exit0 (STABLE_SOURCE).

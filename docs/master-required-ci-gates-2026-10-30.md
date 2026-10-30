# Protected master required PR checks without documentation-only test runs

Date: 2026-10-30. Primary mode: MODE A (CI release-policy change). The owner approved replacing the current master-only update lock with enforceable PR review and source-verification checks. This plan is recorded before executable changes.

## Root cause and constraints

Effective branch ruleset `protect-master-release` (#25588734) on `refs/heads/master` currently has `update`, `deletion`, `non_fast_forward` and an administrator bypass actor. Current PR #39 has green exact-source Preflight/GTK checks but `mergeable_state=blocked` because `update` prevents ordinary PR merges. That rule must be replaced with `pull_request` and required CI checks, never bypassed using administrator rights.

GitHub requires a status check on every PR SHA when it is declared mandatory. A whole workflow skipped by pull-request path filters leaves checks indefinitely Pending, even for documentation-only PRs. The native GTK workflow currently uses such filters. Ordinary documentation changes must not trigger heavy test suites; routine push events remain restricted to the explicit CI marker.

## Plan (one release-admission batch)

1. Add a read-only, fail-closed PR change scope classifier. On `pull_request`, inspect the actual merge commit's first-parent diff, without executing untrusted files or printing file names. On explicit marker push/dispatch, run ordinary suites. Any missing parent/diff or invalid event state fails classification.
2. Preflight workflow: keep all existing heavy jobs, but run them only for changes other than documentation. Add an always-executed `Preflight admission` job requiring scope success and all relevant job results (success for source changes, skipped for documentation-only). A failing prerequisite must never be masked by a skipped required job.
3. Native GTK workflow: remove PR path filtering only for availability of the required check; retain a dedicated GTK-specific path classifier. Run existing expensive GTK job only for affected native/core/CI modules. Add an always-executed `Native GTK admission` job that fails closed on scope failure or missing/failed required GTK result. Other PRs receive a lightweight green gate without starting GTK build.
4. Add regression tests for source classifier boundaries and workflow gate contracts. Preserve current separate Preflight / GTK gates, explicit marker push policy, one-pass ordinary tests, no Mode B or coverage in release CI.
5. Integrate through a PR to the active #39 branch, execute final standard CI once, and merge #39 to protected master **only after** the repository operator updates ruleset #25588734 using a documented API payload to require a PR plus exact `Preflight admission` and `Native GTK admission` status contexts. Keep deletion and force-push prohibition. The connected GitHub app cannot write rulesets, so this operator update is a separate gate; never bypass ruleset with admin merge.

## Risks / rollback

- Incorrect changed-path classification might skip tests: choose fail-closed defaults for unrecognized executable changes; ensure unit tests include docs-only, native, frontend, workflow/config, empty, malformed, and hostile file-name cases.
- Skipped jobs reported success by GitHub are not trusted without their independent always-run admission job.
- On PRs from forks, avoid privileged `pull_request_target` and never run PR code with elevated GitHub credentials.
- Roll back the workflow/code commits separately from this documentation. Before updating branch protection preserve old rule JSON and tag refs as operator-held evidence. Do not loosen other rules, change protected `dev`, publish to R2, retag or trigger signing in this CI policy batch.

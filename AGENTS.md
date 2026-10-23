# Repository agent working rules

These project-specific rules supplement the operator's Universal Feature / Refactor / Testing Agent Prompt (SSOT). They apply to every agent working on rQuickShare. Communication with the owner is Russian; repository content, code comments, docs and commit messages are English.

## Primary development branch and parallel agent coordination

- **`dev` is the primary integration and development branch.** Work, document, test and commit directly on `dev` by default. **`master` is protected, release-only**; never make unrelated development commits, fast-forward or merge to `master` merely to synchronize branch names.
- The **only normal reason** to open a temporary feature branch and a PR is verified concurrent work by another agent on `dev` that could cause overlapping writes, conflicting CI or lost ownership. First check the real `dev` ref and recent activity, open PRs, local working tree/worktrees and any explicitly communicated agent ownership. An old branch, CI run or no-longer-active PR **does not** by itself prove another agent still owns `dev`. Do not overwrite uncommitted work.
- For verified concurrent ownership, use a small task-specific branch based on the current `dev` HEAD and a PR **targeting `dev`**, not `master`. Reconcile changes with the active owner; do not force-push, merge over conflicting work or delete any active agent branch. Once integrated and verified, close its PR and remove the short-lived branch.
- Before any `dev` ref mutation, refresh/check the branch tip and use optimistic compare-and-swap/expected SHA. On a tip conflict, inspect intervening changes instead of forcing a write. Separate documentation and executable commits.
- Avoid long-lived feature branches as a default, speculative branch proliferation, blind cherry-picks and merges of already included changes. CI must follow its established stable/flaky/manual heavy-gate policy.

## Completion means issue lifecycle completion

- Track each functional change against its issue or documented backlog item. **Once a feature/defect is implemented and its required final CI and applicable physical/user acceptance have passed, update documentation and close the corresponding issue** with evidence (source SHA, exact test run, acceptance, limitations). Do not leave fully completed issues open indefinitely.
- A source-only implementation with missing required CI or live acceptance is **not done**. Leave its issue open with a concrete remaining checklist. Do not close architectural blockers on a choice alone: decision, implementation and required proof are distinct.
- For PR-based parallel work, merge the PR to `dev` only after review and required gates, then close out issue tracking and delete its obsolete branch. `master` promotion, protected production tags and R2 signing/publication require independent explicit release approval.

## Branch audit and cleanup

- Inspect **all current remote branches**, their compare base/ahead/behind against `dev`, unique diffs/commits, associated PRs, protection and evidence of concurrent ownership. Never infer that 'behind' means 'safe to delete' without verifying unique work is preserved.
- Integrate only genuinely missing, approved changes. If a branch's unique change already exists in `dev` as identical content (e.g. via an earlier master PR), do not manufacture a redundant merge commit.
- Give the operator commands for repository-local branches **only after** their local status is inspected. Delete remote branches only when their unique work is integrated or proven redundant, they are not protected and no active agent owns them. Never delete `dev` or `master`.
- If the active GitHub connector has no delete-branch action, return a reviewed exact `git push origin --delete <branch>` command, not a fabricated deletion claim.

## Architecture: verified peer identity (approved option A on 2026-10-23)

For issue #16, owner chose **A**: collapse aliases/routes of one real device only after a cryptographically authenticated and session-bound identity match, keeping independently addressable routes and explicit consent. No grouping by user-visible name, IP, MAC, mDNS fullname or uncontrolled peer-advertised metadata may grant trust or silently select a transfer recipient. Document the proof/binding contract, lifecycle, fail-closed ambiguity and corresponding regressions before production changes. The choice resolves the A/B/C decision; it does **not** close issue #16 until actual implementation, CI and physical acceptance are complete.

See [the full decision and 20-locale gate](docs/native-pixel-acceptance-identity-locales-2026-10-23.md) and [branch audit](docs/branch-audit-2026-10-23.md).

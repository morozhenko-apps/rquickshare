# 2026-10-23 GitHub branch and PR audit

Scope: connected remote repository `morozhenko-apps/rquickshare` (not the owner's unobserved local git worktrees or all remotes). This task is **MODE A — integration and branch policy**. Documents-only checkpoint; no executable code, protected master, release tag, installed package, signing secret or R2 changes.

## Actual remote refs and review

The GitHub branches REST list (100/page, all currently returned) contained exactly **three** branches:

| Branch | SHA at audit | Protection | Unique changes / disposition |
| --- | --- | --- | --- |
| `dev` | `1358067aa8f604a498d988f65316856fbed08a42` | unprotected; active primary | Keep; updated by this documentation task |
| `master` | `3f6f8b32cd9414bce30865402f8b707e6b2b12e0` | protected | Keep; release-only, not a stale feature branch |
| `ci/ignore-docs-only-main-20261018` | `2bca80df0c6cd3feb4b3b5f3dcbd76c319029f2d` | unprotected | **Stale candidate for deletion**. Only one unique commit versus dev, entirely `.github/workflows/lint.yml` adding docs-only `paths-ignore`. The file's Git blob SHA is exactly `aae57954a7897a297748d37e210a99a8db9898c2` in **all three refs**. Thus its unique behavior/content is already incorporated, despite `compare dev...branch` reporting `ahead_by=1, behind_by=300` because Git history differs. A synthetic merge/cherry-pick into dev would add no content; do not do it. |

Existing source PR [#10](https://github.com/morozhenko-apps/rquickshare/pull/10) `ci: do not run preflight for docs-only pushes` was **already merged into master on 2026-10-18**. GitHub returned **zero OPEN PRs**, and historical PRs #1–#15 observed were all closed/merged as appropriate; #15 `gtk4-noble` is closed unmerged and its source branch is no longer present in the remote branch list. No current remote branch demonstrates other-agent ownership of dev; this does not prove that no agent has uncommitted work locally.

## Decision and immediate action

1. **Retain** protected `master` and active `dev`. Don't FF/rewrite/master-release without release gate. `dev` is the primary branch per owner.
2. **Do not merge** obsolete `ci/ignore-docs-only-main-20261018`: its sole unique modified file already exactly equals `dev` and `master`. Mark safe-to-delete after one fresh read verifies unchanged remote blob/ref and no active agent claims it.
3. GitHub connector exposes no remote branch deletion operation. After review, operator may execute:
   ```bash
   git -C "$HOME/Documents/GitHub/rquickshare" fetch origin --prune
   git -C "$HOME/Documents/GitHub/rquickshare" push origin --delete ci/ignore-docs-only-main-20261018
   ```
   The agent has **NOT** executed this command and does not claim the branch is deleted.
4. Owner's `Alcotester` may have **additional local-only branches**. GitHub tools cannot inspect local reflogs, local refs, dirty worktrees or alternate remotes. Before local deletion, operator must collect `git branch -vv`, `git branch -a`, `git worktree list`, `git status --short --branch`, `git remote -v` and inspect divergence and unique patches. Never bulk-delete all local branches with `git branch -D`.
5. On future tasks, work directly on `dev` unless another active agent is genuinely using it. If conflict, use a short-lived PR branch targeting `dev`, merge verified work and clean it up. Close completed feature issues after their final actual acceptance, never while tests/physical gate remain open.

## Evidence boundaries and rollback

- `compare dev...ci/ignore-docs-only-main-20261018`: `diverged`, ahead 1, behind 300, modified only `.github/workflows/lint.yml`.
- `compare master...ci/ignore-docs-only-main-20261018`: ahead 1, behind 1, one modified file; blob equal on both.
- No GitHub branch/PR mutation performed by this docs audit. No code was merged because there was **no missing remote code to merge**.
- If any other owner produces local unique commits, first cherry-pick/rebase/PR-review them safely onto `dev` with tests as required; update this audit before deleting their branch. Never assume the previous remote audit covers newly pushed refs.

## Alcotester follow-up: remote branch deleted, local old Vite branch superseded

Owner executed the lease-guarded `git push --force-with-lease=refs/heads/ci/ignore-docs-only-main-20261018:<expected> origin --delete ...`; terminal output returned `[deleted] ci/ignore-docs-only-main-20261018`. A fresh connected GitHub branches query independently confirmed only `dev` and protected `master` remain, with zero open PRs. The earlier CI-branch **is deleted**, not merely pending.

Owner's `git status --short --branch` reports local `dev` clean, `behind 6`, no worktree changes; `git worktree list` shows only the root worktree on `dev`. `git branch --no-merged origin/dev` shows one local-only branch `dependabot/npm_and_yarn/app/main/vite-6.4.2` at `950f5a5`, tracking `upstream/dependabot/npm_and_yarn/app/main/vite-6.4.2`. Local `master` tracks `origin/master` and must remain.

GitHub commit `950f5a5` is available to read and changes only `app/main/package.json` (Vite `^5.4.10` → `^6.4.2`) and `app/main/pnpm-lock.yaml` (older lock graph). `compare dev...950f5a5` is diverged, 1 ahead / 1368 behind, changes exactly those two files. Current `dev` `app/main/package.json` already declares `vite: ^6.4.3`, `vitest: ^3.2.7`, `@vitejs/plugin-vue: ^5.2.4`; its lockfile resolves `vite@6.4.3`. The old Dependabot change is **superseded** and must **not** be merged/cherry-picked, because that would replace newer vetted package metadata with older versions. The original dependabot git commit is left available in upstream/history, but no unique desired effect is missing from `dev`.

Owner's next safe local actions: fast-forward clean `dev` via `git pull --ff-only origin dev`; verify unchanged `950f5a5` local branch tip and existence of newer `^6.4.3` source before `git branch -D dependabot/npm_and_yarn/app/main/vite-6.4.2`. Explicit `-D` is justified **only for this audited, intentionally unmerged superseded branch**. Do not delete local master, dev, upstream remote branches or any other branch. GitHub connector cannot execute commands on Alcotester; local branch deletion is **not yet verified**.

With the sole unmerged local branch removed, root local branches should be `dev` and `master` only. Remote cleanup is complete, but local cleanup awaits owner execution.

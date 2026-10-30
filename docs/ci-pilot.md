# Bounded CI pilot

**Current status:** the ordinary-CI pilot passed its full applicable source
and real GTK/Bubblewrap checks and is merged into dev. Native **1.0.0-1** was
[published successfully](https://github.com/morozhenko-apps/rquickshare/releases/tag/native/v1.0.0)
from the separately reviewed release-repair source on master. Public package
bytes, checksum and the signed APT index were independently verified.

The published workflow includes removal of Mode B/coverage/source-test replay
from tag publication and the signer Path correction. The additional pilot
timeouts, ordinary Rust caches, checkout/token restrictions, locked resolution,
scope fixes and semantic CI contracts remain in dev; they are not present in
the released master source yet. Product/package/publisher bytes match between
these lines, but their ordinary CI configuration differs. Do not report full
master/dev CI parity or rewrite the published tag to change that distinction.
The planning and local-only receipts below are earlier checkpoints.

Mode A. Owner scope: use rQuickShare as the first CI pilot; transfer the proven
recipe to RepoTicker, CFVigil, ArcVault and Picturix only after pilot acceptance.
This is workflow/helper engineering, not a new product test-coverage audit.

## Baseline and ownership

Start at `74ec3d7e367a7cf80d9c47346918a2948dd95a7e`, the existing reviewed
release-cleanup branch in PR #39. Its newer contract already removed Mode B
and coverage workflows and runs every applicable ordinary test once, including
risk probes. Preserve that contract rather than resurrecting the older dev
cadence. The original dev checkout and existing PR branch remain untouched;
actual concurrent work requires this isolated pilot branch and a PR to dev.
Protected master promotion, tag recovery, private signing and publication need
their separate owner authorization.

Completed baseline runs on that source:

- Preflight `41110055066`: success, 18:48:37Z to final admission 18:53:55Z
  (5m18s including startup). Longest job: Tauri Clippy/tests, 5m01s.
- Native GTK `41110055041`: success, 18:48:37Z to final admission 18:50:13Z
  (1m36s including startup); native worker 1m23s.

These are previous-source observations, not pilot results or cold-cache promises.
Preserve the existing independent parallel jobs and existing hosted capacity.
No extra workflow, coverage instrumentation, scanner installation, build matrix,
whole-suite repetition or release-time source-test replay is planned.

## Analysis and diagnostic hypotheses

1. The classifier skips all files under docs, although the native workflow
   consumes `docs/prototypes/preview-bus.conf` as executable test configuration.
   This can falsely treat a runtime fixture change as prose.
2. Its blanket Markdown exemption can hide packaged material. The native DEB
   manifest explicitly copies `native_desktop/packaging/README.md`.
3. Default rename detection may show only a new prose pathname when moving an
   executable input; classification must consider both old and new paths.
4. Missing synthetic merge ancestry can make a PR diff untrustworthy; existing
   code already rejects non-two-parent HEAD. Preserve and exercise this denial.
5. Empty merge diffs cannot prove successful applicability; existing code
   rejects them. Preserve that denial rather than producing an empty green run.
6. The one-off release contract workflow still references the removed
   `mode-b.yml`, so it can fail despite a valid ordinary CI structure.

Use synthetic Git repositories to establish the first three cases before the
fix. Do not run domain suites as diagnostics or change product assertions.

## Implementation plan

1. [x] Analyze current guides, actual open work, completed baseline job times,
   workflow/helper dependencies and package inputs. Record this plan first.
2. [x] Diagnose the classifier against real synthetic Git merges, then narrow
   the prose exemption and include deleted/renamed inputs. Unknown executable
   inputs require applicable checks; known legacy frontend-only changes retain
   the native skip. Keep the existing fail-closed aggregate gates and names.
3. [x] Harden ordinary workflow execution: explicit read-only token permissions,
   no persisted checkout credentials, locked Rust resolution, dependency cache
   reuse restricted to trusted dev writers and separate configuration keys.
   Keep compilation reuse within a job and native acceptance unchanged.
4. [x] Bound existing job duration: scope/admission 5 minutes, lightweight
   formatting/contracts/frontend 10 minutes, Rust/GTK workers 20 minutes.
   Budgets allow setup/cold compilation beyond the measured baseline; a timeout
   fails visibly and does not retry. Queue time is not a promised wall-time cap.
   Remove the deleted Mode B path from the manual workflow syntax checker.
5. [x] Add focused parameterized regression tests for scope and actual aggregate
   shell gates. Extend cheap policy checks so missing time/permission/cache
   boundaries and dangling workflow inputs cannot drift silently.
   Parse workflow structure with pinned CI-only PyYAML via BaseLoader (including
   the literal `on` key), not regex approximations. Add `requirements-ci.txt`
   extending the existing R2 test requirements; publisher dependencies stay
   unchanged. This adds one small parser wheel to the existing Python job,
   without another job or runtime dependency.
6. [x] At the final local gate, execute Node helper contracts and Python offline
   contracts once, shell syntax/policy and workflow validation. Preserve the
   existing full source/native checks on the final pilot PR. Record exact
   commands, results and unverified native scope. No repeated full local Rust
   build merely to duplicate the PR's platform-specific execution evidence.
7. [x] Complete remote pilot acceptance: commit docs separately from executable changes, push the isolated branch
   and create a PR to dev. Trigger only this final gate. Per AGENTS.md, stop at
   the handoff URL; do not poll an active CI or roll out other repositories
   before a completion event or a new owner instruction supplies the result.

## Regression inventory and branch map

| Artifact | Planned branches / interactions | Level / rationale |
| --- | --- | --- |
| `ci-pr-scope.sh` | Pure prose, runtime config in docs, packaged Markdown, unknown extension, mixed change, newline filename, add/delete/rename, invalid event/output/merge, empty merge, push/dispatch | Integration: real Git history plus the actual shell classifier; no mock of the subject |
| Preflight admission shell | Required/all success; unneeded/all skipped; each dependency failure/cancel/skip/unknown; bad/missing scope; failed scope | Contract: execute the actual workflow shell with synthetic dependency observations |
| GTK admission shell | Required/success; unneeded/skipped; every incompatible result; bad/missing/failed scope | Same fail-closed contract; no new runtime abstraction |
| Ordinary workflows | Independent jobs; preserved required names; time budgets; read-only credentials; locked resolution; trusted cache writes; no stress/Mode B/coverage; no stale syntax-check input | Static policy plus actionlint where available; meaningful mutations must fail |

Applicable negatives: N1 invalid event/output/result; N2 empty inventory/diff;
N4 rename/delete transition; N5 cancellation observations and independent jobs;
N6 stale/unknown workflow input; N7 credential/ref/result admission; N9 canceled
mandatory work. N3 live network, N8 locale/time, N10 actual PII processing, N11
monetization and N12 real disk faults are not new logic in this CI scheduling
change; existing product suites retain their contracts. Do not claim product
branch coverage from these scheduling regressions.

## Risks, rollback and acceptance

Narrower skip rules can run more checks for ambiguous files; that is deliberate.
Cache misses must still pass and caches never authorize release bytes. No public
API, distribution layout, data format or authorization boundary is redesigned.
Rollback reverts the pilot executable commit, then its documentation commits;
do not revert or rewrite the independent PR #39 work.

Pilot acceptance requires all applicable ordinary checks and native GTK on the
final source, unchanged test inventory, visible real failures, preserved release
admission and an observed runtime without a material regression. Compare cold
and warm behavior honestly; do not claim speedup before execution. Until that
receipt arrives the other four repositories remain unchanged.

## Diagnostic result before implementation

Three minimal real-Git merge reproductions returned exit 0 and
`preflight=false`, `gtk=false`: changing the live preview bus configuration,
changing packaged README content and renaming a native runtime input into a
prose path. All three should require source/native checks. The first two are
caused by broad directory/extension exemptions; the third additionally exposes
rename collapsing. Use `git diff --no-renames --name-only -z` and a narrow prose
allowlist with executable/package paths taking precedence. A new scope test
must cover each actual case, including old and new rename directions.

Supporting references: [workflow syntax and timeouts](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax),
[cache input contract](https://github.com/Swatinem/rust-cache/tree/v2.9.0),
[existing pinned workflow validator](https://github.com/rhysd/actionlint/blob/v1.7.7/docs/install.md).

## Local validation receipt

Executable source commit: `de0b003dd8176c7c651c96ff7f8e8d487d6dda47`.
The final local gate ran once after the last executable change:

| Command | Exit / concrete result |
| --- | --- |
| `bash -n scripts/ci-pr-scope.sh` | 0 |
| `bash -n scripts/ci-core-test-groups.sh` | 0 |
| `bash -n scripts/ci-native-gtk-test-groups.sh` | 0 |
| `bash scripts/ci-check-workflow-policy.sh` | 0 |
| `node --test scripts/tests/*.test.mjs` | 0; 17 passed, zero failed/skipped/canceled; 1.818s process time |
| `work/rqs-pilot-python/bin/python -m unittest discover -s scripts/tests -v` | 0; 102 passed, no skips; 49.664s process time |
| `work/ci-pilot-tools/actionlint -config-file .github/actionlint.yaml -shellcheck= -pyflakes= .github/workflows/lint.yml .github/workflows/native-gtk-ui.yml .github/workflows/native-release-check.yml .github/workflows/native-tag-publish.yml .github/workflows/native-apt-refresh.yml` | 0; five relevant workflows, pinned v1.7.7 archive checksum verified |
| `git diff --check` | 0 |

The `work/` executable paths above belong to the external task workspace, not
the application. Python 3.14.4 / Node 24.22.1; the isolated Python environment
installed exactly `scripts/requirements-ci.txt`. All eight commands took
51.627s combined locally. This is helper validation, not the PR wall time.
No source file was changed after this local gate, only this receipt.

Added nine test methods (seven Python, two Node), plus expanded table-driven
scope cases. The scope harness exercises 31 real-Git/event/output cases; the
new gate tests execute 56 dependency/scope outcome combinations and semantic
workflow assertions. Mutations removing a required result, accepting cancel or
missing scope, skipping packaged content, collapsing renames, widening cache
writes, removing locked resolution or time bounds are protected by these
assertions. No instrumented mutation run or product coverage claim is made.

No separate known-FLAKY inventory is registered by this pilot; it neither
quarantines failures nor labels risk probes flaky. Repetition was not run.
At this earlier local checkpoint, complete Rust/frontend/Tauri and real
GTK/Bubblewrap execution on the final PR were PENDING; the current accepted
status is recorded above. Native product logic, live packages, device state, protected
master, tag/signing/R2 operations and the other four products were not changed.
All owned local commands completed; terminal closed.

Source acceptance is complete through merged PR #42; release repair PR #39 is
also merged. The incremental pilot diff starts at `74ec3d7e`. Any subsequent
promotion must preserve required checks, source identity and the immutable
published release; do not treat the existing publication as proof that the
extra ordinary-CI hardening is already on master. Other-product rollout is
not performed by this documentation update.

# Bounded source CI

The accepted pilot is integrated into protected master. It adds scope, cache,
checkout, locked-resolution and execution-budget controls. The published Native
1.0.0-1 identity remains unchanged; see [distribution](FLAT_DISTRIBUTION.md).

## Required source evidence

Preflight retains independent format/contracts, frontend, core and desktop Rust
lanes. Native GTK/Bubblewrap remains a separate applicable check on its required
OS. Stable suites, including existing risk probes, run once per required
environment. Repeated stress/coverage campaigns are explicit work, not ordinary
release-time replay or a reason to reclassify tests as known-flaky.

Scope/admission jobs have five-minute budgets, lightweight lanes ten, and
Rust/native workers twenty. Budgets bound execution rather than runner queues
or manual approvals. A timeout fails visibly without retry-to-green.

PR scope examines the complete two-parent synthetic merge diff, including both
rename/deletion sides. Only known prose may skip work. Packaged Markdown, live
fixtures within docs and unknown inputs require checks. Invalid ancestry,
missing output and empty diffs cannot establish a skip.

Existing required results always report. Successful scope must show all
applicable dependencies succeeded or all inapplicable dependencies skipped.
Failed, canceled, missing or inconsistent observations remain red. Ordinary
tokens are read-only and checkouts do not persist credentials. Rust resolution
is locked; cache configurations are separate and only the explicit trusted-ref
condition may save them. A cache never authorizes release bytes.

## Development validation

Use scripts/requirements-ci.txt in an isolated Python environment, then run
Python and Node contracts under scripts/tests, workflow policy and pinned
actionlint. Regressions execute actual shell admissions and classifiers against
dependency outcomes and real temporary Git merges. Retain failure, cancellation,
missing-evidence, packaged-input and rename/delete cases.

Run applicable stable checks once at the final gate. Repeat only affected
checks after a diagnosed defect. Never weaken product assertions or quarantine
new failures to accept CI. After final CI starts, record its exact source/URL
and stop polling until a completion event or owner instruction.

Native/device, signing and publication requirements remain independent.
Historical rollout comparisons, run ledgers and diagnostic transcripts remain
in Git history and private operational evidence rather than this active guide.

The sole permanent source branch is master. PR gates and explicit verification
markers target that branch; only trusted non-PR mainline runs may save caches.
Private candidate builds retain separate markers and never authorize release.
Retirement of dev does not remove signing, source/tag or publication approval.

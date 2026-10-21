# Native release Git-tag namespace: approved MODE A migration

Date: 2026-10-22. **MODE A (refactor of release/public source identity)**. User explicitly selected **Option A** after discovering that upstream's original `v0.9.0` tag already exists in the fork.

## Motivation / established decision

- The native Linux product is independently versioned; current planned product SemVer is **0.9.0**, Debian artifact **rquickshare-native_0.9.0-1_amd64.deb**, with stable native **1.0.0** later. Upstream Tauri/RQuickShare tags include `v0.9.0` (commit `f850713a75c219299ba797742f4269c6e61a0c84`) and `v0.11.5`. They belong to preserved historical source and must never be moved/deleted/reused.
- Native release *Git ref* is now **`refs/tags/native/vX.Y.Z`** (for example `native/v0.9.0`, later `native/v1.0.0`). Public GitHub Release uses that same tag; display title can remain rQuickShare 0.9.0. Original `vX.Y.Z` tags must be ignored by native publication/recovery workflows.
- The *production delivery channel* is unchanged by this migration: existing protected `master`, `apt-release` GitHub Environment with Required reviewer, private `rquickshare-native-apt` R2 bucket, HTTPS Worker `download.morozhenko.com/dist/rQuickShare/`, signed APT suite `rquickshare-native-ubuntu26.04`, immutable package paths, ABI Ubuntu 26.04 amd64. Only namespace of the Git source marker changes. No new GPG key or R2 secret, and no early tag/publication is authorized.

## Repository scan and per-owner plan

| Owner | Current old v* behavior / risk | Approved migration |
| --- | --- | --- |
| `.github/workflows/native-tag-publish.yml` | `push tags v*`, `^vX.Y.Z$`, title suffix `#v`, first-release note, may consume inherited upstream tag | Trigger **only** `native/v*`; validate exact selected tag and protected `master` matching HEAD and native product version through single helper; require immutable **annotated** tag; title & release URL use native form. Retain Mode B, signing, R2 and public HTTPS gates unchanged |
| `.github/workflows/native-apt-refresh.yml` | Accepts upstream `vX.Y.Z`, derives `v` from historical signed version and fetches that tag | Accept only canonical `native/vX.Y.Z` and assert it matches independently recovered signed product version. Fails closed if selected ref is absent |
| `scripts/native_ci_archive.py` | `current_tag()` returns `v` + authenticated pool version | Return native namespace using shared strict tag utility; signed snapshot itself remains authenticated and unchanged |
| `scripts/native_release_tag.py` (new) | No centralized release-tag grammar | Pure strict two-way mapping `native/vX.Y.Z` ↔ product `X.Y.Z` with CLI for shell workflow; full-string regex, no prerelease/build metadata, leading zeros, path injection, slash escapes, old tags, or arbitrary outputs |
| `scripts/tests/test_native_ci_archive.py`, new `scripts/tests/test_native_release_tag.py` | Assert upstream-style tag; lack adversarial namespace checks | Cover correct conversion, roundtrip, incorrect/noncanonical inputs, signed archive source tag, previous-snapshot version ambiguity; enforce mapping with real source scripts |
| `.github/workflows/p0-{bwu-router,client-introduction,weave-worker}.yml` | Manual tag-specific 10× diagnostics only admit original `^vX.Y.Z$` | Keep dispatch-only and annotated/master gates; accept `native/vX.Y.Z`, no old-tag acceptance |
| `scripts/ci-check-workflow-policy.sh` | Enforces no dev heavy push and tagged Mode B | Preserve all existing cadence/authorization checks; add checks that native publisher triggers only native namespace and restore cannot produce upstream tags |
| `docs/product-release-strategy.md`, `docs/native-tagged-release.md`, release readiness and issue #14 | Prior `v0.9.0` public tag instructions now collide | Rewrite current/future operational instructions to native prefix, retain historical notes explicitly marked superseded |

## Test inventory & risks (positive / N1–N12)

| Category | Required outcome |
| --- | --- |
| Positive | `native/v0.9.0` maps to `0.9.0`; `native/v1.2.3` to `1.2.3`; restore from signed `1.0.0-1` fixture proposes `native/v1.0.0` |
| N1 malformed | Reject empty, `v0.9.0`, `v0.11.5`, `native/v`, missing segment, whitespace, `native/../`, mixed-case, `native/v1.2.3/extra` |
| N2 boundaries | Reject leading-zero components, negative, prerelease or `+build`; allow canonical 0.0.0 and large digit sequences within existing SemVer grammar |
| N3 network | Archived R2 select-tag/network failure remains distinguishable from genuinely empty signed channel |
| N4 retry | Repeated identical tag does not generate new bytes or retarget immutable Git ref; no tag creation in this migration |
| N5 concurrency | Only single signed production `InRelease` CAS, existing release concurrency and reviewer contract unchanged |
| N6 stale | Upstream old tags and a tag pointing off protected HEAD cannot be admitted |
| N7 authorization | Required protected master, annotated tag, `apt-release` reviewer, signer-only key and narrow R2 scope remain, no new privileges |
| N8 time | Scheduled three-day signed APT renewal continues to derive exact current native tag; manual reviewer may delay seven-day expiry and is documented |
| N9 interruption | No secret access or public writes when admission fails; package signing and publish failure semantics unchanged |
| N10 confidentiality | No logging of tokens or key material, new helper only consumes public tag/version strings |
| N11 monetization | N/A: no billing |
| N12 storage | Missing private snapshot, malformed/corrupt archive still fail closed; no authenticated authority constructed from a tag alone |

**Mutation review targets:** remove `native/` from tag conversion, allow a bad `v0.9.0` input, relax `master` equality/annotated guard, change `native_ci_archive.current_tag` result to upstream format; focused tests/policy checks must detect. Retain old-test historical package fixture `1.0.0-1` where it's not an actual release source reference.

## Implementation and completion gates

1. **Documentation-first** (this file, docs-only commit); read release workflows/archive/CI policy and fixed existing tags.
2. **Single atomic production+unit-test commit:** native tag mapping, release + refresh + standalone P0 workflows, scoped Python archive regression and CI source policy. No signing keys, Worker deployment, master or R2 changes.
3. **Docs-only completion record:** update active release docs and #14. Historical evidence notes stay historical.
4. **Final checks:** normal fast Preflight, manual native release actionlint and manual native package smoke according to mandatory no-heavy-dev-push policy, then an actual authenticated native-tag Mode B 10x **only after** real hardware acceptance. Do not claim tests without exact-source completed CI.
5. **Operator-only GitHub UI:** add `native/v*` **as a second Include pattern to the existing active** `protect-release-tags` ruleset (do not remove its historical `v*` protection). Under `apt-release` Selected branches and tags, retain `master` and replace the previous `v*` entry with `native/v*`; retain Required reviewer. Confirm exact effective rules via GitHub API once changed.
6. Do not create `native/v0.9.0`, promote `master`, issue a GitHub Release or publish to R2 until package/Mode B/physical/reviewer/security gates pass and the user explicitly initiates the real release. After publication never move this ref or overwrite same-version package bytes.

Rollback before first native tag: revert this migration's isolated code commit, remove only the additional GitHub tag ruleset/env pattern if configured (owner action); upstream refs and previously signed APT remain unchanged. After first release, tag namespace is permanent and must not be silently reverted.

**Important:** This migration restores correctness of independent Native versioning; it does *not* resolve unrelated open P0 BWU security #8 (explicitly deferred), networking #9/#11, mDNS #12, or live incoming Devices #13 acceptance.

## Implemented checkpoint (2026-10-22)

- **DONE** — docs-first architectural approval/plan, docs-only commit `f42a8bc1`.
- **DONE** — canonical Native release namespace in code/test-only commit `13417e1ea9fc835e1ac1bfb7260ec912fabba0cb`: strict Python two-way version/tag mapping and CLI, `native/v*` GitHub production release admission with protected-master + annotated tag, signed APT refresh and authenticated archived-version mapping, three manual P0 tagged diagnostics, regression tests and dev CI policy assertions. Release package, signing protocol, Core/GTK runtime and R2 storage untouched.
- **DONE** — current release strategy, operational setup and historical release readiness migration documented separately in `04b610cd6dc8f9163312db8cffc3e5f534683cba`.
- **PARTIAL** — [Preflight 39692302214](https://github.com/morozhenko-apps/rquickshare/actions/runs/39692302214) on exact executable SHA `13417e1e`: Rust format, Core Rust tests, frontend and Core Clippy jobs **SUCCESS**; Tauri desktop Clippy/units was **still in progress** at last read. These are not final workflow PASS until GitHub records completion. Workflows are manual for separate actionlint and expensive native package smoke by established policy, not automatically triggered by ordinary `dev` pushes.
- **BLOCKED BY OPERATOR UI** — extend existing active protected-tag Ruleset with include `native/v*` while preserving original `v*` protection; update GitHub `apt-release` Environment selected tag restriction to `native/v*`, keep `master` and manual reviewer.
- **NOT DONE** — real `native/v0.9.0` tag, full release-tag Mode B (10×), real 0.9.0-1 .deb, signed R2/APT/public acquisition and Pixel/KDE physical acceptance. Do not conflate source-only validation with production release readiness.

**Separate pre-existing release-package test blocker:** native package smoke [39674598404](https://github.com/morozhenko-apps/rquickshare/actions/runs/39674598404), from before this namespace migration, failed `test_native_signer_backup` with GPG exit code 2 while importing a test-only key inside a nested recovery temp GnuPG home. The global short-TMPDIR test fixture change solved earlier bulk GPG setup errors, but not this nested recovery error. Core 783 and Worker passed. No package was produced. Diagnosing root cause and fixing that distinct test fixture is required before release-profile package validation and should not be hidden behind a version namespace change.

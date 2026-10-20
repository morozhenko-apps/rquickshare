# Release readiness and GitHub → R2 rehearsal

Date: 2026-10-20. Primary task MODE A. Status: documentation and backlog preparation, NOT approval to create a public tag, write R2, sign artifacts, provision secrets, merge branches or install software.
Related authorities: product-release-strategy.md, native-tagged-release.md, native-essential-priorities.md, native-essential-acceptance.md, native-r2.md.

## Confirmed baseline

- dev at review: 9045cadd15ab0f1ea2423ed5d3673efc0f9de33c; master at 3f6f8b32cd9414bce30865402f8b707e6b2b12e0. They differ. GitHub branch endpoint reported protected=false for each; effective repository rulesets and tag protections still need verification.
- User built and verified the Ubuntu 26.04 amd64 native development package rquickshare-native_1.0.0-2.debug_amd64.deb (SHA-256 d88144bb7274c1d7615c7ca85659485c16e78d7c46f416c04d4b0ae175f3b710). APT dry run: 1 upgrade from 1.0.0-1.debug, 0 removals, 25 unrelated packages not upgraded. Real installation of revision 2 has not been reported.
- Revision 2 source exact Preflight https://github.com/morozhenko-apps/rquickshare/actions/runs/39486686043 and Native GTK https://github.com/morozhenko-apps/rquickshare/actions/runs/39486686120 both completed SUCCESS.
- Earlier 1.0.0-1.debug Pixel 6a → Ubuntu BWU migration and end-to-end transfer passed; current revision 2 UI behavior has not been physically tested. A received-file independent hash comparison was not reported.
- Local build's nine rqs_lib warnings, bluer future incompatibility and dpkg-shlibdeps loader diversion warnings were nonfatal. Investigate compiler/dependency sustainability before promising stable support.
- GitHub Releases API showed no published/draft releases in this repository at this review.

## Prioritized remaining checklist

| Priority | Work / owner issue | Status / release evidence |
| --- | --- | --- |
| 1 | Incoming Devices attention, badge and exact sender focus (#13) | Source and CI PASS; install revision 2 after idle check and test on KDE, collapsed/open, one/multiple requests, focus/scroll, reduced-motion and lock/DND |
| 1 | Android ordinary two-way sharing | One Pixel inbound BWU transfer PASS, earlier outgoing smoke known; current build needs actual text/URL, multiple files/folders, explicit consent, matching sender/received SHA256 and cancellation checks |
| 1 | Recovery & file safety | Verify disconnect mid-transfer, sleep/resume, full disk, inaccessible destination, conflicting/symlinked paths, partial-file publication, fail-closed behavior and deliberate explicit retry |
| 1 | KDE installed lifecycle | Test tray and device drawer, file manager Send, single instance, autostart opt-in, updates/removal, settings retention, daemon reconnect/recovery |
| 1 | #11 unroutable LAN IPv4 fallback | OPEN, architecture policy choice remains required; test unusable/multicast/broadcast and legitimate non-RFC1918 addresses |
| 1 | #12 repeated mDNS discovery generation | Implementation + exact-source CI completed; reconcile still-open issue with evidence and remaining physical results |
| 1 | #9 mDNS SDK reachability/test seam | OPEN, explicit architecture choice required; deterministic browse event/network transition tests pending |
| Deferred | #8 BWU TCP pre-commit authentication | OPEN/DEFERRED by user, not fixed. Preserve Google interop, explicitly review residual DoS risk before stable public claims |
| 2 | Complete release-candidate QA | Actual final tagged source Mode B, negative N1–N12 applicability, stable/flaky separation, package provenance and physical smoke. Do not equate Preflight with full Mode B gate |
| 2 | GitHub → R2 rehearsal | Existing producer/publisher available but no actual tag→R2 end-to-end proof; validate tag/branch policies, isolated signing secrets, conditional R2 upload, immutable hashes and independent client readback |
| 2 | Production APT + public direct .deb | Verify key source/signatures, actual isolated APT acquisition, Worker availability, install/enrollment, index refresh within 7-day expiry and failure alerting |
| 3 | Official v1.0.0 | Release only after Linux readiness, approved provenance/security/branding/GPL/source notice and a verified master tag; public GitHub release last |

Outside first Linux essential release: Google Account/Contacts, trusted auto-accept, QR interoperability, previews, resumable offsets/dedup, stream/AirDrop and native Windows/macOS ports. These remain longer-term roadmap items and must not be falsely called implemented for v1.0.0.

## Existing deployment boundary (important)

The approved first stable public tag is v1.0.0. The native-tag-publish.yml workflow currently triggers on all v* tag pushes, but its admission step only accepts stable vMAJOR.MINOR.PATCH tags, equal to native_desktop/Cargo.toml version and pointing to current master HEAD. After a 10x Mode B reusable gate it builds a release Debian package, signs with protected apt-release environment, conditionally uploads immutable pool/by-hash objects and then InRelease in dedicated PRODUCTION bucket rquickshare-native-apt, checks public https://download.morozhenko.com/dist/rQuickShare and actual APT download, then creates GitHub Release.

The production bucket and active dists/.../InRelease object are hardcoded in native_r2.py and the read-only Worker configuration. No test tag must overwrite them. GitHub environment/secret names and effective protection rules were not available for verification in this inspection; never request/export secret values through chat.

## Release rehearsal architectural decision: pending

OPTION A (recommended): reserved version v1.0.0-rc.1 (release candidate), real Git tag + GitHub Actions pipeline, but only a separate private staging R2 bucket, staging signer and staging-scoped credentials. Modify stable workflow filters to reject RC *before* secrets/build; RC workflow must not use apt-release or deploy the production Worker. Canonical mapping from semver prerelease to Debian lower-than-stable 1.0.0~rc1-1, with tests and no reuse of mismatched 1.0.0-2.debug bytes under a new immutable version. Read back R2 bytes/SHA256 and verify test signed metadata via isolated APT. Staging success does not automatically release public v1.0.0.

Pros: proves real Git-tag CI→bucket upload without changing live APT/download route, preserves first stable identity. Cons: separate CI trust boundaries, keys, credentials, version policy and setup; production Worker must still pass final release smoke.

OPTION B: make v0.9.0 a genuine public production release through the existing protected master/tag workflow and production signed R2 bucket. Pros: tests full public download/APT path without a second staging environment. Cons: NOT a dry run; publicly trusted bytes and irreversible channel history, dependency/ABI/version downgrade against installed 1.0.0-2.debug, higher operational risk. Requires actual product/security release approval, not just testing intent.

STOP FACTOR: creating separate staging distribution authority, changing prerelease version/tag policy, or authorizing production public publication is a material security/release decision. Do not create tag, bucket, keys, workflow or real R2 objects before user chooses A/B. The user requested reserving a near-term release and testing Git→bucket, not publishing unverified production assets.

## Planned Option A delivery after approval

1. Inspect protected master/tag rules and release environment metadata only; retain all existing production secrets and Worker route unchanged.
2. Document and test release candidate tag syntax, exact SHA source identity, Debian prerelease version ordering and separation of prod/staging signing + R2 credentials.
3. Implement dedicated staging release workflow and publisher targeting private staging only, with production flow refusing prerelease tags. Add fail-closed tests for missing/incorrect credentials, tag/master mismatch, wrong bucket, unsigned/mismatched artifact, conflicting immutable object, ambiguous R2 state and CAS/replay.
4. Run one immutable RC tag after the agreed acceptance gate, verify jobs, object bytes and hashes from independent R2 readback, signed index from isolated APT, no production mutation. Record evidence and explicit cleanup/retention.
5. On a later separate approval, ship v1.0.0 from protected master with full real-device checks, signed production snapshot and public HTTPS/GitHub Release verification.

No other repo, R2 bucket, existing release, installed user service or master branch may be mutated in this planning task.

## DECISION UPDATE — 2026-10-20: Option B selected, public v0.9.0

**Owner decision:** choose **Option B**, an actual early public `v0.9.0` release into the existing production Cloudflare R2 bucket and signed APT channel. No isolated staging bucket or `v1.0.0-rc.1` staging release is planned. This decision supersedes the prior staging recommendation and staging STOP FACTOR in this document, but **does not waive required validation, credential custody, GitHub branch protection, source provenance, user acceptance, security disclosure or actual release gates**.

**Mode A, implementation stages:**

1. Record the selected public `v0.9.0` product semantics and existing distribution guarantees. This is a genuine, limited-scope Ubuntu 26.04 amd64 **early release**, not full Google Quick Share parity; `v1.0.0` becomes a later stability target. Keep `rquickshare-native` package identity, GPL/attribution, `download.morozhenko.com/dist/rQuickShare/`, existing bucket `rquickshare-native-apt`, signed suite and tag-gated CI workflow.
2. Prepare a candidate **only on `dev`**: update native_desktop product crate and its exact lockfile package entry from `1.0.0` to `0.9.0`, reset Debian revision to `1` for a fresh never-published `0.9.0-1`; add negative version ordering fixtures. Do not rewrite historical internal `1.0.0-2.debug` evidence or the independent core and Tauri versions. Retain current `1.0.0-2.debug` binary for rollback/device testing; never re-label/reuse its bytes as `0.9.0`. Follow the existing package builder instead of custom direct uploads.
3. Review complete exact-source Preflight/Native GTK and package/Mode B workflows; complete linked P0 Linux/Android tests, #13 visual acceptance, package lifecycle, security review and all required release gates. Actual authenticated `master`/tag rulesets and `apt-release` environment/secret presence must be verified by the repository operator. GitHub branch read currently reported `master.protected=false`; **do not enable automated signing from an unprotected production branch**.
4. Once the candidate is accepted, fast-forward verified `dev` to protected `master`; create a **single immutable annotated** `v0.9.0` tag at the matching commit; let the existing tag workflow run release Mode B 10x, build, signed APT publish and public download checks. Do not bypass it through raw Wrangler upload or upload a debug .deb. Preserve source/binary digest receipts, prove signed APT acquisition and GitHub Release; never overwrite same-version bytes or move the tag.
5. After publication, validate `InRelease` renewal before its 7-day expiry, explicit enrollment and update semantics. The owner's installed `1.0.0-1.debug` is **higher** than public `0.9.0-1` in Debian version order. `apt install` will **not** automatically downgrade it; any separate deliberate downgrade test requires preserving settings/rollback and explicit user authorization. The initial early public announcement and upgrade instructions must disclose this limitation.

**Release stop conditions still active:** missing environment protection or publisher signing-only secrets, incomplete exact-source Mode B or meaningful physical acceptance, release-critical regression, conflicting signed R2 snapshot, mismatched immutable package or binary provenance. Scope selection authorizes preparation on dev; do **not** create the tag or modify production bucket while these are unresolved. No runtime environment/secrets are available to this connector for operator-only publishing.

**Rollback before tag:** revert isolated development changes; retain existing debug packages and signing materials. **After tag/publishing**: no tag movement or overwriting immutable objects; use a new patch release for corrections, with explicit signed archive transition. Option B is an operational release, not a reversible dry run.

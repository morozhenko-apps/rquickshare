# Flat R2 distribution

**Current status:** source refactor is merged into dev and live download Worker
retirement is verified. Both exact routes/Workers are absent; existing public
objects and all sibling routes are preserved. Git disconnection and the RQS
app-dist token replacement are owner-confirmed. No new application release,
version/tag change or R2 object upload was performed.

## Approved contract (2026-10-27)

Mode A. The owner approved replacing the download Worker, nested APT/release
layout and full remote snapshots with CFVigil's flat public distribution.
The owner confirmed there are no existing users, so no transitional release
or legacy APT compatibility layer is required. Preserve already published
objects, Git tags, signing identities, application versions and native behavior.

The existing `app-dist` R2 Custom Domain serves `dist/rQuickShare/` directly.
Only platform directories are used: versioned Linux packages and checksums,
`Packages`, `Packages.gz`, signed `InRelease` and the public archive keyring
share `linux/`; genuine Windows installers use `windows/`. Root `latest.json`
points to immutable platform URLs. Do not advertise unsupported platforms.
The signed flat APT source uses the `linux/` URI and distribution `./`.
Its `Filename: ./PACKAGE.deb` references the exact same object as direct HTTPS.
No `pool/`, `dists/`, `releases/v...`, `by-hash/`, remote `snapshots/`,
`latest` binary copies, GitHub Release binary attachments or download Worker.
Local isolated staging directories are temporary verification inputs.

## Publication invariants and rationale

Validate package identities, exact file inventory, checksums, archive key and
signed index before writes. Upload immutable files once, verify private and
public body size/SHA256, then update both flat indexes, signed InRelease,
and finally schema-checked latest.json. Errors never authorize cleanup or a
success receipt. Existing serialization and tag/mainline/environment approval
gates remain. Signed-by and signature/hash verification are never weakened.
Flat indexes omit by-hash: a concurrent APT reader may fail a hash check during
a metadata switch and safely retry; no incorrect package is accepted.
R2 credentials are separate repository secrets deliberately scoped to the
shared app-dist bucket; directory names are not an IAM boundary. Never upload
private keys, debug data or private build artifacts into this public bucket.
Keep existing nonbrowser APT/curl security exceptions; no new WAF/rate policy.
CORS for a browser website and actual live route retirement are operational
cutover gates, not properties proven by source tests.

## Plan and recoverable milestones

1. [x] Analyze guides, source, branch ownership, current CFVigil reference,
   publication contracts and credential names. Use an isolated short-lived
   branch because dev is concurrently active. No shared working tree edits.
2. [x] Record owner decisions, risks, validation and rollback before code.
3. [x] Implement flat signed APT generation/verification and R2 publication;
   preserve native package validation, explicit dependencies and trust checks.
4. [x] Adapt tagged CI, refresh/enrollment and text-only release metadata;
   remove retired Worker sources/workflows and obsolete snapshot-only logic.
5. [x] Adapt existing tests to the approved format and add meaningful
   ordering, direct/APT byte identity, tamper, conflict and public-readback
   regressions. Run normal suites only at the final validation gate.
6. [x] Final stable release-contract suites and workflow/static checks once;
   record commands, outcomes, known-flaky separation and external limitations.
   Publish through a reviewed dev-targeted PR after refreshing concurrent dev.

Validation covers positive flat APT acquisition with an ephemeral test signer;
wrong-key/signature/index/package rejection; exact filenames and limits;
immutable conflicts/idempotency; failed public readback preventing promotion;
manifest ordering and source/tag admission. Mock only external R2/HTTP adapters.
No normal native GUI/device suites are required for unchanged native code;
mandatory repository CI remains gating. No production signing, version bump,
release tag, mainline promotion, credential mutation or R2 upload is performed
to test the refactor.

## Risks, cutover and rollback

Main risks: stale Worker routes blocking new URLs, wrong shared-bucket token,
index/manifest publication order, or incomplete GitHub artifact cleanup.
After source acceptance, retire only each product's exact Worker route and
Git-connected deployment before the first flat publication; preserve sibling
routes, DNS, bucket contents and signing keys. A repository-only deletion does
not disable a deployed Worker. Require explicit release authorization and
actual public byte checks before declaring operational completion.
Rollback source through the separate atomic documentation and executable
commits; restore only the product route if an operational cutover is later
rolled back. Never delete another product's R2 objects or rewrite published tags.

## Implementation checkpoint before final execution

Flat producer, signed-catalog validation, same-object direct/APT mapping,
public-readback ordering, final manifest and receipt-gated exact CI archive
cleanup are implemented. Retired Worker deployment sources and Worker-only
tests/workflows are removed; their historical evidence remains in Git.
No product code, version, signing key, tag, protected mainline or live provider
state changed. Python source AST and Git whitespace checks pass. Normal
stable suites have not yet run; final execution is the next milestone.

The existing seven-day signed metadata freshness and scheduled renewal remain.
Recovery reads current InRelease, verifies its signature, verifies the flat
indexes, and downloads the existing immutable package object. No bucket
listing or remote snapshots are needed. Source tag discovery uses bounded
latest.json as an advisory pointer, followed by historical protected-source
and signed-archive verification. Deployment evidence was retained at
[evidence/native-download-deployment-2026-10-14.json](evidence/native-download-deployment-2026-10-14.json).

Expected validation commands:
`python -m unittest discover -s scripts/tests -v`,
`node --test scripts/tests/*.test.mjs`,
`bash scripts/ci-check-workflow-policy.sh`, and pinned actionlint on changed
workflows. Tests use an isolated pinned boto3 environment and ephemeral GPG
homes; no production credentials. Hosted Preflight remains independently
required before dev integration.

All expected apt-release secret and variable names exist. The old RQS token
was provisioned for rquickshare-native-apt; the owner must ensure
RQS_R2_ACCESS_KEY_ID/RQS_R2_SECRET_ACCESS_KEY authorize object read/write
on app-dist before an actual release. Secret presence does not establish
bucket permissions. Other signing secrets and the selected fingerprint remain.

## Local final validation evidence

Final STABLE gates passed: 73 Python contracts; 4 Node distribution/cleanup tests. No failed or skipped tests
remain in those gates. Pinned actionlint v1.7.7, workflow policy where present,
and Git whitespace checks exit 0. No separately configured known-FLAKY group
exists in these affected Python/Node suites; no claim is made about unrelated
native stress/device behavior. Native source and release acceptance gates remain.

The initial final run exposed two test adaptation defects: a foreign-package
mutation accidentally replaced the new product name with itself, and a flat
APT request assertion omitted APT's explicit `/./` segment. Diagnostic review
considered no-op mutation, missing identity validation, stale signed index,
compressed-index mismatch, and obsolete URL expectations. The observed fixture
bytes and actual HTTP request log confirmed the first and last causes.
The mutation now asserts its input actually changes; the HTTP test requires
the exact flat InRelease and DEB URLs. Existing signature/byte assertions were
retained. Only the affected Python stable gates were repeated, both passed;
already-passing Node/workflow checks were not rerun without a source change.

Real isolated APT update/candidate/download verifies the same DEB bytes and
wrong-key/tamper rejection. S3/HTTP failures, immutable conflicts, conditional
publication, malformed signed catalogs, incomplete receipts and foreign GitHub
artifact provenance are tested without production access. No secret values,
production signing, R2 mutation, route retirement or release tag was used.
Hosted CI and final dev integration are the remaining source-delivery gates.

## Hosted source acceptance and integration (pre-cutover checkpoint)

PR [#30](https://github.com/morozhenko-apps/rquickshare/pull/30) merged into
`dev` as `2e38352d219338575f7016564796e895415e3962` after exact source
`11e853df839ea673d192cb7719632482d9b31a28` passed
[Preflight 40602815633](https://github.com/morozhenko-apps/rquickshare/actions/runs/40602815633)
and [Native GTK UI 40602815618](https://github.com/morozhenko-apps/rquickshare/actions/runs/40602815618).
All source checks succeeded: formatting, offline publication contracts, frontend,
core tests/Clippy, Tauri Clippy and native GTK widget/strict lint verification.
No required check was bypassed or reclassified as flaky.

Source refactor is accepted. Operational retirement is still pending: the
read-only Cloudflare inventory confirms `rquickshare-download` bound to
`rquickshare-native-apt`, exact route
`https://download.morozhenko.com/dist/rQuickShare/*`, route ID
`4646f9b9a5274c829b3c8e06771250ca`. The shared app-dist Custom Domain is
active. Current OAuth can inspect Worker/settings/routes, but the Builds
triggers API returns 403 (authentication scope); the in-app browser is not
authenticated. The owner was asked to disconnect both retired Workers' Git
build integrations before their scoped removal. No provider mutation or
production upload was performed. The RQS R2 credential must authorize
app-dist before a future explicitly approved release; secret names alone
do not prove this permission. No signing-key change is required.

The release readiness issue #14 stays open: source simplification does not
prove production signing, actual R2 publication or physical release acceptance.
Rollback is a scoped executable/docs revert; preserve the original private
bucket and signing identity, existing objects and all sibling routes.

## Verified operational cutover

After the owner confirmed both Git build connections were disconnected and
the rQuickShare R2 token was replaced, Wrangler 4.151.0 deleted only
`repoticker-download` and `rquickshare-download`; both commands exited 0.
Neither deletion used --force or changed bucket objects, signing keys, DNS,
WAF, unrelated Workers or any other application's configuration.

Independent post-deletion Cloudflare settings GET returns 404 for each Worker.
Zone routes changed from 13 to 11, exactly removing the two recorded product
routes. Every other route ID and field matches the immediate pre-cutover
baseline. The app-dist Custom Domain remains enabled. Public full-body GET
readback confirms unchanged Picturix latest.json (565 bytes), RepoTicker's
published InRelease (1,418 bytes) and v0.2.0 DEB checksum (94 bytes).

Picturix manifest SHA256:
`73a2d36a8a1e4b37ba9ddd05266f66323b9a763a845a9de45defbc88238b7fdc`.
RepoTicker historical InRelease SHA256:
`f0c129e8737c87e9126d1eb037aa4632bccf9e073f4d20427806e310f23141d9`.
This proves scoped routing retirement and preservation of those existing
public bytes; it does not claim that a new flat release has been published.

The earlier Builds API 403 remains a limitation of the agent's OAuth scope;
Git disconnection and token scope are owner-provided evidence, not an API
verification. GitHub keeps secret values encrypted. A future normal protected
release must still verify actual S3 permissions and full public byte readback.

Rollback remains scoped: revert the source refactor and recreate only the
product's former Worker/route from its retained Git source/config if needed.
The dedicated old rQuickShare bucket and all historical published R2 objects
were retained. No remote snapshot or alternate DEB copy is created by the
new pipeline. All task-owned command sessions exited; no watcher/server remains.

Final comment audit removed a retired Worker-ownership comment from the
tag workflow and corrected the publisher's CLI description to name the
shared app-dist prefix. Python AST excluding the module description and
workflow content excluding YAML comments remain identical to the accepted
source. No behavioral test is added for this documentation correction;
mandatory automatic workflow policy remains enabled.

Final delivery: merged-dev Preflight 40603490677 and Native GTK 40603490593
completed successfully. The final description-only correction at
8da547087c2547a30ed35196d66a9e132ea84630 also passed the full automatic
[Preflight 40604395151](https://github.com/morozhenko-apps/rquickshare/actions/runs/40604395151).
Only this documentation receipt follows that exact verified source. Source
and scoped Worker retirement are complete; actual signing/R2 publication
remains the normal separately authorized protected release, not part of this
refactor. No release version or tag was allocated or rewritten.

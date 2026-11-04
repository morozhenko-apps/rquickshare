# Native tagged release contract

Native Linux 1.0.0-1 was published. Its accepted version, build, tag and artifact
bytes are immutable. See [signed flat distribution](FLAT_DISTRIBUTION.md) for
current layout, successful publication and verification boundaries.

## Source and package identities

Use annotated native/vMAJOR.MINOR.PATCH tags for the native product, independently
of upstream tags and Rust/Tauri crate versions. The approved tag must identify
the exact reviewed protected master HEAD. Develop on dev; promotion and owner
publication approval are separate. Never publish from an arbitrary development
ref or infer acceptance from a tag alone.

Native package rquickshare-native has its own Debian revision and installs the
canonical /usr/bin/rquickshare. Preserve Conflicts/Replaces and tested migration
from overlapping older package identities. Publish only physically accepted
OS/architecture/ABI combinations; Linux acceptance adds no Windows/macOS claim.
Direct installation does not automatically enroll an APT source.

## Build, sign and publish

Tag admission precedes one credential-free package build. Source tests run in
[bounded CI](ci-pilot.md); periodic Mode B coverage/stress is independent and is
never an automatic release step or a reason to replay every suite ten times.
Protected signing imports only the approved encrypted signing subkey into a
fresh temporary GPG home, with pinned public authority and no usable secret
primary. Passphrases enter private temporary files, never arguments or logs.
Never upload primary/revocation/recovery material to CI or distribution storage.

Verify prior signed archive authority, package identity, native revision and
complete hashes before writes. Use conditional immutable creation and exact
ETag replacement; conflicts or uncertain outcomes stop without blind retries.
Versioned packages are shared by direct HTTPS and flat APT. Publish indexes,
signed InRelease, then latest.json last, with complete private/public byte
verification and isolated signed APT acquisition. Cleanup needs the exact
completed provenance-bound receipt. GitHub Releases carry explanatory metadata,
not a second installer store. Directory names do not isolate writer grants.

## Metadata reissue and recovery

New signed metadata deliberately omits Valid-Until. GPG signatures and all
package/index hash checks remain mandatory, but replay prevention by metadata
expiry is no longer provided. Existing seven-day signed metadata remains
eligible for authenticated historical recovery, even after its expiry; it
cannot be treated as an unexpired active legacy index.

Metadata-only reissue is manual, runs exclusively from reviewed master tooling
and requires the protected apt-release environment. It verifies the original
released tag and package identity, keeps the release's original source commit
in the manifest and does not create a new application binary or tag. No cron
job may use release signing secrets or silently write R2. Owner approval and
public signed-index readback remain mandatory. The manual recovery workflow uses
one protected job and therefore one deployment approval. Recovery logs may
report stage names, non-sensitive outcome classes and package counts, never
credentials, provider internals, package paths or signing material. An uncertain
prior snapshot blocks publication; do not turn verification errors into a new
first-release path. Rerun only after the failing stage has been identified.

When a manual reissue reports a conflicting S3 NoSuchKey for an existing signed
index, use the separate protected, read-only R2 visibility diagnostic with the
SHA256 independently retrieved through Wrangler. It checks repeated S3 GETs
across fresh SDK connections using the same environment-scoped credentials and
also checks release-tag selection. It never loads signing material or writes
storage objects. An absent, mismatching or inconsistent read blocks publication.

A repeated manual recovery failure is treated as a trust-boundary problem, not
a reason to downgrade to first-release behavior. Admission should select the
published native tag and restore the authenticated archive in one R2 client
session, reusing the exact bounded initial InRelease observation, and verify
that the manifest tag matches the signed package version. Do this with the
public APT keyring before importing any private signing subkey. At the end of
recovery, re-read the active R2 authority and reject any mismatch or absence.
Only then validate the immutable native tag, import the signer and prepare
fresh metadata. Preserve existing conditional R2 CAS and public readback.
The earlier separate-step tag selection is the rollback boundary; no package
bytes, release tag or publication identity is changed by this reordering.

Local [signing](native-local-signing.md) and [encrypted recovery](native-signing-backup.md)
remain optional controlled tools. Key possession and source identity do not prove
build provenance or authorize release. After accepted publication, corrections
require a new forward package/release identity. Preserve unrelated objects,
existing signing identity and peer/native acceptance requirements.

The executable owners are native-tag-publish.yml, the manual
native-apt-refresh.yml and scripts/native_apt.py/native_r2.py. Private credentials, custody locations and
provider resource wiring belong outside public documentation.

Machine-facing recovery commands must write exactly one result token to stdout:
`previous` for standalone prior-baseline restore, or a canonical `native/v*`
tag for combined restoration. Library validation must not write human-readable
package paths or status lines to stdout. Such output corrupts Bash command
substitution and must not be interpreted as a missing R2 object. Keep user-facing
verification messages at the explicit CLI boundary; verify the exact stdout
contract with a real signed archive fixture and the fake S3 transport before
requesting another protected run. A green source gate alone does not prove the
production workflow stdout contract.

[Historical design and operator record](https://github.com/morozhenko-apps/rquickshare/blob/ed0a626807711008e6ed4ddeb8bdcef564f58d59/docs/native-tagged-release.md) remains in Git history; its old nested archive,
Worker, binary-attachment and pending-publication instructions are superseded.

<!-- Historical heading anchors retained for existing links. -->
<a id="automated-tagged-release-and-independent-package-versioning"></a>
<a id="frozen-public-contract"></a>
<a id="automated-release-trust-boundary"></a>
<a id="operator-wiring-map-repoticker-comparison-and-two-installation-channels"></a>
<a id="public-download-and-apt-enrollment-contract"></a>
<a id="operator-setup-for-unattended-tag-publication"></a>
<a id="stages-and-rollback"></a>

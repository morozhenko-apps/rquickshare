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

## Renewal and recovery

Signed metadata expires after seven days. The existing scheduled refresh verifies
prior signed authority and reuses identical package bytes; it does not require a
new application version. Its protected environment may require review, so a
schedule is not proof of timely renewal. Keep approval and failure ownership;
never remove a reviewer or weaken signatures to avoid expiry.

Local [signing](native-local-signing.md) and [encrypted recovery](native-signing-backup.md)
remain optional controlled tools. Key possession and source identity do not prove
build provenance or authorize release. After accepted publication, corrections
require a new forward package/release identity. Preserve unrelated objects,
existing signing identity and peer/native acceptance requirements.

The executable owners are native-tag-publish.yml, native-apt-refresh.yml and
scripts/native_apt.py/native_r2.py. Private credentials, custody locations and
provider resource wiring belong outside public documentation.

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

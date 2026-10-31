# Signed APT staging and verification

The current production channel is [platform-only flat APT](FLAT_DISTRIBUTION.md):
one canonical versioned DEB, signed fresh authority and explicit client enrollment.
Native revision and seven-day metadata renewal remain independent of app semver.
The toolkit also retains earlier nested/by-hash offline staging support. The
rules below describe that local verification API; they must not be read as an
instruction to restore its old hosted pool layout or advertise another platform.

## Local staging and trust contracts

The local nested-stage toolkit supports explicit suite/component and actual
architecture inputs, Origin rQuickShare and Label rQuickShare Native. Current
production Ubuntu26.04 ABI and accepted amd64 scope are defined by the release
contract; generic toolkit architecture support is not a shipping claim. Never
advertise a Debian/Ubuntu24 compatible build from this ABI. Native package revision
is tracked separately from Rust semver and incremented for every changed release.
Debug artifacts are never admitted. Same source/version across architectures;
one release package per supplied architecture, no invented missing architecture.

Fresh output only, temporary sibling stage then atomic rename; no overwrite or
symlink/special-file input/output. Release contains SHA256 hashes and sizes of
exact Packages/Packages.gz, Acquire-By-Hash=yes, UTC Date and Valid-Until seven days
later. The bounded validity protects against indefinitely replayed metadata; a
publisher must refresh/sign before expiry even with no code release. Serializer
clock is injected for tests. Verification rejects expiry/future dates, altered
fields, unknown paths, wrong architecture/version/identity, digest/size mismatch,
truncated gzip and dangling/mismatched by-hash copies. Package hashes are streamed.

Only public key material may enter bootstrap/artifacts. Expected full primary key
fingerprint is explicit and checked independently; signing subkeys must chain to
it. gpgv verifies both InRelease (exact cleartext Release) and Release.gpg with a
temporary empty home and explicit public keyring. No ambient trust/key discovery,
keyserver retrieval, trust-all, unsigned fallback, private-key export or executable
remote installer. SHA256 sidecars alone are not authentication.

Bootstrap generates reviewable deb822 .sources with HTTPS URI, exact suite/actual
architectures and Signed-By scoped to an explicit fingerprint/public keyring.
It writes only a fresh output directory, never /etc/apt or a user trust store.
HTTPS URI rejects credentials, query/fragment, whitespace and control characters.
Parameterized local tools do not choose the production trust anchor or grant
publication authority. Use the accepted flat production contract and pinned key.

Signing/promotion verify packages and the entire repository before release. A
publisher must compare the previous authenticated snapshot: reject downgrade or
same-version different bytes, retain old immutable pool/by-hash objects while
clients may reference them, publish metadata last/atomically, and keep rollback
packages as a higher revision. No application binary replacement, service restart,
login grant or trust enrollment is introduced. Existing no-maintainer-script
packages leave live mapped processes/transfers alone; installed upgrades/removal,
manager cache refresh and mixed old-process/new-package behavior remain acceptance.

## Verification inventory before tests

Run focused checks once at the final applicable gate; historical ten-run
campaigns are not a routine release gate. Cover version/revision boundaries
and dpkg ordering;
Release/time/URI/fingerprint tables; actual signed stage with wrong key/signature/
cleartext/index/gzip/by-hash/package/path tamper; duplicate/debug/architecture and
same-version drift/downgrade; immutable output failure; actual isolated APT update,
candidate and download with no host lists/status/trust/config/hooks or installation.
Fake only external signing authority (ephemeral real GPG keys); do not mock repository
validation or APT. Test-only expired/future clocks are explicit. Close test-owned
GPG agents; no HTTP server/watcher. Use local file transport only for isolated APT.
Applicable Positive,N1,N2,N3,N4,N5,N6,N7,N8,N9,N10,N12; N11 not applicable.

Use scripts/native_apt.py --help for the actual prepare/sign/verify/bootstrap
interfaces and explicit reviewed inputs. Tests use isolated APT state and
ephemeral signing identities; no system trust enrollment or real package install
is part of source verification. Installed upgrades/removal require native proof.

[Historical staging plan and experiments](https://github.com/morozhenko-apps/rquickshare/blob/ed0a626807711008e6ed4ddeb8bdcef564f58d59/docs/native-apt.md) remain in Git; obsolete package paths,
host setup notes and pending-publication claims are not current instructions.

<!-- Historical heading anchors retained for existing links. -->
<a id="native-signed-apt-distribution"></a>
<a id="analysis-and-plan-before-code"></a>
<a id="frozen-staging-and-trust-contracts"></a>
<a id="primary-references-inspected"></a>
<a id="current-state"></a>
<a id="local-operator-commands"></a>

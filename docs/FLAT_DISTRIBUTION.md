# Signed flat distribution

Native Linux **1.0.0-1** was published successfully. The package, checksum,
public key, signed APT index and isolated APT acquisition were verified.
The [release notes](https://github.com/morozhenko-apps/rquickshare/releases/tag/native/v1.0.0)
retain the published identity. Later CI and documentation changes do not
rewrite published tags or artifact bytes.

## Layout and client contract

One versioned DEB serves direct HTTPS and signed APT acquisition. Its platform
directory also holds the checksum, Packages, Packages.gz, InRelease and public
keyring. Root latest.json references supported immutable artifacts. Flat APT
uses distribution `./`; Filename `./PACKAGE.deb` identifies that same package.
Local staging is temporary. Do not create duplicate latest binaries, nested
release copies, remote snapshots or a download service. Advertise only platforms
with their own native acceptance; Linux publication does not prove peer-device
interoperability or Windows/macOS support.

## Publication safety

- Require the owner-approved tag at reviewed protected mainline, exact source
  and package identities, signing admission and required platform checks.
- Validate complete file inventory, sizes, checksums, archive key and signed
  catalog before writes. Immutable creation is conditional; reuse only
  identical verified bytes. Mutable replacements require the prior ETag.
- Verify complete private and public byte length/SHA-256. Availability, object
  metadata, redirects and challenge/error bodies cannot substitute for proof.
- Publish artifacts first, flat indexes next, signed InRelease after them,
  and schema-checked latest.json last. Failures never authorize a success
  receipt or cleanup. Temporary CI artifact cleanup requires verified
  repository/run/artifact provenance and the completed receipt.
- Keep installers cacheable and authority metadata fresh. Never publish private
  keys, diagnostics or build inputs. Directory names do not isolate credentials.

Flat indexes are not a multi-object transaction. APT may reject a transient
signed-index mismatch during a switch and retry; it must not bypass signatures
or hashes. Retain the seven-day signed metadata lifetime and scheduled renewal.
Recovery verifies current InRelease, indexes and the existing immutable package.
Use the latest pointer only as advisory source discovery, followed by protected
source and signed-archive verification.

## Validation and recovery

Executable contracts live in scripts/native_apt.py, scripts/native_r2.py and
their offline tests. Use ephemeral test signers and external storage/HTTP
adapters. Cover wrong key/signature/index/package, tampering, malformed catalogs,
write conflicts, same-byte reuse, failed public readback, pointer ordering and
foreign/incomplete cleanup receipts.

Source CI, [installed acceptance](native-essential-acceptance.md) and publication
are separate gates; see [bounded CI](ci-pilot.md). Revert scoped source commits
while preserving signed identities and unrelated products. Operational custody,
provider identifiers and execution ledgers belong in private records and Git
history, not active public instructions.

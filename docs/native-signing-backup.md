# Encrypted signing recovery contract

Recovery is an operator-owned encrypted backup, independent of public package
hosting. Preserve the selected existing destination and signing identity; this
document does not provision storage, change retention or request a new key.
Concrete remote names, local mounts, key locations and restore receipts belong
in private operational records, outside the repository.

## Frozen recovery format and invariants

Copy only identity.json, archive-keyring.gpg, the three protected private key files
from authority/signer matching validated public keygrips, and the expected primary's
automatically generated revocation certificate. No sockets, trust database, logs,
user configuration, unrelated GPG homes or extra files. Do not archive/restore an
arbitrary pinentry-program config; generate the known safe agent policy instead.
Source homes must be idle; refuse an existing agent rather than stopping it.

Plain recovery payload is a bounded uncompressed deterministic USTAR in memory:
six fixed source files plus canonical manifest binding names/sizes/SHA256 and the
expected primary. Maximum4MiB archive, maximum1MiB each member; regular0600 files
only, bounded count, no absolute/traversal/duplicate/PAX/link/device/directory
members. Parse into bounded memory; never use extractall. Validate exact inventory,
manifest digests, identity and public key before constructing any private profile.

GPG symmetric AES256, SHA256 S2K, maximum supported iterated S2K count65011712,
no compression, no embedded filename, no symmetric passphrase cache. Use a fresh
private0700 crypto home with the existing enforced16-character Pinentry policy;
no password args/env/loopback, no password copied to cloud/receipt/Git. Bound
ciphertext and GPG decrypted output. Check GPG success before accepting plaintext;
no ignored MDC/integrity errors. Plain recovery payload never reaches Google Drive.
Only ciphertext and non-secret checksum/receipt/instructions may be published.

Restore requires independently supplied primary fingerprint and a fresh canonical
caller-owned0700 destination under a safe parent. Use validated names, exclusive
private file creation and generated agent config. Import the exact public keyring,
compare identity/keygrips and actual KEYINFO protected disk key sets. Sign a
non-release challenge with the restored signing subkey and verify exact primary/
subkey/SHA256 with gpgv. In another disposable public-only profile, import a copy
of the recovery revocation certificate and verify it revokes exactly this primary;
never import revocation into active profiles. Agents belong exclusively to newly
created homes and are closed in finally. Failure preserves partial restore for
review but removes its readiness manifest; no overwrite or automatic key rotation.

Backup creation decrypts and verifies its own ciphertext before committing a fresh
private output directory. Ciphertext is content-addressed; sidecar is integrity,
not independent authentication. Read-only cloud publication must use bounded exact
paths and readback, no cached-only success or indefinite retry/polling. Quota errors
may delay cloud acceptance without invalidating a verified local backup.

## Scoped verification inventory before tests

Fixed inventory and source-idle guards; protected source layout; canonical manifest,
member limits/counts/digests; unknown/traversal/link/duplicate/malformed/oversized TAR;
wrong primary, corrupt/missing metadata and changed file; safe fresh restore/output
paths; exact restored roles/protection/public identity; ciphertext tamper/wrong
password/cancel; real restored subkey signature and revocation usability; owned
agent/temp cleanup and no active-key mutation. Table-driven where rules share
branches. One appropriate development run; rerun affected failures/changes only.
Existing native CI discovers test_native_*.py with GPG/Python prerequisites already
present. No desktop/Worker/Rust rebuild, unchanged suite or real Drive test upload.

## Operation and recovery

Use scripts/native_signer_backup.py with explicit reviewed inputs. Initialization,
unlocking, backup upload, removal and restoration require their own authorization.
Readback must bypass local mount caches and compare complete ciphertext and
sidecar bytes. Quota/authentication failures remain incomplete remote acceptance;
never compensate with a public recovery location or a password stored beside it.
Never overwrite active profiles or import revocation into them.

[Historical backup/readback record](https://github.com/morozhenko-apps/rquickshare/blob/ed0a626807711008e6ed4ddeb8bdcef564f58d59/docs/native-signing-backup.md) remains in Git; no repeat upload,
password prompt, background follow-up or provider mutation is requested here.

<!-- Historical heading anchors retained for existing links. -->
<a id="encrypted-recovery-backup-in-the-users-existing-google-drive"></a>
<a id="destination-observations-and-ownership"></a>
<a id="plan-before-code"></a>
<a id="primary-references"></a>
<a id="b1b2-implementation-and-measured-checks"></a>
<a id="production-operation-prepared"></a>
<a id="b3-actual-local-recovery-verification--b4-preflight"></a>
<a id="actual-mounted-copy-and-remaining-cloud-acceptance"></a>
<a id="quota-recovery-readback-plan-2026-10-15"></a>
<a id="actual-retry-and-scheduled-continuation"></a>
<a id="local-mount-diagnostics-plan-2026-10-15"></a>
<a id="local-mount-diagnostic-results"></a>
<a id="first-successful-independent-archive-readback"></a>
<a id="user-authorized-sidecar-retry-and-repair-plan"></a>
<a id="user-authorized-repair-observations"></a>
<a id="completed-independent-cloud-verification"></a>

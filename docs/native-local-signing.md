# Local signing and recovery tooling

The local tools provide separate authority and signer GPG homes on one OS/user.
This is role separation, not protection from compromise of that workstation and
not an offline-primary claim. Preserve the existing signing identity; do not
create a replacement merely to repair CI. Hosts, secret locations and provisioning
receipts belong in private operational records.

The approved [tagged release](native-tagged-release.md) may import only the
protected signing subkey into its protected CI environment. The secret primary,
revocation certificate and recovery archive stay outside CI and public storage.
Local build/publisher commands receive neither private home. Key possession and
a supplied source SHA do not authorize a build or signature request.

The existing initializer uses a certification-only Ed25519 primary and one
signing-only subkey (five-year primary, one-year subkey validity).
Extension/rotation needs operator
approval; preserve pinned authority and independently approved signing requests.
Seven-day APT metadata renewal remains a separate production obligation.

## Frozen initialization contract

No ambient ~/.gnupg, environment secret, secret CLI option, loopback or generated
password. Production initialize uses normal system Pinentry for the user's
passphrase. Initialization is a separate explicitly authorized operation; the
operator enters their own password locally. Prepare/test all commands first.
Default CLI state is ~/.rquickshare-signing; allow an explicit absolute
canonical state path, never an existing directory, symlink or repository child.
Parent must exist, be owned by the caller and not group/world writable. Root and
GPG homes0700, state/config/temporary secret files0600, no overwrite/resume magic.

Fresh agent config enforces a16-character passphrase, disables loopback and
external password caching, and bounds ordinary agent cache to60s/default300s/max.
These are local key-cache policy, not scheduling sleeps. Refuse unprotected keys
using agent KEYINFO protection status, not assumptions about Pinentry dialogs.

Generate certification-only primary, check exact public fingerprint/keygrip/roles,
add one signing-only subkey, export public keyring, export protected subkeys to a
private temporary file and import into signer. Remove temporary export in finally.
Validate KEYINFO: authority has exactly the two protected local disk keys, signer
has exactly the protected signing subkey and no usable primary. Validate public
keyring identity and write public metadata only after successful checks. Kill only
agents started in this command's exclusively new/owned GPG homes before returning;
never stop ambient/user/other-agent GPG processes. Inspection must not start/kill
an agent or print private paths/key material/arbitrary tool output.

On failure retain the newly created private directory for recovery rather than
silently deleting generated identity material. No ready manifest on partial
initialization; refuse existing output on retry. Operator can inspect/recover or
explicitly discard the failed profile after review. Do not start another key
identity automatically. No directory rename while GPG agents own socket paths.

## Test inventory before tests

Public key listing parser: exact cert-only primary + one sign-only subkey, public
fingerprints/keygrips/algorithm/expiry; missing/duplicate/malformed/extra roles fail.
KEYINFO parser: exact protected disk key sets; clear/unknown/token/unexpected keys,
invalid framing and missing successful completion fail. Path policy: existing/
symlink/relative/repository/noncanonical/insecure parent and unsafe ownership fail.
Public state/keyring consistency, permission validation and no-autostart inspection.
Actual GPG initialization with deterministic TEST ONLY Pinentry passphrase, protected
KEYINFO, no usable primary in signer, existing request+sign+gpgv snapshot acceptance,
owned agent cleanup and temporary-export cleanup. No production keys or fixtures
uploaded. Fixture Pinentry is an explicit test dependency, not a production CLI
password bypass. Failures/cancellation preserve private partial state with no
success manifest. Focused run once; rerun affected failures only.

## Primary references inspected

https://gnupg.org/documentation/manuals/gnupg/OpenPGP-Key-Management.html
https://www.gnupg.org/documentation/manuals/gnupg/Agent-Options.html
https://gnupg.org/documentation/manuals/gnupg/Unattended-GPG-key-generation.html
https://lists.gnupg.org/pipermail/gnupg-devel/2015-January/029303.html
Installed GnuPG2.4.8/options and an owned temporary agent's KEYINFO help are checked
before relying on current command/protocol details. Use current official primary
sources and local protocol help, not third-party recipes.

Use scripts/native_signer_local.py and scripts/native_signer.py with their
explicit reviewed inputs. No initialization, private-key inspection, password
entry or export occurs by reading this document. See [encrypted recovery](native-signing-backup.md).

[Historical provisioning receipt](https://github.com/morozhenko-apps/rquickshare/blob/ed0a626807711008e6ed4ddeb8bdcef564f58d59/docs/native-local-signing.md) remains in Git; local locations,
operator account details and obsolete pending-release notes are not current setup instructions.

<!-- Historical heading anchors retained for existing links. -->
<a id="native-release-signing-on-the-existing-linux-workstation"></a>
<a id="accepted-boundary-and-limitations"></a>
<a id="milestones-before-code"></a>
<a id="measured-verification-and-local-operator-handoff"></a>
<a id="production-initialization-preflight-correction-2026-10-14"></a>
<a id="actual-production-profile-initialization-2026-10-14"></a>

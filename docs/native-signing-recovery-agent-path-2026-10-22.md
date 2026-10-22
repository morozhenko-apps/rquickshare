# MODE B — GnuPG recovery test failure under nested temporary homes

Date: 2026-10-22. Primary **MODE B / root cause and deterministic test fix**, not a release security redesign. Existing key custody, encrypted backup, signer-only private material, subkey, PIN entry and verification invariants **remain unchanged**. No production GPG identities, R2 objects, Git tags or master changes.

## Reproduction and candidate causes (R1–R4)

- **Expected**: Native package smoke on Ubuntu 26.04 runs all isolated Python signing tests, then builds and verifies release-profile `.deb`.
- **Actual**: [Native Linux package smoke 39674598404](https://github.com/morozhenko-apps/rquickshare/actions/runs/39674598404) ran 783 Core tests successfully and most private-signing fixtures; `test_native_signer_backup.SigningBackupTests.test_given_actual_encrypted_backup_when_verified_and_restored_then_protected_keys_signature_and_revocation_work` failed before packaging. `native_signer_backup.restore_values` attempted to import a **test-only** public keyring into `/__w/_temp/rqs-backup-test-.../source/.rqs-recovery-check-.../custody/authority`; GPG exited 2. `Gpg.run` hides stderr inside `CalledProcessError`. This is **not** evidence of an operator signing-subkey or publishing failure.
- Considered: (1) AF_UNIX socket pathname length (primary); (2) GPG agent unavailable (secondary); (3) pinentry failure; (4) GPG test key packet corruption; (5) test temporary folder ownership/permissions; (6) GPG binary absent; (7) resource exhaustion. The initial shorter `TMPDIR` change solved six global GPG setup failures, confirming sensitivity to path layout, but left this *nested* recovery path.
- Minimal independent local diagnosis (GnuPG 2.4.7) using **new disposable test-only keys**, with no user/project identities: GPG key generation under private homes at 75 and 84 UTF-8 bytes of pathname succeeded; at 89 bytes failed with `agent_genkey failed: No agent running`. The relevant longest agent Unix socket path is `<home>/S.gpg-agent.browser`; AF_UNIX path buffer is 108 bytes including NUL, so its pathname must fit **107 bytes**. The CI failed directory used 89-byte homedir; `S.gpg-agent.browser` would be 109 bytes. Thus the nested temporary directory layout exceeds the kernel socket limit. Native Ubuntu 26.04 uses GnuPG 2.4.8; exact stderr wasn't exposed by wrapper but the deterministic cause is consistent with its observed exit code and directory length.
- Some later recovery checks nest `.rqs-backup-crypto-...` inside `.rqs-recovery-proof-...` and can hit the same limit even after shortening the first profile path.

## Invariants and chosen root-cause correction (no STOP FACTOR)

- **Bound Unix agent sockets before creating test/recovered GPG homes**, as a pure path-length guard, rather than hide failure behind retries, sleep or disabled test cases. Fail with an explicit error on an unsafe deep user-selected output, without writing secrets or claiming successful recovery.
- Use shorter **same-parent, same-private-permissions** temporary prefixes and eliminate a redundant `/custody` level in disposable verification. Keep all temporary recovery data under the caller-owned private source parent; do **not** move it to the global `/tmp` or widen permissions.
- Keep the test-only revocation/agent verification in a private sibling crypto-home rather than nested inside the proof directory. Same caller-owned parent, same cleanup and independent key material; no leaking PINs/keys into logs.
- This is a security-preserving path/layout correction, not a new key-management format, R2/signature flow or trust boundary. Preserve the exact real GPG signing+gpgv and revocation checks.

## Per-file test inventory / negative matrix

| Owner | Positive | Negative/security | Planned evidence |
| --- | --- | --- | --- |
| `scripts/native_signer_backup.py` | Recovery from test-only encrypted archive signs with correct subkey; pubkey/revocation exact | Deep restored GPG profile rejected **before mkdir**; no partial `identity.json`; no leftover temp directory | Existing full encrypted-backup integration, added boundary regression |
| `scripts/tests/test_native_signer_backup.py` | Unchanged actual GPG/GPGv path tests; agent closed; original source and bytes preserved | Validate 107-byte socket boundary and 108-byte rejection; malicious/expired/corrupt archive still denied, retry semantics unchanged | Pure path-boundary test plus existing integration |
| `scripts/native_signer_local.py` (unchanged) | Actual `Gpg.run` still owns existing command invocation | No fallback to ambient GNUPGHOME, no exporting primary private material | Existing local signer tests |
| `.github/workflows/native-package-smoke.yml` (unchanged) | Package smoke remains manual-only, GPG fixtures use short `$RUNNER_TEMP` | Do not weaken dev CI policy, no secrets or production publication | Deliberate manual one-time `workflow_dispatch` after final change |

Applicable N1 invalid/deep path; N2 socket-size limit and Unicode byte lengths; N3 unavailable GPG agent; N4 explicit retry/clean temp; N5 foreign agent nonownership; N6 stale/changed backup data; N7 private source custody and temp perms; N8 N/A except GPG validity expiry already tested; N9 cancellation/cleanup; N10 secret data never logged; N11 N/A (no billing); N12 corrupted bytes/partial recovery refusal. The root-cause regression should catch a mutation that increases socket pathname beyond the 107-byte Linux limit.

## Completion gate

1. [x] Establish concrete CI failure, root-cause candidates and independent Unix socket reproduction.
2. [ ] Code-only atomic correction of disposable recovery layout and explicit GPG agent-socket bound; add targeted tests. No docs in this commit.
3. [ ] Confirm fast Preflight source CI on the exact new code SHA and deliberate manually triggered native package smoke from **dev**, after the candidate is final. Source validation is not release Mode B.
4. [ ] Record actual run/artefact hashes and outstanding KDE/Pixel and full tag Mode B gates; issue #14 status.

**Rollback:** revert only the recovery layout/bound implementation if actual new failures surface; preserve existing production signing identity, APT archive and immutable tag namespace.

# Automated tagged release and independent package versioning

2026-10-18. Mode A, user selected option B: GitHub Actions signs and publishes
after an immutable annotated `native/vMAJOR.MINOR.PATCH` tag. This supersedes the earlier
operator-only signing selection **for public release CI**, while preserving the
isolated local signer/recovery tools as an optional disaster-recovery path.
This is a new explicitly approved security trust boundary; no release key,
secret, bucket object or tag is created by this source task.

## Frozen public contract

- Product: **rQuickShare**. Targets Linux, Windows and macOS in the roadmap;
  only a physically accepted OS/architecture gets release artifacts.
- Selected first public early release: Native product `0.9.0`, Git tag **`native/v0.9.0`** (not a sandbox). Later stable target is independent product `1.0.0`, Git tag `native/v1.0.0`. Preserve old upstream `v0.9.0` and `v0.11.5` refs without reuse. Future Native tags use `native/vMAJOR.MINOR.PATCH`.
- Native Debian package: `rquickshare-native`; version `0.9.0-1`;
  source binary remains `/usr/bin/rquickshare`. Crate `rqs_lib` and
  historical Tauri remain independently versioned.
- Keep private R2 bucket `rquickshare-native-apt`, read-only Worker route
  `https://download.morozhenko.com/dist/rQuickShare/`, and APT suite
  `rquickshare-native-ubuntu26.04`. Same immutable pool .deb supports
  direct installation and signed APT. Current ABI only; no invented platforms.
- No automatic installation or APT source enrollment on a user's machine.
  Original `rquickshare` cannot coexist at the same filesystem paths.
  Debian `Conflicts` + `Replaces` and exact installed migration tests
  are mandatory before first production promotion.

## Automated release trust boundary

1. A tag push starts the native release workflow. Validate **exact tag
   syntax**, match native product version, and require tag commit equals the
   current protected `master` HEAD; fail closed for dev/untrusted refs.
2. Validate the exact tag and protected master HEAD **before spending build time**.
   Invoke the existing strict complete Mode B 10x as a reusable workflow on
   **that tagged source**, requiring its successful job result as an actual
   dependency, not a historical/API-provided success badge. Then build and
   verify the Ubuntu 26.04 ABI without secrets and stage immutable package/
   unsigned APT bytes as GitHub artifacts. Do not invoke arbitrary workflow
   code from PRs.
3. The isolated `apt-release` GitHub Environment holds only the
   *signing subkey* exported without the certifying/primary secret, its
   passphrase, pinned public keyring and bucket-scoped R2 credentials.
   GitHub CI can now sign, so compromise of protected tag/master/workflow
   or secrets could forge trusted releases; this is an explicit residual
   risk of user-selected unattended option B. Never upload a private
   primary key or recovery archive to GitHub or R2.
4. In the protected job, import the encrypted subkey into a fresh
   temporary `GNUPGHOME`. Validate pinned primary/public keyring;
   refuse extra secret primary keys. Use loopback passphrase from a
   private temporary file (never command arguments/output).
5. For existing channels, fetch and independently verify the prior
   published authenticated snapshot from private R2 before signing,
   to enforce monotonic versions and same-version identical package bytes.
   Never infer an empty channel from a network/auth error.
6. Sign `InRelease`/Release.gpg, verify the entire package/index hash
   chain and exact commit. Publish to R2 through the existing
   conditional immutable-object admission and last-step CAS `InRelease`.
   No overwrite/delete/unauthenticated fallback on publication failure.
7. Publish corresponding GitHub release only after R2 publication and
   public HTTPS/isolated APT checks succeed. A GitHub tag by itself does
   not prove a package was publicly released.
8. Signed APT index metadata expires in seven days. Provide an
   independently authenticated refresh path that preserves package bytes
   and prior archive authority, with explicit failure monitoring. A
   new application version is **not** required to refresh indexes.

Secrets are not available in the repository or to this agent. Their
registration, access-scoped environment rules, subkey-only export and
end-to-end R2 acceptance require the repository operator. Do not
print or ask the user to paste secrets in chat.

## Operator wiring map: RepoTicker comparison and two installation channels

Updated 2026-10-22 after owner selected **real public native/v0.9.0** in the production `rquickshare-native-apt` bucket. All values below belong in **this repository's** GitHub Settings → Environments → `apt-release`. An environment with the same name in `morozhenko-apps/repoticker` is *not shared* by GitHub.

| GitHub location | Exact name | Source | Privilege/usage |
| --- | --- | --- | --- |
| Environment variable | `RQS_R2_ACCOUNT_ID` | Same non-secret Cloudflare account ID as RepoTicker's `REPOTICKER_R2_ACCOUNT_ID`; already present in `infra/native-download/wrangler.jsonc` | R2 SDK account endpoint |
| Environment secret | `RQS_R2_ACCESS_KEY_ID` | New R2 S3 API token, Access Key ID | Object access scoped to `rquickshare-native-apt`, not RepoTicker's `app-dist` |
| Environment secret | `RQS_R2_SECRET_ACCESS_KEY` | Secret Access Key from that same new bucket-scoped S3 token | Conditional read/write existing signed snapshot, pool/by-hash and `InRelease` |
| Environment secret | `RQS_APT_PUBLIC_KEYRING_BASE64` | Base64 of existing **public** `scripts/keyrings/rquickshare-native-archive.gpg` | Pinned primary identity; never use RepoTicker archive keyring |
| Environment secret | `RQS_APT_SECRET_SUBKEY_BASE64` | Base64 of already existing **signer-only** protected rQuickShare subkey, not primary/certifying key | CI GPG signing, strictly no usable primary key |
| Environment secret | `RQS_APT_SUBKEY_PASSPHRASE` | Passphrase belonging to the rQuickShare signer-only subkey | Do not substitute backup/recovery or RepoTicker passphrase |

Use only the already-approved rQuickShare public primary fingerprint `6D8854762CF0C86881138A56695A0888D09FA616` (workflow constant). Owner must confirm effective branch/tag rules, `apt-release` deployment policy, secret **presence** and least-privilege access before creating the immutable real public tag. Never print/export secret contents to issue, chat, CI logs or Git.

**Worker deploy ownership correction (2026-10-21).** The production Worker `rquickshare-download` is connected to GitHub and has a previously successful Cloudflare Git Build on 2026-10-18 (source/receipt in [native-download-git-build-2026-10-18.md](native-download-git-build-2026-10-18.md)). On user confirmation of this workflow, source commit `8a5d6650` removed the redundant release-time `wrangler deploy` and `RQS_CLOUDFLARE_API_TOKEN` secret requirement. Only R2 object publisher secrets remain. Git Build deployment is not itself a signed archive upload, so both independent owners stay: Cloudflare for read-only Worker code, GitHub Actions for signed R2/APT bytes. Final public HTTPS/isolated APT verification still fails closed if the Worker route is wrong.

**Existing signing identity — DO NOT GENERATE A SECOND KEY:** the owner created the rQuickShare primary on 2026-10-14, fingerprint `6D8854762CF0C86881138A56695A0888D09FA616`, signing subkey `1F5150D98D94C43109FBA2503E7DE744C1014357`. Profile is `~/.rquickshare-signing/signer` (primary secret stub), and public keyring at `scripts/keyrings/rquickshare-native-archive.gpg`. Production evidence in [native-local-signing.md](native-local-signing.md). Reinspect that exact local identity with `python3 scripts/native_signer_local.py inspect` before any export. Owner-selected private CI handoff directory `/home/morozhenko/morozhenko/secrets/rquickshare/` is **outside this repository**; only the operator can create/review that host directory. Reuse the *existing signing passphrase*, not a newly chosen one. Export only signer-only protected subkey packets, verify imported test GPG profile has stub primary and exactly one signing subkey using `scripts/native_ci_signing_key.py`; stage base64 and plaintext passphrase files mode 0600 in a 0700 directory, never in Git/backup sync by default. Set three secrets directly in `apt-release` GitHub Environment using the exact names; do not paste values into chat or job logs.

**Two distribution methods use identical R2 bytes after a successful production tag run:**

| User path | Published location / expected asset | Behavior |
| --- | --- | --- |
| Direct public HTTPS | `https://download.morozhenko.com/dist/rQuickShare/pool/main/r/rquickshare-native/rquickshare-native_0.9.0-1_amd64.deb` | Single versioned direct .deb, served by read-only Worker from private R2; not live until release completed |
| Public GitHub Release | `https://github.com/morozhenko-apps/rquickshare/releases/tag/native/v0.9.0` | Same verified .deb, .sha256 and public keyring/.sources after R2/HTTPS acceptance; not live until release completed |
| Signed APT | `https://download.morozhenko.com/dist/rQuickShare/`, suite `rquickshare-native-ubuntu26.04`, component `main` | Owner's clients explicitly enroll a public keyring and generated `rquickshare-native.sources`; then normal `apt update` / `apt install rquickshare-native` / `apt upgrade` |

APT signing/verification and direct download are distinct UX, **not** separate storage buckets or two builds. Never make private R2 bucket public or enable `r2.dev`: Worker exposes only allowlisted package/by-hash/`InRelease` paths. No `latest.deb` endpoint currently exists. Direct .deb install alone does not enroll signed APT, and historical installed `1.0.0-*.debug` sorts **above** public `0.9.0-1` so it cannot automatically upgrade/downgrade to this earlier product version.

## Public download and APT enrollment contract

GitHub Releases publishes one immutable verified `.deb` and its `.sha256`
alongside the **public**, non-secret `rquickshare-native-archive-keyring.gpg`,
`rquickshare-native.sources` and `FINGERPRINT` from the exact signed
snapshot's verified `native_apt.py bootstrap` output. The separate
`rquickshare-native.sources` references
`/etc/apt/keyrings/rquickshare-native-archive-keyring.gpg`, the pinned
primary fingerprint, only the selected suite, and By-Hash: force.
A direct .deb install does not enroll APT. Users explicitly install the
keyring and .sources before `apt update`/`apt install rquickshare-native`.
These public assets are not a substitute for independently validating the
signed archive, never include a secret key, and must not be published before
the R2 signed archive and isolated client acquisition both succeed.

## Operator setup for unattended tag publication

These steps are deliberately **not** run by repository CI and must be
performed with an authenticated maintainer's GitHub/Cloudflare access.
Never print a private key, passphrase, R2 secret, backup plaintext, or
encrypted recovery archive into a GitHub issue, chat or job log.

1. Under repository **Settings → Environments**, use existing `apt-release`.
   Restrict deployments to protected `master` and protected **`native/v*`**
   according to the repository environment deployment policy. The operator
   deliberately configured a **Required reviewer** for the first public
   `native/v0.9.0` release (observed 2026-10-21), in addition to master/tag Rulesets.
   This means release signing is **not fully unattended**: the protected
   `publish` job waits for manual approval before accessing signing secrets.
   Scheduled APT metadata renewal also uses `apt-release`, and may therefore
   wait for the same review; ensure an accountable review cadence before the
   seven-day signed metadata expiry. Remove the reviewer only after a separate
   explicit owner decision to enable unattended signing and refresh. Keep
   rulesets and restricted workflow/environment edits in either case.
2. Set environment **variable** `RQS_R2_ACCOUNT_ID` to the exact
   32-character Cloudflare account ID. This is a variable, not a password.
3. Add the following exact environment **secrets**:

   | Secret | Contents |
   | --- | --- |
   | `RQS_APT_PUBLIC_KEYRING_BASE64` | Base64 of the existing checked public `scripts/keyrings/rquickshare-native-archive.gpg` |
   | `RQS_APT_SECRET_SUBKEY_BASE64` | Base64 of a freshly exported **protected signing-only subkey** from the existing signer GPG home; no usable private primary |
   | `RQS_APT_SUBKEY_PASSPHRASE` | The existing signing-only subkey passphrase, not a backup/archive passphrase |
   | `RQS_R2_ACCESS_KEY_ID` | Access key belonging solely to the dedicated `rquickshare-native-apt` R2 bucket |
   | `RQS_R2_SECRET_ACCESS_KEY` | Corresponding scoped R2 secret for conditional object reads/writes |
   
   Do not reuse Picturix's `app-dist` publisher credential and never
   upload authority/master/revocation private packets. In CI a new
   temporary GPG home enforces the pinned primary and signing subkey
   fingerprints, sole secret subkey with no usable primary, private
   passphrase-file permissions and actual signed-index verification.
4. Provision via GitHub Settings secret fields or `gh secret set` with
   **standard input**, never CLI positional secret values. The public
   keyring can be encoded locally; the private subkey export must be from
   the isolated protected `signer` profile, *not* the `authority` profile.
   Confirm `gpg --list-secret-keys` shows the stub primary (`sec#`) and
   signing-only (`ssb`) **before** uploading. Do not upload secret values
   or a keyring path to this repository.
5. Check the configured environment, tag protection and scoped R2 token before
   authorizing any production tag. CI must fail cleanly if a secret or permissions
   are missing. **Cloudflare Git Builds owns deployment of the already connected
   `rquickshare-download` Worker from protected master.** Verify the exact
   Cloudflare build and route are healthy before publication; GitHub Actions must
   not execute a second `wrangler deploy` or require a Worker API token.
6. For the selected first public `v0.9.0`, obtain verified real Android/Linux,
   Debian install/migration/rollback and exact-source complete Mode B
   evidence. Fast-forward verified `dev` to protected `master`, then
   create exactly one new annotated `vMAJOR.MINOR.PATCH` tag on
   `master`. Tag push performs verification, complete 10x Mode B,
   native compilation, protected signature, conditional R2 promotion,
   public HTTPS/isolated APT acquisition and finally GitHub Release
   publication. Never move or recycle a published tag.
7. After release, ensure both distribution options are documented:
   direct versioned .deb download (no automatic APT enrollment) and
   explicit installation of the attached **public** archive keyring and
   `.sources` into the appropriate `/etc/apt` paths before
   `apt update` / `apt install rquickshare-native`. Signed metadata
   refresh runs on the default branch at 08:17 UTC on every third
   day-of-month, including months with uneven lengths. Its results must
   be monitored and renewed before the seven-day validity window
   expires; a successful schedule is not guaranteed delivery.

Failure boundaries: tag validation/Mode B/build errors must leave R2
untouched. If Worker deployment or any previous-snapshot/signing/R2
publication step fails, do not create GitHub Release. The R2 publisher
never unconditionally overwrites `InRelease`: partial immutable objects
may remain privately but cannot authorize an unverified channel. If
public readback fails after a successful R2 commit, treat the result as
an operational incident and diagnose the public route; never roll back
by overwriting immutable package bytes or moving tags.

## Stages and rollback

| Stage | Planned change | Gate / rollback |
| --- | --- | --- |
| S0 | Freeze this architecture/identity contract | Docs-only commit |
| S1 | Decouple native SemVer from `rqs_lib`, package/name/version metadata and migrated package ownership | Tests for valid/invalid Debian versions, dpkg comparisons, conflicting files |
| S2 | Align APT producer/verifier/bootstrap and Worker allowlist with new immutable pool identity | Existing real signing/APT/Worker tests, negative path grammar and hash checks |
| S3 | Create tag-gated workflow, protected CI subkey/bootstrap, authenticated previous-snapshot path, online publisher and metadata refresh | Offline CI-contract tests; no secrets during ordinary preflight |
| S4 | Verify all affected tests/CI and update SSOT/handoff | Exact-source evidence, atomic commits |
| S5 | Operator provisions GitHub Environment and secrets, verifies signed test publication and rollback, and manually authorizes the first public tag | **Not performed in this source task** |

Explicit non-goals: full Google Quick Share parity, macOS/Windows ports,
new desktop UX, new R2 bucket/route, automatic enrollment, publishing
`v0.9.0` before its verified release gates, replacing the installed package, or modifying active
signing/recovery material. Existing transfers and Linux settings remain
untouched. Scope is reversible per atomic commit until public signed
artifacts or a new package identity are installed/published.

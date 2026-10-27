> Current distribution contract: [flat R2 distribution](FLAT_DISTRIBUTION.md).
> Historical Worker, nested archive and full remote snapshot instructions
> below are superseded. The owner approved a fresh flat channel without
> a legacy migration; production cutover has not been performed.

# rQuickShare product identity, platform direction and release distribution

> **Current release readiness, native tag namespace and blockers:** [Active backlog and release handoff](active-backlog.md) and [issue #14](https://github.com/morozhenko-apps/rquickshare/issues/14). Dated `v0.9.0` suggestions elsewhere in this journal are **superseded** by the user-selected native tag `native/v0.9.0`; do not create or replace the upstream `v0.9.0` tag. No production publish is authorized by the strategy document.

Decision: 2026-10-18. Owner-approved product direction; documentation only.
This contract supersedes earlier Linux-only product-positioning language. It does
not supersede dated implementation evidence, user-approved short-term priorities,
security controls, or the requirement for physical device/release acceptance.

## Brand, independence and provenance

- The product and public-facing application name is **rQuickShare**. No
  platform qualifier ("for Linux"), "Native" sub-brand, or "Beta" suffix belongs
  in the public product name. Use an OS-specific file type, support matrix and
  installation instructions to communicate platform availability.
- This repository is an independently maintained **fork** of
  [Martichou/rquickshare](https://github.com/Martichou/rquickshare); it is
  neither endorsed by nor an official update from that project. Preserve the
  original authors' copyright and license notices and keep a plainly visible
  upstream link, including in release/about information where applicable.
- Source and binaries remain subject to GPL-3.0 as applicable. On distribution,
  preserve license/notices, identify modified versions, provide the corresponding
  source and required installation information where applicable, and check
  non-code artwork/branding and third-party licenses separately. Do not
  represent this fork as an upstream-authorized release.
- Upstream permission or notification is not a release prerequisite under the
  GPLv3. An informational comment in an appropriate existing upstream discussion
  is optional, not a request for collaboration, a promise to review external
  PRs, or an obligation to merge back. Reports are welcome in this fork's Issues
  under the established bug-report template.
- GitHub remains `morozhenko-apps/rquickshare`; this decision does not rename the
  repository, binaries, desktop/D-Bus identifiers, services, or user settings.

## Long-term compatibility goal and target platforms

**Full feature parity and interoperability with Google Quick Share** are the
long-term product goals. The full capability inventory remains
[full-feature-roadmap.md](full-feature-roadmap.md), including account/contact
identity, Google-compatible QR, resume/stream payloads, platform-specific
transports and applicable Apple interoperability. Neither modeled protocol
fields nor automated tests imply demonstrated Google compatibility.

Target desktop platforms are **Linux, Windows and macOS**. Current
implementation/release acceptance is platform-specific:

| Platform | Present basis | Remaining work before announcing support |
| --- | --- | --- |
| Linux | Rust sharing core, Linux daemon/D-Bus, GTK4/Layer Shell tray, systemd user services, native Debian packaging and limited installed Pixel roundtrip | Final real-peer scenarios, installed lifecycle, named ABI/distro checks and signed distribution |
| Windows | Portable pieces of the Rust protocol/core and retained cross-platform historical code; Google Quick Share for Windows is also an interoperability **peer** | Dedicated Windows discovery/Bluetooth, system service/IPC, GUI/tray, filesystem integration, installer, signing and real-device acceptance |
| macOS | Retained legacy Tauri/macOS project, **not** the new native GTK client | Native platform service/transport/IPC/GUI integration, packaging/signing/notarization where needed, real-device acceptance; investigate applicable Apple transport separately |

Linux-specific `cfg(target_os = "linux")`, BlueZ, D-Bus, systemd, GTK
Layer Shell and portal code must not be described as portable by default.
Preserve/share OS-independent protocol, security and session logic; add
platform adapters deliberately. Do not assume the current .deb builds or
runs on Windows/macOS, or that the retained macOS Tauri build offers the new
native feature set. Mac/Windows ports are independent deliverables and do
not block the first accepted Linux package.

## Independent releases and versioning

- **`native/v0.9.0`** is the selected **first public early Native** source tag, built from independent product SemVer **`0.9.0`** and Debian **`0.9.0-1`**. Future Native Git tags use `native/vMAJOR.MINOR.PATCH` (including later `native/v1.0.0`); original upstream `vMAJOR.MINOR.PATCH` Git tags remain historical and are never moved. This is a real signed production R2/APT release channel, **not** staging, and `0.9.0` remains the early public Linux product version.
- SemVer measures the stability/compatibility contract of the released
  product, **not the percentage of Google's full roadmap completed**.
  Announce only verified features/platforms; full parity is not a
  `v1.0.0` claim.
- Existing `0.11.5+nativeN` and `1.0.0-N.debug` artifacts are **historical internal test builds**; they do not establish a public release and are not the selected `v0.9.0` artifact. The library/core and retained Tauri version identifiers remain separate from the native product.
- Selected planned Debian package identity: `rquickshare-native`, to avoid
  APT conflating the fork with upstream `rquickshare`. A conventional
  first public candidate is `rquickshare-native_0.9.0-1_amd64.deb`
  for the verified Ubuntu 26.04 amd64 channel. Keep the CLI `rquickshare`
  only if upgrade/conflict tests prove its ownership safe. Internal Rust
  crate/library versions may be independently managed; do not mechanically
  rewrite the retained Tauri/macOS project to match the desktop product tag.
- The existing native builder, APT validator, Worker allowlist and desktop assets already support the independent `rquickshare-native` Debian identity and stable three-part SemVer. The newly selected product version needs a matching native Cargo/lockfile revision, regression checks and release-candidate validation; do not relabel old 1.0.0 debug bytes.
- Do not publish early public `native/v0.9.0` or announce stable `native/v1.0.0` on source
  coverage alone. Release acceptance is based on the actual supported-platform
  ordinary sharing, refusal/cancel/retry, safety, install/upgrade/removal,
  automated Mode B and signed-download gates in
  [native-essential-acceptance.md](native-essential-acceptance.md) and
  [current-state.md](current-state.md).

## Direct .deb download and signed APT are two views of one R2 repository

The already selected HTTPS root is
`https://download.morozhenko.com/dist/rQuickShare/` and the dedicated
private bucket is `rquickshare-native-apt`. The deployed read-only Worker
routes only this path to the dedicated bucket; Picturix remains served from
its separate `app-dist` ownership. Keep the existing route and bucket;
a name/version refactor must not silently repoint DNS or alter Picturix.

```text
verified release build (.deb + hashes)
           |
approved source/artifact and signed APT snapshot
           |
immutable R2 pool/ + by-hash/ objects; publish signed InRelease last
           |
https://download.morozhenko.com/dist/rQuickShare/
         /                                      \
versioned direct .deb link              signed APT index + same .deb
manual user installation               enrolled APT clients and OS updates
```

R2 **stores bytes**, APT **resolves, authenticates and updates packages**.
Both distribution paths may coexist and reference exactly the same immutable
`.deb`; do not upload different packages under the same version. The current
Worker admits only the narrow `InRelease`, by-hash and versioned pool
paths; it does **not** yet provide a friendly `latest.deb` endpoint,
a general public object browser or an auto-install action.

- Direct file download does **not** automatically add an APT source or signing
  key, and a bare `apt install rquickshare-native` cannot find this custom
  repository until the user has explicitly enrolled its signed source.
- APT uses independently approved metadata (`InRelease`, SHA256 package/index
  hashes, `Signed-By` pinned public signing key, the existing Ubuntu 26.04
  suite). The metadata expires after seven days; maintenance and re-signing
  before expiration remain an operational release requirement.
- The planned package name differs from the original, but both versions
  currently claim overlapping executable/service/desktop paths. Do **not**
  promise side-by-side installation. Specify/test exact `Conflicts` and
  migration/rollback behavior, settings preservation, source pinning and
  uninstall semantics before releasing; adding `Provides`/`Replaces` is
  not an automatic substitute for ownership review.
- Once approved, publish immutable package/index objects before the single
  authenticated mutable `InRelease` pointer, then verify public HTTPS
  download, actual isolated APT acquisition, wrong-key rejection and no
  impact on Picturix. The R2 publisher/signing paths remain separate from
  untrusted CI. No real production release or R2 package upload is claimed
  by this document.

Technical contracts: [native-apt.md](native-apt.md),
[native-r2.md](native-r2.md),
[native-download-route.md](native-download-route.md),
[native-local-signing.md](native-local-signing.md).

## Automated release implementation decision

The user selected **GitHub Actions auto-sign and R2 publication on validated Git tags** (option B), with an isolated signing subkey in protected GitHub Secrets. See [native-tagged-release.md](native-tagged-release.md) for the approved implementation plan and security boundary. The older manual-signing handoff remains available for recovery; production CI still requires a separately verified first release and secret provisioning.

## Release readiness checkpoint — 2026-10-20

The owner requested a prioritized remaining-work list and a safe rehearsal of GitHub Actions → Cloudflare R2 ahead of the intended first stable `v1.0.0`. Current green revision-2 CI and the verified local debug package do not yet satisfy real-device/installed final acceptance. The release pipeline is a production signer/publisher and must not be exercised with an arbitrary early stable tag as a dry run.

The complete current **readiness list, source/hash/CI evidence, staging-vs-production architectural choice and concrete release rehearsal gates** are in [release-readiness-r2-rehearsal-2026-10-20.md](release-readiness-r2-rehearsal-2026-10-20.md) and [issue #14](https://github.com/morozhenko-apps/rquickshare/issues/14). The recommended `v1.0.0-rc.1` staging tag is a **proposal**, not an existing tag or a released package. This checkpoint does not authorize new secrets, buckets, tags, public R2 writes, master promotion or the publication of `v1.0.0`.

## Next implementation handoff and non-goals of this docs change

1. Finish the currently running **strict complete Mode B ten-pass** gate and
   retain its exact-source verdict; do not cancel it with an unrelated commit
   before results are recorded.
2. Record a version/name migration design, repository/file-owner inventory,
   exact old/new version ordering, conflicts and rollback. Separate product
   SemVer from Rust core and historical Tauri packaging as needed.
3. Update the native .deb producer/verifier, APT index/verifier/bootstrap,
   Worker allowlist, UI About/desktop display name and their regression
   fixtures together. Keep stable service paths until an accepted migration
   explicitly requires changing them.
4. Verify Linux real-device essential cases and installed package lifecycle.
   Independently assess Windows/macOS port architecture and each platform's
   release/CI requirements; never relabel a Linux artifact as portable.
5. With explicit release approval, check live bucket/Worker state, controlled
   R2 write/readback/denial, signer identity/recovery and metadata refresh,
   then release signed APT plus a versioned direct .deb from the same bytes.
   Only after verification, publish the selected public `native/v0.9.0` with source/attribution/release
   notes and a truthful supported-platform matrix; retain `v1.0.0` as a later stable target.

This task is **documentation only**: no code, version, branch promotion,
signing, key handling, R2 writes, live installation or release publication.
Rollback is one isolated documentation commit. Old dated engineering receipts
remain historical, even when they mention a now-superseded platform strategy.

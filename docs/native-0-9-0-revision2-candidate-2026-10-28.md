# Native 0.9.0-2: unshipped Debian test candidate

## Repository scan / evidence

- Source baseline c4ded8ccc43e0ec5d7aab874700e000c41877da8: Preflight 40764210352 SUCCESS; Native GTK 40764209968 SUCCESS.
- native_desktop/Cargo.toml and Cargo.lock select 0.9.0. packaging/release.toml still selects revision 1.
- docs/native-essential-acceptance.md records an operator-installed native 0.9.0-1 on KDE. Rebuilding new bytes with the same package identity cannot demonstrate an ordinary version upgrade.
- scripts/native_deb.py already provides bounded revision validation, package/asset verification and an offline smoke check. The native-package-smoke workflow is manual-only and uploads an unsigned verified .deb and SHA256.

## Decision and invariants (docs-first)

Advance Debian package revision 1 -> 2, retaining product SemVer 0.9.0, lockfile, signed distribution, application identity and services unchanged. This is a disposable development candidate, NOT a release, tag, signed APT upload or approval to change master. Do not modify the manual-only packaging workflow to trigger on dev pushes.
Update the existing exact-version Python contract test to require current candidate 0.9.0-2 and reject stale revision 1. Use real dpkg --compare-versions, not lexicographic ordering. The candidate must sort above 0.9.0-1 and 0.9.0-1.debug, yet below 0.9.0-2.debug, 0.9.0-3, 0.9.1-1, and historic 1.0.0 debug versions.
Do not update historic hardware/installation evidence referencing 0.9.0-1. No package is ever installed or a daemon launched from source tests. Do not overwrite a previously published package under an unchanged version; after any published 0.9.0-2, rollback requires a *higher* revision.

## Scoped Mode B risk/branch/test inventory

| Branch and risk | Positive | Negative / interaction |
| --- | --- | --- |
| Cargo/lock 0.9.0 + revision 2, P1 | Candidate version equals 0.9.0-2 | Old 1 must not pass current-revision assertions |
| Debian comparator at previous/current, P1 | old < current | No reinstall masquerading as an upgrade |
| Debug/current/future order, P1 | current < same-rev .debug, next revision and next SemVer | Guard against rollback or collision |
| Exact native metadata, P0 | Existing package validator accepts candidate | Existing malformed identity/ELF/deps checks still reject |
| Candidate artifact, P0 | Manual native-package-smoke validates staged package and checksum | No fake result without actual workflow artifact |

Positive, N1 malformed input, N2 numeric limits and N6 ordering/downgrade are applicable here. N3 network, N7 authorization, N8 Unicode payload, N9 race, N10 PII, N11 payment and N12 disk-failure behavior remain in their actual domain test suites, not in version arithmetic. Existing Python package fixture tests cover malicious archives, paths, modes, control assets and checksums.

Source change scope: release.toml and the focused test_native_deb.py identity test only. Expected named-test count change: zero; stronger assertions inside one existing method. Run normal Preflight once at final gate; Native GTK may also trigger for packaging path. Rust branch/line coverage is not measured by version tests. Mutation audit is reasoned only: changing revision back to 1 or reversing comparator order must fail test; no mutation tooling executed.

After source CI, use the existing manual native-package-smoke workflow on dev. The independent candidate is for Ubuntu 26.04 amd64 and does not certify installed KDE, real Android peer consent, interrupted transfer or BlueZ/mDNS operation. Download and hash-verify the produced .deb, install/upgrade with the package manager and record PASS/FAIL/UNTESTED for startup, tray, Dolphin, settings recovery, explicit consent, files and text. Keep peer identifiers and content out of public reports.

## Implemented and evidence boundary

Source c8d07e03acb33ab42779e8c1171805829b8c23b3 changed only native_desktop/packaging/release.toml (revision 2) and scripts/tests/test_native_deb.py (one existing named test extended for an upgrade identity). No runtime code or app SemVer, signed release, protected branch or deployment changed.
Confirmed with Debian dpkg version comparisons in the isolated ChatGPT Linux container: both 0.9.0-1 and 0.9.0-1.debug sort below 0.9.0-2; candidate sorts below 0.9.0-2.debug, 0.9.0-3, 0.9.1-1, 1.0.0-1.debug and 1.0.0-2. Exit status: 0 for each comparison. This is local arithmetic evidence only; not a full Python/Python-package or native binary build.
Mutational rationale: a returned revision 1 fails the exact revision and identity asserts; a reversed comparator fails the explicit previous/current/future pairs. No mutation runner executed. All other package security/disk failure gates remain owned by existing tests and candidate workflow.
Next gate: source-exact dev Preflight and Native GTK. Both must be green before running manually dispatched Native Linux package smoke for candidate 0.9.0-2. No installed KDE/Android acceptance, signed APT update or publication was performed in this change.

## Current candidate source CI and dispatch handoff

Exact source SHA c8d07e03acb33ab42779e8c1171805829b8c23b3 has completed both required source gates successfully:
- Native GTK verification: https://github.com/morozhenko-apps/rquickshare/actions/runs/40766951788 (completed/success).
- Preflight: https://github.com/morozhenko-apps/rquickshare/actions/runs/40766951861 (completed/success).

These checks validate source and tests, NOT the Debian package artifact. Packaging remains pending and no binary artifact was generated by the ordinary dev CI.

Manually dispatch the *existing* candidate-only workflow on ref dev, with no protected tag or release:

    gh workflow run native-package-smoke.yml --ref dev -R morozhenko-apps/rquickshare

GitHub UI equivalent: Actions -> Native Linux package smoke -> Run workflow -> branch dev. Its tested job builds native package using scripts/native_deb.py and uploads rquickshare-native-ubuntu-26.04 (including .deb and .sha256), plus explicitly unsigned staging metadata. Never describe that staging as a published trusted APT repository. Download files only after workflow completion and confirm exact version 0.9.0-2 and checksum before attempted KDE installation. It is not possible to prove package execution from source CI.

Version policy clarification: 0.9.0-2 is Debian *revision two* of the selected 0.9.0 product release, not product SemVer 0.9.2. Issue #17 still describes comprehensive 20-locale/native 1.0.0 context/RTL acceptance as a future final feature, after remaining product and interoperability work. Issue #13 still requires installed long-list request focus/scroll validation. Raising product version to 1.0.0 now would misrepresent unresolved release acceptance; an explicit product version decision must be separate from a disposable, source-green package candidate.

Static review of native_desktop/src/panel/rows.rs on this branch confirms that focus_transfer already schedules a one-shot frame callback, computes a keyed row's bounds and adjusts scrolling; focus_scroll_tests covers the interval solver. This addresses the earlier source-level omission, but the physically observed long-list issue is still OPEN pending actual KDE/Pixel validation. No duplicate speculative scroll implementation was added.

## Native package smoke 40772104257: Dolphin validation adapter mismatch

### Executed evidence and root cause

The first manually dispatched `native-package-smoke.yml` run [40772104257](https://github.com/morozhenko-apps/rquickshare/actions/runs/40772104257) completed **FAILURE** at **Core and package quality gate** on source/docs SHA `813dd71decf2ba99ddd3f1409d468566086af6ee`. Its log reports Core **902/902 unit tests passed** and completed native Python package/signature tests, then two `desktop-file-validate` errors on `share/kio/servicemenus/io.github.morozhenko_apps.RQuickShare.Send.desktop`: for `Type=Service`, the freedesktop validator rejects `MimeType` and `Actions` as Application-only keys. Subsequent GTK compilation, release `.deb` build, unsigned APT staging and artifact uploads were all **SKIPPED**. Therefore no new installable `0.9.0-2` artifact exists from that run. A test fixture log mentioning `1.0.0-1` is a synthetic Python test artifact, not the actual candidate identity.

KDE's own service-menu documentation (https://develop.kde.org/docs/apps/dolphin/service-menus/) explicitly requires `[Desktop Entry]`, `Type=Service`, `MimeType`, `Actions`, and `[Desktop Action ...]` for a Dolphin `kio/servicemenus/` definition. Our file correctly implements that KDE-specific format; the generic freedesktop validator is the wrong contract for this file. The ordinary application launcher `share/applications/*.desktop` **is** freedesktop `Type=Application` and must retain `desktop-file-validate`.

### Scoped MODE B inventory and decision before implementation

**Files:** `.github/workflows/native-package-smoke.yml` (CI adapter only), `scripts/tests/test_native_deb.py` (test matrix). Existing production checker `scripts/native_deb.py::validate_localized_desktop_entries` uses strict `ConfigParser`, exact two Dolphin section names and exact expected `Type=Service`, `MimeType=application/octet-stream;inode/directory;`, `Actions=Send;`, `X-KDE-Protocols=file`, URL bounds, `X-KDE-Priority=TopLevel`, `[Desktop Action Send]` and `Exec=rquickshare send -- %U`, 20 locale cases, direction/escape and duplicate-key rejection. The existing Python package suite executes that checker, but the workflow must explicitly select it rather than accidentally applying the generic freedesktop validator again.

**Solution:** retain the generic validator only for the real application launcher; invoke existing `native_deb.validate_assets()` as a separate fail-closed check of KDE service menu, launcher and session assets from the workflow. Do not rewrite the service type/fields; do not delete validation, relax the Python checker, use `|| true`, or skip test cases. Add test-only assertions for the dual-validator workflow routing and independent negative mutations for service type, MIME, action ID/command. One atomic workflow/test source commit follows this docs-only plan.

| Branch/mutation | Positive expected | Negative rejection |
| --- | --- | --- |
| Application launcher `Type=Application` | Generic `desktop-file-validate` PASS | Removing generic validator from workflow is a test failure |
| KDE service `Type=Service` | Strict `native_deb.validate_assets` PASS | `Type=Application`, changed `MimeType` or `Actions` fail closed |
| Dolphin click action | `[Desktop Action Send]`, exact `Exec ... -- %U` | `%F`, shell substitution, missing action/translation fail existing checks |
| Workflow tool selection | Exactly one freedesktop validator invocation, for `share/applications` only | Reintroduction of `desktop-file-validate` for `kio/servicemenus` rejected by test |

N1 malformed service fields, N2 URL bounds, N6 missing locale, N7 command/consent authority, N8 bidi/escapes and N10 path/body disclosure are meaningful existing negative areas. N3 physical network, N4 UI repeat, N5 lifecycle, N9 live races, N11 payments and N12 real disk faults are **not proven by this static metadata test** and remain in their owner suites. Successful content metadata validation cannot establish Dolphin runtime registration or the user's real KDE file-menu behavior.

**Mutation review:** changing `Type=Service` to `Application`, `Actions=Send;` or MIME selection must fail dedicated Python negative assertions. Replacing the correct CI checker with a generic freedesktop call must fail the workflow routing test. No mutation runner is locally available. Rollback only this workflow/test source commit if subsequent validation contradicts the plan; keep the KDE service file stable and do not authorize package publication.

**Gate:** normal Preflight on the changed `.github/workflows`/Python tests, then the manually dispatched Native Linux package smoke on `dev`. This environment's GitHub connector does not expose `workflow_dispatch`; a manual Actions launch is still required after source CI passes. Do not poll an active run, and do not claim package success before artifact verification. Master, tags, signing, production assets and installed services remain unchanged.

### Corrective source checkpoint

Commit `99cb0df77cf0076ed3f7ffe1eddde259b81582c0` changes **only** `.github/workflows/native-package-smoke.yml` and `scripts/tests/test_native_deb.py`. Generic `desktop-file-validate` remains mandatory for the `Type=Application` launcher, while the Dolphin `Type=Service` file is now explicitly checked by the existing fail-closed `native_deb.validate_assets()` and its existing 20-locale/value contracts. Three new negative mutations cover KDE Type, MIME set and `Actions` ID. One new named test verifies that the workflow selects the launcher-only freedesktop invocation and calls the project validator for service menus; existing mutations continue to check the exact file-selection command, locales, actions and unsafe fields.

No native executable, GTK behavior, KDE service file, package revision, product SemVer, consent or signing/release source changed. Positive and N1/N6/N7 negative rows have actual test implementations, but **not** a completed current-source execution until the new CI gate finishes. The package workflow must be manually re-dispatched on `dev` after ordinary CI is green, because it intentionally has no push trigger. Its failed first run cannot be rerun on the old SHA as if it tested this fix. The initial 902-case Core pass is unchanged historical evidence, not a claim of `0.9.0-2` artifact creation.

Scope-local per-file map: `native-package-smoke.yml` owns tool selection and failure propagation; `test_native_deb.py` asserts workflow selection and rejects Type/MIME/Actions mutations; `native_deb.py` keeps exact asset/command validation unchanged. Static mutation review predicts each wrong Type/MIME/Actions change fails the exact-key base validator, while reintroducing the generic Dolphin validator fails the new workflow-selection test. No mutator or real Dolphin GUI executed. Overall Mode B Stage 12 and physical package/Android acceptance remain **OPEN**.

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

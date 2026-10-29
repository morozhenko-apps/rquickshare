# rQuickShare release contract

rQuickShare is an independently maintained fork of [Martichou/rquickshare](https://github.com/Martichou/rquickshare). The original project's GNU GPLv3 license and the obligations for modified distributions apply.

## Product and platform identity

The application brand remains rQuickShare; native package names must be unambiguous and upgrades must not unexpectedly replace unrelated upstream installations. Linux, Windows and macOS each require their own implementation and physical acceptance before the platform can be presented as supported.

## Packages and approval

Publishing a release requires a reviewed source revision, an authorized native version tag on the protected release branch, signed Linux package metadata and verified immutable package checksums. The same immutable DEB may serve direct downloads and a signed APT repository without duplicate storage.

The APT public key is distributable; private signing subkeys and credential backup material are not. A package build, a passing test suite or an object-storage upload acknowledgment alone does not establish a public release. Download byte verification, metadata signature verification and installation/upgrade acceptance remain separate steps.

Release automation and platform source code provide executable details. No bucket identifiers, personal machine paths, token names, CI run transcripts or secret-recovery procedures are documented in this public contract. Publication requires explicit owner authorization.

## Native 1.0.0 known-limitations release decision (2026-10-29)

The owner explicitly authorizes the **Linux Ubuntu 26.04 amd64 native 1.0.0 release with existing issues #8 and #16 unresolved**. Those issues are known limitations that accompany the release; neither is a completion gate and neither may be represented as fixed. Preserve the existing fast BLE-to-TCP compatibility mode and keep unverified routes independent, with explicit incoming consent. Do not imply cryptographically verified grouping or universal interoperability. The reported relation to Google's own outstanding bugs is the owner's characterization, not an independently verified upstream fact.

This product-scope acceptance is separate from **technical distribution integrity**: final source and package tests, reviewed protected master/tag identity, signed APT and immutable artifact verification remain mandatory. The owner requested no intermediate installed human review after the physically accepted 0.9.0-4, so clearly distinguish that historical acceptance from the as-yet-uninstalled 1.0.0-1. Windows and macOS are unsupported until separately implemented and accepted. New independent defects discovered by final CI must still be fixed; do not bypass the gates.

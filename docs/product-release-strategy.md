# rQuickShare release contract

rQuickShare is an independently maintained fork of [Martichou/rquickshare](https://github.com/Martichou/rquickshare). The original project's GNU GPLv3 license and the obligations for modified distributions apply.

## Product and platform identity

The application brand remains rQuickShare; native package names must be unambiguous and upgrades must not unexpectedly replace unrelated upstream installations. Linux, Windows and macOS each require their own implementation and physical acceptance before the platform can be presented as supported.

## Packages and approval

Publishing a release requires a reviewed source revision, an authorized native version tag on the protected release branch, signed Linux package metadata and verified immutable package checksums. The same immutable DEB may serve direct downloads and a signed APT repository without duplicate storage.

The APT public key is distributable; private signing subkeys and credential backup material are not. A package build, a passing test suite or an object-storage upload acknowledgment alone does not establish a public release. Download byte verification, metadata signature verification and installation/upgrade acceptance remain separate steps.

Release automation and platform source code provide executable details. No bucket identifiers, personal machine paths, token names, CI run transcripts or secret-recovery procedures are documented in this public contract. Publication requires explicit owner authorization.

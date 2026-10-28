# rQuickShare active development

The current [GitHub Issues](https://github.com/morozhenko-apps/rquickshare/issues) are the source for task ownership, blockers and actual acceptance. This page contains only the principal work areas, not copied CI job logs.

## Device identity and interoperability

[Issue #16](https://github.com/morozhenko-apps/rquickshare/issues/16) tracks authenticated device and route identity. Only session-bound cryptographic proof can establish equivalence; visible names, IP addresses and advertisements are insufficient. Preserve explicit user consent and independent routes until verified.

## Desktop localization and presentation

[Issue #17](https://github.com/morozhenko-apps/rquickshare/issues/17) covers native GTK presentation and localization. Strings need correct contextual meaning, placeholders, directional isolation and actual on-screen readability. Source completeness must not be confused with native/editorial acceptance.

## Release and installed-device checks

[Issue #14](https://github.com/morozhenko-apps/rquickshare/issues/14) tracks protected signed publication and verified artifact delivery. Device and transfer acceptance remain separate. Review the current issues and the appropriate native testing contracts before any release step.

## Operational rules

Implementation, stable CI, race-sensitive checks, installed peer tests and owner release approval are distinct gates. Once CI starts, do **not poll** it; record the run URL and stop. Avoid creating a second long-lived chronological report here.

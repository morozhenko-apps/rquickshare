# Native GTK #17 remaining-surface source inventory

Snapshot 2026-10-27. Current implemented static/dynamic i18n includes the canonical 20 locales in `src/i18n.rs`, plus typed PIN/byte/file count and Settings static/status/error/direct-warning slices. Last executable `ac152f6941daaa4abd599351a14f77a8f63a8c86`: Native GTK workflow `40548418258` terminal **SUCCESS**; matching Preflight `40548418257` **NOT YET TERMINAL** at inventory time. This document is a read-only audit; it does not change or certify any UI.

## Remaining user-facing source surfaces

Inspection of all 19 Rust modules under `native_desktop/src/panel/` and their direct English literal context reveals the following **candidate leftovers**. Lists are representative, not an exhaustive lexical/localization proof: literal parsing may include tests/identifiers and miss multiline/macro-built text. Before a source edit, inspect exact surrounding function and ensure a value is truly shown to the user.

| GTK source | Examples / bounded remaining work | Risk and test gate |
| --- | --- | --- |
| `panel/settings.rs` | `Default Downloads folder`; 1st-use computer name hint; sign-in help/waiting/on/off and start-sharing conditional caption; folder picker access/open error; restart notice; About/license prose; dynamic `Name for new connections: {}`, user-controlled `directory_display` | Keep `status`/`failure`/login branch source semantics and `confirmation_applied`; use typed display-only FSI/LRI for dynamic user/path, no change to saved user data |
| `panel/rows.rs` | `Waiting for details` / `Unsupported content`, consent instruction `Check this code on the sending device before accepting.`, `Incoming request`, `Nearby advertised device`, `No nearby devices`, `Saved files`, `Show in folder` | Consent status vs received content never conflated; delayed metadata `0 files` issue #22 independently verified |
| `panel/reveal.rs`, `panel/transfers.rs` | `Show in folder`, reveal cancellation/access/security errors, `Opening containing folder…`; journal row action | Respect user intent, file capability and stale operation guard; isolate filename/path display, no implicit opening |
| `panel/results.rs`, `panel/text.rs` | Completed but saved content unavailable, text no longer available for review | Distinguish completed metadata from clipboard/file content, preserve explicit Copy |
| `panel/countdown.rs`, `panel/idle.rs` | `Visible to nearby devices · {}:{:02}`, `Updating visibility…`, fallback operation error | Exact timer/visibility and correct 60s Receive expiry; no fabricated open/visible status |
| `panel/outbound.rs`, `panel/presentation.rs` | Failed content preparation, recipient/connection changed, clipboard admission limits, card dismissal failed, unavailable monitor | Keep explicit Send, no replay, no unauthorized recipient change |
| `panel/menu.rs`, `panel/tray.rs` | About fork attribution, project website and remaining menu/tooltips; tray candidates mostly tests | No trademark/ownership drift, correct noninteractive tray/status labels, no unsupported locale |
| `panel/compose.rs`, `panel/image.rs`, `panel/movement.rs` | Already cataloged main composer/image captions; remaining hits largely previously localized labels, tests or static English fallbacks to verify | User-visible exact-source inventory and 20/20 visual check; avoid translating production wire identifiers or test strings |
| `native_desktop` outside `panel/` | Desktop launcher metadata, notifications, errors, accessibility descriptors and packaged resources not yet exhaustively inventoried | Inspect packaging/install and icon titles separately; human semantics and Arabic RTL tests required |

## Suggested remaining code slices (before final MODE B)

1. **Settings residual labels and typed name/path**: finish direct texts, verify 20-locale source coverage and Arabic bidi isolation of untrusted path/name without altering `SettingsEdit`.
2. **Consent/transfer/action UI**: incoming/rows, reveal/history/status, countdown/outbound/presentation; strong negative tests for Cancel/Decline/Accept, text, stale metadata and file permissions.
3. **Tray, About, notifications, accessibility and packaging**: audit runtime/desktop metadata for visible English and contextual translation, not merely a static key count.
4. **20-language semantic/visual acceptance**: contextual privacy/permission meanings, PT-PT, Traditional Chinese, RTL direction and fonts, long narrow card overflow, available native-speaker reviews. Source unit tests alone cannot sign off.

## Non-localization pre-final gate status

At inventory, GitHub listed **17 OPEN issues**, of which **8 titles contain P0**. Source work on several P0s exists, but those issue statuses are not evidence of current physically installed PASS. Most urgent gate dependencies remain authenticated verified identity #16, BWU route admission residual risk #8, cold/warm discovery and Pixel/KDE Cancel/completion observations (#18/#23/#25/#26), stale/tombstone handling and thread lifecycle (#12/#19), with owner release decision where applicable. Do not automatically close source-fixed issues without accepted regression/hardware evidence.

The release order is feature/reliability blockers → all intended native UI text → exact 20/20 locale context + physical KDE/Pixel acceptance → exhaustive Mode B stable/race-sensitive coverage and mutation → signed/reproducible packaging/release approvals. This is **not a calendar duration prediction or permission to publish**. Final acceptance remains BLOCKED and no device binaries were installed in this task.

## Native settings residual update 2026-10-27

Executable `723e7c03` now covers 15 formerly English static Settings guide/pending/login/restart/chooser/About messages and initial Settings unavailable label (reused), including default Downloads text. Dynamic name/path isolation was previously covered. Remaining Settings visible text, if any, must be found with a precise source/behavior audit, not inferred from earlier inventory; transfer row, reveal, countdown, outbound, tray/a11y and packaging are still unfinished. Source-exact Preflight `40571863031`, Native GTK `40571862841` PENDING. No release/Mode B claim.

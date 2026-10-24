# Native compact card — retire completed rows when new inbound starts

Date: 2026-10-24. Direct operator observation on current `0.9.0-1.debug` KDE package (installation inferred from current physical screenshot, version should still be verified with `dpkg-query`): old Mercury MP4 completed row remains adjacent to new incoming Mercury MP4 consent row. Distinguish session history from visible card rows. [Issue #20](https://github.com/morozhenko-apps/rquickshare/issues/20), [active backlog](active-backlog.md), [device attention #13](devices-incoming-attention-2026-10-20.md).

## Root cause and selected behavior

`native_desktop/src/panel/results.rs::Results::observe` preserves `visible` completed results until a dismiss/epoch change/removed row. `native_desktop/src/panel/rows.rs::render_rows` projects both terminal results where `results.visible(transfer-id)` and current nonterminal inbound requests, creating duplicate-looking stacked rows. The distinct transfers should **not** be merged: each has a different daemon capability, verified consent revision, outcome and saved result. Prior same-name devices are not cryptographically verified as one physical identity (#16).

**UI-only mode A:** The first observation of a *new* same-epoch, nonterminal **inbound** transfer ID retires previously visible completed-result rows from the nearby card. Preserve the actual daemon `StateView.transfers`, separate History, latest completion UI, `Results.phases` record for transition detection and pending result-reveal queue (a newly completed older file should still open or be available via History). Outbound new transfer does not erase an inbound completion. Repeated snapshots of an already observed inbound ID do not clear its own freshly finished result. Different concurrent active inbound sessions each remain visible and independently actionable, irrespective of same/unverified display name. No filesystem dedup, silent replacement, transport/wire change, storage delete or retry.

Risks: hiding a completed row when new unrelated inbound request arrives; accepted trade-off for **compact current-work card**, not persistent History. A preview/idle/dismissal race must not suppress a legitimate reveal. New-ID+direction+phase guards keep prior cases stable. Existing `Results::observe` is the exact real state projection; do not duplicate transfer filtering in `rows.rs`.

## Test matrix and per-file coverage

- Positive: A transitions Receiving→Finished, appears as a completed-card result exactly once, with queued reveal. New B inbound AwaitingConsent appears → previously visible A disappears, while both underlying transfers and reveal queue remain intact. B later Finished → B appears, not re-adding A.
- N1: invalid/forged peer name cannot influence grouping; only transfer ID/phase/direction used. No guessed identity.
- N2: zero/new/repeated/multiple incoming IDs, baseline/epoch reset and terminal-only old records.
- N3/N4: repeated stale snapshot must not dismiss latest Finished; interrupted new incoming still clears older card, but the new failure remains reviewable in History.
- N5: simultaneous A/B inbound consents continue independent; two arrival snapshots same source order not re-trigger hidden completion.
- N6: old epoch and missing records pruned as before.
- N7: no consent commands or call to `Respond`/reject triggered by presentation.
- N8: Unicode sender names irrelevant; no timeouts introduced.
- N9: hide/dismiss remains explicit; automatic one-time recent file reveal preserved.
- N10: no logs with content, PIN or device names.
- N11 monetization N/A; N12 storage N/A (presentation-only).
- Interaction combinations: new/repeat × inbound/outbound × terminal/nonterminal × baseline/steady epoch × completed/reveal-pending; new active B during pending A completion; two new simultaneous.
- Single production owner: `native_desktop/src/panel/results.rs` plus same-file deterministic tests. `rows.rs` should remain unchanged; `StateView`, journal and daemon files unchanged. No estimated line/branch coverage claim until measured.
- Mutation review: removing new-ID, direction or nonterminal condition must break a regression assertion; clearing `pending` instead of only `visible` must fail queued-result assertions; regrouping by display name is forbidden.

## Completion gate

1. [x] Physical observation and root-cause / alternative hypotheses documented, issue #20 opened. Android `Failed` on Linux Decline belongs to #18 separately: `inbound.rs::apply_consent(false)` already sends encrypted Sharing `ConnectionResponse::Reject`, and updates Linux `Rejected`. Google Android's UI mapping is unproven without a protocol trace; do not send false `Cancel`/failure codes by guess.
2. [x] Source change limited to new nonterminal inbound ID admission and tests in `Results`; source commit `d886146aafd0b479eb5efad7fc44101c40090bb9`, one code commit separate from docs plan. CI pending at checkpoint.
3. [ ] Source-exact nightly formatting, GTK strict Clippy/test and stable preflight final terminal results; full Mode B later.
4. [ ] Operator installs candidate after existing debug package with controlled rollback, visually verifies new consent does not show prior Completed row, but History retains original. Old installed package cannot prove new fix; do not close #20 without physical updated version.
5. [ ] Backlog + issue completion evidence update, no `master` promotion, tag, cloud or auto-install.

**Do not change startup visibility without an explicit UX contract:** `core_lib/src/settings.rs::StartupSettings::default` currently chooses `Visibility::Visible`, `process.rs` loads settings and `NativeRuntime::start` starts discovery, regardless of window opening; `panel/menu.rs::receive` uses 60-second temporary `Visible` state and supports Hidden. A window showing a 0:45 countdown means the daemon reports temporary mode for that minute, not that the GUI itself initiated an unbounded scan. The owner asked *why*, not yet explicitly to change the default. Distinguish passive background discovery, mDNS advertise and temporary visibility; document a product decision before changing daemon startup defaults or breaking inbound availability.


## 2026-10-24 exact-source automated verification checkpoint

Production code `d886146aafd0b479eb5efad7fc44101c40090bb9`; solely nightly-rustfmt correction `2de1bb70d7b864e3faae849891176d510f678e35` is the final executable SHA for these tests. The first GTK run [40100886094](https://github.com/morozhenko-apps/rquickshare/actions/runs/40100886094) reported formatter failure, no assertion failure or functional diagnosis. The corrected [Native GTK 40101207886](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101207886) completed **SUCCESS**: native GTK source formatting/strict Clippy, **10/10** in one test partition, **92/92** in core GTK widget/controller tests and 0 failures. Both new regressions for retiring stale card result on consent and parallel/unrelated/terminal arrivals printed `ok`; exact job `120893552998`. [Preflight 40101208122](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101208122) Core stable `790/790`, two integrations `4/4` each, Rust format, Core Clippy, frontend **SUCCESS**. At this checkpoint Tauri Clippy `120893731534` still **IN PROGRESS**: source exact 5/6 successes, **do not infer full final gate** until that job terminates. No updated KDE package installed from this source; physical screenshot remains pre-fix repro.

Next: verify terminal Tauri job; update the issue/backlog. Then build distinct `rquickshare-candidate-<commit>` local debug package, package checksum+contents, graceful idle replacement with preserved rollback .deb and **verify installed executable hashes** after reinvocation. The package version may still be `0.9.0-1.debug`; directory name + SHA and binary comparison, not dpkg version alone, establish freshness. Do not touch `master`, public `native/v0.9.0`, APT or R2.

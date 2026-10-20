# Incoming request attention and Devices focus (issue #13)

Date: 2026-10-20; primary task **MODE A**. Implementation is user-requested GTK behavior only; no daemon, protocol, transport or consent authority change. Branch: `dev`. Independent BWU P0 #8 remains OPEN and blocked at authenticated proof.

## Baseline / root cause

`native_desktop/src/panel.rs::render` currently adds `(N)` to the `Receive` caption based on `Inbound/AwaitingConsent` transfers. The `Devices` button has no attention state. `native_desktop/src/panel/menu.rs` toggles the drawer but does not target the inbound row; `native_desktop/src/panel/rows.rs::focus_transfer` can focus keyed consent rows. New transfer policy is already represented by immutable StateView snapshots; there is no need to change IPC/protocol or add global process state.

## Implementation plan

1. Own a bounded per-epoch attention projection in the GTK `Model`: pending request keys are (Transfer ID, consent revision); a consumed/acknowledged set is retained only while the corresponding consent is pending. A new pending key becomes the priority focus target. Reset on disconnect or epoch replacement, remove stale targets and acknowledged keys when a request resolves.
2. On new inbound consent when the drawer is open and controls can be safely focused, render the authoritative request row and focus the new request using existing `focus_transfer`. Do not steal focus while text/selection/settings/consent dialogs are in progress, hidden, or desktop lock prevents it. Never call `respond` or change any daemon state.
3. On a deliberate collapsed `Devices` activation, expand `nearby`, acknowledge already-present pending requests and focus the specific latest newly arrived request, rather than always the first. Ordinary button toggling remains unchanged.
4. Keep `Receive` caption `Receive` with existing temporary visibility action semantics and review fallback; show authoritative waiting consent `N` on `Devices (N)`. While collapsed with unacknowledged pending consent, add theme-aware static highlight + finite CSS pulse. Respect desktop `gtk-enable-animations` for animation suppression; opening acknowledges pending keys and stops pulsing even if later collapsed until a genuinely new consent arrives.
5. Extend in-place GTK unit/widget tests for epoch/reset/duplicate/1→2→0/multiple requests and rendered action labels/classes, plus per-file source inspection. Do not fabricate a full fake transfer handle to bypass NativeClient authority. Defer full system-level consent lifecycle and physical KDE acceptance to user testing.

## Invariants / risk / rollback

- Existing policy-based automatic card presentation and lock/DND restrictions remain authoritative; no new auto-show, no drop of pending consent or prepared user sources.
- Focus is only requested on the existing keyed UI row, not on an untrusted device name; consent keys are scoped to daemon epoch and revision, not remembered across a session reset.
- One bounded CSS finite pulse and static attention style; no perpetual new timer, busy polling, new long-lived async task or hidden IPC.
- Multiple simultaneously pending requests count independently; late duplicate view refreshes do not retrigger attention.
- Risk: render-before-focus after a newly inserted row, stale row between snapshot/render, focus stolen from other UI; guard exact current state and non-modal window.
- Rollback: revert focused GTK source/CSS changes in one source slice. The plan and results are docs-only commits. No release, system install, or main branch changes.

## Validation plan

STABLE native GTK: new pure attention-state test and existing isolated `panel/tests.rs` suite; `cargo fmt`, native GTK Clippy, and Preflight via the repository's final CI. Race-sensitive checks separate from stable CI, no unnecessary repeated test loops. Runtime KDE visual acceptance remains OPEN until user tests a new package.

## Progress

- [x] Baseline repository scan, user requirements, issue #13, prior SSOT.
- [x] Documentation-first implementation plan.
- [x] GTK model, button actions, CSS and tests implemented in `4506647a2a87c59a35eaf20b0938d076fcd07f25` (source-only atomic commit).
- [ ] Exact-source CI/format/lint/GTK suite verified.
- [ ] Physical installation/visual acceptance (separate future hardware gate).

## Implementation checkpoint — 2026-10-20

Source commit `4506647a` changes exactly four GTK files and no Core/daemon/wire sources:
- `native_desktop/src/panel.rs` adds `DeviceAttention` per-epoch pending/reviewed keys, clearing on disconnect/unsynchronized view, retiring resolved requests and honoring consent-round updates; real `Panel::update` detects new requests and may focus a keyed row only if the drawer is open, the window is visible and no critical modal/consent control has focus.
- `native_desktop/src/panel/menu.rs` acknowledges existing requests on a deliberate collapsed Devices activation and focuses its last recorded incoming request; retains normal open/close action.
- `native_desktop/ui/panel.css` adds a persistent theme-aware Devices pending highlight and a finite four-cycle pulse, selected only when desktop GTK animation setting permits.
- `native_desktop/src/panel/tests.rs` adds a deterministic request revision/arrival/ack/reset state test that does not forge a NativeClient transfer capability.

No hardware build or installed candidate was touched. CI exact-source gates: [Native GTK](https://github.com/morozhenko-apps/rquickshare/actions/runs/39478722621), [Preflight](https://github.com/morozhenko-apps/rquickshare/actions/runs/39478722626); both were in progress on first read and must not be claimed green without completion evidence.

Remaining acceptance beyond source: widget-level live transfer projection and physical KDE/Wayland focus/pulse observation with two incoming requests. P0 BWU proof issue #8 remains independently blocked.

## Post-implementation static / CI findings

- Source-only `4506647a` initially failed [Native GTK 39478722621](https://github.com/morozhenko-apps/rquickshare/actions/runs/39478722621) at `cargo +nightly fmt --check`: exact formatting changes in the new `DeviceAttention::review_pending` expression, focus guard and long table-driven unit test assertions/signature. No compiler/test outcome may be inferred from that early format failure.
- Independent follow-up `af4c6ce8` filters `Devices (N)` against disconnected/unsynchronized snapshots to avoid stale badges; same initial formatting deficiency remained in its run. No change to daemon authority, ID handling or consent.
- **Final executable source at this checkpoint**: `2fdda13e68c24e6ca43da3c2fd03f005f2502cd0` contains the *exact* CI rustfmt output, with no behavior/guard changes from format correction.
- Exact-source Native GTK [39479242761](https://github.com/morozhenko-apps/rquickshare/actions/runs/39479242761) and Preflight [39479242776](https://github.com/morozhenko-apps/rquickshare/actions/runs/39479242776) were queued/running at latest observation. Do not represent these as completed or green without GitHub evidence.
- Known residual user-level acceptance: focus/scroll on a real live consent row in expanded Devices list, reduced-motion GTK session, simultaneous requests, and small-monitor presentation. This is neither a product release nor a device-security fix.

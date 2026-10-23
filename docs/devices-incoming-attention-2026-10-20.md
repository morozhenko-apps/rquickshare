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
- [x] Exact-source Preflight and Native GTK workflows completed SUCCESS for `230a2408`; packaging revision-only follow-up awaits its own gate.
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

## MODE B per-file risk and scenario inventory (2026-10-20)

This is an auditable *scenario plan*, not evidence of executed complete coverage. Current pure-state test covers some entries, not physical widget interactions.

| Production file / affected contract | Branches / invariants | Present deterministic evidence | Pending test evidence |
|---|---|---|---|
| `native_desktop/src/panel.rs` | Per-epoch/consent-round `DeviceAttention`; enter/leave/replace/reconnect and previously reviewed status; only authoritative `AwaitingConsent` count | `given_authoritative_incoming_revisions_when_devices_attention_is_reconciled_then_only_unreviewed_requests_pulse_and_new_requests_focus` exercises arrivals, review, duplicate, revised consent, removal, missing and fresh epoch | UI `Panel::update` with actual `StateView` projection, 0→1→2→1→0 count and disconnected/stale/unsynchronized scenarios; focus suppression when modal/consent controls and lock; exact displayed button label |
| `native_desktop/src/panel/menu.rs` | Devices toggles once; deliberate collapsed→open consumes *view* attention only; `Receive` action unchanged; target exact transfer instead of first device | Existing `native_desktop/src/panel/tests.rs` click/open/close and inactive preview | Connected synthetic client-authoritative view with two incoming transfers and reliable focus indication; stale target removed before click, wrong/unavailable device |
| `native_desktop/src/panel/rows.rs` (unmodified) | Keyed `("nearby", id)` row/Accept action ownership; absent target is harmless | Existing `verify_keyed_layout` plus focused production `focus_transfer` method | Verify actual keyed request is in scrolled viewport on small monitor, selected controls focus after render, row lifecycle and accessibility |
| `native_desktop/ui/panel.css` | Finite pulse on Devices only if waiting+collapsed and GTK animations enabled; static contrast survives reduced motion; incoming row warning unaffected | Existing GTK test loads full CSS with parser-error assertion | Runtime style class/timing/pulse count and `gtk-enable-animations=false` static focus cue on KDE themes |
| `native_desktop/ui/panel.ui` (unchanged) | Quick action structure/IDs stay fixed; `quick-devices-label` is present | Existing actual-builder test verifies four button IDs and label content | Positive `Devices (N)` with `Receive` unchanged, accessibility announces pending count |
| `core_lib/src/service/client/state.rs` (unchanged) | Authoritative view denies forged transfer handles and carries epoch, consent round, direction and phase | Existing decoder/immutable projection tests in Core | Verify UI reacts only to synchronized current epoch; no consent mutation via attention pathway |

### Positive, N1–N12 review (C = covered in current source test; P = partially covered, NA = genuinely inapplicable)

| Class | Applicable example and expected behavior | State |
|---|---|---|
| Positive | A arrives collapsed → pulse, click Devices → exact A consent focus; B arrives expanded → exact B consent focus; count tracks 0→1→2→0 | P |
| N1 invalid input | Missing/new epoch view, unready transfer, absent row, wrong direction/terminal phase cannot create usable consent action | P |
| N2 boundary | 0/1/2/256 max supported view rows; revised consent with same transfer ID; badge layout when count grows to 3-digit | P |
| N3 network failure | Core disconnect, delayed snapshot, daemon restart removes stale attention/count/focus | P |
| N4 retry/idempotency | Repeated identical view does not reflash; repeated Devices toggle preserves intended open/close semantics | C/P |
| N5 concurrency/races | Two new requests in same snapshot; arriving B while user handling A; request disappearing between click/render; epoch update mid-focus | P |
| N6 corrupt/old data | Unsynchronized or superseded state cannot generate current attention or count | P |
| N7 security/permissions | No Accept/Decline invocation, no spoofed name-based focusing, lock/DND manual-vs-automatic precedence unchanged | P |
| N8 locale/time | Unicode/RTL device name is untrusted display-only; finite animation respects GTK reduced motion (wall-clock expiry itself not used) | P |
| N9 user interruption | Hide/close/open during pending request, escape, manual dialog and focused consent controls do not lose daemon request | P |
| N10 privacy/PII | No new log of peer ID/name, code, transfer payload or clipboard; only label and CSS state projected | P |
| N11 monetization | NA: sharing UI contains no purchase, paywall or entitlement rules | NA |
| N12 storage failure | NA for the *attention projection* (pure in-memory GTK). Transfer storage failure remains under separate inbound contract | NA |

### Final interaction/negative gates

Combinations: drawer closed/open × zero/one/two pending × unseen/reviewed × automatic/manual window × normal/reduced animation × service healthy/disconnected/new epoch × consent ready/unready × modal/idle × current/older consent round. Targeted integration must exercise the core interactions without fabricating a writable transfer handle. Mutation review: invert one `direction == Inbound`, `phase == AwaitingConsent`, `epoch` reset, `reviewed` clearing or drawer-open guard and verify the focused tests fail; reject changes that only inflate covered lines.

**Current completion gate**: Pure-state testing is not a substitute for an actual widget-level StateView integration, strict Clippy/GTK suite and a user-confirmed KDE hardware observation. These remain open unless executed and documented with exact candidate SHA.

## Exact-source CI confirmation at 230a2408

Source-only correction `230a240858566f1d36ffe7b006df7475930c7a0d` resolved the ambiguous `GtkWindowExt::focus` vs `RootExt::focus` method lookup (`E0034`) by qualifying `gtk::prelude::GtkWindowExt::focus(&self.window)`. The previous source `2fdda13e` passed Preflight but its GTK workflow failed to compile at the ambiguous method; **not** a transport fault.

- [Preflight 39485853461](https://github.com/morozhenko-apps/rquickshare/actions/runs/39485853461): workflow **completed SUCCESS**; all five jobs passed (format, frontend, Core Rust, Core Clippy, Tauri Clippy) on exactly `230a2408`.
- [Native GTK 39485853451](https://github.com/morozhenko-apps/rquickshare/actions/runs/39485853451): exact-source native GTK formatting/lint and isolated widget/controller suite steps **SUCCESS**. At the check time the job was still in successful-step cleanup; do not call the workflow completed until the terminal GitHub result confirms it.
- No production Debian package was rebuilt, installed or smoke-tested on KDE after `230a2408`, and the code is not merged into main. The user's confirmed Pixel BLE→TCP success belongs to the earlier debug candidate, not this UI change.

## CI terminal gate and diagnostic package revision — 2026-10-20

Both workflows on **exact executable source `230a240858566f1d36ffe7b006df7475930c7a0d` finished SUCCESS**:
- [Preflight 39485853461](https://github.com/morozhenko-apps/rquickshare/actions/runs/39485853461): all five jobs successful.
- [Native GTK UI verification 39485853451](https://github.com/morozhenko-apps/rquickshare/actions/runs/39485853451): strict formatting/lint and isolated GTK widget/controller suite successful; clean terminal workflow outcome.

After this gate, packaging-only `native_desktop/packaging/release.toml` revision was increased `1 → 2` by `3c9ccf057d8a25e88e8793cebf70d375d7323e85` to reserve an **independent local diagnostic `rquickshare-native_1.0.0-2.debug_amd64.deb` candidate**, rather than silently replacing already installed/preserved `1.0.0-1.debug` artifact bytes. Package build, its SHA-256, APT simulation and physical KDE acceptance have **not** been executed or claimed. `master` / published APT / signing are unchanged. Exact-source packaging-only CI after this revision remains a separate check.

Physical acceptance still required: two incoming requests while closed/open, correct sender focus, static/animated Devices attention, counter and Receive caption, response/terminal cleanup, manual focus/lock constraints, no regression to Pixel BLE→TCP transfer.

## Physical follow-up — 2026-10-23 installed 0.9.0-1

Owner reported that newly inbound consent was not brought visibly into focus when an already open Devices list held many duplicate Android rows. This is a FAIL of the **physical viewport/scroll acceptance**, not a proof that the consent state failed to arrive. Current `focus_transfer` grabs keyboard focus on an existing keyed row/Accept but lacks explicit `GtkScrolledWindow` viewport ensure-visible behavior after row arrangement. Fix must keep existing lock, focus-stealing, modal guard and exact transfer/consent revision binding. Add a long-list widget integration test for pending consent arriving below or above viewport; confirm physical KDE before issue #13 closure. See [2026-10-23 Pixel findings](native-pixel-acceptance-identity-locales-2026-10-23.md).

## 2026-10-24 — One-target incoming navigation correction (MODE A)

**Observed source-level defect:** `native_desktop/src/panel/menu.rs` handles the `incoming` action by iterating **all** currently awaiting-consent transfer IDs and calling `Panel::focus_transfer` for each. Every call can enqueue a distinct GTK one-shot frame scroll in `rows.rs`. The resulting final focus/viewport can reflect transfer iteration order rather than the authoritative `DeviceAttention` priority. This is not an observed peer-authentication or Pixel wire failure.

**Root-cause hypotheses reviewed:** stale StateView, outdated daemon epoch, incorrect inbound phase filtering, repeated focus scheduling, layout allocation delay, and missing row on re-render. For this focused defect, the repeated `focus_transfer` loop is directly evidenced; old epoch/phase filtering and GTK allocation remain separate acceptance concerns. Existing `DeviceAttention::review_pending` already supplies an epoch-scoped, consent-revision-aware single target and records acknowledgment.

**Change plan:** replace the menu action's multi-target loop with one call to `review_pending()` when activating `incoming`; keep the original `nearby` navigation and drawer rendering. Add a deterministic regression for two live requests with opposite lexical ID order, showing that the most recently observed request remains the selected target and a duplicate snapshot cannot refocus it. No new trust, transport, consent, persistence, accessibility shortcut or timer. Keep the single focus call on the GTK UI thread. Rollback is one `menu.rs` source change plus focused test; revert without touching Core or installed packages.

**Validation / acceptance:** review the exact diff, then complete the native GTK formatting/Clippy and isolated GTK tests at the final gate, with existing Preflight as applicable. Physical KDE acceptance must show correct sender controls/viewport with two simultaneous requests, including reversed row order, no auto-accept, no focus stealing from active consent interaction. Do not close issue #13 until CI and physical acceptance pass. Work on `dev` only; source/test changes committed separately from this documentation.

- [x] Source inspection and independent risk/rollback analysis.
- [x] Documentation-first correction plan.
- [ ] Implementation and focused regression.
- [ ] Latest-source CI verification and physical KDE/Pixel acceptance.

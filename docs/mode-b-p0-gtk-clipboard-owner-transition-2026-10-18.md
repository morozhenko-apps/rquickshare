# MODE B P0: GTK clipboard owner loss and stale preparation completion

Date: 2026-10-18. Primary mode: **MODE B — Testing**. Branch:
`dev`. Only add test-only code inside
`native_desktop/src/panel/outbound.rs` and documentation.
No production API/security/behavior changes; never touch installed
desktop, `master`, release tags or real external clipboards.

## Stage 1: scan and authoritative context

Read user-supplied 378-line `agent.md` testing SSOT,
`docs/mode-b-completion-audit-2026-10-18.md`,
`docs/mode-b-p0-native-async-clipboard-2026-10-18.md`,
`docs/mode-b-inventory.md`,
`native_desktop/src/panel/outbound.rs`, `native_desktop/src/panel.rs`,
`native_desktop/src/panel/tests.rs`, and
`.github/workflows/mode-b.yml`.
The existing GTK4/Xvfb private-display widget smoke already invokes
real `clipboard_paths` via GDK ContentProvider and
real `Panel::wait_selection`/`close_input` through two test
fixtures. Exact previous source `1501fa1e` passed Native GTK
57/57 and quick Preflight 5/5; latest current HEAD is documented
in `docs/current-state.md`. The production methods are
unmodified and must remain unmocked.

## Stage 2: risks and invariants

- When the GDK clipboard source relinquishes ownership **before**
  the async file read begins, `clipboard_paths` must reject
  without replaying bytes of the previous local clipboard offer.
- A newly provided valid MIME offer must recover normally and
  yield **only** its current local paths. A later remote-URI
  offer must fail atomically; no cached prior paths may surface.
  All inputs are isolated in the existing Xvfb display fixture,
  not a user's clipboard or Wayland compositor.
- A completed oneshot response (including `None`) not yet
  polled by the GTK main loop must not revive a UI input after
  an explicit `close_input`. Generation changes and an
  aborted `glib::JoinHandle` must keep prior notice,
  `input_active=false` and no stale strip target.
- Closing pending selection preparation twice must be harmless,
  revoke old cancellation token and not preserve a stale task.
- Distinguish this **owner loss before acquisition** from
  **owner loss during an already open GInputStream**.
  The latter requires an external controllable stream
  implementation to block an actual read and interrupt it.
  Do not claim mid-stream coverage or fake errors with
  magic sleeps.
- Never create a fabricated `Selection`, mock a domain
  controller or claim actual GTK→daemon→Android sending.

## Stage 3: scoped exhaustive testable artifact inventory

| Production artifact | Risk / type | Planned implementation |
|---|---|---|
| `clipboard_paths` + GDK Clipboard owner `set_content(None)` before async read | P0 GTK/SDK integration | 1 fixture inside existing widget smoke: valid offer → loss → recover → remote invalid → recover |
| `Panel::wait_selection`, `Panel::close_input`, `finish_prepared` stale generation | P0 GTK async lifecycle | 1 fixture: completed-but-undelivered `None`, cancel/repeat cancel, old generation callback ignored |
| Actual GInputStream already open while remote owner disappears | P0 hardware/SDK failure | OPEN; no controllable midstream provider seam |
| Actual `finish_prepared(Some(Selection))` connected native Panel and device send | P0 cross-process E2E | OPEN; no connected controller in native widget fixture |

Exactly **two new test-only fixture functions** called from the
existing real GTK smoke. **Zero** new independent `#[test]`
methods; the test count must remain 57. This is honest
branch coverage expansion, not method-count inflation.

## Stage 4: branch map

1. `given_clipboard_owner_loss_before_async_read_when_source_is_replaced_then_old_paths_never_reappear` fixture:
   valid local source `A`, actual GDK offer/async decode,
   `clipboard.set_content(None)` and unsupported MIME
   state, `clipboard_paths` returns Err without old
   paths, valid new source `B` yields exactly B, invalid
   remote `C` rejects, fresh valid source `D` yields
   exactly D. Restore clipboard text after the fixture.
2. `given_preparation_completion_queued_before_user_cancels_when_mainloop_runs_then_old_callback_never_changes_notice` fixture:
   existing real GTK Panel; pending oneshot on current
   generation, sender completes `None` synchronously
   while GTK task has not been polled; call `close_input`
   once and again (both real methods), drive one
   non-blocking GLib iteration, assert old generation
   revoked, no stale notice or input task, no strip.
   Direct old-generation `finish_prepared(None)`
   remains inert. A fresh valid `None` path is
   already covered in the prior fixture and must
   not be recounted.

## Stage 5: Positive and N1–N12 matrix

Legend: P planned, C existing baseline, X not applicable with reason,
OPEN real OS/hardware bound.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| GDK owner loss and reacquisition | C+P valid current | P absent/remote MIME | X byte limits covered earlier | P GDK no source | P repeated recover | P owner changes between reads | P old bytes | P local-only | P Unicode | P owner loss | P no stale data | X no entitlements | X no local writes |
| Cancel after queued completion | C+P current idle | P absent result | X typed None | X no socket | P duplicate close | P delivered-before-polled race | P stale generation | P no send | X no timezone | P cancel | P no notice/data leak | X no billing | X no filesystem |
| In-flight stream loss / real send | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN | X | OPEN |

## Stage 6: interaction matrix

GDK content {valid A, no provider, valid B, malicious C, valid D}
× reader state {before acquisition, after previous completion}
× identity {same display, changed owner}
and preparation {pending receiver, completed None not yet
polled, canceled once/twice, stale callback} ×
{generation current/stale} × {notice unchanged}.
Mid-stream loss and actual connected send are explicitly
unverified, not silently sampled.

## Stage 7: estimated coverage contribution

Two new fixture groups, one existing GTK smoke
method. Method-level new test count: **0**.
Overlapping fixture category attribution:
Positive 2; N1 2; N2 0 (earlier baseline);
N3 1; N4 2; N5 2; N6 2; N7 2;
N8 1; N9 2; N10 2; N11 0 (N/A);
N12 0 (N/A). Counts describe fixture groups,
not independent tests or coverage percentages.

## Stage 8: per-production-file coverage map

`native_desktop/src/panel/outbound.rs`: cover
`clipboard_paths` no provider before acquisition
and fresh content ownership; `wait_selection`,
`close_input`, `finish_prepared(None)` stale
completion before event loop dispatch. Prior direct
`paths`, `input_allowed`, valid stream, size and
malformed input tests remain. **OPEN** true
mid-stream owner replacement, actual `Some(Selection)`
and send to daemon.
`native_desktop/src/panel.rs`: exercise existing
real GTK Panel in isolated widget smoke; does not
prove all rendering/UI functions.
Global production-file Stage 8 and Stage 12 remain NO.

## Stage 10: mutation sensitivity, planned

Removing the empty-clipboard error, returning cached
paths after ownership loss, rejecting a newly valid
provider after prior error, allowing remote authority,
not invalidating generation, skipping task abort or
leaving `input_active` / strip target set must
violate explicit assertions. A mutation of genuine
mid-stream `close_future` is **not** guaranteed killed;
that remains a known surviving category. No
mutation engine claim.

## Milestones, acceptance and rollback

1. [x] Stage 1–8 inventory, risk, branch map,
   N1–N12, interaction, estimates and per-file map
   written **before tests**.
2. [x] Add both fixture groups and call from existing
   `outbound::verify`.
3. [ ] Verify same-SHA native GTK, stable Core, Tauri
   and frontend/Clippy/format via CI; log red triage.
4. [ ] Record D1–D5, mutation review and reconcile
   `docs/current-state.md` and `docs/mode-b-inventory.md`.
5. [ ] Strict full ten-pass Mode B, true in-flight
   GInputStream loss and connected physical sending
   remain OPEN; global Stage 12 = NO.

Rollback only test-module and docs commits.
No production security/OS contract/API change.

## Stage 9 implementation checkpoint

The source-only commit `9306ae91` adds both documented
test fixture groups and invokes them within the existing
`outbound::verify` actual GTK widget smoke:
- Real GDK provider A => read exact A, revoke clipboard
  ownership before the next read => error, provider B => exact B,
  hostile provider C => error, provider D => exact D.
- Actual pending oneshot `wait_selection` result delivered
  to the receiver *before* GLib task polling, explicit
  `close_input` (twice), cancellation token and generation
  invalidation, stale callback inertness and cleared input state.

No production code, application interface or original user's
clipboard was changed. There are zero new `#[test]` methods,
two new test fixtures exercised by the existing widget smoke.
The initial same-source runs are:
[Native GTK 39130166873](https://github.com/morozhenko-apps/rquickshare/actions/runs/39130166873)
and [Preflight 39130166917](https://github.com/morozhenko-apps/rquickshare/actions/runs/39130166917).
These links are not proof of passing tests until each
job completes; do not infer a 10× or Stage 12 closure.

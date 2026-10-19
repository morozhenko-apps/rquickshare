# MODE B Stage 12 completion audit — 2026-10-18

**Status: NO — repository-wide MODE B testing coverage is NOT complete.**
**Primary mode: MODE B (Testing).** This is a source/document audit
of actual evidence, NOT a feature, refactor or relaxation of
coverage criteria.

## 2026-10-19 P0: reveal ingress ownership regression suite

Following the instrumented baseline and the first GTK remote filename label suite, the **actual ingress-owned file reveal worker** was tested in [the MODE B P0 plan, exact stage matrix and final D1–D5 evidence](mode-b-p0-reveal-ingress-lifecycle-2026-10-19.md). Test-source SHA `b912108f`, documentation only afterward. The 2 newly added async tests exercise the production `native_desktop/src/ingress.rs::Ingress::reveal_job` and `Ingress::close` with real Tokio tasks, `PreparationGate` ownership and event-ordered cancellation/close; only the external KDE portal future is replaced at its boundary. Assertions include original portal error propagation, exclusive permit until completion, safe readmission after a successful/rejected reveal, dropped GUI reply without premature task shutdown, and shutdown blocking until a cancelled worker joins.

[Native GTK 39268764105](https://github.com/morozhenko-apps/rquickshare/actions/runs/39268764105) **SUCCESS: 84 passed / 0 failed**, both new named methods logged `ok`, formatting/strict lint pass. [Preflight 39268764131](https://github.com/morozhenko-apps/rquickshare/actions/runs/39268764131) **SUCCESS: 5/5 jobs**, Core 769/769, desktop Tauri 10/10, frontend/format/Clippy green. **This does not cover `panel/reveal.rs::reveal_target` itself, and does not alter 10.87% measured line coverage of `panel/reveal.rs` at the older SHA.** Re-run instrumented reports to make any new source-specific coverage claim.

Global MODE B Stage 12 remains **NO**: on-source entire-suite 10× repetition, full instrumented branch mapping, real KDE Portal/BlueZ/Android/mDNS hardware/runtime acceptance, and automated mutation gate remain open. Production code and `master` unchanged.

## 2026-10-19 additional GUI-safe filename projection evidence

After the measured `99c26335` baseline, two Native GTK P0/P1 tests were introduced in `native_desktop/src/panel/rows.rs` (code SHA `291b7733`) with [documentation-first matrix](mode-b-p0-gtk-untrusted-file-labels-2026-10-19.md). [Native GTK 39252238328](https://github.com/morozhenko-apps/rquickshare/actions/runs/39252238328) **SUCCESS: 82/82** (both new methods named `ok`). [Preflight 39252238333](https://github.com/morozhenko-apps/rquickshare/actions/runs/39252238333) Core Rust **769/769**, formatter, frontend, and Core Clippy **SUCCESS**. At the recorded checkpoint, the Tauri desktop job remained in progress; full Preflight requires independent final-success observation.

Do not mutate coverage metrics for the new source: instrumented Native GTK **70.75% lines** measures the preceding commit `99c26335`. The strict whole-suite 10×, live KDE/Android/BlueZ acceptance, MDNS discovery test seam and Stage 12 continue to be **OPEN**.

## 2026-10-19 current-source instrumented measurement and GTK follow-up

Manual coverage experiments **both completed SUCCESS** at `dev` commit `99c26335`: [Coverage measurement 39209606436](https://github.com/morozhenko-apps/rquickshare/actions/runs/39209606436) and [Native GTK instrumented coverage 39209632695](https://github.com/morozhenko-apps/rquickshare/actions/runs/39209632695). The complete file-by-file Rust output and methodology are preserved in [the source-matched MODE B coverage baseline](mode-b-coverage-baseline-2026-10-19.md).

| Instrumented component | Line coverage | Function coverage | Region coverage |
| --- | ---: | ---: | ---: |
| Core Rust | 90.42% | 86.03% | 87.77% |
| Native GTK | 70.75% | 65.73% | 68.96% |
| Tauri Desktop Rust | 23.86% | 19.29% | 24.98% |
| Frontend V8 | 56.00% | 75.86% | n/a |

Frontend V8 reports 88.04% branch coverage; all three Rust tables report no actual branch instrumentation (zero-count fields), not 100%. Coverage universes/denominators differ and must not be averaged. P0 Core hardware gaps: `hdl/mdns_discovery.rs` 0%, `hdl/ble.rs` 5.59%, `hdl/gatt.rs` 40.55%. GUI gaps: `panel/reveal.rs` 10.87%, `preview.rs` 23.33%; Tauri `cmds/discovery.rs` 0%. The low lines largely reflect lifecycle/hardware rather than trivial missing unit assertions. Real Android/BlueZ/mDNS and runtime testability require independent integration or a separately authorized MODE A seam.

Documentation-first [GTK remote filename/manifest label plan](mode-b-p0-gtk-untrusted-file-labels-2026-10-19.md) has been implemented in `native_desktop/src/panel/rows.rs`, code SHA `86e2bb41` (two pure P0/P1 contracts). Source-exact Preflight and Native GTK have started; outcome and new coverage percentages remain **UNCONFIRMED until separate successful runs**. The latest instrumented percentages are baseline **before** these test additions.

Status remains **NO** for global Stage 12: strict full 10× current-source suite, all branch categories, mutation review, hardware and packaging acceptance are open. The earlier 2026-10-18 paragraphs below are retained as historical exact-source snapshots, not as an assertion that manual coverage was still unrun after 2026-10-19.

## Latest additive client/D-Bus evidence (2026-10-18, post-baseline)

This section updates the **current-source status** without rewriting the historical instrumented baseline recorded below. Latest green **test-code** source is `85faca64`; later `dev` commits through `40dc61c2` are documentation-only.

- [Preflight 39174986191](https://github.com/morozhenko-apps/rquickshare/actions/runs/39174986191): **5/5 SUCCESS**, Core library **767/767** stable cases, eight separately filtered risk-sensitive probes, Tauri **10/10**, frontend, formatter and both Clippy jobs green.
- [Native GTK 39174986234](https://github.com/morozhenko-apps/rquickshare/actions/runs/39174986234): **SUCCESS**, **80/80**.
- [Authenticated forged Settings2/Visibility2 full branch/N1–N12 scoped plan](mode-b-p0-settings-client-adversarial-2026-10-18.md): two new original test methods, **18** D-Bus-encodable invalid typed fields crossed with `Read`/`Reload`/`Set` (**54** negative wire calls) plus three NUL-bearing values explicitly invalid under the production validator but impossible inside D-Bus STRING. The complete failed-run/root-cause/resolution audit is in the linked document.
- The earlier [forged Visibility2 Read/Set suite](mode-b-p0-forged-visibility-dbus-replies-2026-10-18.md) and [local client / daemon owner replacement suite](mode-b-p0-visibility-client-owner-change-2026-10-18.md) passed independently on their documented source revisions.
- The **strict full-suite ×10 run** and current-source instrumented per-file coverage are **NOT verified at `85faca64`**. The previous all-green release-gate run and coverage percentages below are historical, **not** a present-source completion gate. `.github/workflows/mode-b.yml` is `workflow_call`-only from the controlled release process; do not bypass the release gate merely to improve a testing status. `coverage.yml` and `native-gtk-coverage.yml` have separate manual diagnostic triggers, but no current diagnostic run is claimed.
- Global Stage 12 still **NO**: hardware BLE/BlueZ/Android/mDNS, exact exhaustive branch mapping across all production files, automated mutation review, current full ×10 and OS/UI acceptance remain open.

## 2026-10-19 additive P0: authenticated wrong-signature and unrequested-FD replies

Latest scoped code source `4d9da957` (later doc-only commits): [Preflight 39208447151](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447151) **5/5 SUCCESS**, Core Rust **769 passed**, Tauri desktop **10 passed**; [Native GTK 39208447106](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447106) **SUCCESS, 80 passed**. The two original test methods in `core_lib/src/service/client/tests/visibility_forged.rs` both appear `ok` in the exact-source Core logs.

[Scoped MODE B Stage 1–12 inventory and D1–D5 for D-Bus signature/FD replies](mode-b-p0-dbus-reply-signature-fd-2026-10-19.md): tests two authenticated same-owner faulty response forms on `Visibility2.Read`, `Visibility2.Set`, `Settings2.Read`, `Settings2.Reload`. **10 actual malformed reply rejections** across 2 independent tests, plus exact retained pending request and second-command refusal after corrupted `Set`. An unexpected FD is returned alongside a valid typed payload value, but raw reply signature remains changed by its extra FD field; no false claim of an isolated raw-message FD-header mutation test. A redundant test-only import triggered the initial strict Clippy failure and was corrected with unchanged assertions.

This evidence expands test inventory, **not** global coverage closure. `mode-b.yml` is a release-controlled `workflow_call` entrypoint; the final full ×10 suite is **NOT** confirmed on `4d9da957`. `coverage.yml` / `native-gtk-coverage.yml` can be manually dispatched for diagnostics, but **no** current-source diagnostic has been run or measured in this update. Hardware BlueZ/mDNS/Android, KDE session behaviors, per-method branch maps, mutation engine and overall Stage 12 remain **NO**.

## Binding SSOT and interpretation

The user-supplied `agent.md` (UNIVERSAL FEATURE / REFACTOR /
TESTING AGENT PROMPT, 378 lines) is authoritative:
- Stage 3: EVERY testable artifact, risk, level, rationale, planned
  count and applicable Positive/N1–N12.
- Stage 4: every executable branch of every public/business method
  mapped to at least one intended test.
- Stage 5: matrix with each Positive/N1–N12 cell Covered /
  N/A with reason / Planned; no empty cells.
- Stage 6: relevant combinations of independent inputs and states.
- Stage 7: estimate planned test count by artifact/category.
- Stage 8: each *production* file, function, branch, covered,
  remaining and explanation.
- Stage 9: ALL planned tests; representative/sampling-only suites
  cannot close it.
- Stage 10: assertion-by-assertion mutation review (and kill
  meaningful security/state mutants).
- Stage 11: full suites ten consecutive successful runs, without
  flaky/ignored results.
- Stage 12: YES to **all** public methods, branches, business
  rules, invariants, limits, applicable N1–N12 and interactions.
  **Any NO leaves entire MODE B incomplete**.

A successful ten-pass CI is Stage 11 evidence, **not** Stage
12 coverage; reported LLVM *regions* are not a Rust branch
matrix. Measured lines or percentages alone do not prove
rule, negative or boundary exhaustiveness.

## Verified latest source CI and instrumentation

Final tested workflow/product source
`5776e0ea65b0240cdbf3057b5d5ab7d84d4f9935`.
[Strict MODE B run 39078162262](https://github.com/morozhenko-apps/rquickshare/actions/runs/39078162262):
all **3/3** jobs SUCCESS, checked from GitHub job status and
complete logs, NOT inferred from workflow configuration.

| Strict execution | Verified result |
|---|---|
| Complete Core Rust | **710** library tests × **10**, all passed; integration bins (4+4) × 10 |
| Legacy desktop Tauri | 10 tests × 10 |
| Frontend/Vitest | complete suite × 10 |
| Offline Python contracts | 54 × 10 |
| Native GTK isolated X11/D-Bus | 52 × 10 |
| Dependent instrumented coverage job | SUCCESS after the ten-run jobs |

The 6 P0 inbound flow tests (user decline, broken Reject
transport, peer Cancel mid transfer, accepted UTF-8 text/link,
nested folder with existing directory, fresh explicit retry after
TCP interruption) were each checked for exactly one occurrence
in the compiled test inventory prior to Stage 11 repetitions.
This is meaningful **focused** P0 coverage, not a global
Positive/N1–N12 closure claim.

Exact latest coverage job `117533751822` reports:
- Core Rust: **89.65% lines**, **85.49% functions**,
  **86.59% regions**. Includes test-module instrumentation;
  branches not separately measured.
- Frontend/V8: **56.00% lines/statements**,
  **75.86% functions**, **88.04% branches**.
- Legacy Tauri Rust: **23.86% lines**,
  **19.29% functions**, **24.98% regions**.
- Native GTK: prior separate verified one-shot
  [instrumented run 39071770811](https://github.com/morozhenko-apps/rquickshare/actions/runs/39071770811)
  yielded **67.00% lines**, **63.09% functions**,
  **65.35% regions**. The strict latest run tests
  Native GTK 10× but does not measure it in its dependent
  coverage job; the previous GTK instrumented report is
  reference evidence, not falsely labelled a latest-source
  combined coverage result.

Latest specific core production blind spots in the actual
report: `hdl/ble.rs` **5.59%** lines,
`hdl/gatt.rs` **27.45%**,
`hdl/mdns_discovery.rs` **0.00%**,
`hdl/inbound.rs` **90.71%**,
`hdl/outbound.rs` **94.21%**. A line not executed does
not automatically imply a defect, but it is not explicitly
proven negative-branch coverage either; distinguish true
hardware-only activity from testable rule branches.

## Stage 12 gate: honest YES/NO

| Mandatory question | Answer | Evidence / actual gap |
|---|---|---|
| All public methods tested? | **NO** | Legacy 2026-10-12 inventory is scoped primarily to Pixel/BLE maintenance; full current Native GTK/service/modern protocol public methods lack an exhaustively reconciled inventory |
| All executable branches tested? | **NO** | Per-production-function branch-to-test map is incomplete; llvm-cov Rust output reports regions/lines and **0 counted branches**, so no complete Rust branch gate |
| All business rules tested? | **NO** | P0 consent/transfer/file ownership have many regressions, but complete rules across device discovery, lifecycle, status/UI and installed flows are not traced individually |
| All invariants tested? | **NO** | Explicit tests protect many security/file invariants; remaining GATT/BlueZ/transport handoff, repeated peer discovery, KDE/Wayland/portal and crash/restart/lifecycle invariants require further evidence |
| All boundary values tested? | **NO** | Several parsers and file size/offset cases are strong; every public method has not received min−1/min/min+1/max−1/max/max+1 and null/malformed/overflow applicability reconciliation |
| All applicable N1–N12 variants tested? | **NO** | Existing tables contain C/P/S/NA, **not all C or justified NA**; detailed failures and combinations per rule remain open |
| All meaningful interaction combinations tested? | **NO** | Consent × transport × stream interruption × multi-file outcomes have scoped cases; full router, lifecycle, multi-recipient, visibility, UI/desktop, background/foreground, and error-recovery combinations are not all mapped and tested |
| Full suite 10 consecutive clean runs? | **YES, automated exact-source subgate** | GitHub run 39078162262 verified all four complete groups ×10 and native GTK ×10, final coverage job successful |

**Global gate: NO.** A red/unmapped applicable case stays
NO; a hardware-only branch stays unverified until physical
evidence, not silently NA. Hardware constraints do not justify
marking a unit-testable pure rule untestable.

## Negative category by category (coverage is partial, not global)

| Category | Status | Example evidence and remaining responsibility |
|---|---|---|
| **Positive** | PARTIAL | Core inbound text/link/files consent and atomic publication + managed receipt tested. Not every public/UI/hardware flow or rule |
| **N1 input validation** | PARTIAL | Malformed metadata, name/path guards, UTF-8/NUL, invalid frames have tests. Full validators/commands/UI contracts matrix not reconciled |
| **N2 boundaries/limits** | PARTIAL | File-byte lengths, frame bounds, resource quotas, port examples; all per-artifact ±1/null/max boundary variants unproven |
| **N3 network failure** | PARTIAL | EOF, truncated encrypted frames, TCP RST, failed Reject; live BLE/GATT/mDNS reconnect/handoff and failure combinations remain |
| **N4 retry/idempotency** | PARTIAL | Explicit retry after interrupted receive, no automatic replay; multi-recipient duplicates, repeated commands, UI taps and lifecycle intersections remain |
| **N5 concurrency/races** | PARTIAL | Eight risk-sensitive tests ×10, cancellation/state task probes; full simultaneous session/peer/workflow combinations unproven |
| **N6 corrupt/old data** | PARTIAL | Malformed wire/default versions/settings, some journal recovery. Full schema/version/corrupted artifacts matrix unproven |
| **N7 security/permissions** | PARTIAL | Consent token, access-capability and path/symlink safeguards. All D-Bus/BlueZ/input privilege boundaries and peer identity combinations still need traceability |
| **N8 locale/time** | OPEN BY APPLICABILITY | No billing/currency applicable. Visibility/countdown, timing, clock shifts, Unicode/locale where impactful must be evaluated per artifact; blanket repo-wide NA unjustified |
| **N9 user interruption** | PARTIAL | Decline/Cancel, peer loss, shutdown. Actual card close, lock, restart/kill, notification, multiple recipients and Wayland behavior unproven |
| **N10 privacy/PII** | PARTIAL | Received-file/secret capability guards and log redaction contracts exist. Full log/diagnostic/notification/reveal audit per sink incomplete |
| **N11 monetization** | N/A TO SHARING PRODUCT IF NO ENTITLEMENTS | The sharing feature has no in-scope payments/subscription policy in inspected SSOT; confirm per module, not use N11 to cover anything unrelated |
| **N12 storage failure** | PARTIAL | ENOSPC, unsafe symlink parents, read-only FD, anonymous staging, failure cleanup; real filesystem/mount/restart/destination privilege combinations still open |

These rows describe **qualitative evidence**, not validated
numbers of independent tests per category. The SSOT
deliverable **D2 (counts by Positive, N1, ..., N12)** is
therefore **NOT COMPLETE**. Do not manufacture counts by
mapping one test name to many category tags.

## Missing mandatory artifacts: what must happen next

1. Reconcile **Stage 3** across EVERY current production
   component in `core_lib`, `native_desktop`, both legacy
   desktop/UI trees, native D-Bus/public commands and script
   contracts. Exclude genuinely declarative/generated files
   with stated reason. Record method, risk, level, test name,
   expected count, N applicability. The original
   `docs/mode-b-inventory.md` last updated 2026-10-12
   is a historical maintenance-scope inventory, not
   proof of full-repository latest inventory.
2. Expand **Stage 4–6**: real per-method branch map, each
   Positive and N1–N12 cell as C/P/NA(reason), meaningful
   interaction matrix and explicit negative test totals.
   Label hardware-only S/untested rather than claiming
   it was covered by mocks. Prioritize P0 and privacy/
   security before any coverage-percentage exercise.
3. Complete **Stage 7–9** per-production-file/function
   coverage and implement **all** planned deterministic
   tests. In particular classify GATT/Weave packet
   framing vs actual BlueZ, BLE advertising/monitor
   fallback, mDNS rerouting, multi-recipient cancellation/
   retries, native UI send/receive/reveal failure states.
4. Conduct meaningful **Stage 10 mutation review**
   for each important production assertion, documenting
   mutants killed vs equivalent/surviving; the 2026-10-12
   focused prior mutation review is not a comprehensive
   audit of the latest protocol and user flows.
5. Re-run **Stage 11** after any added production/test
   changes on the exact final candidate; verify logs and
   give separate user-led installed native/KDE/Android/
   Windows acceptance evidence for genuinely OS-bound
   branches (without operating the user's devices).
6. Close **Stage 12** only when the seven mandatory
   product-coverage questions are all **YES**. Create
   the required D1–D5 summary, counts, refactor needs,
   category outcomes and documented exceptions in repo.

No MODE A behavioral refactor, security boundary, UI
decision or changes to installed native29, `master`,
release signing or peer device were made in this audit.

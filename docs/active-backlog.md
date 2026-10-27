## 2026-10-27 Read-only trust and visibility invariants

Current source audit: #16 has fail-closed route-credential *infrastructure* but **no proven real Pixel certificate source+authenticated advertised-route association**, so A's alias-collapsing acceptance remains blocked (a new trust/API decision needs owner STOP FACTOR). #8 compatibility exception **does not cover this**. #21 startup/inactive settings reload preserves default Hidden/no implicit saved Temporary rearm at source, but present installed/race tests still required. [Pre-Mode-B issue and trust audit](pre-mode-b-readiness-gates-2026-10-27.md). Latest Settings dynamic display `378834ac` source audited; Preflight `40561114184` and GTK `40561114130` pending, not terminal PASS.

## 2026-10-27 Pre-Mode-B owner closure mandate, #8 tracked exception

Owner requests full pre-acceptance implementation/issue closure before running final Mode B, with **#8 explicitly excepted** from fixes but retained as transparent compatible-A2.2 security risk. [Current 17-issue gate matrix and exact acceptance criteria](pre-mode-b-readiness-gates-2026-10-27.md). The existing source work is not equal to current installed/hardware success, notably #16 authenticated peer identity, #23/#25 Pixel BLE/terminal correctness, and #27 native Windows. Latest code `378834ac` adds display-only locale-safe Settings resolved-name/received-directory path formatting; static scan PASS. Source-exact Preflight `40561114184` and Native GTK `40561114130` **PENDING**; previous warning source `ac152f69` both workflows SUCCESS. No master/tag/R2/installed binary, and no final MODE B.

## 2026-10-27 #17 direct Settings consent and validation warnings

Code `7b8cc599` + strict GTK rustfmt-only `ac152f69`: **12 new settings warnings ×19 non-English locales = 228 translated drafts**, including per-transfer consent, port 0–65535 and explicit unknown save/login confirmation. No control/IPC/auth changes; source coverage audit PASS. Initial native GTK `40547964619` FAIL in **formatter only**, corrected without changing tests/behavior. Final exact-source Native GTK `40548418258` and Preflight `40548418257` **PENDING**. [Detailed scope, coverage, and rollback](native-settings-direct-warning-i18n-2026-10-27.md). Remaining static/dynamic GTK messages, verified identity (#16), 8 open P0-tagged issues and physical acceptance mean #17/release still NOT complete.

## 2026-10-27 Native GTK preview test ETXTBSY incident and fixture publication fix

On `e9b4e4b4`, Preflight `40537441279` **SUCCESS**. Native GTK `40537441309` passed rustfmt/strict Clippy but FAILED **1 of 10** preview tests in `src/preview/tests.rs`: Linux `ETXTBSY` (`Text file busy`) spawning a newly written executable fixture. Not a proven localization regression and **not** counted as flaky/pass. [Failure evidence and root-cause plan](native-preview-fixture-etxtbsy-2026-10-27.md). Test-only fix `1fe649ed` stages, closes and atomically publishes fixtures and reports the original Decoder classification on diagnostic spawn failure; no production preview/security/locale changes. **Final exact-source Preflight `40539063998` and Native GTK `40539064009` PENDING**. Issue #17 still OPEN; no release or physical acceptance claim.

## 2026-10-27 #17 final exact-source gate for 32 Settings states/errors

Error + login status catalog source `fe49c9ec` failed Native GTK `40536975905` **only** in strict `rustfmt` (two exact source layout issues). Style-only fix `e9b4e4b4`; new final-source Preflight `40537441279` and Native GTK `40537441309` **IN PROGRESS**, not called green. Existing original domain error/status branch code unchanged. Third Settings UI warning/dynamic fields are inventoried but **not implemented**. [Detailed chronology and test plan](native-settings-i18n-staged-plan-2026-10-27.md). #17 still OPEN, release/master/tags unchanged.

## 2026-10-27 #17 Settings errors, locale Stage 2b implemented

Source `fe49c9ec`: 17 more exact source messages for controller/save/login failures ×19 translated locales = **323 new translated entries** in `src/i18n/settings_errors.rs`. Stage 2a+2b: **32 settings/error keys, 608 draft translation entries** across 19 non-English locales; original status/error branch functions byte-identical and only GTK output localized. Static safety/locale wiring audit PASS; final source-exact Preflight `40536976093` and Native GTK `40536975905` **PENDING**, not declared green. Prior Stage 2a source `2dcab389` Preflight `40535127513` SUCCESS; Native GTK `40535126993` FAIL in formatter only, fixed by separate style commit `f934906e` included before new Stage 2b source. [Design and exact evidence](native-settings-i18n-staged-plan-2026-10-27.md). #17 remains open; physical/native-linguistic acceptance and final MODE B not performed.

## 2026-10-27 #17 SettingsView source statuses (15×20 locales)

Production `2dcab389` adds **15 settings-status source keys ×19 non-English translated rows = 285 draft translations** in `src/i18n/settings_state.rs`, selected *after* unchanged `status(SettingsView)` error/success priority logic and GTK-only `i18n::label`. Existing `Saved and applied.` tests remain authoritative. Static catalog/branch/UI audits PASS; **source-exact Preflight `40535127513` and Native GTK `40535126993` PENDING** at checkpoint. [Stage plan, coverage and future error mapping](native-settings-i18n-staged-plan-2026-10-27.md). The 10 settings-action and 7 login-action error messages are NOT yet translated. #17 stays open; no physical RTL/human locale review or Mode B release gate.

Previous static settings-chrome `ea4ec8a8` Preflight `40533944102` terminal SUCCESS; GTK `40533944580` still pending as of last observed state. Do not claim a combined pass yet.

## 2026-10-27 #17 Settings source-exact Preflight proven

Executable `51b416e6`: Preflight `40513766961` **SUCCESS, all five jobs** (Rust format, Core tests/Clippy, Tauri Clippy, Frontend). Native GTK `40513766561` **IN PROGRESS** at last read (prerequisite install), so full GTK source acceptance is still PENDING. [Settings proof and remaining stages](native-settings-i18n-staged-plan-2026-10-27.md). Issue #17 remains OPEN; no 20-language human/RTL and no hardware/1.0.0 release claim.

## 2026-10-27 #17 Settings chrome (first source slice, 20 locales)

Executed source `51b416e6` creates isolated `src/i18n/settings.rs` with **25 new static keys ×19 translated rows = 475 draft translations**, hooks static buttons/visibility choices/pages in `panel/settings.rs`; existing visible/hidden/cancel reused, and settings write/recovery/retry/restart/login/port validation callbacks remain untouched. Static source audit PASS; **Native GTK 40513766561 and Preflight 40513766961 pending** (no source-exact terminal proof). [Settings staged plan and remaining status/error work](native-settings-i18n-staged-plan-2026-10-27.md). #17 remains PARTIAL.

Image `9e6251d3` GTK run `40511449070` SUCCESS; the matching Preflight `40511449089` was **CANCELLED**, not a terminal PASS (four jobs passed, Tauri Clippy uncompleted), superseded by current newer code's Preflight. [Image details](native-image-dialog-i18n-2026-10-27.md).

## 2026-10-27 #17 native image formatter follow-up

Exact Native GTK `40511038773` on `b849003e` failed in rustfmt-only new test/status layout. Follow-up `9e6251d3` exactly applies four suggested formatting changes without behavior changes; new exact-source Native GTK `40511449070` and Preflight `40511449089` **PENDING**, not yet PASS. [Source diagnosis and gate](native-image-dialog-i18n-2026-10-27.md). Next planned #17 surface: settings validation, destructive recovery and login state captions (separate bounded implementation).

## 2026-10-27 #17 native image capture dialog localized (20 locales)

Production `b849003e` includes 10 new static image-capture safety labels ×19 additional translated locale rows; existing Cancel/queue-failure messages reused. Static audit PASS; actual native image copy/prepare/epoch/authorization logic is unchanged. Frame dimensions and byte count remain exact with `px`/`B`. **Source-exact Preflight 40511038788 and Native GTK 40511038773 IN PROGRESS**, final result must be read. [Image locale design and negative tests](native-image-dialog-i18n-2026-10-27.md). Image translation is a bounded partial #17 stage, not 20-language holistic UI/RTL signoff.

Earlier typed transfer metadata formatter `b8064a09` is **confirmed PASS**: Preflight `40509819442` and Native GTK `40509819419` terminal SUCCESS. [Counts/PIN/bidi record](native-dynamic-counts-bidi-2026-10-27.md).

## 2026-10-27 #17 typed counts and bidi-safe dynamic transfer labels

Production `7b7ac325` introduces pure GTK presentation helpers: integer-aware localized file count, exact byte symbol `B` without rounding, LTR isolation of four-digit session PIN, and display-only isolation/stripping of untrusted Unicode direction controls for peer/file/kind text. Applied to nearby/device/history/transfer text only; original transport/consent/data structures unaffected. Documented five focused regression tests; **actual exact-source Preflight `40509466138` and Native GTK `40509466348` are running, not yet PASS**. See [dynamic formatting and RTL invariants](native-dynamic-counts-bidi-2026-10-27.md). #17 OPEN/PARTIAL; source and physically installed 1.0.0 status unchanged.

## 2026-10-27 #17 history exact-source CI refresh

Latest executable history localization source `34e168e2` follows the Dutch reset/clear semantic correction and formats its two assertions per the GTK Rustfmt log. Previous `36c3a1b7` GTK run `40485030921` failed in formatter only; new exact-source Preflight `40485357024` and Native GTK `40485357147` **PENDING**, not green by inference. No other functional or protocol changes. Documentation: [history translation and CI record](native-history-metadata-localization-2026-10-27.md). #17 remains PARTIAL.

## 2026-10-27 #17 Dutch destructive history label correction

Code `36c3a1b7` narrows the Dutch history reset vs completed-clear distinction, with literal semantic regressions: `Geschiedenis resetten` is not a restoration claim; `Voltooide items wissen` denotes completed record removal. No journal deletion logic or backend authority changed. New exact-source Preflight `40485030868` and Native GTK `40485030921` **PENDING**; #17 remains OPEN/PARTIAL and human 20-locale review is still outstanding.

## 2026-10-27 #17 native GTK history metadata translations, bounded source slice

Production `ae1c5cdc` introduces isolated history static-message catalog `native_desktop/src/i18n/history.rs` (18 keys ×19 translations across the existing 20 locales), reuses base menu keys, and keeps terminal failures/unknown journal status separate from `Completed`. History storage/metadata disclosure, clear/reset distinction, 30-day/1000-operation privacy limits and existing action permissions are unchanged. Initial static completeness/semantic collision checks PASS. `cargo fmt` gate failed on new test layout, corrected by style-only `25d8daa8`.

**Source-exact final Preflight `40484330580` and Native GTK `40484330621` PENDING** as of this checkpoint; no 6/6 green or hardware/translation signoff asserted. [Design, risk, test and rollback record](native-history-metadata-localization-2026-10-27.md). Counts/bytes/PIN/untrusted peer formatting require their own plural/bidi-aware design. #17 remains OPEN/PARTIAL, #16/#8/release unchanged.

## 2026-10-26 #16 authenticated session → exact route proof bridge, with fresh peer reauthentication

**New code `57567bfca328732f1e12c4b59106f00622ebbd34`** (final source SHA before docs; CI still pending). Core now has a **private, bounded route-proof channel** from actual mutually verified outbound PairedKey → managed SessionLease → NativeRuntime → exact endpoint-generation registry. Connect latency is measured by the actual TcpConnector; proof can only attach to the still-live original numeric address and SessionId, never mDNS name, temporary ID or IPC. Authenticated proof TTL is **60 seconds**, with immediate invalidation on route removal/address change or loss of any proof event. Small-transfer proof is drained before terminal session cleanup. No stable credential is logged or sent to UI.

**Critical payload safety:** every new send using cached confirmed-peer routing pins the *expected credential* privately to Pending and outbound session, and the fresh PairedKey MUST independently verify the identical credential before sending Introduction, consent or payload. Anonymous/rotated/unavailable key or expired expectation while queued fails closed, no silent fallback or second recipient; ordinary anonymous Pixel send with no trusted cached expectation remains the original consent flow. [Stage implementation details, tests and limitations](native-route-proof-propagation-2026-10-26.md). Stable source CI `57567bfc` is pending completion; 6/6 NOT claimed and no physical acceptance.

**#16 still PARTIAL:** Google-compatible trusted certificate provider, actual advertised route-to-device binding (beyond credential identity), outbound addressable BLE adapter, accountable per-medium quality measurements and verified-only backend dedup projection remain. Windows Native adapter unchanged; 20 GTK locales #17 separately PARTIAL. No visual, master/tag/release or installed package changes. Refrain from releasing an untested new binary.

## 2026-10-27 #17 native GTK text review and composer localization slices

Received-text review completed as a bounded source slice via merged [PR #28](https://github.com/morozhenko-apps/rquickshare/pull/28) at `e1abea6d`. Exact-source feature PR Preflight `40468865775` and Native GTK `40468865774` **SUCCESS**, while KDE, Arabic RTL and professional linguistic acceptance remain OPEN.

New text composer production `120ca2f18f0481636687c18192778f77a6c37030` adds 11 catalog keys × 19 translations for the 20 canonical locales, wiring the explicit paste/preparation dialogs without changing clipboard access, bounds or Send semantics. Static inventory and wiring checks PASS; exact-source `120ca2f` GTK formatter FAIL was corrected by format-only `9cfe63fa`, whose fresh Preflight/GTK runs remain **pending**; no general 20-language GTK completion, 1.0.0 Mode B or physical acceptance claim. [Implementation and separate negative/fallback test plan](native-text-compose-localization-2026-10-27.md). #17 stays OPEN, #16 verified-route requirements and #8 BWU residual risk are unchanged.

## 2026-10-26 #16 Core exact-route registry: implementation / CI gate

Production-compiled `core_lib/src/service/runtime/route_registry.rs` now owns all Native outbound **exact route generations** instead of a raw map; `NativeRuntime::dispatch` calls its source-owned route selector and pins one concrete chosen transport before an outbound job enters the queue. Real anonymous Pixel peers retain their exact choice and existing UI. Independently verified same-peer routes *can* be ranked by known connection latency under a bounded certificate-evidence lifetime, with stable tie order; route removal, IP/port generation replacement and expiry revoke the linking evidence. Separate verified peers, equal display names and temporary Google 4-byte IDs cannot redirect or multicast a user Send. Only `#[cfg(test)]` fixtures currently create route proofs to verify the selector end-to-end. There is **no production certificate provider or route attestation hook yet**; this is architectural groundwork and **#16 remains OPEN**.

[Detailed design, real source changes, positive/negative tests, blockers, source SHA and CI caveats](native-route-registry-2026-10-26.md). Executable code `84c1b32fc6e34c0b8f5b9f379b5ae4b80c5cc28d`, format-only followups `20f081fd` and `c42102f1`. First exact-source Core Clippy and Rust tests PASS; exact-source final Preflight and GTK CI for `c42102f1` **must be independently verified before declaring 6/6 green**. No BLE outbound integration, peer account linkage, UI change, binary install, master, tag or signed release.

## 2026-10-26 #16 route-generation correctness implemented (MODE A)

**Latest executable source `c94d47a69631a21429b9665762fd81b50bdd6038`** (format-only successor of `edc3cca8`). Native `core_lib/src/service/runtime.rs` now treats a same-fullname IP/port replacement as an **unverified new endpoint generation**, revoking the old exact-recipient capability and allocating a fresh epoch/sequence ID instead of retargeting pending Send to a possibly different physical device. Label-only changes preserve the route; existing active sessions remain on their original transport. This leaves GTK and Windows visuals unchanged. Focused tests cover queued vs backpressured Send, already active Send, route churn, handle release, and the accidental ReplayConflict fixture now uses unique request IDs. Existing mDNS probes automatically pick the **first actually reachable IP** among a single service's multiple addresses, newly regression-tested against a slower lexically earlier address. No multicast, second consent or guessed same-device proof. [Technical record](nearby-endpoint-identity-comparison-2026-10-26.md).

**Source gate:** previous `edc3cca8` Core stable tests, Core Clippy and GTK passed; nightly format required two test-line wraps, fixed in `c94d47a`. The exact `c94d47a` six-check final CI result is pending verification. No physical Pixel check, no installed binary, and no master/tag/R2 change. **#16 still OPEN**: genuine Google/Windows-compatible credential source, route-to-session binding, integrated outbound BLE + LAN route registry, trustworthy current quality metrics and automatic best **cross-medium** route selection are not yet implemented. Do not infer completed identity from this mDNS-only security fix.

## 2026-10-26 #16 FINAL owner decision: no UI changes, automatic best eligible route

The owner explicitly **rejected manual route selection and any redesigned/expanded device card**. Replaces prior #16 Stage 2 B draft and earlier issue recommendations to reveal individual Wi-Fi/BLE routes. Keep **all existing UI visuals, controls, text and layout unchanged**. When the backend has sufficient **positive, non-conflicting** evidence that several current routes belong to the same peer, expose one existing device entry and automatically choose the **best currently usable eligible route**. Do not pretend one transport is fast by default without measured connectivity, or send over multiple transports in parallel. A user Send creates one exact recipient session: exactly **one** peer receives a request/PIN/consent/file notification, never both even if ephemeral IDs collide.

Missing positive correlation, inconsistent asserted identity/generation/proofs, stale/tombstoned routes, or ambiguity → **keep two separate entries with the original appearance**; a four-byte endpoint ID or same name alone never proves identity. Different IP addresses can be legitimate transport addresses of one device; an IP difference by itself is neither identity proof nor disproof. Once the chosen peer has seen consent/payload or Reject/Cancel, no silent switch to a second potential device; in-session authenticated BWU can migrate media under its existing owner. Revocation and epoch changes invalidate associations. #8 risk persists.

[Final owner decision, Android/Windows comparison, source prerequisites, R1–R12 tests](nearby-endpoint-identity-comparison-2026-10-26.md); [#16](https://github.com/morozhenko-apps/rquickshare/issues/16). **Status:** current Core still has only mDNS outbound routes; same-generation BLE/mDNS registry and real verified route-bound identity/quality metrics are NOT IMPLEMENTED. Source test `0d9026f` enforces no fanout across two conflicting same-hint advertisements (CI to be confirmed). Do not mark #16 DONE or change GTK visuals, master, tags, package or publication.

## 2026-10-26 operator-prioritized implementation gate: #16 → #17 → physical checks → full MODE B

**This supersedes the previous release order.** Owner explicitly prioritizes [#16](https://github.com/morozhenko-apps/rquickshare/issues/16) *verified-only cross-route device identity*, followed by [#17](https://github.com/morozhenko-apps/rquickshare/issues/17) *complete 20-locale GTK localization*, **before** next installed Pixel/KDE checks and broad MODE B. Source security/focused regression checks and six-source CI still required; do not defer safety testing until after physical trials. Windows port/layout preparation may occur in parallel but is not a Linux GTK copy. [Source-grounded plan and CFVigil Windows reference commits](identity-locales-windows-priority-2026-10-26.md).

Current #16 source slice introduces a **verified credential fingerprint only on actual certificate- and token-bound success**, fail closed for anonymous/unable/expired peers. **No real Android certificate provider, advertised-route binding or GTK grouping exists yet.** Peer aliases remain separate and user recipient choice is unchanged. Do **not** merge by name, IP or BLE/mDNS ID, and do not mark #16 complete on new mock ECDSA tests. #17 current **partial** GTK slice has 17 UI strings for the 20 canonical locales, system locale selection, fallback and initial card/context/tray wiring; remaining GTK catalog and contextual/native audit are NOT DONE. Neither feature is yet hardware accepted. This priority makes the planned 0.9.0 promotion contingent on the newly ordered feature gates, despite the historical early-release strategy. No release action authorized.

> **Current release GO/NO-GO board (2026-10-26):** [release-gates-live-2026-10-26.md](release-gates-live-2026-10-26.md). Separates source/CI PASS from physical 0.9.0 blockers, independent #8 risk disposition, signed tag/R2/APT publication, and 1.0.0 verified-route/20-locale roadmap. No release action authorized by the report.

> **Canonical 2026-10-25 Pixel/KDE hardware measurements:** [native-pixel-kde-field-measurements-2026-10-25.md](native-pixel-kde-field-measurements-2026-10-25.md). Consolidates #25 Cancel before/after, #23 first BLE Weave/slot0 and BWU stages, #18 off-LAN discovery, exact `elapsed_ms` tables, CI/payload SHA and unresolved gates. Read this measurement record before interpreting historical in-progress snapshots below. Source code unchanged; no release promotion.

## 2026-10-26 final owner decision: preserve existing fast BWU (#8 option 1)

Owner selected **option 1: preserve the existing Google-compatible fast BWU**,
reaffirming A2.2. **#8 remains P0 OPEN / DEFERRED / NOT FIXED**: untrusted
plaintext IDs can consume a pending route and disrupt a legitimate upgrade.
Existing encrypted-frame HMAC/sequence validation remains in place; this
decision does not establish file decryption or approve a security-release waiver.
The proposed candidate/KEEP_ALIVE architecture is not selected for implementation.
[Final decision and rationale](bwu-compatible-candidate-proof-design-2026-10-26.md#final-owner-decision--2026-10-26-preserve-existing-fast-bwu).
Reopen on new compatible proof/rollback evidence or an explicit owner request.
This is documentation only; #26's independent six-green source correction and
pending physical acceptance remain as recorded below.

## 2026-10-26 #26 corrected in source; earlier #8 research checkpoint

Source `263aa3dcc0f8736c7945e5db147f6adcd0eb272f` fixes the committed-migration
local Cancel ownership/delivery defect. [Plan, exact local gates and limits](bwu-cancel-security-followup-2026-10-26.md):
**871 stable +4+4 integration PASS**, eight race-sensitive methods separately
once PASS; nine new methods/fifteen table cases. Strict Core Clippy, nightly
format and workflow policy PASS. Exact-source remote Preflight `40367247177`
and GTK `40367247126` are now **completed SUCCESS, all 6/6 checks** on `263aa3d`.
Terminal readback is recorded in the implementation report. No updated package installed,
no physical Cancel/BWU or Google Windows acceptance; #26/#25 remain OPEN.

Owner reopened #8 and requested a compatible solution, explicitly rejecting a
mere repeat of the old BWU-versus-BLE-only choice. Deeper pinned Google source
review produced [a new standard-keepalive candidate design](bwu-compatible-candidate-proof-design-2026-10-26.md):
non-destructive candidate mailbox, authenticated BLE readiness, isolated crypto
verification and candidate-specific challenge echoes before logical route
commit. It is **not implemented or physically proven**. It revises the old
pre-BLE-retirement proof/rollback contract and requires that material decision;
the owner has not selected BLE-only. Existing plaintext-ID admission and its
expected-vulnerability sentinels remain unchanged, #8 **OPEN / NOT FIXED**.
No account/QR/payload expansion/AirDrop or release work is included.

Latest owner discussion: investigate keeping a usable BLE fallback when a fast
path cannot be authenticated within the waiting budget; **still discussing, no
approval to implement post-drain proof**. Current optional TCP wait is 15s;
committed synchronization/close is bounded by existing 5s+5s budgets. The proposed
proof after BLE retirement does not yet satisfy always-available BLE fallback.
See the design's owner checkpoint; do not interpret a local retained BLE handle
or an untrusted TCP arrival as proof of a recoverable/authenticated transport.

## 2026-10-26 MODE B independent audit: new P0 Cancel/BWU race #26

[Issue #26](https://github.com/morozhenko-apps/rquickshare/issues/26) and
[reproducible branch/test matrix](mode-b-committed-bwu-local-cancel-2026-10-26.md)
identify a source-level false `local_cancel_flushed` on KDE-initiated Cancel
**during an already committed BLE→TCP migration**. The GATT worker may cancel
`do_bandwidth_upgrade` after `deferred_upgrade_writes=Some`, then
`send_local_cancel()` queues both Sharing Cancel frames, returns success and
logs them as flushed although neither reached the sender. This specific race
was **not** physically reproduced and **not** covered by the 31 new tests.
The ordinary LAN KDE Cancel still has its earlier genuine Pixel PASS. Avoid
force-writing on the prior BLE channel after LAST_WRITE or changing accepted
A2.2 BWU security policy. Design/implementation of terminal ownership during
committed migration remains OPEN; no code or CI change in this audit.

The latest `088d543` candidate is six-green but **not installed**; its
known SHA-256 is `5e575d3cd6e242587da477e52b2488cf3058df6d9ee7cdafba926f58c5fb2525`.
A guarded local acceptance script for the owner's later test verifies both
new/rollback package digests, current installed ELF identity, three new ELF
matches and live user services, rolling back on installation failure.
It was syntax-checked via `bash -n` in a separate ChatGPT environment; **not
executed on Alcotester**. No master/tag/R2 changes.

## 2026-10-26 current connection corrections: six-green source, physical pending

Executable source `088d543` contains inbound control/BWU (`b0c62d9`), outbound
backpressure/cancel/framing (`f4cb225`), nonblocking multi-address mDNS (`e320961`)
and GATT write peer/session isolation (`088d543`). [Full plan, regressions,
commands and limits](connection-corrections-2026-10-26.md). **31 new methods**;
local **862 stable +4+4 integration PASS**, all eight risk-sensitive methods once
PASS, strict Core Clippy/nightly format/workflow policy PASS. One intermediate
terminal-state regression was fixed with an absorbing-state guard and a new test.

Exact-source [Preflight 40334258112](https://github.com/morozhenko-apps/rquickshare/actions/runs/40334258112)
and [Native GTK 40334258097](https://github.com/morozhenko-apps/rquickshare/actions/runs/40334258097)
are both **completed SUCCESS, all 6/6 checks** on `088d543`. Core CI confirms
**862 stable +4+4 integration PASS**. Verified uninstalled candidate:
`~/Downloads/rquickshare-connections-088d543/rquickshare-native_0.9.0-1.debug_amd64.deb`,
SHA-256 `5e575d3cd6e242587da477e52b2488cf3058df6d9ee7cdafba926f58c5fb2525`.
Installed daemon/GUI hashes differ from this candidate; equal package version
strings do not prove installation. The old uninstalled `6ae62f6` package lacks
these changes. Next: guarded current candidate install/rollback, then cold/retry
Pixel text/file, cancel and discovery acceptance. No installation or Pixel
acceptance yet. GATT notification subscriber identity,
idle prehandshake lifecycle, measured disk/progress throughput and #8 A2.2
cryptographic route proof remain explicit limits. All applicable hardware issues
stay open. Payload categories/new types, account/QR/new transports/AirDrop remain
future backlog under the owner's narrowed scope.

## 2026-10-26 owner scope: correct current connections; defer payload expansion

Owner explicitly deferred richer file categorization and additional payload
types together with account/QR and future features. Ordinary file bytes already
use the generic file path; specialized categories do not gate their transfer.
Mixed attachments, Wi-Fi payload UI/send, split APK, STREAM and Remote Copy
extensions remain future backlog, alongside new transports and AirDrop.

**Active source work:** [connection correction plan](connection-corrections-2026-10-26.md)
starts with keepalive and mDNS scheduling/address fallback, then reviews remaining
BWU/duplex/GATT ownership corrections. This owner instruction allows source work
to progress while the earlier installed Pixel acceptance remains pending; it
does not claim installation, waive physical gates or approve a new trust model.

## 2026-10-26 full compatibility research: ordered follow-up work

[Quick Share / AirDrop audit](quickshare-airdrop-compatibility-audit-2026-10-26.md)
records six source-backed backlog items (QS-AUDIT-01..06), a capability/physical
acceptance matrix and pinned Google reference sources. Confirmed findings:
serialized first-address mDNS probing, ACK-to-ACK keepalive handling, BWU drain
without a required SAFE_TO_CLOSE transition, unattributed shared GATT ingress,
bulk sender control starvation/redundant copies, and missing outbound/off-LAN
transport capabilities. Runtime impact beyond existing traces remains unmeasured.

**Execution order:** first physically accept the existing `6ae62f6` candidate;
then separate keepalive/discovery fixes, BWU/control-channel correctness and
GATT ownership design; profile throughput after reliable connections. New
off-LAN transports, account/QR parity and AirDrop need their own documented
contracts and acceptance. GATT security boundaries, network ownership and
AirDrop production architecture require owner decisions before implementation.
Approved #8 A2.2 and #16 verified-only identity remain unchanged.

This slice changes documentation only. No package installation, service restart,
new compatibility claim or functional issue closure. AirDrop requires a distinct
Linux radio/protocol backend; OpenDrop/archived OWL are research references, not
accepted production dependencies. Existing issue/physical/release gates below
remain open; the audit does not replace historical evidence.

## 2026-10-26 #23 initialization fix: source six-green, installed acceptance pending

**Source `6ae62f62b17b058160d3246873ecdd34377ec9f0` on dev:** optional BWU TCP
wait no longer starves the encrypted BLE negotiation/consent loop. Authenticated
UPGRADE_FAILURE ends the wait immediately; partial frame prefixes survive route
readiness/deadline; established-session errors retain terminal semantics. The
15-second TCP fallback deadline, successful Wi-Fi upgrade, manual visibility,
explicit consent and approved A2.2 admission policy are unchanged.

**Verified final CI:** [Preflight 40287507193](https://github.com/morozhenko-apps/rquickshare/actions/runs/40287507193)
and [Native GTK 40287507248](https://github.com/morozhenko-apps/rquickshare/actions/runs/40287507248),
all **6/6 SUCCESS**. Core **831 stable + 4 + 4 integration passed**, eight
race-sensitive tests excluded per policy; all eight also passed individually
once locally. Fourteen new regression methods protect the changed path. Local
workspace-TMPDIR final gate matches CI; the earlier tmpfs storage failure was
reproduced on unchanged baseline and remains disclosed, as does an extra
non-required no-default-features Core Clippy failure. No assertion weakened.

[Plan, diagnosis, commands, cache-evidence correction and limitations](inbound-bwu-initialization-2026-10-26.md).
[#23 evidence comment](https://github.com/morozhenko-apps/rquickshare/issues/23#issuecomment-6413626764).
**#23 remains OPEN / overall PARTIAL:** new source is not installed or accepted
on Pixel. Next: verified candidate/rollback, cold phone-to-PC text send and retry,
record common WLAN state and redacted offer/consent/failure-or-TCP stages. Do not
claim this fixes every historical discovery or stale-tile problem. #18/#25/#8
remain separate open gates; no master/tag/R2 or user-service change.

## 2026-10-25 #25 Pixel physical KDE Cancel: LAN PASS; repeat/ BLE remain OPEN

**Owner-confirmed UI:** after native receiver-side KDE Cancel while transferring Pixel→Alcotester, **Google Quick Share on Pixel displays 'Сброшено' (canceled) instead of earlier false 'Success'**. Owner actually executed guarded local installer on Alcotester: six pinned-source CI gates PASS for `213fd3db7c3fac9dd2e51662d6250dc361006967`; verified previous rollback .deb; built `rquickshare-native_0.9.0-1.debug_amd64.deb` in `~/Downloads/rquickshare-cancel-mOClKmN6/` (SHA-256 `38932d873894725f121eb859cc42a02e2d3be4792a4f743d45c534d82ce2cfeb`), reinstalled it and compared all three installed ELFs byte-for-byte to the new package. `INSTALL_VERIFIED=PASS`. Existing rollback at `~/Downloads/rquickshare-connectfix-c9aMs0Hz/` retained.

**Privacy-safe daemon evidence, same boot:** `inbound_connection_request_decoded=179309`, `inbound_ukey2_client_init=179333`, `inbound_consent_entered=179759`, `inbound_accept_flushed=182689`, `inbound_first_file_chunk_persisted=183210`, `inbound_file_progress_25=183488`, `local_cancel_flushed=183808`, `local_cancel_peer_eof=184202`. Cancel fully flushed after persisted partial, sender closed after **394 ms**; **no** `inbound_files_completed`. No `gatt_notify_*` markers → this is the **native LAN/non-GATT** cancellation case, not BLE physical acceptance. After collection, owner unset `RQUICKSHARE_RECEIVER_TRACE` and restarted user services.

**Disposition:** previous sender false-Success bug **reproduced then fixed and physically accepted on LAN once**. `#25` remains OPEN for (a) immediate small-file retry **without restarting either app after Cancel**, NOT present in this one-transfer trace; (b) BLE Cancel + repeat, contingent on Google Quick Share genuine off-LAN discovery `#18/#23`; (c) remaining Mode B BWU mid-switch/file-write collision and storage-failure tests. Source CI 6/6 does not imply full Mode B. Compiler dev-profile warnings / bluer future-compat are nonblocking housekeeping; do not mix with observed functional fix. No master/tag/R2, no new executable source changes in this entry.

## 2026-10-25 #25 exact-source six-green gate PASSED; physical install / Pixel acceptance pending

**Pinned code SHA:** `213fd3db7c3fac9dd2e51662d6250dc361006967`. Verified actual check-run results: [Preflight 40227898996](https://github.com/morozhenko-apps/rquickshare/actions/runs/40227898996) 5/5 completed/success (Rust format, Core Rust, core Clippy, Tauri Clippy, frontend) and [Native GTK 40227898978](https://github.com/morozhenko-apps/rquickshare/actions/runs/40227898978) 1/1 completed/success. At readback `dev=ee2b14f888f4792c85759a0287eec9570394a484`, exactly one documentation-only commit ahead of pinned source: no executable, workflow or scripts drift.

**Build/install boundary:** Remote GitHub connector has no access to owner workstation. Two guarded scripts in ChatGPT conversation downloads (NOT committed to GitHub): `rquickshare-cancel-build.sh` builds an uninstalled `.deb`, while `rquickshare-cancel-acceptance.sh` performs CI/hash/source guard, builds, compares all three currently installed ELF binaries against the known verified `af951088` rollback .deb, reinstalls same-version package, verifies installed ELF files against the new `.deb`, restarts two user services with privacy-safe trace and attempts rollback on install failure. Both scripts passed `bash -n` syntactic validation in ChatGPT container only. **NO remote workstation build, install, service restart or device test has occurred from these scripts yet.** The artifact is installed only when the owner explicitly runs the local script.

**Next physical criterion:** on same normal home WLAN with Pixel, send a file that visibly progresses, click **KDE Cancel**, confirm Pixel shows cancellation rather than Success, and send a second file without restarting. Capture only filtered `rqs-receiver event=... elapsed_ms=...` lines, especially `local_cancel_flushed`, `local_cancel_peer_eof`, `local_cancel_drain_deadline`, `local_cancel_write_failed`, `inbound_files_completed`; distinguish each transfer stage. Run fail-closed rollback if unexpected behavior/side effects. #25 OPEN. #23 BLE connect and #18 no-common-LAN discovery remain OPEN. #24 speed deferred. MODE B **PARTIAL** until physical acceptance and remaining BWU/file-write concurrency cases are addressed. No master/tag/R2.

## 2026-10-25 P0 #25 MODE B extended Cancel tests: exact-source final gate active

**Source SHA:** `213fd3db7c3fac9dd2e51662d6250dc361006967`, after `e876e7ef` initial source fix + successful 6 CI. New source/test changes limited to `core_lib/src/manager/session.rs`, `core_lib/src/hdl/gatt.rs`, `core_lib/src/hdl/inbound.rs` and documentation. Shared pure `should_notify_sender_on_cancel()` owns the service/user/terminal decision for LAN and BLE; table-driven tests span active+terminal states and simultaneous request/service cancellation. True encrypted duplex tests cover exact Sharing Cancel and terminal LAST_CHUNK, pre-auth/terminal absence of any bytes, peer EOF, no-EOF deterministic bounded drain, socket closed before first frame, close after first frame before LAST_CHUNK, and missing keys. This is **MODE B partial**, not 100% or formal coverage percentage. Remaining hard corners: transfer in progress during BWU, cancel competing with in-flight file commit, storage failure, and real Android terminal response. [Detailed MODE B coverage/risk table](native-kde-local-cancel-false-success-2026-10-25.md), [#25](https://github.com/morozhenko-apps/rquickshare/issues/25).

**Current exact-source workflow gates (pending at write time):** [Preflight 40227898996](https://github.com/morozhenko-apps/rquickshare/actions/runs/40227898996) and [Native GTK 40227898978](https://github.com/morozhenko-apps/rquickshare/actions/runs/40227898978). Prior iteration failed only Clippy `items_after_test_module` and nightly formatting; the test module was moved to the end of `manager/session.rs` and exact formatting diffs applied. **Do not claim final green until all six jobs complete.** The same-source checked but **not yet executed** local build artifact `rquickshare-cancel-build.sh` only constructs and SHA-verifies a `.deb`; it keeps the previous installed package from `af951088` as rollback and never installs or restarts the user's services. No master/tag/R2 changes. Pixel acceptance after exact-source green must test mid-file KDE Cancel→sender shows Cancelled rather than Success→retry without either app restart. BLE-only Google discovery remains separate [#18](https://github.com/morozhenko-apps/rquickshare/issues/18).

## 2026-10-25 P0 #25 KDE Cancel terminal mismatch: functional patch staged on dev (NOT installed)

Owner clarified **receiver-side KDE Cancel** caused Pixel to display **Success**, although the receiver recorded only 25% of first file with no completion event. The old native path `panel.rs -> service runtime -> binding.control.cancel() -> cancellation token -> native inbound worker until_either_stops()` interrupted the socket without delivering a Sharing Cancel frame to Android, on both LAN and BLE. This is a **verified wire-protocol omission**, distinct from the Google Android UI internals.

Candidate [`e876e7ef`](https://github.com/morozhenko-apps/rquickshare/commit/e876e7efce4b89100d549f52a312eca04148d258) includes source edits `f53186c9/8985b35d/08cff787/b00471ef` and exact rustfmt correction: `core_lib/src/hdl/inbound.rs::send_local_cancel()` uses the existing authenticated two-packet Sharing Bytes send to emit `FrameType::Cancel`, including terminal LAST_CHUNK, before the existing bounded rejected-peer drain; this holds connection until remote EOF or the already defined 2s delivery grace. Both `manager/session.rs` LAN and `gatt.rs` BLE invoke it exclusively for **user-session** cancellation (not daemon lifetime shutdown) before recording terminal Cancelled. Pre-authentication and already-terminal transfers skip encrypted Cancel. Five opt-in privacy-safe `local_cancel_*` markers added. Regression tests decrypt two actual transmitted frames, check Cancel and LAST_CHUNK and no premature close, verify prekey/terminal handling. [Dedicated P0 issue #25](https://github.com/morozhenko-apps/rquickshare/issues/25). [MODE B test inventory, branch map, P/N1-N12, interaction matrix and known gaps](native-kde-local-cancel-false-success-2026-10-25.md).

**Release/physical gate: OPEN.** Early source CI for `b00471ef` had green Core Rust tests, Frontend and Core Clippy but Rust format rejected three exact line breaks; resolved in `e876e7ef`. Do NOT claim six-green on final SHA until both workflows end. More complete MODE B test matrix (including mid-BWU, race cancellation vs final file, sender Cancel and negative peer failures), mutation analysis and real Pixel receiving Cancel remain outstanding. Installed baseline remains `af951088` and does NOT have this patch; current rollback preserved. Do not make/ship new .deb before six source CI PASS; no master/tag/R2. #23 cold BLE, #18 Google off-WLAN discovery still open, #24 throughput deferred.

## 2026-10-25 P0 correction: sender falsely reports Success after Cancel; off-WLAN chooser still absent

**Owner's corrected physical outcome overrides the earlier language "cancel acceptance PASS".** In a single-daemon native LAN run, transfer A reached `inbound_file_progress_25` with no `inbound_last_file_published` and no `inbound_files_completed`; owner explicitly reports that after pressing **Cancel**, **Pixel showed "Success"**, not "Canceled". Transfer B nevertheless entered a fresh inbound session and completed normally, without restarting either application. Therefore only **next-LAN-transfer/reuse=PASS**, **cross-device cancellation terminal status=FAIL/UNRESOLVED**. Do not conflate a subsequent successful session with correct cancellation. The location of the Cancel button (Pixel or KDE) was not yet established. Dedicated [P0 #25](https://github.com/morozhenko-apps/rquickshare/issues/25) tracks false completion and strict terminal semantics, linked to [#23](https://github.com/morozhenko-apps/rquickshare/issues/23).

**Verified source design concern, not yet attributable to the observed Android UI:** `native_desktop/src/panel.rs` Cancel calls `handle.cancel(transfer)`; `core_lib/src/service/runtime.rs` applies `ControlAction::Cancel` on an active managed binding using `binding.control.cancel()`; `core_lib/src/sessions.rs` cancels the token and `core_lib/src/hdl/gatt.rs::run_inbound_worker` returns from cancellation via `until_either_stops`. This path can terminate an active sender-facing connection without a graceful encrypted Sharing `Cancel` frame or explicit Nearby `Disconnection`. Distinct legacy inbound Cancel and received Android Sharing Cancel handlers do call `disconnection()`, not a sender-side terminal ACK. The native terminal and protocol lifetimes need an architectural contract + MODE B coverage. No code mutated pending direction-specific matched evidence. File `Finished` is guarded by final chunk flag, exact length and file publish; no local completion event appears for transfer A.

**Reconfirmed off-WLAN failure:** owner's Google Quick Share on Pixel still fails to show Alcotester with phone detached from home Wi-Fi but both radios enabled. nRF Connect previously independently saw actual Linux FEf3 BLE advertisement and successfully connected/read GATT slot0, proving local BLE transmit/reception. Neither that nor HCI controller enable proves that the **Google Quick Share chooser** recognizes a discoverable, routable peer without shared LAN. Do not repeat equivalent pure-Bluetooth trials as routine regressions. Keep [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) discovery OPEN, keep #23 first BLE repeatability OPEN; BWU throughput [#24](https://github.com/morozhenko-apps/rquickshare/issues/24) secondary/deferred. Current stable tested source `af951088` remains installed; no protocol tweaks, package rebuild, master/tag/R2.

**Next proof gate**: establish whether owner pressed Cancel on Pixel or KDE. Existing journal (not a new transfer) may contain `Transfer canceled` from received Sharing Cancel; capture terminal marker evidence without peer IDs/data. Then design lossless cancellation signaling, end-state invariants, EOF semantics, worker/bridge release, preservation of partial-file cleanup, safe tests and exact-source CI. Google's phone UI "Success" after incomplete receiver transfer is a release-blocking correctness discrepancy until demonstrated otherwise.

## 2026-10-25 P0 #23 repeat-after-incomplete regression: non-GATT PASS (same daemon)

**Latest physical trace (installed fixed binary `af951088`):** one fresh daemon with `visible=18421`, `advertisement_started=18440`. Session A: `inbound_connection_request_decoded=35033`, `inbound_ukey2_client_init=35054`, `inbound_consent_entered=35389`, `inbound_accept_flushed=36718`, `inbound_file_progress_25=37886`, **no completion marker**. Session B **without daemon or desktop restart**: `inbound_connection_request_decoded=65672`, `inbound_ukey2_client_init=65681`, `inbound_consent_entered=65980`, `inbound_accept_flushed=70694`, progress 25/50/75, `inbound_files_completed=72996`. `advertisement_stopped/hidden=78422`, precisely 60.001s after initial visibility, after both sessions. A pre-start `mdns_withdrawal_emitted=1063748` is from a different process; exclude it. **No second `visible`** between A and B, so both ran in the same initial 60s manual Receive window. Neither inbound shows GATT or BWU events: these are non-GATT, likely LAN TCP, transfers. The first incomplete sequence is consistent with the requested Android Cancel, but existing static trace has no terminal Cancel event, so cancellation reason is not independently proven.

**Scope:** same-daemon repeat-after-incomplete on non-GATT transport **PASS**; this did not reproduce the previously reported stale-device error. The one independently successful true BLE/GATT StartNotify→Weave→UKEY2→Consent→file flow remains confirmed in an earlier hardware run. **BLE cancellation→next BLE discovery/handshake is still unverified**: the GATT worker holds the sole bridge until subscription stops; if Android holds Notify open, a new connection can be rejected as busy, but no failed trace shows this. Do not declare this as root cause, remove the mutex/semaphore, change timed visibility, or launch throughput work. Next only if needed: single controlled BLE-based Cancel→retry trace that spans both attempts on the same daemon, with `gatt_notify_*` and `gatt_bridge_busy` stages; preserve normal Wi-Fi in routine usage. #23 P0 OPEN, #18 discovery OPEN, #24 throughput DEFERRED. No production changes, no master/tag/R2.

## 2026-10-25 P0 #23 hardware GATT fix accepted once; cancellation recovery unverified

**Current installed status supersedes 2026-10-24 archived pending notes below.** Operator installed and verified `rquickshare-native_0.9.0-1.debug_amd64.deb`, executable source `af951088a9de6b67a42eda2ebbec0b471e5479d8` (package SHA-256 `a4905ce65c1f8906d07ff6a93fcf35d36f832d947de9e2424eccfab94cbffaad`). All three installed ELF binaries matched the package payload; all six source CI checks completed/success at [Preflight 40138169704](https://github.com/morozhenko-apps/rquickshare/actions/runs/40138169704) and [GTK 40138169705](https://github.com/morozhenko-apps/rquickshare/actions/runs/40138169705). Later `dev` was documentation-only at the time of build; no master/tag/R2.

**New genuine Pixel→Ubuntu BLE/Weave acceptance evidence** (not mere LAN success): `visible=4852`; `gatt_slot0_read=20016`; `gatt_notify_requested/opened/callback_returning/session_opened=23914`; `gatt_weave_connect_write/connect_parsed/confirm_sent=23974`; `gatt_first_frame_forwarded/inbound_connection_request_decoded=24154`; `inbound_ukey2_client_init=24217`; `bwu_offer_sent=28103`; `bwu_tcp_timeout=43103`; `inbound_consent_entered=43280`; `inbound_accept_flushed=48063`; `inbound_files_completed=78037`; `gatt_notify_worker_exited=78038`. StartNotify callback **returned immediately**, first framed inbound request followed 240ms later, no bridge busy, secure user consent obtained and file completed. Therefore the previously fixed source-level callback deadlock has **one actual hardware successful path**, not broad repeated reliability acceptance.

**Crucial network condition:** user previously disconnected Pixel from home WLAN while leaving Wi-Fi and Bluetooth radios enabled for BLE-isolation. Whether home WLAN was rejoined for this final test is **unknown**. Protocol source `core_lib/src/hdl/inbound.rs::do_bandwidth_upgrade` has `timeout(Duration::from_secs(15), socket_receiver)`; timeout returns `Ok(())` and intentionally **continues on the existing encrypted BLE channel**. Exact `bwu_offer_sent→bwu_tcp_timeout=15000ms` and missing `bwu_tcp_switched` prove TCP upgrade failed in this run; BLE-only throughput plausibly explains an earlier ~30-minute video estimate. This is *not* evidence of a speed regression until common-LAN case is tested. Do not alter the 15s protocol timeout or optimize transfers in place of P0 connection reliability.

**Reproduce/cancel gap:** owner first queued a video, discovered host, started send, canceled after seeing ~30-minute estimate; next small-file send showed only cached stale Linux shortcut and Failed, no fresh device until restarting both apps; after restart the reported image transfer succeeded. Capture file date was reset and daemon restarted twice before the filtered log; first line `hidden=321010` belongs to an earlier process, then a separate `start=0`. Thus trace **does not cover** cancelled video or immediate failed rediscovery and cannot identify stale Android cache vs 60s manual Receive expiry vs unclosed GATT worker/slot vs true advertising recovery failure. In successful session `hidden=64852` is **exactly 60 seconds after visible** and occurred during in-flight file transfer, which continued to Finished; expiration by itself does not prove a cleanup bug.

**Next isolated physical acceptance** with normal home WLAN restored on Pixel and PC, nRF Connect fully disconnected: keep the **same daemon alive** for two attempts, send then deliberately cancel one modest transfer, explicitly re-enable manual Receive and immediately send a small image without restarting either app, observe fresh device discovery/consent; capture one uninterrupted, redacted `rqs-receiver event=... elapsed_ms=...` journal for **both attempts** with timestamp file not rewritten. Distinguish `advertisement_started/stopped`, `gatt_notify_worker_exited`, `gatt_bridge_busy`, `gatt_slot0_read`, `bwu_tcp_received/switched/timeout`, and timing of manual 60s expiry. If the second attempt fails, this is the primary next root-cause regression to address architecturally, with Mode B tests, not a restart/timer workaround. Issue [#23](https://github.com/morozhenko-apps/rquickshare/issues/23) remains P0 OPEN; broader discovery [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) OPEN; throughput [#24](https://github.com/morozhenko-apps/rquickshare/issues/24) deferred.

## 2026-10-24 #23 P0: verified nonblocking GATT StartNotify repair, CI/Pixel gates outstanding

**Real source bug confirmed:** pinned `bluer 0.17.3` handles `StartNotify` by awaiting the user notify callback `notify_fn(notifier).await` before replying over D-Bus. Native `gatt.rs` previously **awaited the entire long-lived Weave task JoinHandle inside that callback**, so the subscribe reply was withheld until session termination. This directly violates the BlueR callback contract and is consistent with the intermittent first-send `Connecting…` symptom. The fix returns from the callback after bounded worker admission, while the existing `SessionTasks` / TaskTracker continues owning the BLE worker until stop; the one-permit bridge, explicit consent, session isolation and manual visibility are unchanged. [Implementation `9bf88bd4`](https://github.com/morozhenko-apps/rquickshare/commit/9bf88bd46aabb8258010105e3f791c58a19559cb), [static callback marker `1644ccca`](https://github.com/morozhenko-apps/rquickshare/commit/1644cccadace09cabfb8d0993b87db8bf3879843), diagnostic-first-boundary code `e81fa9ed/6cac9a81`, nightly-format follow-ups `4a5ae490/7b5b2bb6/af951088`. [Detailed evidence and root-cause audit](native-pixel-weave-connection-stall-2026-10-24.md), [issue #23](https://github.com/morozhenko-apps/rquickshare/issues/23).

**Current gate:** executable and test source `af951088a9de6b67a42eda2ebbec0b471e5479d8` has pending [Preflight 40138169704](https://github.com/morozhenko-apps/rquickshare/actions/runs/40138169704) and [GTK 40138169705](https://github.com/morozhenko-apps/rquickshare/actions/runs/40138169705). Nightly rustfmt failures on intermediate commits were corrected using the actual job diffs; no 6/6 assertion until all final jobs complete. **Installed binary is still `ae2e1a40` and does NOT have callback fix**. No new .deb, no Pixel acceptance, no master/tag/R2 actions. Do not request physical retest until exact-source green and guarded rollback verification. Once installed, compare first cold Pixel→KDE target selection→PIN across several *independent* fresh daemons, and a second attempt without restart if first fails. Check `gatt_notify_callback_returning` before authenticated UKEY2, no `gatt_bridge_busy` overlap or packet mixing. Keep #23 OPEN until live reliability is proven; #24 throughput stays deferred.

## 2026-10-24 OWNER PRIORITY RESET: P0 cold Pixel→KDE connection #23

**Primary release blocker:** Pixel quickly discovers Alcotester, yet the first Send sometimes remains `Connecting…` for ~50 seconds and fails without consent; retry can succeed. [Open issue #23](https://github.com/morozhenko-apps/rquickshare/issues/23), [focused trace/Mode B plan](native-pixel-weave-connection-stall-2026-10-24.md). Owner explicitly directed **STOP spending cycles on transfer speed** until this connection is actually resolved. [Issue #24](https://github.com/morozhenko-apps/rquickshare/issues/24) is OPEN/DEFERRED as a recorded secondary regression, not current execution work.

**Installed/current source proof:** latest local package `0.9.0-1.debug` built from `ae2e1a407ceda708af0c75815aeb12c7ff75d130`, verified rollback+sha256+three binary comparisons. All 6 exact-source GitHub Actions jobs are completed/success: [Preflight 40134407179](https://github.com/morozhenko-apps/rquickshare/actions/runs/40134407179) and [Native GTK 40134407156](https://github.com/morozhenko-apps/rquickshare/actions/runs/40134407156). This **does not validate first cold connection**. Last Linux trace contains `visible=11658`, `inbound_connection_request_decoded=22867`, `inbound_consent_entered=23207`, **no GATT Weave stage events**; this demonstrates a successful non-GATT (likely LAN TCP) path and must NOT be mistaken for a failed BLE test. It also recorded ~23.847s from file Accept to Finished for a ~55 MB file; speed details parked under #24.

**Next decisive step using already installed instrumentation (no code or package change):** reproduce **first Pixel→KDE Send on a fresh daemon**, starting Android chooser before manually enabling Receive; if it fails, retry once without restarting and capture only opt-in safe `rqs-receiver event=... elapsed_ms=...` stage lines plus Android UI outcomes. Inspect the first divergence: no GATT subscriber/write → BlueZ/admission; `gatt_bridge_busy` → bridge ownership; CONNECT confirmed but first offline frame absent → write ingress/fragments/Android pause; Core UKEY2 stalled → cryptographic exchange. Earlier successful cold A/B had ~29.207s between Weave confirm and full first offline frame, but did **not fail**; not causal proof. Do not add timing hacks, alter advertise cadence, remove semaphore/peer isolation or weaken explicit consent without a matched failed trace. First actual failed trace drives the architectural patch + deterministic Mode B regressions and final physical acceptance. No master/tag/R2.

## Documentation housekeeping CI readback — 2026-10-24

Latest inspected executable/test source `f61a6735837d2829500af86089158fd81a14df40`
has **five terminal successes and one failure**, not six-green acceptance:
[Preflight40132406104](https://github.com/morozhenko-apps/rquickshare/actions/runs/40132406104)
passes Core Rust, frontend and both Clippy jobs; its
[Rust format job120986828602](https://github.com/morozhenko-apps/rquickshare/actions/runs/40132406104/job/120986828602)
fails specifically at **Check formatting**. Subsequent policy/Python steps in
that job were skipped, not passing. The independent
[Native GTK40132406060](https://github.com/morozhenko-apps/rquickshare/actions/runs/40132406060)
passes. This readback supersedes only the old "not independently verified"
statement below; it does not replace any failed/physical evidence.

Issue24 remains OPEN. No throughput improvement, new installed package,
physical trial, issue closure, master promotion or signing/publication is
claimed. Nightly-format failure is pre-existing functional-source work and was
not changed or hidden by this documentation-only cleanup. The original live
backlog body remains verbatim below; [navigation](README.md) groups dated evidence.

## 2026-10-24 #24 inbound throughput diagnostic source checkpoint

**Physical observation:** owner measured approximately 55 MB transferred in ~8 seconds after Accept with normal receiver settings (roughly 6.9 MB/s end-to-end by eye); connection on that run was prompt. Same file had subjectively faster progress on previous occasions. Existing trace on a **different** physical run proves BLE->TCP upgrade before consent; do not infer the latest 8-second run used that route without matching evidence.

**MODE A diagnostic code (source only):** `core_lib/src/receiver_trace.rs` commit `faad6733` adds seven opt-in static redacted stage names. `core_lib/src/hdl/inbound.rs` commit `8ca32262` adds a per-session allocation-free milestone observer and five deterministic regression tests; `8dd06353` is formatting only; `f61a6735` restricts the ACCEPT trace marker to actual file sessions. New fixed events: `inbound_accept_flushed`, `inbound_first_file_chunk_persisted`, `inbound_file_progress_25`, `inbound_file_progress_50`, `inbound_file_progress_75`, `inbound_last_file_published`, `inbound_files_completed`. No packet, receiver setting, route, consent, file write order, UI update cadence or timeout changed. [Detailed SSOT and test matrix](native-inbound-throughput-diagnostics-2026-10-24.md); [open issue #24](https://github.com/morozhenko-apps/rquickshare/issues/24).

**Gate:** CI not yet independently verified. Final executable/test SHA `f61a6735837d2829500af86089158fd81a14df40` must pass all six source-exact stable jobs (Core Rust/Frontend/Rust format/Core Clippy/Tauri Clippy/Native GTK) before building/installing another debug package. GitHub connector's push-run listing is unavailable here; empty legacy statuses are not CI evidence. No new installed file/physical throughput measurements. Continue with one timed same-file cold/warm transfer and opt-in milestones after green; compare dev vs optimized release only in a separately verified package gate. No master, tag, signed APT or R2.

## 2026-10-24 VIDEO REVISION: fast discovery; first inbound GATT/Weave connection stalls

**First-hand video:** owner attached `rqs-discovery.mp4` (82.457 s, 1080×2400) from Pixel. Alcotester **appears in Pixel's Quick Share list promptly** at roughly t≈12s when KDE manual Receive is activated; tapping Send at t≈15s shows **Connecting… without confirmation for ~50+ seconds**. The first attempt ends **Failed** around t≈66–69s, approximately when the 60-second manual visibility window expires. The owner reenables Receive, taps Send again and Pixel shows **Waiting… with confirmation code** around t≈78–81s; user says retry subsequently proceeds. **These are frame-sampled UI timings, not BLE packet timings; do not reproduce PIN or account from video.** The visibility expiry and failure correlate but are not yet causally connected. This supersedes the earlier undifferentiated claim “slow first discovery.”

**Actual root-cause boundary:** post-discovery Pixel→KDE BLE **Weave connection / authentication before consent**, **not** mDNS browsing or first on-air advertisement. Native Core `core_lib/src/hdl/gatt.rs` maintains one active GATT notify bridge permit and shared incoming write queue. A stale bridge or rejected second notify while busy is a **specific unproven candidate**, as are missed CONNECT control, confirm, first offline frame and UKEY2 stages. [Focused #23](https://github.com/morozhenko-apps/rquickshare/issues/23); [docs & Mode B branch/interaction plan](native-pixel-weave-connection-stall-2026-10-24.md). #18 remains OPEN for wider cold/reliability issues; #22 UI placeholder is independent.

**Source-only opt-in instrumentation:** `ba53f9870478967b4646e670cb1982a31659b561` adds **10 static events** tracing GATT write/notify admission/busy, Weave CONNECT parse/confirm, first offline frame, Core connection request, UKEY2 init and consent entry; tests verify exactly-once semaphore permit ownership and redacted event names. Strict-nightly-format test-only follow-up `5a32a5c6f2d80d950bb9c68266a5a34fd140d50d`. No BLE packet bytes, connection ownership behavior, 60s timer, protocol or storage were changed. **Source exact CI pending** at this checkpoint. No new compiled/installed package contains this instrumentation yet; prior 6/6 source status for unrelated GTK #22 cannot be reused to mark the new source all green.

**Next gate:** after exact-source six-success CI, build one local diagnostic `.deb` and verify rollback/installed binaries. Then record one genuinely fresh first attempt and one retry in same daemon under existing `RQUICKSHARE_RECEIVER_TRACE=1`, supplying only fixed `rqs-receiver event=... elapsed_ms=...` milestones (no raw video/account/PIN). The stage missing from the failed attempt but present in success will determine the architectural patch. Do **not** extend visibility, recycle GATT server, disable the bridge permit or add retry timers as an unproved workaround.


## 2026-10-24 Pixel scan timing: host vs Android observation

**Operator's observation is correct:** the Linux daemon has no event for *Android Quick Share search opened* or *rQuickShare appeared in the Pixel chooser*. In the two supplied cold host traces: first manual Visible→first BLE advertisement took **27/16ms**; from that advertisement to first GATT read took **53.190/17.571s**, but Pixel scan-start time is **unknown** and GATT read is not necessarily first Pixel discovery. Core ready in 54/21ms. mDNS-first-route isn't linked to a verified Pixel. This is evidence of a **cross-host-stage interval**, not measured Android scan latency. **Do not optimize/retry Bluetooth or mDNS based on these unsynchronized numbers.** Use Pixel's built-in screen recording (or `adb shell screenrecord` if USB debugging already enabled) from *before* Quick Share sender UI launch until the PC first appears, optionally gather a cold vs warm comparison; report only relative UI timestamps and whether manual Receive was already active. Complete method and decision matrix: [Pixel-side discovery measurement](native-android-side-discovery-measurement-2026-10-24.md). Tracking [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) OPEN; no discovery production patch.

**Separate #22 current source gate:** GTK premetadata status fix `7dc44071`, strict Clippy cleanup `36c8c751`, legacy ready-fixture correction `e78306b3`. Independently checked exactly six commit checks at `e78306b3882bc1cc73a66a31fe2333994312aacd`: **6/6 completed/success** (Native GTK [40118332040](https://github.com/morozhenko-apps/rquickshare/actions/runs/40118332040), Preflight [40118332028](https://github.com/morozhenko-apps/rquickshare/actions/runs/40118332028)). This **does not** mean the user's installed debug package contains the UI fix; physical result in screenshot is old installed code. #22 OPEN for new installed package, separate from Pixel scan metric. No master/tag/R2 actions.

## 2026-10-24 #22 strict GTK gate: fixture correction

Live phantom card UI root/source remains narrowly fixed in `native_desktop/src/panel/rows.rs` at `7dc4407173aa0fcdb86d00b5adc1c7e634265518`: `Unavailable` shows `Waiting for details`; unnamed peer `Connecting…`; an actual cancellable session/consent and History are retained. Strict Clippy flagged `needless_lifetimes` in the helper at source 7dc, fixed source-only `36c8c75134a92040c38352cb13f92eab59597b0e`. The following exact GTK run compiled but 2 **existing** payload-label test cases failed because their file-name fixtures implicitly used `TransferDetails::default()`, which is intentionally `Unavailable`; both new tests passed. Test-only `e78306b3882bc1cc73a66a31fe2333994312aacd` explicitly sets `DetailsStatus::Ready` for **only** the existing safe filename/manifest label tests. This preserves the security assertions and avoids pretending an unavailable handshake has already delivered zero real files. [Latest GTK validation 40118332040](https://github.com/morozhenko-apps/rquickshare/actions/runs/40118332040) pending at checkpoint; do not claim source all green until terminal job success. New executable build not installed; old displayed `0 files` is historical installed-package screenshot. #22 remains OPEN for KDE acceptance.

**Cold first discovery has NOT been fixed or measured.** The existing opt-in trace already covers startup/core, mDNS browse, advertising and first GATT read; the operator's previous journal query matched `reject_*` only and thus excluded these existing markers. Next ask for safe filtered existing-daemon stage lines and the separately observed Pixel scan-to-first-appearance time; do not change BLE startup concurrency/mDNS probing yet.

## 2026-10-24 second reboot test: Decline 6/6, cold discovery still slow, misleading premetadata row

**Fresh operator report:** after manually stopping/restarting native user services and enabling the opt-in fixed-event trace before starting the process, **three more Pixel→KDE Declines correctly displayed `Declined`**. Total latest result **6/6 successful across two distinct daemon sessions**. Second session stage pairs `reject_last_chunk_flushed → reject_peer_eof`: (70139→70539) **400ms**, (91284→91679) **395ms**, (110484→110933) **449ms**. All three local REJECT payloads and LAST_CHUNKs flushed; no write failures or 2-second deadlines recorded. The user disabled the tracing environment afterward. This does not prove reliability or causality in the face of three earlier Failed observations. [Issue #18](https://github.com/morozhenko-apps/rquickshare/issues/18) remains OPEN, including **slow first Pixel discovery**.

**New screenshot:** compact GTK displays desktop icon, `Transfer · 0 files · Cancel` before payload metadata has been received; nearby availability countdown shows `Visible to nearby devices · 0:12`. Countdown is remaining manual visibility time, **not discovery elapsed**. Source root cause: `TransferDetails::default` means **Unavailable** and initializes `kind=Files, files=[]`, while `panel/rows.rs::payload_label` rendered that placeholder as `0 files`. [Issue #22](https://github.com/morozhenko-apps/rquickshare/issues/22) opened; scoped GTK-only code [`7dc44071`](https://github.com/morozhenko-apps/rquickshare/commit/7dc4407173aa0fcdb86d00b5adc1c7e634265518) now renders `Connecting…` and `Waiting for details` instead; preserves genuine cancellable session. [Design, alternatives, N1–N12 and coverage inventory](native-pending-transfer-metadata-ui-2026-10-24.md). **Source CI and new installed package acceptance not yet completed at writing**; do not close #22 based on original screenshot or source change alone.

**Exact next diagnosis for initial discovery:** existing daemon `RQUICKSHARE_RECEIVER_TRACE=1` contains `runtime_start_requested`, `runtime_core_ready`, `runtime_discovery_worker_ready`, `mdns_browse_started`, `first_mdns_route_admitted`, `visible`, `advertisement_started`, `gatt_slot0_read`, `gatt_header_ready`. Reproduce on *one fresh daemon* and share ONLY these allowlisted elapsed-time markers plus separately observed time from manually opening Pixel Quick Share to first KDE listing on Pixel. The earlier user `grep reject_*` **intentionally filtered these markers out**. Avoid premature Bluetooth/SDK retry/timer changes without knowing which of startup, advertising, SDK browse, TCP probe, Pixel scan or GTK projection actually stalls. No master, release tag, APT publish or R2 changes.


## 2026-10-24 installed diagnostic: Pixel Declined 3/3; correlated frame-stage evidence

The operator supplied a full local build-and-reinstall transcript and filtered, privacy-safe daemon trace (support artifact **Pasted text(10).txt**, October 24). Build from **dev `29ce6d3db1e1df037da9dbdbccf7613505236a97`**, containing executable trace changes `afce867644baab8e4f5e9ffa66e70b9a27b09ff9` and docs-only follow-ups. The operator's preflight script checked **all 6** source-exact GitHub check-runs SUCCESS (independently verified now: [Preflight 40115424146](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424146), [Native GTK 40115424114](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424114)). Core stable: **796 passed, 0 failed, 8 filtered**, 4/4 and 4/4 integration; GTK SUCCESS. Local build/embedded `.deb` verifier and independently checked package SHA-256 **OK**. Original rollback `0.9.0-1` SHA **OK**. `apt install --reinstall` completed with 0 removals; version `0.9.0-1.debug`; **all three installed ELF executables `cmp` MATCH** extracted local diagnostic `.deb`. The `_apt` permission warning is nonfatal.

**Owner explicitly reports all three Pixel→KDE→Decline attempts now display `Declined` on Pixel (3/3 PASS).** The earlier build with essentially the same wire/drain logic had **3/3 Failed**, so this is **not enough to claim permanent/causal repair**. The new build's code is principally fixed-stage instrumentation and enhanced test proof, **not a protocol schema or timeout change**. Do not credit tracing itself as a valid protocol fix or claim GMS acknowledged a frame.

Filtered `RQUICKSHARE_RECEIVER_TRACE=1` output reported for each observed consent:
| Attempt | reject_consent | reject_write_start | reject_payload_flushed | reject_last_chunk_flushed | reject_peer_eof | Last chunk → EOF |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 34383 ms | 34383 | 34384 | 34385 | 34792 | **407 ms** |
| 2 | 135685 ms | 135685 | 135686 | 135686 | 136130 | **444 ms** |
| 3 | 140736 ms | 140736 | 140737 | 140738 | 141213 | **475 ms** |

There are **no** `reject_write_failed`, `reject_peer_read_error` or `reject_drain_deadline` markers in these 3 filtered attempts. The daemon locally flushed **both** encrypted Sharing Response payload and LAST_CHUNK. The peer then signaled EOF approximately 0.4–0.5 s later. Sender peer EOF **does not independently prove Google Android decrypted/accepted either message**; the UI's `Declined` is separate operator observation. The operator ran `systemctl --user unset-environment RQUICKSHARE_RECEIVER_TRACE` after collecting diagnostics; **no permanent debugging environment is requested**. Settings, files, history and protocol unchanged.

**Disposition:** keep [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) OPEN for intermittent reproducibility and cold/warm discovery measurement. With 3 prior failures and 3 latest successes, a high-confidence reliability/causal gate needs repeat cold-start tests across daemon restarts and ideally a matched FAIL trace (if it recurs); check especially partial/absent `reject_last_chunk_flushed`, unexpected `reject_drain_deadline` vs peer EOF and BWU route. Avoid adding arbitrary delays, changing Reject to Cancel or attributing unproved failure to Pixel UI. Normal Core 796/796 + GTK check gate is green, but **no release-wide 10x Mode B / 8 filtered races** yet. No master/tag/APT/R2 operations.

## Current source-exact stable verification — 2026-10-24

Executable source **`afce867644baab8e4f5e9ffa66e70b9a27b09ff9`**; final source [Preflight 40115424146](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424146) and [GTK 40115424114](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424114).
- Core Rust stable job `120935659444` terminal SUCCESS: **796 passed, 0 failed, 0 ignored, 8 filtered**; both integration binaries **4/4** each. All three diagnostic regression tests individually `ok`, including full encrypted response+LAST_CHUNK read and opt-in marker formatter. Strict Rust format SUCCESS, Core Clippy SUCCESS, frontend SUCCESS, Native GTK SUCCESS.
- **Tauri Clippy job `120935659581` still IN PROGRESS as of this documentation checkpoint**, therefore **5/6 verified, not full 6/6**. Do not ask operator to install until terminal source-exact sixth gate is confirmed.
- No physical Pixel trace run on this source. The last user-reported data remains **3/3 Pixel `Failed`**, despite preceding drain change; **root cause not established**. Next step is the single user-approved trace on a fresh installed debug candidate, not another protocol/timing guess. See [plan](native-pixel-reject-wire-trace-2026-10-24.md).

## 2026-10-24 diagnostic code validation handoff

Current **executable source SHA** `afce867644baab8e4f5e9ffa66e70b9a27b09ff9`. Only docs will follow until evidence arrives. Four distinct code/test commits since the previous fully-validated package:
- `43d9e3ab`: two scoped source files, privacy-safe rejection markers, full two-frame encrypted delivery test and discriminating EOF/timeout drain outcomes; **no packet/timeout or consent behavior changes**.
- `17de08b0`: nightly test-format-only one-line adjustment.
- `8a7c5e8f`: tests return the `RejectionDrainOutcome` from Tokio spawned closure rather than `()`; first CI exposed this compile problem.
- `afce8676`: nightly test-only closure formatting correction to one line for exact formatting profile.

CI on `43d9e3ab` had rustfmt failure; CI on `17de08b0` had *test-only* `expected (), found RejectionDrainOutcome` compile error; CI on `8a7c5e8f` required the final nightly closure formatting. **Do not report these intermediate CI runs as successful.** [Final exact-source Preflight 40115424146](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424146) and [GTK 40115424114](https://github.com/morozhenko-apps/rquickshare/actions/runs/40115424114) are running at this checkpoint; verify **all six** check-run terminal SUCCESS before asking the operator to rebuild/install. No new device test has been performed; Pixel still FAILED on 3/3 previous release attempts. No local artifact path, contents hash, installed executable or journal for this new diagnostic source is verified.

Once CI is green, request one diagnostic dry-run package built from current dev, preserving old `0.9.0-1` rollback and passing extracted-vs-installed binary comparisons, then restart user services under `systemctl --user set-environment RQUICKSHARE_RECEIVER_TRACE=1` before the app starts. User supplies ONLY `rqs-receiver event=... elapsed_ms=...` lines filtered from journal, not raw logs; `systemctl --user unset-environment` after testing. A line `reject_last_chunk_flushed` confirms local write/flush of final encrypted frame, **not application-level receipt by Pixel**. Absence of that marker localizes failure earlier; `reject_peer_eof/read_error/deadline` shows how our transport observed completion, not GMS result. Full root cause remains OPEN.

## 2026-10-24 SECOND PHYSICAL FAILURE (3 of 3) — protocol proof investigation

Operator response after installing previous two-second-drain candidate: **all three Pixel→KDE→Decline attempts still showed `Failed` on Pixel**. The previous delay hypothesis **did not resolve observed behavior**. KDE→Pixel→Decline already PASS; manual-only Receiver and compact repeat/history remain accepted. #20 closed with user acceptance; #18 remains OPEN.

Current priority: prove the in-flight encrypted Sharing `Reject` **both chunks** are flushed, and classify how the connection ends, not increase delay or change status arbitrarily. [Detailed evidence, ranked hypotheses, branch map and test matrix](native-pixel-reject-wire-trace-2026-10-24.md). **Opt-in evidence source** `43d9e3ab4abd730ae58f0171aed857c58af70b97` changes `core_lib/src/receiver_trace.rs` and `core_lib/src/hdl/inbound.rs` only: existing `RQUICKSHARE_RECEIVER_TRACE=1` allowlisted monotonic event names (`reject_consent`, `reject_write_start`, `reject_payload_flushed`, `reject_last_chunk_flushed`, `reject_write_failed`, `reject_peer_eof`, `reject_peer_read_error`, `reject_drain_deadline`). Adds deterministic real encrypted **second-chunk** sequence/id/size/flag confirmation, and enum-valued bounded EOF/deadline drain tests. No wire schema, peer trust, packet bytes, delay or storage change. First source CI failed only one nightly formatting line; follow-up **test-only** `17de08b0524d81ca980e22640499e4e07c18f30a`. Exact-source terminal CI NOT YET VERIFIED at this checkpoint; do not direct user to rebuild/install until final gate. No trace has been collected from the Pixel yet, so no root-cause claim.

After six terminal checks PASS, rebuild one new local debug .deb from dev and verify source package hashes and installed binaries against rollback, then run **one isolated** Pixel→KDE→Decline with opt-in env. Extract **only** `rqs-receiver event=[a-z_]+ elapsed_ms=[0-9]+` and omit full journal; unset diagnostic environment on completion. Use stage gaps and EOF/deadline to establish failure, then design actual architectural correction. No release promotion.

## FINAL source-exact CI for bounded inbound Reject drain — 2026-10-24

**Executable source `3a35097f5c0afe3140d2b52f81ebc79677faf1a9`: ALL SIX check-runs terminal SUCCESS.** [Preflight 40111072020](https://github.com/morozhenko-apps/rquickshare/actions/runs/40111072020) and [Native GTK 40111071885](https://github.com/morozhenko-apps/rquickshare/actions/runs/40111071885) were inspected independently. Core stable **795 passed, 0 failed, 0 ignored, 8 filtered**, plus 4/4 and 4/4 integration tests; both newly added rejection tests logged `ok`: real encrypted Sharing REJECT delivered/read and peer EOF before transport retirement; silent/chatty drain bounded to 2s. Rust fmt, Core Clippy, Tauri Clippy+desktop tests, frontend and GTK all **SUCCESS**. The prior source `57739042` had only one nightly rustfmt test layout failure, corrected in `3a35097f` without production changes; earlier cancelled jobs are not the final CI status.

**Still NOT a Pixel fix validation:** none of this source has been built/installed/tested on the operator's Pixel/KDE. Next build fresh current `dev` after this 6/6 CI, verify SHA and rollback, reinstall only after transfers idle / tray Quit and byte compare all three extracted binaries. Physical Pixel→KDE Decline needs a new repeat test to see whether Pixel shows `Declined` consistently; also ensure KDE→Pixel Decline, manual Receive and successful sending do not regress. Do not close #18 until actual acceptance; cold discovery remains separately unmeasured. The full final release-wide Mode B (including 8 filtered race-sensitive tests and hardware scope) has not been claimed. No master/tag/R2 changes.

## Latest acceptance and remaining blocker — 2026-10-24

- **Validated on the installed 0.9.0-1.debug package `e67134a`:** KDE→Pixel manual Decline shows KDE **Declined**; manual Hidden at startup, Receive 60s on demand/expiry, restart stays Hidden, and repeated incoming transfer removes the previous Completed row from compact Devices with original saved content and History preserved. These are **operator-reported physical PASS**.
- **Reproduced remaining failure:** Pixel→KDE incoming user Decline displays **Failed** on Pixel again. This was intermittent, including one earlier Declined observation, so #18 remains OPEN; exact Google device transport reason has not been independently traced.
- **#20 CLOSED** after user physical acceptance and earlier exact six-check GTK/Core CI; **#21 remains OPEN only to audit reapplication of saved temporary/Visible during live settings reload**, despite normal startup/manual/expiry acceptance.
- **#18 correction (uninstalled):** `core_lib/src/hdl/inbound.rs` source `577390421778627bf23443f264fdfddff035f9d5` retains a successfully flushed encrypted Sharing `Reject` connection for up to 2s, returns early on peer close/read error, cannot accept payloads after terminal and remains worker-cancellable. This matches the **delayed connection-close** lifecycle published by Google's Nearby Sharing service; no new wire frame or storage changes. [Design/positive and N1–N12 test inventory](native-pixel-decline-drain-2026-10-24.md). The first CI revision failed **one rustfmt test layout**; exact follow-up test-only `3a35097f5c0afe3140d2b52f81ebc79677faf1a9` corrects it. Source-exact preflight/GTK for final executable SHA were still **in progress** at documentation checkpoint; **NO NEW LOCAL INSTALL, NO VERIFIED PIXEL FIX**.
- Next: inspect terminal CI (all 6 expected success) and test evidence, rebuild one new debug `.deb` from post-fix code only then replace the installed package with original rollback preserved; Pixel→KDE Decline repeatedly, plus one successful image and KDE→Pixel Decline; user-provided observations required. Do not promote `master`, tags or R2.

## 2026-10-24 latest physical installation gate — verified by operator

The operator ran the full guarded replacement procedure and supplied **terminal success evidence**. The operator's installed binary came from **local candidate folder `rquickshare-candidate-e67134a`**, built from a documentation-only head after code fix `a3976ad7`. The later `ded1f1d2` commit changed only two managed BLE/TCP **test expectations**, not executable production paths; its **exact-source Preflight [40109101697](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101697) and GTK [40109101791](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101791) are now 6/6 completed SUCCESS** (independently verified and gated by the operator's successful `PASSED == 6` check). No false claim that the installed binary was built from a later test-only SHA is needed.

Observed `sha256sum -c` for both the original rollback `0.9.0-1` and new candidate reported `OK`. The package manager reinstalled **`rquickshare-native 0.9.0-1.debug amd64`** successfully with **0 removals**. `dpkg-deb -x` plus `cmp` against the new package produced **MATCH** for all three paths: `usr/bin/rquickshare`, `usr/libexec/rquickshare-daemon`, `usr/libexec/rquickshare-preview-helper`. Therefore the currently installed program files are exactly the packaged candidate, not stale leftovers. The nonfatal `_apt` unsandboxed-root local permissions notice is not a package failure.

**Physical behaviors after installation are NOT YET TESTED / REPORTED.** Stage A now: launch native app, assert no active Receive timer and Pixel cannot freshly discover KDE while Hidden (allow for Android cache), activate Receive manually, verify advertised 60-second timer and automatic expiry, close+reopen to ensure no temporary reset/auto-advertise. Stage B: KDE→Pixel send then Pixel explicitly Decline, expect live KDE `Declined` and History; Pixel→KDE inbound one successful transfer, then send same attachment again and verify completed prior row no longer shares compact card with new request, but History and saved file(s) remain. Separately verify normal Linux discovery of Pixel while Linux inbound visibility Hidden. No device pass or issue closure until operator returns observations. Original rollback installer remains present; do not delete. No master/tags/R2/published release.

# Active backlog and release handoff (SSOT)

**CURRENT engineering snapshot (2026-10-24):** #11 and #9 have been implemented in source at `2545e0519bb8f236e1948993eef28b31d2e58ac0`; exact [Preflight 40093504811](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504811) and [Native GTK 40093504760](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504760) completed **6/6 SUCCESS**, Core **790/790 + 4/4 + 4/4**. The next step is **installed current-source KDE/Pixel acceptance** plus remaining #13/#18 UX, not reimplementing #9 or #11. All 10 issues remain OPEN pending their separate gates. Previous source baseline before this work: `c34db153b29095293a6ebc2ac3349857efb50480`. **Use this page for today's task order and gates.** GitHub issues own mutable task state; the historical journals in [current-state.md](current-state.md), [full-feature-roadmap.md](full-feature-roadmap.md) and per-slice evidence docs are *not* an alternate live to-do queue. Refresh issue statuses and source-exact CI before claiming newer completion. This page is not evidence of physical tests.

## 0. Latest owner decision and current source — 2026-10-24

- **Owner supplied actual installed-package evidence:** `sudo apt install --reinstall` of `rquickshare-candidate-e2304e9/rquickshare-native_0.9.0-1.debug_amd64.deb` succeeded, package `rquickshare-native 0.9.0-1.debug amd64`, SHA-256 package valid, all THREE `cmp` binary comparisons `MATCH`. APT user `_apt` warning was an ordinary local-permissions sandbox fallback, no installation failure. Original rollback `0.9.0-1` .deb and SHA retained. This **disproves stale-binary hypothesis** for auto-receive (not a closed physical issue).
- **Owner decision is explicit:** inbound discoverability/Receive must be **manual only on every daemon launch or restart**. Even saved `visibility=2` Temporary must not rearm and an old persisted `visibility=0` Visible must not silently advertise on launch. Background discovery of recipients for sending remains independent. Shared native+legacy startup-core factory creates live `Invisible` and new default absent setting is Invisible; no settings file was deleted/rewritten. Existing explicit manual Visible/Temporary actions remain. Production commit `17f59c76`, nightly format follow-ups `74ed2dac` and `e47e6a9c`. Exact-source CI and fresh build/installed acceptance **pending**.
- **Android result advanced:** operator says Pixel now displays **Declined** correctly after Linux declines, so that earlier specific Android `Failed` scenario is a device-side PASS. **Opposite direction is an independent open defect:** KDE compact card hides a terminal outgoing `Rejected` after Pixel Decline. Source `e47e6a9c` shows terminal Outbound Rejected status in GTK, **and crucial Core protocol fix `eba8a2f5`** maps actual Pixel `Status::Reject` to terminal `State::Rejected` rather than `Disconnected`; three other failure codes keep their separate disconnected semantics. New wire/rejection regression tests added, exact CI/physical acceptance pending; #18 stays OPEN for cold discovery too.
- #20 old Completed row fix had earlier 6/6 source CI and a **correct package installed** at `e2304e9`, but no reported repeat-transfer acceptance after that reinstall. Leave #20 OPEN until owner observes historical Finished row retire on new inbound + History/real file retained.
- Current execution/release contract: [manual-only and Declined design/test plan](native-manual-receive-and-decline-2026-10-24.md). Latest code SHA `e47e6a9c7a1d37d095b2c8fa6b2627048200aa6c` (final fmt correction), after which only docs are allowed until exact CI gate. **No master, tag, signed release, R2, installed new source package or full release Mode B.** Historical 2026-10-24 snapshots below are retained as chronological evidence; this Section 0 supersedes their interim statuses.

**LATEST CORRECTION: [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) required a Core protocol-state fix in addition to GTK.** Actual Pixel `Reject` was being classified as `Disconnected` by `core_lib/src/hdl/outbound.rs`, so UI-only feedback would never activate. New separate production source `eba8a2f5` classifies only explicit Reject as terminal `Rejected`, other errors remain disconnected; the real protocol-response/wire-disconnect regression fixture needed test-only `.await` fix `a3976ad7`, **current exact source**. New Core CI and installed acceptance still pending. [Design + evidence](native-manual-receive-and-decline-2026-10-24.md).\n\n**NEWEST TEST GATE:** `a3976ad7` produced 791 passing Core tests, two outdated BLE/TCP expected-state failures; source contract now intentionally reports sender `Rejected` on explicit Pixel Decline. Updated both integration expectations in test-only `ded1f1d2`; new exact-source CI in progress. Owner already built previous candidate but **must not install it** for this gate. Latest operator build SHA/hash not independently inspected. See Section below for failure evidence and [#18](https://github.com/morozhenko-apps/rquickshare/issues/18).\n\n## 1. Release and branch contracts (do not lose)

- `dev` is development/integration; `master` is protected and release-only. Commit docs separately from code/tests. Do not create production tags, deploy R2, change installed KDE, or promote to `master` without separate release authorization.
- **First public native Linux release:** `0.9.0`, **native-tag namespace** `native/v0.9.0` (not the older upstream `v0.9.0`), Debian `rquickshare-native_0.9.0-1_amd64.deb`. The installed `0.9.0-1` reported on the operator's KDE is **older than the latest `dev` fixes**. It is not proof that newer fixes work on hardware. [Issue #14](https://github.com/morozhenko-apps/rquickshare/issues/14) / [native tag migration](native-tag-namespace-migration-2026-10-22.md) / [release strategy](product-release-strategy.md). No GitHub Release was observed at snapshot.
- **1.0.0:** remaining functional fixes and peer acceptance first; [20 locales #17](https://github.com/morozhenko-apps/rquickshare/issues/17) are the **last feature**; *then* per-locale audit, final stable/race-sensitive/coverage/Mode B gates, installed-device acceptance, security and release approval.
- **Essential ordinary sharing before broad parity:** [native-essential-priorities.md](native-essential-priorities.md). Preview UI/encoding, resume/dedup, Google accounts/trust/QR, streams/Remote Copy, Wi-Fi/app/Apple parity, Windows/macOS native ports, expanded packaging remain deferred long-term, not accidental release blockers. Existing safe preview input/decoder code is not authorization to reintroduce its UI.
- **BWU risk decision:** #8 **A2.2 explicitly selected**. Keep existing fast Google-compatible BWU; authenticated TCP candidate routing is future architectural Option A **deferred and NOT fixed**. Expected-panic regression tests *reproduce* current untrusted-ID route-consumption risk. No claim of a security fix/secure release without separate residual-risk evaluation. [Decision](bwu-option-a-admission-design-2026-10-24.md).

## 2. Current GitHub issues (12 open after installed-card findings)

| Workstream | Issue | Executable state / remaining gate | Current priority |
| --- | --- | --- | --- |
| LAN IPv4 advertisement | [#11](https://github.com/morozhenko-apps/rquickshare/issues/11) | **Source done, 6/6 CI SUCCESS** at `2545e051`, three targeted tests `ok`. Physical current-version Pixel BWU and VPN-on/off acceptance remains OPEN | SOURCE DONE / ACCEPT OPEN |
| mDNS deterministic SDK tests | [#9](https://github.com/morozhenko-apps/rquickshare/issues/9) | **Source done, 6/6 CI SUCCESS** at `2545e051`, four production-driver tests `ok`. Actual SDK multicast/closure and physical Pixel/KDE discovery lifecycle remain OPEN | SOURCE DONE / SDK ACCEPT OPEN |
| Discovery restart worker | [#12](https://github.com/morozhenko-apps/rquickshare/issues/12) | Cancel/join + closed browse receiver hot-loop fix done on `dev`; [Preflight 40085426478](https://github.com/morozhenko-apps/rquickshare/actions/runs/40085426478) and [GTK 40085426477](https://github.com/morozhenko-apps/rquickshare/actions/runs/40085426477) SUCCESS. Open for hardware/SDK regression and #9 tests | TEST/ACCEPT |
| Discovery tombstones | [#19](https://github.com/morozhenko-apps/rquickshare/issues/19) | Removal fix implemented; [Preflight 40080949959](https://github.com/morozhenko-apps/rquickshare/actions/runs/40080949959) / [GTK 40080949968](https://github.com/morozhenko-apps/rquickshare/actions/runs/40080949968) SUCCESS. Physical disappear/reappear and similar-name devices needed | TEST/ACCEPT |
| Old Completed row alongside new consent | [#20](https://github.com/morozhenko-apps/rquickshare/issues/20) | **CLOSED — source 6/6 CI and operator-reported installed Pixel/KDE acceptance PASS**: new inbound retires previous Completed from compact card; History and saved files remain. No name-based identity grouping | COMPLETED |
| Manual-only receive at every startup | [#21](https://github.com/morozhenko-apps/rquickshare/issues/21) | **Physical PASS**: Hidden startup/restart, explicit 60-second Receive/expiry, outbound discovery still works. **OPEN for further settings reload/reapply security audit** because persisted mode 2 can remain on disk. | PHYSICAL PASS / EDGE AUDIT OPEN |
| Incoming attention/scroll | [#13](https://github.com/morozhenko-apps/rquickshare/issues/13) | Single-target navigation and focus/scroll source done in slices; GTK test evidence [40037554078](https://github.com/morozhenko-apps/rquickshare/actions/runs/40037554078) SUCCESS. Full Devices counter/pulse contract and KDE two simultaneous consent requests / viewport acceptance remain open | UI + ACCEPT |
| Cold discovery, bilateral decline | [#18](https://github.com/morozhenko-apps/rquickshare/issues/18) | KDE→Pixel Decline **operator PASS**. Pixel→KDE Decline **operator FAIL** (Pixel Failed): suspected immediate receiver socket teardown after encrypted Reject; narrow peer EOF / 2s drain source `57739042`, fmt test-only `3a35097f`. Exact CI + installed Pixel proof pending; cold/warm measurement remains. | NEW SOURCE / ACCEPT OPEN |
| Verified route identity | [#16](https://github.com/morozhenko-apps/rquickshare/issues/16) | Owner selected **A**, merge only after actual crypto proof connecting peer certificate to exact discovered route. Production currently uses anonymous peer certificate source; no proven cross-route identity. Never group by name/IP/MAC | ARCHITECTURAL BLOCK |
| BWU TCP authentication | [#8](https://github.com/morozhenko-apps/rquickshare/issues/8) | **A2.2 owner-deferred**; existing fast BWU preserved, P0 route theft/DoS risk known; new protocol-compatible proof needed; release security evaluation still required | DEFERRED P0 |
| Native GTK localization | [#17](https://github.com/morozhenko-apps/rquickshare/issues/17) | **Last feature before 1.0.0 final gate**. Canonical 20 locales are in [Pixel/identity/locale contract](native-pixel-acceptance-identity-locales-2026-10-23.md), including pt-PT intent, zh-Hant, Arabic RTL and 20/20 semantic audit | LATER |
| Signed native release | [#14](https://github.com/morozhenko-apps/rquickshare/issues/14) | Real `native/v0.9.0` tag, CI Mode B, signing environment/ruleset, R2/APT, installation/update verification and owner approval not completed; distinguish local `.deb` from publication | RELEASE GATE |

Issue numbers/links are canonical. A task with code and green CI is still OPEN if user/physical acceptance is missing; conversely, do not repeat completed code because its issue remains open.

## 3. Next authorized execution sequence and stopping rules

1. [x] **#11 — source complete.** Narrow unusable-IPv4 exclusion, focused tests and exact 6/6 CI verified. **Still OPEN:** current installed package/Pixel/VPN acceptance; do not repeat the source change.
2. [x] **#9 — source testability port complete.** Private event+reachability injection, four real production-driver event-loop tests, 6/6 CI verified. **Still OPEN:** physical real-SDK browse, shutdown, generation races and Android/Windows acceptance; controlled external inputs do not replace these gates.
3. **NEXT / owner physical operator action:** [current-source KDE/Pixel acceptance](native-current-source-physical-acceptance-2026-10-24.md). **Operator built and verified** the uninstalled `0.9.0-1.debug` on the clean Ubuntu 26.04 machine from `87af15c7` (only docs after source `2545e051`): build PASS, built-in package verification PASS, independent SHA-256 PASS, independent package verification PASS. Prior installed `0.9.0-1`. **Next mandatory gate: find and verify original `0.9.0-1` rollback `.deb`; diagnostic replacement NOT YET INSTALLED.** After explicit idle/tray Quit and manual install, inspect #11/#9/#12/#19/#13/#18 hardware behavior. All physical cases are presently UNTESTED; do not close issues.
4. Resolve #16 authenticated identity proof only with trustworthy peer-certificate source and exact route binding; otherwise keep distinct route entries with explicit recipient choice.
5. Implement #17 after all functional issues; full 20-locale contextual/security and UI acceptance.
6. Final Mode B + #14 controlled signed release/production publishing **only after independent release authorization**. Independently decide residual #8 risk; the A2.2 backlog deferral is *not* a security release waiver.

## 4. Mode B, test status, and how to hand off

- Most recent exact-code [Preflight 40087230904](https://github.com/morozhenko-apps/rquickshare/actions/runs/40087230904) and [GTK 40087230963](https://github.com/morozhenko-apps/rquickshare/actions/runs/40087230963) **SUCCESS** at source `6974da5e`: Core `783 passed, 0 failed, 0 ignored, 8 filtered` plus two 4-test integration binaries. Includes two **expected-vulnerability panic** BWU diagnostics, not safe admission. Latest docs-only #8 deferral `c34db153`; no newer code claim in this snapshot.
- Per-root-file coverage, stable vs race-sensitive group evidence, true mutation results and physical real-device proof must be distinguished from green compilation. No invented percentages, no unobserved `dev` acceptance. Do not repeat complete 10× suite after every small change: follow repo CI cadence and final release gates.
- **Handoff format for every task:** issue ID, selected architecture/STOP decision, source SHA, docs SHA, exact CI run/job links and terminal status, actual hardware/installed version tested, undone scenarios, branch/rollback point. Keep source and docs commits separate. Update issue on each verifiable milestone and close it only after all applicable gates. A link to a historic failed job must not be presented as the current state.
- Historic text in [current-state.md](current-state.md) and individual dated plans is immutable evidence of what was known *then*; if contradicted, add a superseding timestamped note or link here. Never delete proof merely to shorten a document.

## 5. Historical interim source handoff (superseded by Section 6)

- #11 implementation `bb2c5ee84b51cf7f2546f4cad484f31809e1d110`, final nightly-format test source `e1cf43b6139c08389765db910ec14d592ed3291b`. [Preflight 40092947491](https://github.com/morozhenko-apps/rquickshare/actions/runs/40092947491) and [GTK 40092947495](https://github.com/morozhenko-apps/rquickshare/actions/runs/40092947495) are the source-exact evidence to verify. The older Preflight's **Tauri Clippy was cancelled** after a subsequent source commit, not a demonstrated source failure. Do not claim this isolated SHA 6/6 until independently verified; the newer #9 source carries the #11 fix for a combined validation.
- #9 event/probe driver source `c444ca84c1b5746d47477777631deca49b02c6b7`, format/fixture corrections `df502aa89064f37b1ec2dc9e9dfb31543c8ad2c2` and `2545e0519bb8f236e1948993eef28b31d2e58ac0`. This changes **only** `core_lib/src/hdl/mdns_discovery.rs` and adds 4 tests. First CI revealed only a real `mdns-sd::IntoTxtProperties` test-fixture slice-type compile error and fmt diff; fixed without changing domain assertions. **Final exact-source CI still pending**. [Plan/branch map](p0-mdns-event-ports-2026-10-24.md).
- No `master`, installed binary, R2, signed APT, physical Pixel or Windows acceptance changed. Leave #9/#11 OPEN until final evidence and physical gates are met.


## Final-source automated gate checkpoint

Code SHA `2545e0519bb8f236e1948993eef28b31d2e58ac0` contains the earlier #11 address correction plus this private #9 port refactor; no later executable changes at this checkpoint. [Preflight run 40093504811](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504811) and [Native GTK run 40093504760](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504760) were inspected individually.

At checkpoint, **Core Rust tests SUCCESS (job 120871102186)**: `790 passed, 0 failed, 0 ignored, 8 filtered`, both separate integration binaries `4/4`; all **seven new methods** from #11 and #9 printed `ok`. Rust format SUCCESS, Core Clippy SUCCESS, frontend SUCCESS, Native GTK SUCCESS. **Tauri Clippy still IN PROGRESS**, so this is **5/6 verified, NOT a complete all-green CI claim**. Inspect terminal Tauri job `120871102157` before marking the final source fully green; preserve this specific SHA/run proof rather than relying on the GitHub latest badge. No hardware or real-SDK multicast acceptance has been performed. A2.2 BWU #8 remains unresolved by design.


## 2026-10-24 terminal exact-source CI — supersedes interim five-check checkpoint

**Executable SHA `2545e0519bb8f236e1948993eef28b31d2e58ac0`: ALL SIX CHECKS completed SUCCESS.**

- [Preflight 40093504811](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504811): Rust format, Core Rust tests, Core Clippy, Tauri Clippy (`120871102157`) and frontend all completed success.
- [Native GTK 40093504760](https://github.com/morozhenko-apps/rquickshare/actions/runs/40093504760): Native GTK widget tests and strict lint completed success.
- Core job `120871102186`: **790 passed, 0 failed, 0 ignored, 8 filtered** in stable suite; both independent integration binaries **4/4**. All seven new named tests across #11 (three) and #9 (four) appeared individually as `ok`.
- Combined source gate is green; prior interim `5/6` checkpoints and older cancelled Tauri job were superseded by the *newer* source-exact `6/6` run. This is not an executed release-wide full Mode B 10×, mutation-engine result or real Android/Windows/physical mDNS proof.
- #11 remains **OPEN** until actual new-binary KDE/Pixel LAN/VPN acceptance; #9 remains **OPEN** until real SDK/physical cancellation, browse, reachability and error acceptance. No `master`, installed daemon, signing, tag or R2 work occurred. Production rollback points: #11 `bb2c5ee8` (+ rustfmt test-only `e1cf43b6`); #9 `c444ca84` (+ follow-up fixture/fmt corrections `df502aa8`, `2545e051`).

## 2026-10-24 physical screenshot and scoped follow-up

- Operator on KDE/Pixel 6a reports: after launch `Receive` shows `Visible to nearby devices · 0:45`; on clicking Linux `Decline`, Android sender shows `Failed`; repeated transfer leaves a prior `Mercury · MP4 · 5,264,330 bytes · Completed` row alongside a new `Mercury · MP4 · 5,264,330 bytes · code / Accept / Decline` consent. Note screenshot says **MP4**, not image; do not claim file-kind misclassification until the submitted filename/type is independently established. User has not supplied a direct package-version readout after installation; only the earlier verified `0.9.0-1.debug` candidate and current physical screen are known.
- **#20 code change** `d886146aafd0b479eb5efad7fc44101c40090bb9` removes old completed results from compact card on new incoming nonterminal transfer, preserves History and queued result reveals. UI-only `native_desktop/src/panel/results.rs`, two regression methods. Needs exact CI terminal statuses and updated installed package on KDE before acceptance.
- **#18 Android Failed mapping** confirmed by operator, source sends encrypted `Status::Reject`; no proven Pixel reception or timing and no speculative wire change.
- **#21 startup policy** is separate from actual mDNS discovery and from Receive 60s UI. New-install default is Visible; stored temporary can restart a 60s window. Needs owner decision to preserve saved intentional Visible and make temporary truly one-shot. No source/settings modification yet.

## 2026-10-24 scoped compact card automated gate

Production code `d886146aafd0b479eb5efad7fc44101c40090bb9`; solely nightly-rustfmt correction `2de1bb70d7b864e3faae849891176d510f678e35` is the final executable SHA for these tests. The first GTK run [40100886094](https://github.com/morozhenko-apps/rquickshare/actions/runs/40100886094) reported formatter failure, no assertion failure or functional diagnosis. The corrected [Native GTK 40101207886](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101207886) completed **SUCCESS**: native GTK source formatting/strict Clippy, **10/10** in one test partition, **92/92** in core GTK widget/controller tests and 0 failures. Both new regressions for retiring stale card result on consent and parallel/unrelated/terminal arrivals printed `ok`; exact job `120893552998`. [Preflight 40101208122](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101208122) Core stable `790/790`, two integrations `4/4` each, Rust format, Core Clippy, frontend **SUCCESS**. At this checkpoint Tauri Clippy `120893731534` still **IN PROGRESS**: source exact 5/6 successes, **do not infer full final gate** until that job terminates. No updated KDE package installed from this source; physical screenshot remains pre-fix repro.

Next: verify terminal Tauri job; update the issue/backlog. Then build distinct `rquickshare-candidate-<commit>` local debug package, package checksum+contents, graceful idle replacement with preserved rollback .deb and **verify installed executable hashes** after reinvocation. The package version may still be `0.9.0-1.debug`; directory name + SHA and binary comparison, not dpkg version alone, establish freshness. Do not touch `master`, public `native/v0.9.0`, APT or R2.


## 2026-10-24 FINAL exact-source CI (supersedes earlier 5/6)

**Production+test source `2de1bb70d7b864e3faae849891176d510f678e35` — ALL SIX checks terminal SUCCESS.**

- [Preflight 40101208122](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101208122): Rust format `120893731492`, Core tests `120893731359`, Core Clippy `120893731514`, Tauri Clippy + Desktop Rust tests `120893731534`, frontend `120893731585` all completed success.
- [Native GTK 40101207886](https://github.com/morozhenko-apps/rquickshare/actions/runs/40101207886): `120893552998` completed success; GTK 10/10 auxiliary + 92/92 unit/widget/controller assertions, including the **two new Results regression methods individually `ok`**.
- Core `790 passed, 0 failed, 0 ignored, 8 filtered`; 4/4 + 4/4 integration. The eight filtered race-sensitive tests are not marked as executed. Mode B full 10×, mutation engine and physical user tests have **not** been executed on this change.
- Rebuilding/installing the package on operator KDE is still OPEN; the currently shown MP4 screenshot predates the #20 fix. Old/rollback installed `0.9.0-1` and candidate debug package may share a version with new debug build, so **use source SHA + extracted-vs-installed executable hashes**, not dpkg version alone, before marking acceptance. No `master`, published artifacts or tag.

## 2026-10-24 operator build and confirmed startup root cause

- Operator fast-forwarded clean dev to `e2304e9` and rebuilt `0.9.0-1.debug`. Rust build and native package validation passed; independently checked SHA-256 OK. This is a **new uninstalled candidate**, not installed acceptance; package comparison/reinstall must distinguish two debug packages with the same dpkg version.
- Read-only local settings probe printed **SAVED VISIBILITY: Temporary (60 seconds)**. This confirms persisted mode 2, whose 60-second timer is currently re-armed at runtime startup. Update [#21](https://github.com/morozhenko-apps/rquickshare/issues/21); do not change permanent Visible preference or default policy without a separate decision. Test new consent vs historical Completed row after current-source install (#20), and maintain independent Pixel Decline→Failed investigation (#18).

## 2026-10-24 owner local build / previous-source CI failure / test-oracle correction

Owner reported **"собрал"** after being instructed to build current `dev` from `e67134ad` (production+tests through `a3976ad`). No terminal output or file hash from this latest local build was supplied, so its exact path and checksum are **not independently verified**; do not infer new installation or package replacement. **Do not install that build as acceptance candidate** because exact executable source `a3976ad778035d1580e1614a99d7bdffdee18a81` finished **5/6 checks success, Core Rust tests failure** (Core run 40105012066). Core job `120904955524`: **791 passed; 2 failed; 8 filtered**. The two failed tests were managed BLE and managed TCP integration oracles still expecting outbound `Disconnected` after recipient explicit Reject, while the corrected real protocol now produces `Rejected`. All 791 remaining source tests passed, including the new real `process_consent` Reject regression. This is a test expectation changed by an intentional state contract, **not evidence of a hidden protocol break or validation of the physical UI**.

**Test-only new commit** `ded1f1d2d86ff889174ec04666612bf112f5cc82` updates exactly the two affected integration test assertions to expect terminal outbound `Rejected` (keeping inbound `Rejected` and accepted `Finished` semantics and preserving actual BLE/TCP integration coverage). Zero production code files changed. Source CI [Preflight 40109101697](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101697) and [Native GTK 40109101791](https://github.com/morozhenko-apps/rquickshare/actions/runs/40109101791) were started; all terminal statuses were **not yet verified at this doc checkpoint**. Do not assert 6/6 until checked. Rebuild **once** from `dev` containing at least `ded1f1d2` after source gate becomes green, verify source SHA, package sha, original rollback and installed binary comparison; then owner tests manual Hidden startup, restart, 60s Receive and expiry, Pixel Decline→KDE Declined, repeated inbound compact result & History. No master/tag/R2/release updates.

## Executed source tests (2026-10-24; exact SHA)

Executable source **`5a32a5c6f2d80d950bb9c68266a5a34fd140d50d`**. On [Preflight 40121322026](https://github.com/morozhenko-apps/rquickshare/actions/runs/40121322026) and [Native GTK 40121321985](https://github.com/morozhenko-apps/rquickshare/actions/runs/40121321985): Core stable job 120953343876 **798 passed, zero failed, 8 filtered**, plus two 4/4 integration suites. Explicit successes include `gatt::tests::given_control_frames_and_overlapping_notify_when_admitted_then_only_one_bridge_owns_peer` and `receiver_trace::startup_timeline_contract_tests::given_weave_stage_markers_when_rendered_then_events_are_unique_and_redacted`. Rust format, Core Clippy, frontend and GTK **completed SUCCESS**. **Tauri Clippy still in progress** at this checkpoint: 5/6 completed success, **not 6/6**. No exact-source installed binary or device handshake trace yet. User should be offered CI-gated local diagnostic installation only after all 6 are successful; no daemon restart or install claimed here. Full release Mode B (8 filtered race tests etc.) remains separate.

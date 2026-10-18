# MODE B P0: Authenticated forged Settings2 view and visibility receipt contracts (2026-10-18)

## Stage 1. Repository scan and scope
Read `agent.md`, `docs/mode-b-completion-audit-2026-10-18.md`, `core_lib/src/service/{settings_service.rs,client.rs,client/settings.rs,client/visibility.rs,client/tests/visibility_forged.rs}`, `core_lib/src/identity.rs` and `.github/workflows/{mode-b.yml,coverage.yml}`. Baseline `fa5f549b`; last source-tested code `4b814f6a` has Preflight and Native GTK SUCCESS (765 core library tests, 80 native GTK).
Mode B, tests and docs only. The existing adversarial test-only manager/operations/visibility interfaces form a **real private D-Bus** authenticated by ordinary `NativeClient::connect`. Extend that fixture with an `Settings2` interface, whose `Read` and `Reload` responses share the same injected `SettingsView` used by `Visibility2.Set`. No production/mocks of the client, no product-policy changes or release/master actions.

## Stage 2. Risk and invariants
P0: a same-UID, properly owned daemon may still send forged or corrupted typed settings. `read_settings` and `reload_settings` must refuse invalid `SettingsView` replies as `ClientError::Protocol`; a subsequent valid response must work without a poisoned cache. `set_visibility` must retain the exact original pending intent after a **structurally decodable but invalid** `SettingsView`; no later malformed receipt may clear it; only an explicit retry with a valid envelope and matching request may complete it. Requests and tokens must not appear in `Debug`.

## Stage 3. Exhaustive scoped test inventory
| Production artifact | Branches / risk | Level | Planned tests |
| --- | --- | --- | --- |
| `client/settings.rs::checked_settings` | reject wrongly typed semantic fields; valid envelope | real D-Bus contract P0 | shared field matrix, 2 tests |
| `client/settings.rs::{read_settings,reload_settings}` | reject invalid Read and Reload; accept repaired response | real D-Bus integration P0 | 1 named test |
| `client/visibility.rs::{set_visibility,retry_visibility,has_pending_visibility}` | corrupted successful `Set` result must retain exact pending, valid later retry clears | real D-Bus integration P0 | 1 named test |
| `client.rs::{connect,wire}` | real broker-authenticated unique owner, exact method dispatch | fixture exercised | counted with above |
| `settings_service.rs::SettingsView::valid` | reject every scalar bound and canonical name, token, directory, request combination | existing production validator through D-Bus | shared table of **18** wire-representable + **3** NUL-unrepresentable invalid field variants |
| Invalid D-Bus *signature*, reply with attached FD, wrong owner/UID, transport closure in-flight | separate protocol risks | full fake daemon transport, not yet implemented | OPEN, not falsely counted |

## Stage 4. Branch map
`checked_settings`: fds nonempty -> Protocol (OPEN); tuple deserialize error -> Protocol (OPEN); `!view.valid(epoch)` -> Protocol (planned); valid -> Ok (planned).
`valid(epoch)`: epoch exact; source <=2; visibility <=2; -1 <= port <=65535; card_mode <=1; both names valid; name_status <=3 and if !=0, resolved_name nonempty; directory bytes <=12288 and no NUL; error <=4; applied_live_revision <= revision; token empty only if !available/error==1 otherwise lowercase 64 hex; last_request empty or lowercase 32 hex. Each planned invalid field has an independent mutation target. `set_visibility`: exact pending on Protocol, even on repeated malformed replies; accept corrected matching receipt once. Existing test `given_authenticated_success_reply_with_foreign_request...` already covers valid SettingsView with foreign last_request and need not be duplicated.

## Stages 5–6. Positive / N1–N12 and interaction matrix
| Flow | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Settings Read/Reload | valid recovery | bad name/token/request/epoch | -2/-1/65535/65536 port; 12288/12289 dir | private broker path, no real network | repeat bad Read+Reload and recover | single mutable client, NA race | malformed semantic fields | exact epoch and canonical IDs | Unicode control/name check | no GUI, NA | reject forged visible/configured fields | no payments, NA | no file operations, NA |
| Visibility Set receipt | matched retry succeeds | invalid receipt fields | max scalar and overlimit | real local bus; no link errors injected | repeated Protocol leaves exact pending | second command already blocked in previous suite | corrupted typed response | no authority based on malformed receipt | Unicode in fields | explicit retry | redact token/request; no lost original | NA payments | no real writes, NA |

Interactions: **18 D-Bus-serializable malformed field variants × both `Read` and `Reload` routes**; same malformed fields × pending visibility `Set` reply; then corrected reply. Three distinct authenticated D-Bus method paths tested, each relevant branch touched with real transport. No sleeps or random time-dependent expectations.

## Stages 7–8. Counts and production-file coverage map
Two new named async tests plus test-only `Settings2` interface and table helper, **18** wire-representable malformed variants each applied to both read routes (36 errors) and Set route (18 errors), plus **3** NUL-invalid string cases asserted directly against the production validator. New production file coverage: `core_lib/src/service/client/settings.rs` checked settings validation, read and reload; `core_lib/src/service/client/visibility.rs` retained pending on Protocol; `core_lib/src/service/settings_service.rs` SettingsView validation field branches. No source or new assertions for hardware BlueZ, UI, filesystem or relaunch.

## Stages 9–10. Implementation and mutation review
Implement only two test methods in `core_lib/src/service/client/tests/visibility_forged.rs` plus test-only Settings2 fake service; use `assert!(!view.valid(EPOCH))` for each malformed input *and* assert actual client returns `ClientError::Protocol`, unchanged pending state and exact wire Set call count. Mutations targeted: remove any single validator conjunct; modify <=/>= boundary; allow uppercase token/request; ignore broken name; return stale request; clear pending on Protocol. Some other tests already cover foreign last_request and unreadable store; do not repeat. Mutation review is planned analysis, not automated score.

## Stages 11–12. Execution and completion
Require exact-source Preflight (Core tests, formatting, Clippy, frontend, Tauri) + Native GTK. Measure line/function/region coverage separately when an on-demand run is available. `mode-b.yml` is **workflow_call only**, guarded for release validation; do not bypass its authorized entrypoint in testing mode. Strict full 10× on final current test source and actual hardware acceptance are still OPEN. Global Stage 12 remains NO. Rollback test file and document only; no master merge.


## Stage 11 first execution: protocol-feasibility correction before code fix

- First exact-source [Preflight 39174485470](https://github.com/morozhenko-apps/rquickshare/actions/runs/39174485470) compiled and ran both new methods but Core Rust failed **765 passed / 2 failed**. The first failure in each method was the `device-name-nul` case: actual client reported `Err(DaemonChanged)`, not `Err(Protocol)`.
- **Root cause is test design, not evidence of an accepted corrupt setting:** D-Bus `STRING` is required to contain valid UTF-8 without embedded NUL. The fake method cannot transmit a response with NUL inside its string fields; the ensuing transport/owner failure is not a semantic validator return and cannot be asserted to be `Protocol`. The sibling `resolved-name-nul` and `directory-nul` variants have the same protocol-level impossibility.
- **Corrected test matrix, without weakening any applicable assertion:** put three NUL-bearing values in a separate **direct `SettingsView::valid` rejection** category (N1/N6/N7; no D-Bus wire claim), keep the remaining **18 fully D-Bus-serializable malformed variants** in the genuine authenticated `Read`, `Reload` and `Set` route tests (54 failed-response assertions), and retain exact pending/last-request checks. If an encodable malformed response produces `DaemonChanged` rather than `Protocol`, treat it as a new red failure, not a generic acceptable error.
- Original scoped inventory total remains 21 invalid field variants: 18 D-Bus-wire cases + 3 D-Bus-type-unencodable cases. No product change or new public API. Implementation must follow this plan correction and be rerun on a new SHA.

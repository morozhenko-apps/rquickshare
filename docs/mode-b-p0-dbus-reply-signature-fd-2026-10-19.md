# MODE B P0 — Authenticated D-Bus wrong-signature and unexpected-FD replies (2026-10-19)

## Stages 1–2: repository scan, risks, invariants and selected approach
**Mode:** B — Testing only. **Branch:** `dev`. Baseline `dd9d0294`; prior identical code source `85faca64` passed [Preflight 39174986191](https://github.com/morozhenko-apps/rquickshare/actions/runs/39174986191) and [Native GTK 39174986234](https://github.com/morozhenko-apps/rquickshare/actions/runs/39174986234).
Read `agent.md`, `docs/mode-b-completion-audit-2026-10-18.md`, `core_lib/src/service/client.rs`, `core_lib/src/service/client/{visibility.rs,settings.rs,tests.rs,tests/visibility_forged.rs}`, `core_lib/src/service/bus/tests.rs`, and the current `mode-b.yml`/coverage workflows. Existing client `wire` authenticates same unique sender and requires zero reply FDs for all relevant methods; `read_visibility` and `checked_settings` deserialize exact typed bodies; `retry_visibility` retains pending intent on `Protocol`. Existing injected D-Bus fixture tests semantically invalid but correctly shaped replies. Wrong D-Bus body *signature* and valid typed payload with extra file descriptor remain untested on the actual bus.

**Invariant:** An authenticated, same-owner local service can still be untrusted. Wrong body signature must yield `ClientError::Protocol`, not successfully deserialize or mutate client state. Any unexpected attached FD must be rejected at `wire` even if a valid `VisibilityView`/`SettingsView` is also sent. A rejected `Set` response must preserve exact pending request; a second user attempt is blocked, and explicit retry must not silently succeed. No reply may transfer filesystem authority to a client that did not request a descriptor. Reject without inspecting/opening/consuming attached FD.

**Test-infrastructure choice:** Add separate `#[zbus::interface]` test-only implementations of `Settings2` and `Visibility2` for each malformed response kind. They own the real service name on the existing private D-Bus, with the existing healthy test-only `Manager2` and `Operations1` bootstrap. The standard `NativeClient::connect` and unchanged production `wire` execute every test. This avoids ambiguous racing manual reply injectors, unsafe raw-message fabrication and all production changes.

## Stage 3: exhaustive scoped test inventory
| Artifact | Risk and executable branch | Planned method / count |
| --- | --- | --- |
| `NativeClient::read_visibility` | same unique owner, wrong typed `Read` reply vs valid typed view with unexpected FD | 2 integration tests, each exercises this path |
| `NativeClient::{read_settings,reload_settings}` / `checked_settings` | wrong `SettingsView` signature; valid settings plus unrequested FD; decode or authentication reject | same 2 tests, two method paths each |
| `NativeClient::{set_visibility,retry_visibility,has_pending_visibility}` | wrong typed `Set` response or unexpected FD never clears pending | same 2 tests, explicit double-retry/ownership assertions |
| `client.rs::wire/reply_sender_fds` | expected zero FD, source owner authentic, malformed content | same 2 tests; verified through full path |
| Wrong FD with exactly the expected typed body signature and **no** `h` in signature | raw protocol manipulation required and validity depends on broker/SDK constraints | OPEN; tests below carry legitimate FD as an additional field and assert FD rejection; do not overclaim signature-independent isolation |
| Forged sender name, malformed DBus headers, reply serial manipulation | separate security work | OPEN; previous sender-authentication unit tests exist |

Two **new named Tokio tests** with two adversarial fixture modes and no changes to production code.

## Stage 4: branch map
`wire`: `name_owner()` check → authenticated `reply_sender_fds(reply, owner, 0)` rejects unexpected FD; wrong-signature response without FD proceeds to `read_visibility` / `checked_settings`, whose typed deserialize fails `Protocol`. `retry_visibility`: `Protocol` retains pending; new `set_visibility` during pending returns `PendingOperation`; repeated explicit retry calls only the same original request. `Settings2::{Read,Reload}`: both malformed routes are rejected independently. Real healthy-success variants are already covered in prior source-matched D-Bus suites.

## Stages 5–6: Positive / N1–N12 and interaction matrix
| Contract | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Wrong signature | Existing healthy fixture | wrong typed body (s vs struct) | single wrong reply | real D-Bus | retry exact retained request | single mutable client; N/A race | typed wire mismatch | authenticated same owner | N/A time | explicit client retry | no visibility trust or token disclosure | N/A payments | no files |
| Unexpected FD | Existing zero-FD healthy fixture | unexpected `h` in reply | exactly 1 attached descriptor | D-Bus Unix FD transport | retry does not clear pending | N/A same reason | typed view with additional FD | reject unrequested authority | N/A time | explicit client retry | no read/open of FD; secrets not logged | N/A payments | FD is created only by fixture and must not be adopted |

Interaction matrix: response class {wrong signature, valid typed+FD} × method {Visibility2.Read, Settings2.Read, Settings2.Reload, Visibility2.Set} × first/second `Set` attempt, with two independent new test methods. No timeouts other than existing `scenario` guard, no sleeps, no OS adapters or device network.

## Stages 7–8: test estimate and production-file coverage map
Expected 2 test methods, at least 8 method-route rejection assertions, 2 pending+second-command guards, 2 repeat-retry errors, counters proving two explicit `Set` invocations, and no false discovery state. Production touched **only as test targets**: `core_lib/src/service/client.rs` `reply_sender_fds` branch, `core_lib/src/service/client/visibility.rs` `Read` and `retry_visibility`, `core_lib/src/service/client/settings.rs` `read_settings`, `reload_settings`, `checked_settings`; `core_lib/src/service/visibility_service.rs` and `settings_service.rs` supply typed payloads. Fixture-only edits: `core_lib/src/service/client/tests/visibility_forged.rs`; this plan and completion documents. File-level coverage % not claimed without llvm-cov.

## Stages 9–10: implementation and mutation review
Implement separate tiny `#[zbus::interface]` test-only faulty services with `Read/Reload/Set`; wrong-signature services return a plain string; extra-FD services return a *valid typed view plus one borrowed file descriptor*. Verify wrong signature via real response decode, FD via production `wire`. Assert exact errors, original pending intent, unchanged attempts count after locally rejected second command, read-only route rejection, and no client recovery without an explicit request. Manual mutation candidates: bypass sender/FD check; deserialize wrong signature as default; clear pending on `Protocol`; retry with new token; accept stale/foreign reply. Mutation kills not measured until actual instrumentation.

## Stages 11–12: execution / completion gate
Run existing Preflight & Native GTK on identical test SHA; inspect named Core test logs, counts, Clippy, formatting and Tauri. Do not alter `master`, production semantics, application packaging, or the release-only `workflow_call` strict 10x guard. A validated final-source 10× entire suite, current-source detailed line/branch coverage, Android/BlueZ/mDNS hardware acceptance, and complete repo-wide Stage 12 remain OPEN. Rollback: remove the two test methods and test-only service implementations plus docs.


## Stage 9 implementation checkpoint

- Documentation-first plan at `c2b14f54`.
- Implemented **two** new named Tokio tests and four test-only `#[zbus::interface]` services in `core_lib/src/service/client/tests/visibility_forged.rs` at `a0d02b67`. The test-only services return a wrong STRING signature or a correct typed view plus an extra Unix FD on all four paths. The regular private D-Bus service owner and production client authenticate naturally. The tests assert the exact `ClientError::Protocol`, one retained pending intent, blocking of a second request, and exactly two explicitly initiated `Set` attempts.
- [Initial Preflight 39208235091](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208235091) reported a **Core Clippy failure** from a redundant unused local `AsFd` import inside the test fixture (`-D warnings`). Rust formatting and frontend succeeded. That import was removed without changing the cases at `4d9da957`; no product code changed.
- Exact-source retry [Preflight 39208447151](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447151) and [Native GTK 39208447106](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447106) started. Mark them successful only after the jobs actually complete.
- The FD test intentionally returns a valid typed data *value* plus an FD argument, so it verifies an unrequested-FD rejection but does not claim a valid standalone typed body signature with a mismatched UnixFD metadata count. Hardware acceptance and full strict 10x gate are separate.


## D1–D5 scoped completion report (source `4d9da957`)

**D1 — Execution and commands.** The actual code under test is `core_lib/src/service/client.rs` `wire`/`reply_sender_fds`, `core_lib/src/service/client/visibility.rs` `read_visibility`/`set_visibility`/`retry_visibility`, and `core_lib/src/service/client/settings.rs` `checked_settings`/`read_settings`/`reload_settings`. New tests and four intentionally faulty interfaces are test-only in `core_lib/src/service/client/tests/visibility_forged.rs`. To run locally: `cd core_lib && cargo test --all-features --lib visibility_forged`; complete Core: `cargo test --all-features`. CI run IDs: [Preflight 39208447151](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447151) and [Native GTK 39208447106](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447106).

**D2 — Category membership (not additive independent test counts).** Positive: **0 new normal-success tests** (already covered by the previous honest service suites); N1 malformed typed input **2**; N2 one unexpected FD, **1**; N3 local D-Bus unexpected reply **2** (no physical network loss); N4 exact client explicit retry and no implicit replay **2**; N5 concurrency **N/A** (serialized mutable client); N6 malformed wire shape **2**; N7 unexpected descriptor/file authority and authenticated unique sender **2**; N8 locale/time **N/A**; N9 explicit retry/pending user behavior **2**, without real UI action; N10 no secret debug output **previous coverage**, no new dedicated privacy log assertion; N11 monetization **N/A**; N12 filesystem failure **N/A** (fixture opens a local descriptor only). Category membership overlaps.

**D3 — Actual test assertions.** Exactly **2** named Tokio test methods with two authenticated D-Bus faulty-server modes. Each checks three read-only paths and two `Set` responses, for **10 malformed response rejections** across both modes. Also checks exact pending preservation, rejection of a conflicting second command, and exact call counters (1 then 2) proving no hidden server replay. New Core unit tests were logged independently as `ok` on the exact source under [Preflight 39208447151](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447151): **769 passed / 0 failed / 0 ignored**, 8 separately filtered tests, auxiliary integrations 4+4 passing. Format, frontend and Core Clippy jobs **SUCCESS**.

**D4 — Non-claims and refactor requirements.** Although the FD case includes a valid typed `VisibilityView`/`SettingsView` value, the extra `Fd` argument necessarily changes the D-Bus body signature; this confirms the **unexpected-FD rejection** without proving that the body-signature parser and FD detector are independently mutation-killing for a forged raw reply with an unchanged signature and extra FD metadata. Do not hide this gap. A separate raw-message server with full broker-compliant FD header attachment could measure that independence if protocol encoding permits it. Sender spoofing, serial misbinding, BlueZ/mDNS hardware and installed desktop workflows remain separate.

**D5 — Execution and gate.** Initial exact source `a0d02b67` produced one **test-only Core Clippy warning** (redundant function-local `AsFd` import) and was corrected by `4d9da957` without changing test logic. This document was updated while the complete Tauri job and Native GTK cleanup were still executing; their final workflow conclusions must be independently verified before marking this scoped suite fully green. The 10× full automated Mode B gate, per-production-file line/branch coverage, and repository-wide Stage 12 remain **OPEN**. No production code, `master` or release entrypoint was changed.


## Final scoped exact-source green gate

- Exact test-source `4d9da957d8a6db004a2c86f795a7c4c09d38a624` [Preflight 39208447151](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447151) **COMPLETED SUCCESS 5/5**: Core Rust **769 passed / 0 failed** (eight separately filtered probes; two auxiliary integration groups of four each), Tauri desktop **10/10** and Rust format/frontend/both Clippy jobs **SUCCESS**. Both new original `visibility_forged` test names were separately logged `ok`.
- Same exact source [Native GTK 39208447106](https://github.com/morozhenko-apps/rquickshare/actions/runs/39208447106) **COMPLETED SUCCESS, 80 passed / 0 failed**.
- The initial Core Clippy failure was **test-only** and fixed as documented; no production change or domain assertion relaxation. New scoped P0 signature/FD test gate: **PASS**. Full strict 10x after this source and global Mode B Stage 12: **OPEN**.

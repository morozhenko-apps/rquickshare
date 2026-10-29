# rQuickShare pre-Mode-B closure register (owner instruction 2026-10-27)

Status **IN PROGRESS / NOT READY FOR MODE B**. MODE A feature/defect closure only. Owner authorized making pre-acceptance fixes, without running **full Mode B** or publishing. Owner explicitly accepts preserving the #8 BLE→TCP A2.2 compatibility risk as a tracked **single release exception**, not permission to ignore any other security issue or to claim TCP sockets authenticated. No further unapproved protocol trust waivers follow from #8.

Development branch `dev`, snapshot code `378834ac270517c90b9823c2448b8f1150ce3e8c`. At inventory, GitHub lists **17 OPEN issues including #8**, zero open PRs. Source baseline before the new i18n code `ac152f69` had exact Preflight `40548418257` **SUCCESS** and Native GTK `40548418258` **SUCCESS**. New dynamic-settings source exact CI Preflight `40561114184` and Native GTK `40561114130` were **running**, not yet terminal. Never infer installed-device PASS from GitHub CI.

## Explicit issue acceptance register

| Issue | Source stage visible in dev | Remaining BEFORE final Mode B or release/owner gate | Ready to close now? |
| --- | --- | --- | --- |
| **#8** | Fast A2.2 path retained | Accepted compatibility/security residual must be explicitly documented in release decision; no cryptographic TCP socket authentication asserted | **Excepted**, leave OPEN and track |
| **#9** | Deterministic mDNS SDK-event/TCP seam implemented, 6/6 source CI historically | Real mDNS SDK, negative reachability/cancellation/withdrawal evidence and physical IPv4/Pixel lifecycle | NO |
| **#11** | Unroutable IPv4 exclusions implemented, 6/6 source CI historically | Installed KDE/Pixel LAN BWU, VPN on/off and fallback behavior | NO |
| **#12** | Discovery replacement cancels + joins owned prior worker; source tests | Real repeated start/stop, SDK shutdown ownership and release evidence | NO |
| **#13** | Devices inbound count/pulse, latest request focus and GTK scroll implemented; source tests | Actual KDE two-request focus/scroll/nonstealing, reduced-motion themes and accessibility acceptance | NO |
| **#14** | Independent 0.9.0 signing/distribution architecture prepared | Separate production signed APT/R2/tag/release approval and reproducible artifact; **post-gate publication not automatically authorized here** | NO |
| **#16** | Fail-closed exact-route registry and authenticated-proof event seams implemented | **Material unresolved**: ordinary Pixel session credential provisioning and cryptographically trustworthy route identity association not proven. Route registry documentation explicitly says *no provider currently wires this in*. Must never group anonymous advertised routes by names/addresses; a distinct trust/public API decision requires STOP FACTOR | NO |
| **#17** | Extensive 20-locale catalogs for menu, review, composer, image, history, settings/chrome/status/error/warning plus dynamic files/bytes/PIN/name/path | Residual GTK/tray/reveal/notification/accessibility/packaging visible strings, professional semantic review 20/20, PT-PT, zh-Hant, Arabic RTL/fonts, narrow-card overflow, completed exact-source CI and user physical acceptance | NO |
| **#18** | Cold-start instrumentation and encrypted-Decline handling source, earlier physical observations | Quantitative cold/warm trace, first GATT/Weave stages and bilateral Pixel Decline terminal status after latest package | NO |
| **#19** | mDNS tombstone removal contract and runtime/consumer fixes described in issue | Real remove/readd, multicast SDK, lifecycle and installed latest-source Devices check | NO |
| **#21** | Manual-only default Hidden/startup and saved Temporary startup guard source; earlier installed source user-reported PASS | Prove settings reload/reapply cannot silently re-enable receiving for stored temporary/visible mode; controlled cold-restart/60-second expiry on latest installed source | NO |
| **#22** | Pre-metadata `Connecting…` / `Waiting for details` / `Unsupported content` source fix and tests | Installed GTK/Pixel visual acceptance of premetadata, Cancel retained, transition to Ready | NO |
| **#23** | Measured owner video confirms first BLE Weave connection stalls despite quick discovery; trace stage markers added | Identify and correct actual GATT/Weave negotiation failure, then repeat first-request vs second-request tests on Pixel | NO |
| **#24** | Trace and performance investigation instrumented | Real bytes/sec, Accept→Finished, TCP/BLE path and GTK repaint cadence before blaming throughput | NO |
| **#25** | Inbound Cancel/terminal race analyzed; parts of #26 source fix address adjacent race | Determine actual Cancel initiator/peer terminal frame and fix Pixel false-Success for incomplete/cancelled transfer; installed controlled test | NO |
| **#26** | Committed BWU Cancel flush ownership fix and targeted regressions included, prior 6/6 source | Installed cancel-during-committed-BWU on Pixel↔KDE with encrypted terminal proof and truthful trace | NO |
| **#27** | Windows compact native adapter and DPI/Win32 geometry design only | Implement actual native Windows platform/service/tray UI, Windows x64 build/CI, then Win11 physical DPI/placement/transparent-corner/transfer acceptance. Not present in Linux GTK code | NO |

**Do not close issues merely because they have a source fix.** After current-source exact CI + appropriate owner/device acceptance, update issue with SHA/log/case and then close it per AGENTS.md. Keep 0.9.0 release (#14) distinct from later 1.0.0 and don't quietly turn Windows into a Linux release blocker unless the agreed target scope requires it.

## Execution order / invariants

1. **Security and reliable current Linux transport:** verify #16 proof path / decide missing trust source only via explicit STOP FACTOR; isolate unresolved #8 exception. Correct #23/#25 and confirm #26, #9/#11/#12/#19 lifecycle together with source/unit/integration tests before hardware exercise.
2. **KDE/Pixel field acceptance:** use a matching latest debug `.deb` with verified binary checks and rollback. Inspect initial discovery/GATT states (#18/#23), simultaneous requests, inbound/outbound Cancel/Decline/false Success (#25/#26), BWU throughput (#24), manual visibility (#21), premetadata #22 and #13. Never invent a local install/trace or ask CI to substitute for physical evidence.
3. **Remaining native UI/other platform scope:** finish #17 string source inventory in `docs/native-gtk-i18n-remaining-surface-inventory-2026-10-27.md`, all-locale context/RTL tests and Android→KDE actual UI; build #27 Windows in its own work track with Windows-specific CI+physical tests, if its release scope is confirmed.
4. **Only after non-Mode-B feature and acceptance gates:** run the standalone final exhaustive MODE B stable + race-sensitive suites with per-file coverage/mutation and final acceptance, as already required by the project's Universal Agent SSOT. Do NOT run it during this pre-gate task.

## Source under this execution

- Typed GTK dynamic Settings name/path plan `docs/native-settings-dynamic-safe-text-2026-10-27.md`, source `378834ac`; original edited GTK entry and settings controller authority not modified. Static source audit PASS, source-exact Preflight/GTK **IN PROGRESS**.
- No user-machine shell, Pixel pairing/video input, installed Linux binary or Windows CI runner is accessible to this agent through GitHub; consequently none of those physical gates can be reported passing in this turn. Their cases are owner-run acceptance steps, **not missing functional source proof to be guessed**.
- This register is an audit/plan, not a command to push release, change branch protection or delete history. All historical backlog records are retained. Roll back each future executable slice independently.

**Current release readiness:** NOT READY / #8 exception recorded / no final MODE B / no package publication.

## 2026-10-27 focused no-regression source audits

### #16 authentication provider gap, avoid fake Google identity

Current `core_lib/src/service/runtime/route_registry.rs` explicitly documents that **no production provider wires independent verified route generations into grouping yet**. The registry has correct synthetic/fail-closed infrastructure: proof leases with a 60-second limit, socket-address generation checks, no anonymous name/IP merging and no fallback after a pinned dispatch. `core_lib/src/service/runtime.rs::observed_route_proof` accepts evidence only from live authenticated outbound managed sessions, while `peer_authentication::VerifiedPeerCredential` explicitly warns a session credential is *not itself proof of mDNS/GATT advertisement ownership*. `PeerAuthentication::anonymous` is still the standard source for ordinary inbound/outbound compatibility sessions.

Consequently there is no defensible basis to mark two ordinary Pixel advertisements as one proved identity. To actually close #16, implement authentic real peer credential provenance **and** route-to-certificate proof (a product security/public contract), or obtain owner's explicit revised acceptance. This is a **STOP FACTOR limited to the additional trust/provisioning decision**, not a reason to stop unrelated localization/bug fixes. Never inherit #8's explicit plaintext BWU exception as a blanket waiver of this trust decision.

### #21 no implicit saved-temporary rearm on load (source reasoning; hardware still open)

`core_lib/src/settings.rs::StartupSettings::default` chooses `Visibility::Invisible`. `core_lib/src/service/runtime.rs::configuration_only` also constructs an Invisible core, and `core_lib/src/service/process.rs::startup_core` starts the managed receiver Invisible regardless of stored Temporary=2. `settings_service::Writer` initializes `active_visibility` from its loaded snapshot and only issues an `Apply` under **`save && running`**, while `bus.rs` applies visibility only for a dispatched `Apply` or manual quick visibility operation. A normal read/reload (`edit=None, quick=None`) does not submit a visibility write. This supports manual-only startup against the specific reported stored-Temporary replay concern.

It does **not** prove every accelerated save/restart, reconnection and race invariant: a later explicit `Save` may change visibility if `active_visibility` differs; this is a deliberate user action with an existing consent gate. We have **not** run a new deterministic scheduler test or current-installed cold-restart exercise in this task. Retain #21 OPEN until those exact negative cases are proven, and never rewrite the user's stored preference silently as an audit step.

### New UI source receipt

`378834ac` typed `resolved_name_for(locale, name)` with Arabic/European Portuguese/Traditional Chinese and unknown-locale English fallback, FSI/PDI-only projected values, no alteration of editable input, `gio::File` or persisted directory. Static wiring check PASS. Full source CI Preflight `40561114184` and GTK `40561114130` **still pending on most recent read**, not code-complete acceptance. No physical KDE/Pixel or Windows testing occurred.

## #16 additional security/public-contract STOP FACTOR: external standards evidence

The user's earlier approval of **Option A = only cryptographically proven device aliases** remains binding. This review is a read-only contract investigation, **not** permission to source credentials from Google or merge anonymous routes.

Primary external specifications, consulted 2026-10-27 (independent of current repository code):
- Google Nearby's [PublicCertificate wire schema](https://chromium.googlesource.com/external/github.com/google/nearby/+/refs/heads/upstream/cl/627439305/sharing/proto/wire_format.proto) documents `secret_id`, `authenticity_key`, certificate `public_key`, validity interval and encrypted metadata; the mere presence of protobuf fields does **not** provide our client with a trusted key.
- Google's [Nearby Share certificate manager](https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/664513121dad7ee1b8f29a897cfb337a09c60660/sharing/certificates/nearby_share_certificate_manager.h) manages private and contacts' public certificates and server upload/download; there is no source evidence that an independent Linux anonymous "Everyone" client obtains that backend trust.
- Third-party NearShare's [security notes](https://github.com/Dhiva-Labs/NearShare) independently document anonymous `PairedKeyResult.UNABLE` + user PIN confirmation, not identity-based auto-trust; this is developer testimony and not a normative Google API contract.

**Confirmed repo code:** Our `runtime/route_registry.rs` only accepts a real `VerifiedPeerCredential` for same-generation socket and authenticated route binding, but its own comment says no provider wires this in. New external links explain why naming/mDNS cannot fill that gap, not that any proposed external API is licensed/stable/available.

Material architectural alternatives to resolve **only** the missing certificate/trust-provisioning contract:

1. **Account-backed interoperable certificate source**. Require real authenticated provider certificate provenance, rotation/revocation and cryptographic proof linking each advertised route to a matched certificate, while preserving manual consent/fail-closed anonymous fallback. Advantage: could enable genuine Pixel route grouping. Risks: backend authorization, account/privacy contract, undocumented/certain unsupported service APIs and new persistent trust model. **No permission to implement without separately approved architecture and evidence that the source exists.**
2. **Explicit rQuickShare-to-rQuickShare pairing** using app-owned credentials and a route-bound challenge. Advantage: maintainable and independently auditable cryptographic proof among cooperating clients. Disadvantage: Google's stock Pixel Quick Share does **not** participate in our new pairing contract; anonymous stock-device routes must remain independent. Requires new user-visible product/API scope and lifecycle of certificates.
3. **Retain distinct unverified Pixel routes**, change #16 acceptance to "group proven peers when proof is available, display unverified candidate routes separately". Advantage: no unsafe identity claim or proprietary certificate dependency, preserves current Google-compatible experience and explicit consent. Disadvantage: drops the originally requested automatic grouping of a stock Pixel; this is a **new explicit exception distinct from #8**, so it cannot be chosen silently under the owner's current instruction.

**Engineering preference while awaiting the required decision:** retain independent anonymous routes and keep route proof infrastructure fail-closed; investigate official account-backed integration as research, never invent a Google credential or treat matching IP/name as trust. Because the owner permits only #8 as an exception, #16 **cannot be marked closed by choosing option 3 automatically**. Await explicit owner decision before changing this additional trust or public-API contract. Unrelated bug and localization work can proceed.

Current implementation/CI status: `378834ac` typed GTK projection source was committed; Preflight `40561114184`, Native GTK `40561114130` not terminal at the checkpoint. Mode B remains intentionally unrun.

## #16 owner's final 2026-10-27 decision, external feasibility

User explicitly selects **Option A, Google-compatible certificate identity**. Architecture/security inspection documented at [`native-google-certificate-option-a-feasibility-2026-10-27.md`](native-google-certificate-option-a-feasibility-2026-10-27.md), code-independent docs commit `b9b81ce288d3f5132c245ffcc77a3cf33a4cf762`. Public Google source confirms certificate wire and verification manager but depends on a server-backed contact-certificate sync. The current `CERTIFICATE_INFO` inbound handler is a deprecated/no-op path, not a trusted provider. No source supports claiming the necessary independent OAuth scope/API exists. Therefore **BLOCKED pending lawful/authorized public certificate source**, with no fake trusted session, no route merging by hints and no second release waiver. Continue #17, transport issues and physical acceptance independently. The complete source on `84bcff96` has both exact CI workflows terminal SUCCESS (Preflight `40563016992`, Native GTK `40563016773`). Document-only changes do not require or imply new source checks.

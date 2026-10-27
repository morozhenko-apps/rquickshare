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

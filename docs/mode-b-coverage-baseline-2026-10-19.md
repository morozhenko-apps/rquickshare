# MODE B — exact-source instrumented coverage inventory (2026-10-19)

## Evidence / reproducibility
All measurements are from **identical `dev` SHA `99c26335f4d5b1d36f7280656bcd1bcc777ddf56`** (only documentation changes after code SHA `4d9da957`).

- [Coverage measurement run 39209606436](https://github.com/morozhenko-apps/rquickshare/actions/runs/39209606436): success; jobs `Rust core coverage` (117919842271), `Desktop Rust coverage` (117919842255), and `Frontend coverage` (117919841915).
- [Native GTK instrumented coverage run 39209632695](https://github.com/morozhenko-apps/rquickshare/actions/runs/39209632695): success; job 117919923586. Native test execution: 80 passed, 0 failed.
- Rust: `cargo llvm-cov --summary-only` (Core with all features, Tauri bin, Native GTK UI with isolated X11 and D-Bus). These are **instrumented line, region and function** metrics. Reported Rust `Branches 0 / Missed 0 / -` means **branch coverage not measured**, NOT 100%.
- Frontend: Vitest `v8` text coverage including `src/**/*.{ts,vue}`; 46 tests passed, 8 test files.

## Cross-component source-matched totals

| Component | Covered lines | Covered functions | Covered regions | Measured branches |
| --- | ---: | ---: | ---: | ---: |
| Core Rust | **90.42%** | 86.03% | 87.77% | Not measured |
| Native GTK | **70.75%** | 65.73% | 68.96% | Not measured |
| Tauri Desktop Rust | **23.86%** | 19.29% | 24.98% | Not measured |
| Frontend | **56.00%** | 75.86% | N/A | **88.04%** (V8; branch denominator distinct from Rust) |

These are separate instrumentation universes; **do not average** percentages across the project. Core denominator is 28,717 lines, Native GTK 11,619, Tauri 1,052. Frontend report includes TypeScript declarations and entrypoint/UI shell files with 0% statements in this unit-test environment; V8 branch percentage must not be interpreted as comprehensive UI interaction coverage.

## P0 gaps and decision boundary

1. `core_lib/src/hdl/mdns_discovery.rs` **0% lines**. Existing [issue #9](https://github.com/morozhenko-apps/rquickshare/issues/9) requires event/transport test seam and/or real LAN mDNS acceptance. Explicit MODE A decision before production refactor.
2. `core_lib/src/hdl/ble.rs` **5.59%**, `hdl/ble_monitor.rs` **21.31%**, `hdl/gatt.rs` **40.55%**, `hdl/mdns.rs` **68.31%**. Hardware BlueZ, BLE state, Android interoperability cannot be closed by synthetic unit assertions alone. Current claims are **source-execution gaps**, not discovered production defects.
3. Native GTK `panel/reveal.rs` **10.87%**, `preview.rs` **23.33%**, `panel/text.rs` **50.00%**, `panel.rs` **59.60%**, `panel/settings.rs` **67.12%**. Test the risk-bearing GUI gates via the existing isolated GTK harness and real callbacks before adding test-only seams.
4. Tauri `cmds/discovery.rs`, `cmds/send_payload.rs`, `cmds/send_to_rs.rs`, `cmds/change_download_path.rs`, `store.rs`, and `main.rs` have **0%** in bin unit tests; this is a strong application-integration gap, distinct from Core.
5. Frontend shell `App.vue`, `HomePage.vue`, `ItemSide.vue`, `main.ts` have 0% measured lines; `src/vue_lib/utils.ts` **62.72%**, `useToastStore.ts` **16.66%**. A low rate may reflect import/module instrumentation, not just untested behavior.

## Required SSOT stages and priority
Current-source per-production-file maps appear below (output captured as individual summary rows, not inferred). Prior SSOT generic negative matrix and test inventory remain binding; **two current automated coverage runs do not by themselves close the strict 10× full-suite gate**. Completed local/source-matched checks in Preflight/GTK on code SHA `4d9da957` (769 Core stable, 80 Native GTK, 10 Tauri) are distinct from this coverage experiment. Avoid reporting a single global coverage percentage or claiming a Rust branch metric.

Next independently testable P0/P1: guarded GTK presentation of untrusted file labels, copy/reveal action availability under terminal-transfer conditions and background interaction. This is test-only MODE B; hardware work and startup process-seam refactors are separate authorizations.

## Per-source Core Rust summary
| File | Instrumented lines | Missed lines | Line coverage | Region coverage |
| --- | ---: | ---: | ---: | ---: |
| `bin.rs` | 22 | 22 | 0.00% | 0.00% |
| `byte_payload.rs` | 90 | 1 | 98.89% | 98.51% |
| `clipboard_image.rs` | 146 | 4 | 97.26% | 92.31% |
| `errors.rs` | 4 | 4 | 0.00% | 0.00% |
| `file_reader.rs` | 160 | 5 | 96.88% | 98.69% |
| `file_source.rs` | 284 | 1 | 99.65% | 96.51% |
| `hdl/ble.rs` | 143 | 135 | 5.59% | 5.10% |
| `hdl/ble_monitor.rs` | 61 | 48 | 21.31% | 15.87% |
| `hdl/blea.rs` | 598 | 159 | 73.41% | 73.07% |
| `hdl/bwu.rs` | 470 | 1 | 99.79% | 99.72% |
| `hdl/frame_header.rs` | 183 | 0 | 100.00% | 100.00% |
| `hdl/gatt.rs` | 730 | 434 | 40.55% | 38.35% |
| `hdl/gatt/slot.rs` | 177 | 0 | 100.00% | 100.00% |
| `hdl/inbound.rs` | 4709 | 404 | 91.42% | 88.63% |
| `hdl/inbound/receive_reliability.rs` | 1769 | 2 | 99.89% | 99.90% |
| `hdl/mdns.rs` | 426 | 135 | 68.31% | 63.02% |
| `hdl/mdns_discovery.rs` | 90 | 90 | 0.00% | 0.00% |
| `hdl/migratable.rs` | 130 | 0 | 100.00% | 100.00% |
| `hdl/mod.rs` | 5 | 1 | 80.00% | 75.00% |
| `hdl/outbound.rs` | 3143 | 182 | 94.21% | 92.01% |
| `hdl/outbound/metadata.rs` | 378 | 2 | 99.47% | 99.67% |
| `hdl/receiver_header.rs` | 102 | 8 | 92.16% | 94.64% |
| `identity.rs` | 166 | 4 | 97.59% | 97.21% |
| `lib.rs` | 311 | 152 | 51.13% | 43.65% |
| `managed.rs` | 156 | 12 | 92.31% | 90.82% |
| `manager.rs` | 900 | 19 | 97.89% | 97.49% |
| `manager/session.rs` | 242 | 4 | 98.35% | 95.62% |
| `memory_source.rs` | 82 | 2 | 97.56% | 97.62% |
| `native_daemon.rs` | 65 | 8 | 87.69% | 76.70% |
| `peer_authentication.rs` | 277 | 3 | 98.92% | 98.53% |
| `prepared_source.rs` | 79 | 0 | 100.00% | 100.00% |
| `preview_payload.rs` | 118 | 2 | 98.31% | 98.00% |
| `protocol.rs` | 25 | 1 | 96.00% | 96.30% |
| `receive.rs` | 152 | 9 | 94.08% | 90.52% |
| `received_files.rs` | 219 | 10 | 95.43% | 92.58% |
| `receiver_trace.rs` | 67 | 1 | 98.51% | 95.18% |
| `service.rs` | 726 | 11 | 98.48% | 96.11% |
| `service/activation.rs` | 164 | 7 | 95.73% | 88.37% |
| `service/batch.rs` | 121 | 0 | 100.00% | 97.08% |
| `service/bus.rs` | 1577 | 162 | 89.73% | 85.67% |
| `service/bus/batch.rs` | 27 | 4 | 85.19% | 81.58% |
| `service/bus/files.rs` | 47 | 2 | 95.74% | 93.22% |
| `service/bus/state.rs` | 515 | 7 | 98.64% | 96.80% |
| `service/bus/text.rs` | 96 | 3 | 96.88% | 95.80% |
| `service/bus/transport.rs` | 117 | 12 | 89.74% | 87.17% |
| `service/client.rs` | 797 | 52 | 93.48% | 89.27% |
| `service/client/controller.rs` | 533 | 110 | 79.36% | 79.27% |
| `service/client/desktop.rs` | 769 | 61 | 92.07% | 87.21% |
| `service/client/files.rs` | 161 | 20 | 87.58% | 80.53% |
| `service/client/journal.rs` | 97 | 17 | 82.47% | 86.76% |
| `service/client/notices.rs` | 78 | 4 | 94.87% | 91.94% |
| `service/client/settings.rs` | 78 | 2 | 97.44% | 95.16% |
| `service/client/state.rs` | 579 | 22 | 96.20% | 91.87% |
| `service/client/text.rs` | 120 | 5 | 95.83% | 90.59% |
| `service/client/visibility.rs` | 48 | 0 | 100.00% | 100.00% |
| `service/client/watch.rs` | 225 | 16 | 92.89% | 85.30% |
| `service/journal.rs` | 362 | 54 | 85.08% | 82.03% |
| `service/native_settings.rs` | 432 | 17 | 96.06% | 91.64% |
| `service/notices.rs` | 91 | 1 | 98.90% | 97.44% |
| `service/operations.rs` | 273 | 11 | 95.97% | 93.70% |
| `service/private_file.rs` | 86 | 4 | 95.35% | 93.06% |
| `service/process.rs` | 392 | 79 | 79.85% | 78.22% |
| `service/retention.rs` | 60 | 1 | 98.33% | 94.79% |
| `service/runtime.rs` | 690 | 90 | 86.96% | 82.40% |
| `service/settings_service.rs` | 510 | 56 | 89.02% | 87.87% |
| `service/uploads.rs` | 379 | 3 | 99.21% | 96.55% |
| `service/visibility_service.rs` | 213 | 1 | 99.53% | 99.73% |
| `service/wakeup.rs` | 234 | 1 | 99.57% | 98.58% |
| `sessions.rs` | 434 | 8 | 98.16% | 97.03% |
| `sessions/details.rs` | 194 | 5 | 97.42% | 98.20% |
| `sessions/tasks.rs` | 49 | 1 | 97.96% | 93.51% |
| `sessions/text.rs` | 18 | 0 | 100.00% | 100.00% |
| `settings.rs` | 72 | 4 | 94.44% | 94.21% |
| `utils.rs` | 484 | 27 | 94.42% | 94.47% |
| `visibility.rs` | 190 | 6 | 96.84% | 96.24% |
| `TOTAL` | 28717 | 2751 | 90.42% | 87.77% |

## Per-source Native GTK summary
| File | Instrumented lines | Missed lines | Line coverage | Region coverage |
| --- | ---: | ---: | ---: | ---: |
| `card_policy.rs` | 450 | 1 | 99.78% | 96.65% |
| `card_view.rs` | 135 | 0 | 100.00% | 100.00% |
| `cli.rs` | 398 | 20 | 94.97% | 94.56% |
| `desktop_guard.rs` | 241 | 12 | 95.02% | 90.05% |
| `ingress.rs` | 273 | 69 | 74.73% | 76.98% |
| `lifecycle.rs` | 108 | 11 | 89.81% | 90.00% |
| `login.rs` | 380 | 18 | 95.26% | 89.26% |
| `main.rs` | 505 | 439 | 13.07% | 11.61% |
| `panel.rs` | 1416 | 572 | 59.60% | 60.83% |
| `panel/backdrop.rs` | 42 | 3 | 92.86% | 91.04% |
| `panel/compose.rs` | 395 | 157 | 60.25% | 60.29% |
| `panel/countdown.rs` | 152 | 10 | 93.42% | 92.49% |
| `panel/idle.rs` | 273 | 31 | 88.64% | 89.45% |
| `panel/image.rs` | 368 | 142 | 61.41% | 63.08% |
| `panel/incoming_preview.rs` | 69 | 21 | 69.57% | 67.27% |
| `panel/menu.rs` | 369 | 96 | 73.98% | 72.33% |
| `panel/movement.rs` | 336 | 128 | 61.90% | 61.45% |
| `panel/outbound.rs` | 715 | 143 | 80.00% | 74.98% |
| `panel/presentation.rs` | 198 | 52 | 73.74% | 68.49% |
| `panel/results.rs` | 326 | 70 | 78.53% | 78.57% |
| `panel/reveal.rs` | 276 | 246 | 10.87% | 10.50% |
| `panel/rows.rs` | 862 | 319 | 62.99% | 65.43% |
| `panel/settings.rs` | 1101 | 362 | 67.12% | 66.87% |
| `panel/surface.rs` | 36 | 0 | 100.00% | 94.64% |
| `panel/text.rs` | 262 | 131 | 50.00% | 54.15% |
| `panel/transfers.rs` | 279 | 71 | 74.55% | 73.51% |
| `panel/tray.rs` | 384 | 33 | 91.41% | 91.82% |
| `placement.rs` | 220 | 0 | 100.00% | 98.96% |
| `portal.rs` | 235 | 40 | 82.98% | 73.29% |
| `preview.rs` | 180 | 138 | 23.33% | 17.57% |
| `test_bus.rs` | 40 | 0 | 100.00% | 100.00% |
| `tray.rs` | 258 | 31 | 87.98% | 81.47% |
| `tray/menu.rs` | 337 | 33 | 90.21% | 88.40% |
| `TOTAL` | 11619 | 3399 | 70.75% | 68.96% |

## Per-source Tauri Desktop summary
| File | Instrumented lines | Missed lines | Line coverage | Region coverage |
| --- | ---: | ---: | ---: | ---: |
| `cmds/change_download_path.rs` | 13 | 13 | 0.00% | 0.00% |
| `cmds/change_visibility.rs` | 71 | 28 | 60.56% | 68.22% |
| `cmds/clipboard_image.rs` | 99 | 50 | 49.49% | 41.89% |
| `cmds/discovery.rs` | 23 | 23 | 0.00% | 0.00% |
| `cmds/get_hostname.rs` | 3 | 3 | 0.00% | 0.00% |
| `cmds/open_download_destination.rs` | 158 | 55 | 65.19% | 63.25% |
| `cmds/send_payload.rs` | 14 | 14 | 0.00% | 0.00% |
| `cmds/send_to_rs.rs` | 9 | 9 | 0.00% | 0.00% |
| `logger.rs` | 118 | 83 | 29.66% | 32.03% |
| `main.rs` | 250 | 250 | 0.00% | 0.00% |
| `notification.rs` | 197 | 176 | 10.66% | 13.41% |
| `store.rs` | 97 | 97 | 0.00% | 0.00% |
| `TOTAL` | 1052 | 801 | 23.86% | 24.98% |

## Frontend per-file highlights (raw Vitest text output)
- `src/vue_lib/utils.ts`: 62.72% lines, 85.48% branches, 47.61% functions.
- `src/vue_lib/stores/useToastStore.ts`: 16.66% lines; `src/vue_lib/clipboard.ts` and `network.ts`: 100% lines in V8.
- `src/components/HomePage.vue`, `App.vue`, `src/main.ts`, `src/components/ItemSide.vue`: 0% instrumented lines.
- Other tested component example: `SettingsModal.vue` 98.69% and `VisibilityRecovery.vue` 100% lines.
- Exact per-file frontend full text with truncated path names is retained in [run 39209606436](https://github.com/morozhenko-apps/rquickshare/actions/runs/39209606436); this inventory does not guess truncated path names.

## Completion gate
- Stages 1–3 measured, 7–8 exact-source mapped: **YES for this diagnostic**.
- Production-level all-branch negative + interaction matrix, scoped individual mutation execution: **OPEN**.
- Whole full-suite source-exact 10×, real BLE/BlueZ/mDNS/Android, live desktop GUI, release gate: **OPEN**.
- Overall MODE B Stage 12: **NO**.

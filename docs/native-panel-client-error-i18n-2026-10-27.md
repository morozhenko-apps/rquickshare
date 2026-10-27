# Native GTK client error and uncertain-completion notice inventory (#17)

Date: 2026-10-27. MODE A, design only, direct `dev`. At planning time, `5a29c3cc` panel-status exact Native GTK `40593845460` and Preflight `40593845531` were still pending, not accepted. Do not start further source changes that would cancel these checks before recording the source-exact gate.

## Twelve remaining canonical user-facing error/notice sources

Ten exact return labels from `native_desktop/src/panel.rs::error_message`:
1. `Settings save outcome is uncertain. Open Settings and retry the pending save.`
2. `Settings changed. Open Settings and reload before saving.`
3. `Settings need attention. Open Settings to review the result.`
4. `Service connection lost. Reconnect starts a fresh client; choose unsent files again.`
5. `Service is incompatible or returned invalid state. Update it, then reconnect.`
6. `Operation needs recovery. Do not repeat the send.`
7. `Cannot read the selected files. Check access and select them again.`
8. `A recipient is no longer available. Review the selected devices; nothing was queued.`
9. `Sharing resources are busy. Nothing was queued. Wait for a queued transfer to finish.`
10. `The operation was not accepted. Review the current state and try again.`

Two `Panel` completion/visibility notices assigned in the asynchronous `completion.wait()` outcome:
11. `Visibility was saved but could not be fully applied. Review discovery status and Settings.`
12. `Operation did not complete. Review its state before retrying.`

These are selected exclusively by existing typed `ClientError`, native settings store errors and controller completion results. They are not interchangeable with the already localized generic service-lost status, nor with an action reported as successfully sent or finished.

## Bounded implementation contract

- Add one pure `i18n/client_errors.rs` catalog with exactly these 12 English source keys and 19 draft non-English rows (228). Keep distinct uncertain vs conflict vs rejected, the precise prohibition on a replay when recovery is required, `nothing was queued` in both endpoint/capacity cases, and *saved but not fully applied* visibility. Preserve exact English fallback for unsupported pt-BR and zh-Hans and all unknown keys.
- The `Panel::render` localized display boundary is already present; no need to touch `panel.rs` branches, `error_message` return types, async result handling, client retry/cancel semantics, preparation state, consent or wire protocol. Route the new catalog through `i18n.rs` with cross-catalog disjointness and complete locale tests.
- Positive, N1/N3/N4/N5/N6/N7/N9/N10 relevant label checks: all 20 locales, RU/PT/Arabic literal oracles, unique errors, no untrusted interpolation, source-only exact key list. Existing Core ClientError and GTK controller tests retain authority for actual outcomes; do not mock those state machines.
- Once the previous source-exact CI is terminal, source implementation and tests in one atomic direct-`dev` commit, documentation checkpoint separate, final Native GTK nightly rustfmt/strict Clippy/widget suite and Preflight; physical/20-language semantic acceptance and full Mode B remain distinct later release gates.

## Risks and scope boundary

Do not convert `Operation needs recovery. Do not repeat the send.` into a retry invitation, or a pending/uncertain setting into applied success, or unavailable recipient into queued transfer. Rollback reverts only new translations/routing. Unsupported 0.9 parity, #8 accepted A2.2 DoS risk, #16 genuine Google credential provider, Pixel/KDE Cancel/first BLE connection, Windows native UI and signing remain open independently. This document authorizes no release tag or package.

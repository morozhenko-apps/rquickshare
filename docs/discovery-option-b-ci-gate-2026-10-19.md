# Discovery Option B: exact-source CI gate

On 2026-10-19, code SHA `4f5705f4f888c3c2c6781d8999dc96a6b3d6f491` passed [Preflight 39315807211](https://github.com/morozhenko-apps/rquickshare/actions/runs/39315807211) and [Native GTK 39315807383](https://github.com/morozhenko-apps/rquickshare/actions/runs/39315807383). Both workflows completed successfully.

Preflight: Core stable 775/775 plus 4/4 and 4/4 integrations; Tauri 12/12; frontend 46/46; Python 61/61; format, Clippy and typecheck passed. Native GTK: 85/85 passed. All three discovery generation tests and the cache tombstone ordering test passed individually.

Eight race-sensitive tests were inventoried but excluded from this stable run; they are not established flaky and were not run here. The release-tag 10x gate and physical Android/KDE/BLE/mDNS acceptance remain open. SDK event injection is tracked by issue #9. No main merge, installation or release occurred.

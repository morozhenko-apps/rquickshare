# Native GTK outbound and card lifecycle notices — 20-locale display (#17)

Date: 2026-10-27, MODE A, direct `dev`, base `fbe5a240bb1c94bf917731fb09c16d0ecd74325b`. No new branches/PRs. Source-exact native menu `0e154875e577b7e94a95f17370aaa12869481dd1`: Preflight 40582539717 and Native GTK 40582539722 both terminal SUCCESS. This slice extends presentation only; no final Mode B, device/installed acceptance, publication or master mutation.

## Source inventory and invariants

Six exact notice sources are visible in `native_desktop/src/panel/outbound.rs` (three) and `native_desktop/src/panel/presentation.rs` (three):
1. `Content preparation did not complete. Review Recover or discard the pending preparation.`
2. `The target or connection changed. Content was kept; choose a device explicitly to send it.`
3. `Clipboard content is unavailable, unsupported or exceeds limits. Nothing was sent; review any pending preparation before retrying.`
4. `Card dismissal was not confirmed. It may appear again after reconnecting.`
5. `Card dismissal was not queued. Review the panel before reconnecting.`
6. `Cannot place the card on an available monitor. Open the tray to review requests.`

These strings are currently stored as canonical `Model.notice: Option<&'static str>` or assigned by `Panel::notice`, then displayed immediately or on redraw through `Panel::render`. Localizing only one individual callsite would leave the same message English on a later redraw. User-owned content, actions, file selections and session state must remain untouched.

### Contract for this stage

- Create isolated `native_desktop/src/i18n/operation_notices.rs`: exactly six immutable source keys and 19 non-English draft translations per key (114). Preserve original English as the canonical fallback; pt means pt-PT, zh-Hant traditional Chinese, Arabic RTL. Never map unsupported pt-BR or zh-Hans silently.
- Register catalog in `i18n.rs` and extend complete, disjoint-key checks. Translate **only the GTK output** at both `Panel::notice` and the final `Panel::render` status label. Keep `Model.notice` as the original English source token so status branch precedence, diagnostics, regression oracles, and conditions remain unchanged. Other currently translated status keys benefit from the same consistent display boundary.
- Do not touch any content preparation, clipboard read, selected recipient, transfer send, generation/epoch, dismissal token, notice acceptance, desktop safety, retry, visibility, monitor placement, trust, routing, file or persistence logic.
- Positive and negative coverage: all twenty locale keys nonempty/unique, six distinct failure/result meanings (preparation, stale recipient, clipboard failure, queued/not-confirmed dismissal, unavailable monitor), RU/PT/Arabic source-specific oracles, unchanged English and unsupported locale fallback, unknown key pass-through, disjointness against existing catalogs, and current native widget/integration tests. For failure states, avoid accidental success assertions such as “sent”, “completed” or “dismissed”.
- Formatting and validation only after final edits: `cargo +nightly fmt --manifest-path native_desktop/Cargo.toml -- --check`, `cargo +stable clippy --locked --manifest-path native_desktop/Cargo.toml --all-targets --no-deps -- -D warnings`, native isolated GTK controller/widget suite, plus Preflight. CI source SHA and conclusions must be recorded exactly, stable and known-flaky kept separate; no claiming missing hardware/Mode B gate.

## Rollback and remaining release gate

Atomic code/tests commit on `dev`, documentation in separate `docs:` commits; optimistic expected HEAD before each mutation. Revert catalog and exactly two display callsites to restore former English presentation without reverting any transfer or storage. Manual context/RTL, failed-clipboard and sender-switch physical acceptance, other GTK statuses/tray/notification/desktop text, #16's externally blocked Google-compatible identity provider, #8's explicitly acknowledged A2.2 risk, full Mode B and protected release publication remain OPEN. The #17 issue must not close on source-only evidence.

## Execution checkpoint

Production `2137bd24fe27fd28a9e3a07ec6a08a7356153c84` implements six keys, 114 non-English draft translations and two status presentation-site changes in `panel.rs`. The original `Model.notice` source stays unchanged; original outbound and card-policy modules were not modified. Static check confirmed nineteen non-English lookup arms, six source keys and exactly two display-only changes. Original source [Preflight 40592290693](https://github.com/morozhenko-apps/rquickshare/actions/runs/40592290693) completed SUCCESS. Native GTK `40592290719` failed exclusively at nightly rustfmt before Clippy and widgets; logs identified three long `assert_ne!` layouts. Formatting-only `d29a0614edba731ccb261944f47ce834814b42d2` applies exact CI suggestions without altering logic or expectations. Exact-source [Native GTK 40593093964](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593093964) and [Preflight 40593093965](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593093965) pending. No physical/human RTL or final Mode B proof; do not close issue #17.

## Final source-exact gate

After formatting-only `d29a0614edba731ccb261944f47ce834814b42d2`, [Native GTK 40593093964](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593093964) and [Preflight 40593093965](https://github.com/morozhenko-apps/rquickshare/actions/runs/40593093965) both terminal SUCCESS on that exact SHA. No known-flaky claim, installed source, 20-locale semantic approval or full Mode B. Six messages and two GTK status output sites have source-tested visibility; the canonical notice payload remains unchanged.

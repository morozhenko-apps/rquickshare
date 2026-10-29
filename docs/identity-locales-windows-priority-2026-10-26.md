# Owner-ordered implementation: #16 → #17 → physical verification → full MODE B

Date: 2026-10-26. Target repository `morozhenko-apps/rquickshare`, development branch `dev`. This supersedes the prior assumption that localization must wait until every other UI feature is complete: **the owner explicitly reprioritized #16 and #17 before the next manual hardware gate and comprehensive testing**. Focused tests and exact-source CI remain mandatory for individual executable changes; a later full Mode B is not an excuse to ship unsafe interim code. No release/master/tag/R2 changes.

## #16 verified peer identity: source facts and narrowly completed slice

The root cause of visually duplicated devices is independent unverified discovery routes, not a proven duplicate identity. `core_lib/src/hdl/mdns_discovery.rs` collects untrusted fullname/endpoint/address metadata. `service/runtime.rs` maintains per-route IDs. Both live `hdl/inbound.rs` and `hdl/outbound.rs` instantiate `PeerAuthentication::anonymous`, so real Google Quick Share sessions generally return PairedKey `Outcome::Unable`, and do not have trusted certificate-backed identity. UKEY2 verifies confidentiality of **this connection**, not stable equivalence of two advertisements. [Source-proof audit](native-verified-route-identity-proof-gap-2026-10-24.md).

**First actual Core slice:** production `peer_authentication.rs` now has private `VerifiedPeerCredential` with redacted Debug and constant-time equality over SHA256 of trusted P-256 certificate public key. `finish_with_credential` returns it **only** when a real trusted material source supplied peer credential, a current token/role-bound signature verified, the local key hash matched, the remote PairedKey result was Success, and certificate time validity still holds. Original `finish` remains outcome-compatible and discards optional proof, so no wire/consent/runtime/public API/identity grouping behavior changed. Regression tests use real ECDSA fixtures to verify known certificates produce comparable handles while anonymous, hash-only, remote Unable and expired certificates produce no handle or fail closed. This is **credential verification**, NOT an attested advertisement-to-socket binding; it cannot be used to group live Android rows.

**Still required to close #16:** obtain and validate a trustworthy real peer certificate source for actual Pixel and Windows interoperability, with legitimate key distribution, expiry/revocation, recovery and failure semantics; cryptographically establish that the exact `EndpointInfo` route chosen for sending/observed as inbound belongs to that credential/session and epoch; transport proof through native SessionSnapshot/Runtime without leaking a stable tracking identifier to UI/logs; revocable verified grouping with separate exact route Send choice; safe disassociation on tombstone, restart and certificate change. Approved option A **forbids** a substitute using device name, IP, BT address, mDNS fullname, copied endpoint ID or anonymous UKEY2. Additional trust acquisition/public API authority must undergo security review; do not invent credentials or imply source-level certificate validity is route ownership. Keep #16 OPEN and do not bypass #8 plaintext TCP BWU route ownership risk.

## #17: native GTK 20-language UI, current implementation

Canonical codes (from approved PersonalLanding configuration, not inferred): `en,ru,es,pt,de,fr,it,pl,hi,zh-Hant,ja,nl,sv,tr,ko,bn,th,id,fil,ar`.

Native `src/i18n.rs` was introduced with system locale selection and explicit English fallback. `pt_PT` maps to `pt` (European Portuguese), `zh_TW/zh_Hant` to `zh-Hant`; `pt_BR` and `zh_Hans/zh_CN` are deliberately **not** rebranded as supported; unsupported/uncataloged safety text stays English. The first 17 cataloged keys now have localized draft strings in 19 non-English languages. Native GTK context/sending menus, DBus tray labels, static core controls in the Builder and dynamic Receive/Devices captions use the catalog. Source is installed **nowhere yet**; no human wording audit or live font/RTL acceptance has been performed. Translation drafts are **not** a claim of 20/20 complete GTK coverage.

### Native localization remaining inventory and acceptance

| UI surface | Current status | Required production and final acceptance |
| --- | --- | --- |
| Desktop panel and DBus tray basic actions | First 17-key core catalog drafted/used | inventory and translate remaining labels, tooltip text, sections and dynamic messages without contextual ambiguity |
| Consent/PIN/identity disclosures | **NOT DONE** | placeholder-preserving, security-exact native words; PIN LTR bidi isolation; never make Reject look like Success |
| Incoming/outgoing multi-file rows and counts | **NOT DONE** | proper numerals/plurals, file/folder direction, statuses, cancellation and failure semantics |
| Composer, images, received text and clipboard review | **NOT DONE** | no auto-copy/open/accept, payload size and type limits, source selection warnings |
| Settings/login/permissions/recovery | **NOT DONE** | destructive settings, consent vs visibility, persistence and unknown-state warnings |
| History, notifications, errors, About, desktop menus | **NOT DONE** | translate all viewable text and accessibility names/tooltips/desktop assets |
| 20 locale context and layout | **NOT DONE** | PT-PT accuracy, zh-Hant not simplified, Arabic RTL + bdi isolation and focus, CJK and Indic font fallback, long labels clipping, plural rules, native-speaker review where available |

**Live test plan after all #16/#17 feature slices:** deterministic fake real-key-source and exact-route binding unit/integration N1–N12, all 20 catalogs/no unknown keys/format placeholders/plurals, 20-locale text direction/focus snapshot checks, source stable+flaky partition and mutation review, then real KDE+Pixel and real Windows transport where available. Source-only or machine-assisted translation is never represented as final human linguistic verification.

## Windows workstream: what CFVigil actually teaches us

The referenced repository is **`morozhenko-apps/cfvigil`**, branch `dev` (NOT `cf-vigil` and NOT its empty-ish `main`). Reviewed its actual Windows implementation and recent fixes:

- [`d85fdfb`](https://github.com/morozhenko-apps/cfvigil/commit/d85fdfbaa453): preserve **physical x and bottom anchor** across compact/expanded native height, not saved intermediate top coordinate.
- [`ffb0b85`](https://github.com/morozhenko-apps/cfvigil/commit/ffb0b85ec71d): anchor to actual **outer** Windows frame measurements rather than hypothetical logical WebView dimensions, remove opaque native chrome; tested in Windows-specific contract checks.
- [`576d38c`](https://github.com/morozhenko-apps/cfvigil/commit/576d38cba793): bounded checkpoint of real drag changes, not a config write per raw OS move event.
- `cfvigil/apps/desktop/src-tauri/src/windows_card.rs`: monitor work_area (exclude taskbar), physical DPI, native outer height, distinguish user drag from programmatic resize/move, bounded logs when opted in. `cfvigil` Windows owner has actual native-machine feedback; a green Linux GUI test alone is not Windows validation.

**rQuickShare architecture mismatch:** current `native_desktop` is *GTK4 + gtk4-layer-shell + BlueZ/D-Bus/Linux systemd*, **not buildable as a native Windows card**. Retained `app/main` is a legacy Tauri **900×600 decorated, resizable window**, with a different Vue interface; it is not the same compact GTK/CFVigil card. Its `src-tauri/Cargo.toml` also has desktop Linux dependencies (notably unconditional `dbus`), so no current **Windows build or runtime acceptance** is established. Merely setting `transparent: true` / `decorations: false` on the legacy 900×600 window would create a broken borderless app rather than implementing the compact card correctly. Windows lacks Wayland by design; geometry/opaque corners are **Win32/DWM/WebView2 frame and DPI issues**, not the absence of Wayland.

**Approved parallel preparation (without inventing a Windows runtime):** specify a dedicated Tauri/WebView2 Windows native-card adapter, reuse common protocol/security logic while implementing Windows-specific discovery/IPC/tray/service semantics. It must own bottom-anchored physical (x,bottom), work_area placement, clamping across DPI/multiple monitors, a single native geometry authority, compact/expanded resize using measured outer_size, no hidden app taskbar window or opaque corners, no persistent global CSS fullscreen workaround, coalesced real drag persistence and explicit close/reopen/restore behavior. Before applying Win32-specific production changes: create native Windows application window/card, expose frame transition events and run **real Windows build + VM/physical native smoke**. Do not blindly copy CFVigil storage/settings/secrets or assume a Tauri successful compile proves Google Quick Share for Windows interop.

### Windows acceptance cases

W1 first tray click; W2 compact↔expanded ten times without bottom drift; W3 drag then hide/reopen; W4 app restart position; W5 DPI=100/125/150% and monitor change including negative coordinate; W6 taskbar relocation, work_area and screen edges; W7 transparent corners against light/dark desktop and DWM compositor; W8 no console/taskbar ghost and focus behavior; W9 safe source/recipient/consent semantics; W10 Windows transport/discovery identity proof separately from visual UI. All **NOT TESTED** on rQuickShare Windows.

## Next dependency-aware implementation order

1. **#16** verified credential proof seam available; next is genuine trust material and exact live endpoint/session binding, fail closed and ungrouped until demonstrated. No fabricated grouping.
2. **#17** complete all remaining GTK UI/catalog strings and semantic contexts; no new preference/persistent schema unless separately approved.
3. **Windows (in parallel)** build real compact native card adapter + Windows protocol platform service before porting geometry fixes; use CFVigil commits as reference contracts.
4. After both feature sets, install verified latest candidate on KDE, test Pixel↔Linux and available Windows peer with network/identity/locale evidence; then final project-wide MODE B and separate protected early/stable release gates.

This document is implementation-state evidence, not confirmation of completed #16, full #17, Windows support, 20-locale professional signoff, installed behavior or release permission.

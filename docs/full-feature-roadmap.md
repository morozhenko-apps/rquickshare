# Full Quick Share roadmap

_Last updated: 2026-10-06_

The maintained fork is no longer scoped to "make upstream rQuickShare work again". The product goal is a first-class Linux Quick Share client that implements the useful protocol surface available to Android/Windows peers, while clearly separating open/local protocol work from Google-account or Apple-specific interoperability that requires additional reverse engineering.

## Product principles

- Preserve interoperability with Android Quick Share before adding convenience features.
- Keep transport/protocol state machine changes covered by regression tests and Mode B.
- Prefer local/offline behavior. Google sign-in must never be required for ordinary nearby file/text sharing.
- Do not fake Google Account semantics: local trusted devices are a separate feature from Google's Contacts/Your devices identity.
- Keep OS-bound behavior (BlueZ, mDNS, Tauri desktop lifecycle) behind explicit seams so it can be smoke-tested and later unit-tested.
- New functionality is developed on `feat/full-quickshare`, stacked on the hardened Pixel/BLE foundation.

## Capability matrix

### Foundation — implemented / hardened

- Android <-> Linux file transfer.
- Multiple file payloads.
- Inbound text/URL/Wi-Fi credential payload parsing.
- BLE discovery + modern Pixel GATT bootstrap.
- BLE -> Wi-Fi LAN bandwidth upgrade.
- Fixed listening port for firewall-friendly operation.
- Cancellation, progress state, collision-safe payload IDs.
- Wayland clipboard fallback.
- Background receive/tray lifecycle.
- Security hardening, bounded queues, size/path validation and CSP.
- Mode B/coverage/security/package CI.

### Phase 1 — complete the everyday Quick Share experience

1. **Modern sharing protocol model**
   - Preserve the existing Rust module namespace for compatibility.
   - Add modern wire fields by their canonical field numbers:
     - file parent folder, attachment hash and sensitive-content flag;
     - text sensitive flag;
     - Wi-Fi SAE;
     - app metadata;
     - stream metadata;
     - sharing use cases;
     - preview payload ids and transfer id;
     - response attachment details / resume metadata;
     - QR handshake field;
     - binding frames.
   - Add compatibility tests for old + new frames.

2. **Outbound text / clipboard**
   - Send plain text, URL, address and phone-number metadata as BYTE payloads.
   - Share current clipboard text from UI.
   - Share clipboard images as regular image files without leaking temporary files.
   - Correct outbound Finished state and UX.
   - Tests for metadata classification, framing, payload bytes and cancellation.

3. **Folders**
   - Select files or folders.
   - Recursively enumerate folders without following unsafe symlink escapes.
   - Populate `FileMetadata.parent_folder`.
   - Preserve hierarchy on receive with traversal-safe path validation.
   - Resolve collisions at the root folder level without flattening the tree.
   - Tests for nested folders, duplicate names and malicious parent paths.

4. **Custom device name**
   - Persist a validated display name.
   - Update mDNS endpoint info and outbound connection metadata without restart.
   - Keep the DNS hostname independent from the human-readable device name.

5. **Trusted devices / local auto-accept**
   - Explicit local trust store keyed by stable peer identity material where available, not display name alone.
   - Per-device auto-accept toggle and revoke action.
   - Never auto-trust arbitrary "same name" peers.
   - Separate clearly from Google's account-backed Your devices / Contacts modes.

6. **Visibility and desktop UX**
   - Everyone / temporary visibility / hidden semantics.
   - Tray visibility controls.
   - Background notifications with Accept / Reject / Cancel where supported.
   - Dark/system theme.
   - Localization architecture.
   - Better completed-transfer history.

### Phase 2 — advanced protocol features

7. **Preview payloads**
   - Generate bounded image/video/file previews.
   - Advertise preview payload ids.
   - Receive/display previews before consent when peer supplies them.

8. **Resume and dedup**
   - Stable attachment hashes.
   - Receiver existing-file size / payload details.
   - Safe partial-file resume with integrity checks.
   - Never infer equality from filename+size alone.

9. **Stream payloads**
   - Implement STREAM framing and lifecycle.
   - Expose a clean stream abstraction independent of files.
   - Initial Linux use cases: pipe/stdin or application-generated streams.
   - Keep live media use cases optional.

10. **Remote Copy**
    - Implement `SharingUseCase.REMOTE_COPY` for clipboard-oriented transfers where Android peers support it.
    - Treat this as protocol capability negotiation, not merely a UI shortcut.

11. **Multi-recipient**
    - Queue multiple selected devices by default.
    - Add bounded concurrent transfers after resource/BlueZ testing.
    - Independent cancellation/progress per recipient.

12. **Application / Wi-Fi payload completeness**
    - Android app metadata including multi-APK/split packages where peers support it.
    - Wi-Fi credential outbound support including SAE.
    - Sensitive-content flags and safer UI.

### Phase 3 — interoperability research

13. **Quick Share QR compatibility**
    - Modern proto contains QR handshake data, but Google's public Android QR flow also uses a Quick Share web page.
    - Reverse engineer the interoperable handshake before claiming compatibility.
    - A local-only QR fallback may be added separately and must be labeled as such.

14. **Google Account identity**
    - Research public certificates, account visibility and contact verification.
    - True Contacts / Your devices interoperability likely depends on Google identity/backend services.
    - Do not substitute a local trust list and call it Google account support.

15. **AirDrop interoperability**
    - Android Quick Share on selected devices can interoperate with AirDrop.
    - Linux support is a separate protocol/transport project (Apple discovery/AWDL/AirDrop), not a normal Quick Share frame extension.
    - Research independently after Android/Linux Quick Share is feature-complete.

### Phase 4 — polish and distribution

- First-class Debian/Ubuntu packaging.
- AppImage after Wayland regression coverage.
- Optional Flatpak once Bluetooth/mDNS permissions are proven reliable.
- ARM64 builds.
- Context-menu / file-manager integration.
- DBus/share-target integration where desktop environments permit it.
- Autostart/background mode, update channel, diagnostics export.
- Accessibility, keyboard navigation and polished responsive UI.

## Test policy

- Every fixed protocol bug gets a regression test.
- Normal feature-branch/dev commits run the fast preflight once.
- Mode B stable suite runs once.
- Only stability-sensitive async/network tests run 10x.
- Package build is a separate 1x gate.
- Hardware/OS-bound features retain an explicit smoke matrix: Ubuntu/KDE Wayland + target Pixel first, then additional Android/Linux combinations.
- Coverage is used as a diagnostic map, not an arbitrary vanity threshold.

## Current implementation order

1. Modern protocol model.
2. Clipboard/text outbound.
3. Folder hierarchy.
4. Custom device name.
5. Trusted devices.
6. Visibility/tray/theme/i18n polish.
7. Preview + resume/dedup.
8. Streams / Remote Copy / multi-recipient.
9. QR/account/AirDrop research.

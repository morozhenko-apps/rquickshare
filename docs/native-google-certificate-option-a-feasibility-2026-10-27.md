# Issue #16 — Google-compatible verified certificate identity (Option A)

**Decision:** user approved **Option A on 2026-10-27**: investigate and, if legitimately available, integrate Google's compatible certificate system to bind verified stock-Android Quick Share device identity to exact native routes. Do **not** switch silently to proprietary rQuickShare pairing, invent credentials, group by advertised name/IP/MAC/instance/ephemeral ID, or treat already-excepted #8 BWU plaintext route admission as certificate trust. MODE A research and architecture checkpoint, **not complete feature implementation**.

**Source baseline:** `dev` executable `84bcff96dbd56d93c2693871ca32fd71c35a560a`. Preflight `40563016992` and Native GTK `40563016773` both terminal **SUCCESS** on the same source. This document makes no executable code change and does not initiate Mode B, installation or publication.

## Primary evidence and support classification

| ID | Evidence, exact source | What it DOES prove | What it does NOT prove |
| --- | --- | --- | --- |
| G1 | [Google nearby sharing certificate manager header](https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/664513121dad7ee1b8f29a897cfb337a09c60660/sharing/certificates/nearby_share_certificate_manager.h) | Public/private certificate lifecycle, account/contact server upload/download, decrypt public certificate by advertised encrypted metadata key, sign/hash authentication token, logout revocation and scheduled refresh are part of the Google implementation | A documented OAuth scope, sanctioned certificate endpoint or permission for third-party Linux apps to download the same Google-account certificates |
| G2 | [Google sharing wire-format schema](https://chromium.googlesource.com/external/github.com/google/nearby/+/refs/heads/upstream/cl/627439305/sharing/proto/wire_format.proto) | A `PublicCertificate` message contains `secret_id`, `authenticity_key`, `public_key`, validity start/end, encrypted metadata bytes, metadata key tag; `CertificateInfoFrame` is structurally defined | Authenticity of an unauthenticated peer-supplied frame, legal backend access, or a certificate tied to a particular mDNS/BLE route |
| G3 | [Google NearbySharingService implementation (current published source)](https://chromium.googlesource.com/external/github.com/google/nearby-connections/+/refs/heads/main/sharing/nearby_sharing_service_impl.cc) | Discovery attempts certificate decryption using the manager; new discovery/receive can trigger certificate download; `CERTIFICATE_INFO` is handled as **No-op, no longer used** | A useful protocol route for acquiring and trusting unsolicited certificates from a Pixel `CertificateInfoFrame` |
| G4 | [Google/nearby discussion #2446](https://github.com/google/nearby/discussions/2446), developer response Feb 13, 2024 | Google-nearby project contributor states certificate part is not open source and recommends `Everyone` for independent clients | A binding legal interpretation, guarantee API remains unavailable forever, or direct runtime proof of Android certificate behavior today |
| G5 | [Google Nearby public developer APIs](https://developers.google.com/nearby/) and [google/nearby published project overview](https://github.com/google/nearby) | Nearby **Connections** supports generic app-to-app connectivity; Nearby **Presence** offers its own identity model | A documented interoperable Google Quick Share **contact/own-device certificate** download, enrollment or account identity API for an independent client |
| C1 | [open-quickshare third-party limitations](https://github.com/ignotusbucius/open-quickshare), [NearShare security notes](https://github.com/Dhiva-Labs/NearShare) | Two independently developed Quick Share-compatible clients report `Everyone`/anonymous compatibility and lack Google-account certificate trust | Normative vendor policy or substitute for our physical Pixel proof |

**Feasibility result:** the **wire format and verification primitives are public**, but **legitimate live distribution/provisioning of trusted Google Quick Share public certificates to an independent Linux client is NOT ESTABLISHED** as of this evidence review. This is an *evidence-based blocked external dependency*, not a claim that obtaining such access is mathematically or legally impossible. No confirmed publicly documented API/authorization contract was found in the sources examined. Do not probe private endpoints, scrape Android account credentials or silently copy certificates from another app.

## Source audit against our implementation

1. `core_lib/src/peer_authentication.rs`: `CertificateSource::snapshot` already exposes a trusted-provider-only `Credentials {peer,local}`, `CertificateMaterial {public_key,secret,not_before,not_after}`. Parser validates ECDSA P-256 DER key, 32-byte secret, validity; `finish_with_credential` only returns `VerifiedPeerCredential` on a signed, successful and timely PairedKey handshake, **not** anonymous `UNABLE`. `CertificateMaterial` explicitly must never be supplied from remote `CertificateInfo`.
2. `core_lib/src/hdl/inbound.rs::InboundRequest::new` and `core_lib/src/hdl/outbound.rs` current production constructors both use `PeerAuthentication::anonymous` backed by an `AnonymousSource`. Thus ordinary stock-Pixel sessions do not produce a real verified identity today. A successful encrypted UKEY2 session + user PIN/consent authenticates transfer intent only, **not a persistent Google account/device identity**.
3. `core_lib/src/service/runtime.rs::observed_route_proof` consumes source-owned session proof and maps it to the **exact connected socket of that route generation**, with terminal-state and staleness checks. `runtime/route_registry.rs` only prefers an alternative after **two independently authenticated, unexpired** proofs of an identical verified credential; it drops proof on replacement, removal, lag or lease expiry. This is good fail-closed policy infrastructure but **not a provider**.
4. No code path in current production derives Google trusted certificates from account state, peer advertisement or approved backend, and no real Pixel test evidence exists for `PairedKeyResult.SUCCESS` with a recognized Google public key.

## Required end-to-end trust/data flow if provider becomes legitimate

```text
Google-authorized certificate source (user consent, scoped access)
  -> verified provenance, validity window, rotation and logout revocation
  -> bounded CertificateSource snapshot, zeroized secrets
  -> decrypt advertisement's encrypted metadata key / validate tag
  -> bind authenticated certificate material to EXACT observed generation
  -> independently connect that route (no name/IP alias trust)
  -> UKEY2 session / PairedKey signed token + reciprocal SUCCESS
  -> VerifiedPeerCredential (credential fingerprint, no personal metadata)
  -> NativeRuntime RouteProof of the EXACT socket/session generation
  -> second independently authenticated route with SAME credential
  -> optional visual verified group, exact route selectable and pinned
  -> preserve distinct consent, no auto-accept or fan-out recipients
```

The *advertisement-to-certificate* binding is a separate proof obligation from the *session-to-certificate* binding. The current source may provide the latter given a trusted provider; it does not automatically give the former. A first successful signed session can authorize a short-lived **exact route** observation only. No persistent account/device identity, app preference, or auto-accept may be inferred from a routable address or shared certificate hash alone.

## Pre-implementation admission/STOP gate

Only start production provider integration after all of the following evidence has an explicit reference:

- [ ] Identify public, documented and **authorized** Google Quick Share contact/own-device certificate access for independent desktop clients, including OAuth scope/API availability, allowed use, refresh quotas and error codes. **No credentials needed in this repository.**
- [ ] Establish whether a stock Pixel Quick Share session actually supplies verifiable key material in the intended visibility modes; an encrypted random Everyone advertisement is **not** proof of a contact certificate.
- [ ] Prove how an advertisement generation (mDNS/BLE) is cryptographically bound to the certificate, not merely named similarly; authenticate the exact route/socket before any grouping or alternate-route ranking.
- [ ] Define explicit user consent for account linkage and local secret storage/rotation, logout/expiry/suspend/offline/revocation, and protocol downgrade response. Never log certificate contents, identity, account addresses, hashes capable of cross-session correlation or decrypted contact metadata.
- [ ] Confirm license/policy compatibility and reproducible CI test fixtures without live Google secrets. Do not hardcode first-party Android endpoints or borrow the official Windows client's local credential cache.

If the first checkbox stays unavailable, retain the existing **anonymous-compatible, explicit PIN + per-transfer consent and independent exact route** behavior; do NOT show a verified trust badge or collapse stock-Pixel aliases. Keep #16 **OPEN/BLOCKED** with the distinct external access reason. Only #8 is an owner-accepted exception; silently relaxing #16 would introduce an unapproved second exception.

## MODE A test design for an eventual actual adapter (NOT YET IMPLEMENTED)

**Positive:** valid user-provisioned certificate chain/provenance, fresh signed PairedKey SUCCESS, exact connected route binding, same peer across two independently authenticated routes, bounded latency preference, sunset proof on TTL, same-device reauthentication after rotation.

**Negative N1–N12:**

N1 no authorized backend/anonymous mode; N2 expired/not-yet-valid cert; N3 malformed DER/secret lengths; N4 invalid signature/result UNABLE/FAIL; N5 forged/untrusted `CertificateInfoFrame`; N6 same display name but different public keys; N7 duplicate IP/fullname/ephemeral ID; N8 revoked/logout credentials; N9 stale per-route generation after remove/readd; N10 out-of-order/drop/lagged session proof and cancellation; N11 self replay/cross-session UKEY2 token; N12 mixed BLE↔TCP BWU with #8 exception not granting identity.

**Interactions:** 1/128 endpoints, multiple concurrent incoming consents, selected Send route disappearing, two devices sharing the same advertised name, network switch+local IP rotation, host suspend/resume, certificate upload/download failure, lack of contact visibility, input and RTL unsafe names, wrong clipboard/file payload, real Pixel+KDE cold/warm status.

**Coverage and mutation:** per-file `peer_authentication.rs`, `hdl/{inbound,outbound}.rs`, `service/runtime{,/route_registry}.rs`, `hdl/mdns_discovery.rs`, platform bootstrap/authorized secret store and GTK row owner; seeded mutations should catch removed expiry, wrong signer, credential mismatch, socket-generation mismatch, spoofed advertisement, revoked trust, mixed-route fallback and silent auto-accept. Stable deterministic provider fake limited to crypto/edge tests; owner hardware runs real Pixel/Google APIs independently in acceptance. This is an inventory for eventual implementation; **not executed Mode B**.

## Immediate next engineering work possible without a Google backend

- Maintain existing anonymous fail-closed paths and verified-only route ranking as-is; do not add an unused trust adapter or pretend a `CertificateInfoFrame` equals server-issued identity.
- Complete independent #23/#25 transport correctness and #17 UI localization, #9/#11/#12/#19 discovery, and current-source physical tests without waiting for Google certificate approval.
- Resolve the external dependency through official Google developer channels/documented API access, if any; this might require independent account/security/privacy approval. No public discussion, account linkage or external request was submitted automatically.

**Status:** Option A selected and feasibility audit completed, **real Google certificate provider blocked/not implemented**, no new exception, #16 remains OPEN. `master`, signing, R2, installed packages, public posting and Mode B untouched.

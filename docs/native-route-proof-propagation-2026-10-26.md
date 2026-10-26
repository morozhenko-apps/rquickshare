# #16 Mode A: certificate proof propagation into exact outbound route generation

Date: 2026-10-26. Scope: internal source work on `dev`; no UI, public IPC, storage schema, Google wire, installed binary or release changes. Supersedes the prototype-only proof fixtures described in [native-route-registry](native-route-registry-2026-10-26.md).

## Analysis / architectural contract

Currently `OutboundRequest::process_paired_key_result` calls `PeerAuthentication::finish()` and discards the verified-key result. `SessionLease` owns the actual authenticated connection, while `NativeRuntime` owns exact route generation and its name/address. There is **no secure live path** between them. Route registry `AuthenticatedRoute` currently accepts only `#[cfg(test)]` fixtures. Google Pixel traffic remains anonymous: neither a Google-compatible certificate source nor an outbound BLE peer adapter exists. Never infer cross-route ownership from mDNS name, IP, 4-byte ID, RSSI or UKEY2 handshake alone.

Implement an **internal, bounded, typed session proof** emitted ONLY after mutual PairedKey `Success` yields `VerifiedPeerCredential`, attached to the **managed outbound lease actually driving that TCP connection**. Associate the event in NativeRuntime with an existing live outbound `Binding` referencing the originally admitted exact generation and socket; refuse proof if the generation has been withdrawn or its address altered. Do not publish the fingerprint to public `SessionSnapshot`, IPC, disk, telemetry or debug logs.

A proof is never direct user consent, and ranking an already confirmed alias merely picks **one** outbound connection. Subsequent transfer still does a complete independently authenticated handshake; there is no trust bypass/autoaccept. On cancel, terminal, shutdown, missing proof, contradictory route, mismatched session/direction, stale generation or expired TTL: preserve exact route and do not merge. Expire cached evidence conservatively (short bounded monotonic duration) and revoke immediately on tombstone/replacement.

## Implementation stages

1. Internal `RouteProof` channel in SessionHub separate from public snapshot/events, with fixed bounded capacity, independent subscription before Core startup and fail-closed lag.
2. Measure actual TCP connect duration in SessionContext, pass it to OutboundRequest. On authenticated mutual PairedKey success only, SessionLease emits `(SessionId, verified credential, duration)`. An anonymous `Unable` event emits nothing. Do not change existing protocol frames.
3. Bind `SessionId` in Runtime to the selected exact pending route + socket only after admitted ManagedSender job. On received proof, verify live binding, session state/direction, route generation and address, then store bounded evidence in RouteRegistry. Never accept a proof for an inbound session or unrelated route.
4. Focused tests: verified fixture proof event; anonymous/no event; wrong/stale SessionId rejected; route replacement/tombstone/expiry; backpressure and one exact send; no PII output. Rust format, stable suite, separate risk-sensitive suite and six exact-source CI at final gate. Continue #16 afterwards with legitimate certificate source + BLE outbound route admission, not faked test keys.

## Risks / rollback

Important risks: proof originating from a non-socket-bound caller; session event ordering and loss; route generation rotation before proof arrives; leakage of stable identity; invalidating in-flight transfer during mDNS churn. Ensure proof is emitted only by protocol worker with live managed lease and consumed only by runtime owner. Do not use a public untrusted channel or expose a generic `bind_credential` IPC action. Existing `dev` commit before this slice: `88b3e453785e9b39649cf2951478c167e5ca5d57`. Rollback via reversing atomic source commits; no package/device changes.

**Completion gate:** this document starts PARTIAL. Current Google/Windows source and installed tests cannot prove cross-media grouping until provider/discovery work is finished.

## Handshake revalidation invariant before automatic re-routing

A previously verified route fingerprint is only a **candidate admission hint**; it must never be the authorization to release payload in a new TCP connection. For any route selected from cached proof (including when user selected that same route), carry its *expected trusted credential* as a private, in-memory capability through Pending → ManagedSender → SessionLease → OutboundRequest. The next PairedKey exchange must independently return the **same** valid credential before sending `IntroductionFrame`, requesting consent or releasing any payload; `Unable`, rotation, expiry, mismatch and missing local certificate provider all fail closed **before Introduction**, with **no second candidate retry**. A session with no cached trusted expectation retains current standard anonymous Google consent behavior. On route tombstone or cache invalidation under admission backpressure, abort the queued operation rather than fall back to unverified recipient. Never serialize the expectation to public SendInfo, D-Bus or logs. This is part of the already agreed verified-only #16 security contract, not a new account/trust decision.

# P0 #11 — LAN BWU IPv4 eligibility (selected minimal option A)

Date: 2026-10-24. Mode A production behavior correction. Derived from [active backlog](active-backlog.md), [issue #11](https://github.com/morozhenko-apps/rquickshare/issues/11), [prior MODE B inventory](mode-b-p0-lan-ipv4-selection-2026-10-18.md). Owner instructed proceeding in the proposed #11 → #9 priority and making documentation recoverable. **Narrow Option A** (not B/C) preserves non-RFC1918 valid fallback.

## Root cause and invariants

`core_lib/src/utils.rs::select_lan_ipv4` ignores a bounded list of known virtual interface prefixes, IPv6, loopback and link-local, prefers first RFC1918 private IPv4, else first remaining IPv4 as fallback. It currently admits `0.0.0.0`, `224.0.0.0/4` multicast and `255.255.255.255` limited broadcast as recipient destinations, since none triggers those prior exclusions. These are **not usable unicast LAN endpoints**, so a candidate selected in fallback can cause BWU to advertise an undialable host. This is source-driven evidence, not a proven physical failure.

Plausible causes examined: (1) only two address-class exclusions; (2) first-nonprivate fallback accepts any IPv4; (3) interface-name-only virtual filter is incomplete; (4) no route reachability test; (5) interface enumeration order affects fallback; (6) insufficient boundary-case tests. The direct cause is (1)+(2), while (3)–(5) are broader policy/availability concerns not fixed by a narrow option A. No new OS-specific routing dependency or reverse DNS heuristic.

**Authorized change:** reject only `ip.is_unspecified()`, `ip.is_multicast()` and `ip.is_broadcast()` in addition to the existing two exclusions. Keep existing private-first/public-first fallback, interface ordering, case-insensitive known virtual prefixes, lack of IPv6, no port/timer/security/protocol change. Do not exclude ordinary `192.168.*.255` without subnet mask or infer `240/4` policy beyond explicit scope. No new public API. `local_lan_ipv4()` continues to return `None` if no eligible candidate is found.

## Plan, coverage inventory, verification

1. Documentation-first plan; narrow decision scope and rollback anchor [this document].
2. Implement additional reject predicate inside the existing `select_lan_ipv4` loop.
3. Add independently named table-driven unit tests to the real selector (not mock) covering each invalid class (start/end multicast, unspecified, broadcast), only-invalid set → `None`, invalid-before-valid and valid-before-invalid fallback, private preference unaffected, eligible public-only first fallback, virtual/private, loopback/link-local/IPv6.
4. Mutation review: remove each new predicate individually and verify at least one new test detects it logically; inject wrong selection order or skip valid public fallback and point to existing or new assertions. Check no new unverified hardware claim.
5. Final source-exact formatter, Core Rust stable once, Core Clippy, Tauri Clippy, frontend and GTK CI; gate success on terminal job outcomes. Hardware: on actual updated `.deb` test direct BWU from Pixel, including VPN-on/off, and preserve no-hop network. No automatic install.
6. Docs-only status/evidence commit and issue update; leave #11 OPEN until physical acceptance. Roll back the production change by reverting one atomic code/test commit.

**MODE B branch map:** excluded virtual name; excluded IPv6; excluded loopback; excluded link-local; **new** excluded unspecified; **new** excluded multicast; **new** excluded limited broadcast; accepted private short-circuit; accepted first fallback; no candidates. Full per-production-file coverage map is `core_lib/src/utils.rs` (all listed selector branches); `local_lan_ipv4` is a wrapper of the OS enumeration and is not the direct deterministic test subject; destination wire frames are unchanged.

**Positive / N1–N12:** Positive RFC1918 priority and valid public fallback; N1 invalid/missing/empty all filtered; N2 candidate counts 0,1,multiple; N3 network reachability out-of-scope physical; N4 repeat/reorder deterministic; N5 competing private/public first; N6 OS interface changes out-of-scope physical; N7 malformed/unusable special address cannot be advertised; N8 no locale/time semantics; N9 link up/down and VPN OS integration hardware; N10 synthetic addresses only (no actual user IP logged); N11 entitlement not applicable; N12 file/storage unchanged. Interaction matrix: {invalid class × (before eligible, after eligible, only invalid)} plus private-versus-public order, virtual-versus-physical, duplicate public candidates. No invented coverage percentage or performance claim.

**Completion gate:** source compiled, tests and exact CI successful; **physical gate remains separate**. Do not close the issue merely because source and CI are green.

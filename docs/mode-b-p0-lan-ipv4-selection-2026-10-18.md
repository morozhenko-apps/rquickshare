# MODE B P0: LAN IPv4 selection for Wi-Fi bandwidth upgrade — 2026-10-18

## Stage 1–2: source scan / risks
Test-only scope: `core_lib/src/utils.rs`, production helpers `is_virtual_interface` and `select_lan_ipv4`. This pure routing decision supplies the advertised address for phone-to-desktop Wi-Fi BWU. Related owner routing is covered in `core_lib/src/hdl/bwu.rs`; live mDNS browsing / reachability in `core_lib/src/hdl/mdns_discovery.rs` is not isolated without an event/transport seam. Existing LAN tests: one RFC1918-over-public case, first-public fallback, rejected tunnel/link-local/IPv6/loopback, case-insensitive tunnel prefix.

P0 invariants: real RFC1918 LAN address beats public fallback regardless of earlier candidates; only the first eligible private address wins; all known tunnel/virtual prefixes are excluded; if no eligible private candidate exists, the first eligible public candidate is the fallback; loopback, link-local and IPv6 cannot be advertised. No network, OS adapter reads or production edits are involved.

## Stage 3 inventory
| Method / contract | Risk | Level | Planned named tests |
| --- | --- | --- | --- |
| `select_lan_ipv4`: class-specific RFC1918 priority | P0 | pure unit | 1 parameterized, 4 ranges |
| `select_lan_ipv4`: order and fallback invariants | P0 | pure unit | 1 matrix, public/private permutations |
| `select_lan_ipv4` + `is_virtual_interface`: virtual and ineligible addresses | P0 | pure unit | 1 interaction matrix with tunnel prefixes |
| `local_lan_ipv4`: actual OS interface enumeration | P0 | system integration | OPEN; requires controlled OS fixture or physical validation |
| `is_not_self_ip`: actual OS interface enumeration | P0 | system integration | OPEN; do not fake global adapter state in unit test |
| `mdns_discovery.rs::run`: event cache / TCP reachability | P0 | transport integration | OPEN; current source has no deterministic event/transport seam, needs separately approved MODE A refactor |

## Stages 4–7: branches, category and interaction matrix
Branches: virtual prefix matching, IPv6 skip, v4 loopback skip, link-local skip, private early return, public fallback initialization, repeated public preserving initial fallback, empty/ineligible -> None.
Three named table-driven tests. Interaction matrix: public-first/private-first, two private candidates, public without private, virtual private with physical public, virtual-only, physical IPv6/loopback/link-local with valid private/public. RFC1918 classes: 10/8, 172.16/12 lower and upper, 192.168/16.

| Artifact | Positive | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 | N10 | N11 | N12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Select IPv4 | Planned | N/A typed address | Planned class and ordering boundaries | N/A no socket I/O | Planned deterministic repeat/order | N/A synchronous pure | N/A no persisted state | Planned ignore virtual addresses | N/A clock/locale | N/A no UI | Planned never advertise VPN address | N/A no billing | N/A no filesystem |

## Stage 8–10: per-file map, mutation review
`core_lib/src/utils.rs`: `select_lan_ipv4` virtual skip, IPv4 filter, loopback/link-local filter, private priority and fallback ordering; `is_virtual_interface` all configured prefixes and case normalization. Mutants: reversed private/public priority; first-vs-last fallback; failing to exclude a tunnel; accepting link-local; ignoring class 172.16–31; skipping all private candidates should fail. Manual mutation design only; no measured mutation score.

Known additional policy question: which special non-RFC1918 IPv4 blocks (unspecified, multicast, broadcast, CGNAT) must be considered unusable for BWU. Current code uses standard library `is_private()` and only expressly rejects loopback and link-local; do not invent or change product policy in MODE B. Resolve through separate contract decision before asserting broader address-class validity.

## Stage 11–12 gate
After implementation, verify exact-source Core Rust tests, rustfmt, Clippy, Native GTK; keep strict ten consecutive complete runs, current-source coverage instrumentation, live mDNS discovery and all-repository Stage 12 OPEN. Rollback: only three tests and this document. Test-only commits, `dev` branch, no change to production, releases or `master`.

## Implementation and CI checkpoint
- Stage 1–8 documented first at `bf551826`.
- Stage 9: three new table-driven tests implemented in `core_lib/src/utils.rs` at `7bb5a4c1`: six RFC1918 candidates; four route-order/fallback combinations plus empty input; nine named tunnel/virtual device prefixes with physical public fallback and a mixed all-ineligible candidate set.
- Exact checks include selected IPv4 octets, not just presence or non-null status. No OS adapter or live socket is instantiated.
- First source run found `rustfmt` whitespace-only differences in two vector literals and one iterator mapping; these were corrected at `23f1de8b`, with no changed assertions.
- Verify exact-source [Preflight 39165451726](https://github.com/morozhenko-apps/rquickshare/actions/runs/39165451726) and [Native GTK 39165451625](https://github.com/morozhenko-apps/rquickshare/actions/runs/39165451625) before marking this package green. Neither is claimed as passed here.
- Hardware/mDNS discovery integration, special-address policy decision, mutation-engine execution, ten successive full CI runs and repository-wide MODE B Stage 12 remain OPEN.

## Verified execution evidence and open defect
- Exact test-source `23f1de8b` [Core Rust job 117787382343](https://github.com/morozhenko-apps/rquickshare/actions/runs/39165451726) finished **SUCCESS**: 757 library tests passed, 0 failed, 0 ignored, 8 separately grouped tests filtered; the three new LAN test names are each explicitly listed as `ok` in its logs. Two integration suites each had 4 PASS.
- On the same source, Native GTK run [39165451625](https://github.com/morozhenko-apps/rquickshare/actions/runs/39165451625) **SUCCESS**; rustfmt, frontend and Core Clippy jobs **SUCCESS**. Tauri Clippy / desktop unit job was still in progress at this checkpoint. Do not count full Preflight until that job succeeds.
- Created [P0 source-level fallback address policy issue #11](https://github.com/morozhenko-apps/rquickshare/issues/11) for unspecified/multicast/broadcast IPv4 and unrecognized VPN adapters. Three MODE A options recorded there; no policy change or failing diagnostic test in MODE B.
- The separate [mDNS discovery event testability issue #9](https://github.com/morozhenko-apps/rquickshare/issues/9) already tracks unresolved browse/cache/reachability test isolation. Do not duplicate it.

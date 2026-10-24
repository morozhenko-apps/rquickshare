# rQuickShare documentation navigation

This is a navigation index, not a second live backlog or a new coverage claim.
Read [repository rules](../AGENTS.md), then [active backlog](active-backlog.md)
for the current execution order, exact source/CI and installed-device gates.
[current-state.md](current-state.md) is an immutable historical journal; all
1,731 original lines remain intact. Older pending/complete statements remain
source-bound evidence rather than instructions to rerun or close work.

## Product, build and acceptance owners

- [Product/release identity](product-release-strategy.md) and [full feature roadmap](full-feature-roadmap.md): product direction versus accepted platform scope.
- [Build guide](../BUILD.md), [native development](../native_desktop/README.md) and [packaging](../native_desktop/packaging/README.md): actual source/build and installation contracts.
- [Essential priorities](native-essential-priorities.md) and [physical acceptance](native-current-source-physical-acceptance-2026-10-24.md): implementation and installed/peer evidence are separate.
- [Current throughput diagnostic](native-inbound-throughput-diagnostics-2026-10-24.md) and [GATT/Weave connection investigation](native-pixel-weave-connection-stall-2026-10-24.md): separate open physical findings, not proven performance fixes.
- [Cadence owner](CI_RUNNER_CADENCE_2026-10-18.md), [stable/race-sensitive history](ci-stable-race-sensitive-2026-10-17.md) and [tag namespace](native-tag-namespace-migration-2026-10-22.md): ordinary checks, risk probes and final release10x are distinct.
- [Native release](native-tagged-release.md), [signer custody/recovery](native-signing-recovery-agent-path-2026-10-22.md), [R2 publication](native-r2.md) and [branch audit](branch-audit-2026-10-23.md): preserve approval, key custody and rollback; no publication is authorized by documentation cleanup.
- [Maintenance audit](maintenance-audit.md), [offline maintenance](offline-maintenance.md) and [completion audit](native-completion-audit.md): dated foundations, negative findings and remaining acceptance.

## Mode B evidence by subsystem

All46 original `mode-b*.md` records remain unchanged. The table lists up to three
commit-shaped references that occur in each record and resolve in local Git;
they may describe earlier baselines, diagnostics or test-only successors. They
are **not** automatically the latest executable source or a verified CI gate.
Other references, complete branch/negative/interaction maps, failed attempts,
mutations, filtered/race-sensitive distinctions and hardware gaps remain in the
linked original. A record without a resolvable ref is labelled explicitly.
No test count establishes current per-method/branch coverage or device parity.

### Campaigns, coverage and completion

| Record | Recorded commit references |
| --- | --- |
| [mode-b-completion-audit-2026-10-18.md](mode-b-completion-audit-2026-10-18.md) | `4f5705f4f888c3c2c6781d8999dc96a6b3d6f491`, `0c93990344b4a9cf234372eb88614930d0c7e89d`, `b8545375053bb132bff4d30db744df819df125fc`; 9 more in record |
| [mode-b-coverage-baseline-2026-10-17.md](mode-b-coverage-baseline-2026-10-17.md) | `f8e5d2d01d62200fcfd570c7bdf1f9051874845d` |
| [mode-b-coverage-baseline-2026-10-19.md](mode-b-coverage-baseline-2026-10-19.md) | `99c26335f4d5b1d36f7280656bcd1bcc777ddf56`, `4d9da957d8a6db004a2c86f795a7c4c09d38a624` |
| [mode-b-full-ci-2026-10-17.md](mode-b-full-ci-2026-10-17.md) | `64e8496224c0bf8dee6bb7ef3531a2545dcf450e`, `7107339e49b577be9f292004513d80d09d706188`, `f8e5d2d01d62200fcfd570c7bdf1f9051874845d`; 1 more in record |
| [mode-b-inventory.md](mode-b-inventory.md) | `00a0a76bc53bf22c7d2a4312099de559b06bd362`, `66ddf1fc24f906e74c9246c31ebc1ad967b0e8d2`, `70a6abb076a884341dade7172bcd9103b5d8cb4e`; 16 more in record |
| [mode-b-native-gtk-coverage-plan-2026-10-17.md](mode-b-native-gtk-coverage-plan-2026-10-17.md) | `f8e5d2d01d62200fcfd570c7bdf1f9051874845d`, `668917d837eff37e591d3374e912be7cc31a58df`, `3bb41cb6647820ff82ce2ec70f6943e7d7b11c9c`; 1 more in record |
| [mode-b-test-plan.md](mode-b-test-plan.md) | `f8e5d2d01d62200fcfd570c7bdf1f9051874845d`, `29c3dc86059b35caaa199a52888211f924b21f3b` |

### Discovery and LAN admission

| Record | Recorded commit references |
| --- | --- |
| [mode-b-p0-discovery-lifecycle-2026-10-19.md](mode-b-p0-discovery-lifecycle-2026-10-19.md) | `e734477e63a3296b29711e64aae76eb8cdf3a6d7`, `62aee4859e48c75ab38ae5dbf46470751c22097f`, `b8545375053bb132bff4d30db744df819df125fc`; 12 more in record |
| [mode-b-p0-lan-ipv4-selection-2026-10-18.md](mode-b-p0-lan-ipv4-selection-2026-10-18.md) | `bf5518268ecc30ad08c84e4fcc1f004391f3ba15`, `7bb5a4c1c2f9fbb0bb2c84f6e9da1c335bfdd3bd`, `23f1de8b44a58576d3b1b7c34b3cc884f9d4e21c` |
| [mode-b-p0-mdns-refresh-txt-2026-10-18.md](mode-b-p0-mdns-refresh-txt-2026-10-18.md) | `479f341315e76e94a34bf4a7597fae99ef58c792`, `42ff79484cd42673d35f793f071d4d73e54823ec`, `05cb7d86babb8fe5d1600afed3519b4fe48ef2f7` |
| [mode-b-p0-mdns-service-txt-2026-10-18.md](mode-b-p0-mdns-service-txt-2026-10-18.md) | `b1925b3e4c9fc8611df43e4cc34bd7ffbea0db8a`, `d8363e1e6e485b822ec9a0f50ce82b0c0c920aca`, `d2a1449cab34b06f7e49c6431242a0dee442dfb9`; 4 more in record |
| [mode-b-p0-receiver-discovery-advertisement-2026-10-18.md](mode-b-p0-receiver-discovery-advertisement-2026-10-18.md) | `310d4ecdd4091e31b65e2e4874f41f0615c4cc2f`, `bc0ab2f7f552fc7ba600d15ae63b1caf61a47334`, `4a18f702503378261ec891598b08bd1d01f3ee35`; 6 more in record |

### Core protocol and transfer lifecycle

| Record | Recorded commit references |
| --- | --- |
| [mode-b-p0-bwu-concurrent-isolation-2026-10-18.md](mode-b-p0-bwu-concurrent-isolation-2026-10-18.md) | `21f6faff5c446aabfdf1939e44577cfc4c59b1c8`, `ea1e4edcd34b5ee30c9749a58d8ab5171ce19dbc` |
| [mode-b-p0-bwu-route-2026-10-18.md](mode-b-p0-bwu-route-2026-10-18.md) | `a086a142c8d901e946e35d80110598dc1132d906`, `4f35fc35f261866736d9c900605f447508d7bc29`, `7a08e57cb95d3f4854df69c84cf3a2d85019ed22`; 1 more in record |
| [mode-b-p0-classifier-timeout-cancellation-2026-10-18.md](mode-b-p0-classifier-timeout-cancellation-2026-10-18.md) | `bccbab68d0d02026ffa79b964a359f8728e09123`, `9cbb4291632a5d4c7664cbcfbd1e0cf2e2b2a91b`, `c20af6861184e5ce230997a93c8be7de2572231f`; 1 more in record |
| [mode-b-p0-client-introduction-2026-10-18.md](mode-b-p0-client-introduction-2026-10-18.md) | `c192a608fb32b9a05606e891a443272dbb2abe1f`, `ea84a83ed2630c11eb1840649fa84a56d5fb81ab`, `c6484ec80827c4337f294c639fdc6766c314035c`; 2 more in record |
| [mode-b-p0-frame-header-recovery-2026-10-18.md](mode-b-p0-frame-header-recovery-2026-10-18.md) | No locally resolvable commit reference recorded |
| [mode-b-p0-gatt-epoch-isolation-2026-10-18.md](mode-b-p0-gatt-epoch-isolation-2026-10-18.md) | No locally resolvable commit reference recorded |
| [mode-b-p0-gatt-partial-frame-2026-10-18.md](mode-b-p0-gatt-partial-frame-2026-10-18.md) | `5685b41b7c6a940c4a1c47238951af4ae3063ea2`, `2afd1112f34aff5fdea9ae7d81e3f438d3774ec3`, `86ebe6780693bcac08eb3db363cc919a73cc95dc`; 1 more in record |
| [mode-b-p0-gatt-slot-framing-2026-10-18.md](mode-b-p0-gatt-slot-framing-2026-10-18.md) | `0b17a2f86c0c8211e3e0ac43605d21c4eea6123e`, `6222c70d9061643c85480cd203f632d9ca81e014`, `23b71de30eefddba46cc6ce4615923203ac65fba`; 2 more in record |
| [mode-b-p0-gatt-worker-2026-10-18.md](mode-b-p0-gatt-worker-2026-10-18.md) | `e7f885430b8e592e67876d3925f95a8b41dd8275`, `9030e6d9c30fdee97d57849b792d736fca8db7df`, `c918ebb04fca4aed4bbe118d51c89bac26de1519`; 3 more in record |
| [mode-b-p0-manager-ingress-2026-10-18.md](mode-b-p0-manager-ingress-2026-10-18.md) | `5d48f8500a738baddc85d4c8c970ccbb02ed3359`, `811fd6ef5eaf49ee8a654c4842783a3526939d07`, `811fd6ef5eaf49ee8a654c4842783a3526939d07` |
| [mode-b-p0-transport-halfclose-2026-10-18.md](mode-b-p0-transport-halfclose-2026-10-18.md) | `3e3031ce2d2ac7e65448f7a59361e86d3b0a28c0`, `b59bd73b1bcbdfd9e5bb106402bbd08025dd2917`, `7652ad263f630f23437e9c351e0b6f86543ac166` |

### Native IPC, clipboard and desktop safety

| Record | Recorded commit references |
| --- | --- |
| [mode-b-p0-dbus-reply-signature-fd-2026-10-19.md](mode-b-p0-dbus-reply-signature-fd-2026-10-19.md) | `dd9d029441ac670ba1e4991a503ec77413e6456b`, `85faca64802d1febaf481ced816721962e1bb3f3`, `c2b14f54bfd0448f7588071ef29699a7c387fec9`; 3 more in record |
| [mode-b-p0-forged-visibility-dbus-replies-2026-10-18.md](mode-b-p0-forged-visibility-dbus-replies-2026-10-18.md) | `ede67ecfe997f1479a45b6730f815ad506ac3723`, `56ed0dcbb64586612dba68403306475f2d71b6bd`, `324ee6b2b7122026d5a281c1a3f810c820452ac0`; 1 more in record |
| [mode-b-p0-gtk-clipboard-owner-transition-2026-10-18.md](mode-b-p0-gtk-clipboard-owner-transition-2026-10-18.md) | `1501fa1eed09bfb349ec48fcab038d15bba7a83f`, `9306ae91fffda18c48a010ed3720eeab5e0674d3`, `70a6abb076a884341dade7172bcd9103b5d8cb4e`; 1 more in record |
| [mode-b-p0-native-async-clipboard-2026-10-18.md](mode-b-p0-native-async-clipboard-2026-10-18.md) | `f9551a28e7ceed1b44d28a8f081769d75f693c67`, `1501fa1eed09bfb349ec48fcab038d15bba7a83f`, `1501fa1eed09bfb349ec48fcab038d15bba7a83f` |
| [mode-b-p0-native-outbound-clipboard-2026-10-18.md](mode-b-p0-native-outbound-clipboard-2026-10-18.md) | `44c9fec87ace2907751f3554f0abca941883eaeb`, `3186526e66754cfb9d2b0fe416bebca5bed506ff`, `11f4e87554eaf433bab9750d3e5dd021e28bda6e`; 2 more in record |
| [mode-b-p0-native-portal-directory-2026-10-18.md](mode-b-p0-native-portal-directory-2026-10-18.md) | `ceeefe21e4f2f33e70b4a1fbfe11cdc703fb2bc4`, `ceeefe21e4f2f33e70b4a1fbfe11cdc703fb2bc4`, `f63c3df1cece64de08de57cadf52683875acd088`; 1 more in record |
| [mode-b-p0-portal-response-unexpected-fd-2026-10-19.md](mode-b-p0-portal-response-unexpected-fd-2026-10-19.md) | `b912108f92e9cf2558a8ba05ab54f0a4ac653410`, `b4d7ff33397e14d7deae7a97b7fcd29e7ff197fe`, `ea47d49d485f48169009f7a31fb5dd4d54b5a644` |
| [mode-b-p0-reveal-ingress-lifecycle-2026-10-19.md](mode-b-p0-reveal-ingress-lifecycle-2026-10-19.md) | `291b773343493cde990f754eda87ec1cc849ff60`, `99c26335f4d5b1d36f7280656bcd1bcc777ddf56`, `b912108f92e9cf2558a8ba05ab54f0a4ac653410`; 2 more in record |
| [mode-b-p0-selection-lifetime-2026-10-18.md](mode-b-p0-selection-lifetime-2026-10-18.md) | `11ae9bd7e66de326477d1e557f7003c4e34de286`, `e573193261863da6c65a6a38983f539c6ae78f16`, `e573193261863da6c65a6a38983f539c6ae78f16` |
| [mode-b-p0-tauri-reveal-path-containment-2026-10-19.md](mode-b-p0-tauri-reveal-path-containment-2026-10-19.md) | `981e08b252bb8efba88ba2c6f05032ebf47bd68a`, `ea47d49d485f48169009f7a31fb5dd4d54b5a644`, `3e7494dfc546f8acd6365e291114cdf3b24e5a6b`; 4 more in record |
| [mode-b-p0-visibility-client-owner-change-2026-10-18.md](mode-b-p0-visibility-client-owner-change-2026-10-18.md) | `e45b89ee2795aa7b18d977421228b49c93ebb782`, `c37075bebc802492fd7f663ba56c985eb13cafbe`, `7ac3c8db9cf5b93d1dff11a7e1e6ed7f71f97f20` |
| [mode-b-p0-visibility-view-intent-matrix-2026-10-18.md](mode-b-p0-visibility-view-intent-matrix-2026-10-18.md) | `d5d696eed1ae9d8d525ec28c7f8dffe7fceb416e`, `6754ab0e77db0430f6ee1ce18b59de400c664fc9`, `0d4f3ad54297aaa75444f0da729a81c4837941fd` |

### Native GTK projection and interaction

| Record | Recorded commit references |
| --- | --- |
| [mode-b-p0-batch-native-guards-2026-10-18.md](mode-b-p0-batch-native-guards-2026-10-18.md) | `024aac6fa85259a2bd138dc4a869e3c2b08d6aa4`, `5939949eade93581ad57b41ceb735edb0a00dd05`, `da3871976d8ec266e3be1cf4d881e9c1082e9c27`; 9 more in record |
| [mode-b-p0-device-acceptance-matrix-2026-10-18.md](mode-b-p0-device-acceptance-matrix-2026-10-18.md) | No locally resolvable commit reference recorded |
| [mode-b-p0-gtk-untrusted-file-labels-2026-10-19.md](mode-b-p0-gtk-untrusted-file-labels-2026-10-19.md) | `ac0eb8cac62984375a0bf554f2b40dd6020c480c`, `86e2bb4107cc177c9e1ccb6c5c4e3d4c4b461631`, `291b773343493cde990f754eda87ec1cc849ff60` |
| [mode-b-p0-identity-ack-generations-2026-10-18.md](mode-b-p0-identity-ack-generations-2026-10-18.md) | `adf7570a1ae334278f2038c3ad6415f5155823a1`, `94f602dc2907565084961dd0abd6b0ba3a4a0c70`, `36d1469409ba9cf42a591230e010a7eb458ab205`; 1 more in record |
| [mode-b-p0-native-card-arbitration-2026-10-18.md](mode-b-p0-native-card-arbitration-2026-10-18.md) | `2b3103b0426bc862d8a2a2b7a5b8e69e367025a1` |
| [mode-b-p0-native-row-projection-2026-10-18.md](mode-b-p0-native-row-projection-2026-10-18.md) | No locally resolvable commit reference recorded |
| [mode-b-p0-native-settings-acknowledgement-2026-10-18.md](mode-b-p0-native-settings-acknowledgement-2026-10-18.md) | No locally resolvable commit reference recorded |
| [mode-b-p0-native-shutdown-lifecycle-2026-10-18.md](mode-b-p0-native-shutdown-lifecycle-2026-10-18.md) | `1b0fb2546cabe84eb41048d3fb4ce2ed865b38d4` |
| [mode-b-p0-preparation-gate-races-2026-10-18.md](mode-b-p0-preparation-gate-races-2026-10-18.md) | `4015ad70277c8d321245e39f715e5633002f9467`, `0e227a9e3e39d5d2a22708652b7d3c18065498e0`, `66ddf1fc24f906e74c9246c31ebc1ad967b0e8d2`; 2 more in record |
| [mode-b-p0-settings-client-adversarial-2026-10-18.md](mode-b-p0-settings-client-adversarial-2026-10-18.md) | `fa5f549b4d274b7f722aac56e07bdd9307723aa9`, `4b814f6a1260cce6ebf8bbbf25e40b597791afeb`, `cb579a1bbc9ec8fdf6e9e174e7a92d506b8a1f77`; 2 more in record |
| [mode-b-p1-native-placement-bounds-2026-10-18.md](mode-b-p1-native-placement-bounds-2026-10-18.md) | `66ddf1fc24f906e74c9246c31ebc1ad967b0e8d2`, `f936aab3594d600987c473218d11f6b087e40753`, `5a3056bf2a9435028c8e11a3c0cad1dc65321361`; 3 more in record |


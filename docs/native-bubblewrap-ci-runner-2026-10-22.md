# MODE B: actual Bubblewrap/Glycin decoder vs CI runner namespace capability

Date: 2026-10-22. Primary **MODE B (test infrastructure/root cause)**. Scope: investigate red native package smoke without altering production decoder, softening sandbox, skipping integration test, publishing signed APT or modifying production master.

## R1: observed failure and complete test-owner scan

- Run [39828872782](https://github.com/morozhenko-apps/rquickshare/actions/runs/39828872782), exact source `5783998028c0`, `Native Linux package smoke` manual `dev` dispatch. Worker suite passed; Core Rust 783 and native Python/GPG contract tests progressed past the prior GPG failure.
- `native_debian` failed at `Isolated GTK and native lint gate` running `cargo +stable test --locked --manifest-path native_desktop/Cargo.toml`: 9 of 10 `src/lib.rs` preview tests passed, but `given_actual_sandbox_helper_when_decoding_fixture_then_exact_rgba_is_verified` reported `Unavailable`. Package build/upload never ran; zero CI artifacts.
- `native_desktop/src/preview/tests.rs` runs real `rquickshare-preview-helper`, expects exact PNG/JPEG RGBA, and must remain a real Glycin integration test. `native_desktop/src/preview.rs` spawns helper with closed environment (PATH and XDG_DATA_DIRS only), private stdin/stdout, error-only enum and no diagnostic stderr leak. `native_desktop/src/preview_helper.rs` forces `glycin::SandboxSelector::Bwrap`, strict memory/dimensions/deadlines and no access to files/session authority. **Never disable Bwrap, set sandbox selector to None, skip integration, or weaken RGBA expected values.**
- The failing workflow is `ubuntu-24.04` hosted VM **with** `container: ubuntu:26.04`. Docker isolation may prohibit the user or mount namespaces required by a *second* sandbox (Bubblewrap), despite installing both `bubblewrap` and `glycin-loaders`. Native GTK UI verification runs only `--bin rquickshare` and therefore does not execute the failing library decoder test. The tag-release `package` job and Mode B `native_gtk` jobs have the same 24.04-host/26.04-container pattern. Any fix must address all actual release-test/build paths.
- Crucial newly verified platform fact: [GitHub Sept 17 2026](https://github.blog/changelog/2026-09-17-ubuntu-26-generally-available-and-latest-migration/) declares `ubuntu-26.04` a fully supported GitHub-hosted **VM** label. Running natively on that target ABI avoids nested Docker and the need to weaken the container security profile.

## R2/R3: hypotheses and discriminating diagnostics

Candidate causes, in priority order: (1) container runtime namespace/seccomp denial for Bwrap; (2) guest kernel AppArmor user namespace restrictions; (3) Glycin sandbox executable/helper environment or `glycin-loaders` installation issue; (4) XDG_DATA_DIRS/path pruning obscures loader configuration; (5) lack of session D-Bus/display; (6) Bwrap bind or proc mount denial; (7) helper resource limits. The existing `Unavailable` enum intentionally does not expose sensitive image/loader details, so no exact syscall errno is currently established.

**Cheap diagnostic first**: run an isolated credential-free `ubuntu-26.04` GitHub-hosted VM probe with `bubblewrap` installed; check `/etc/os-release`, effective unprivileged user, `bwrap --version`, AppArmor/unprivileged-namespace sysctls read-only and an actual `bwrap --unshare-user --unshare-pid --ro-bind / / --proc /proc --dev /dev -- /usr/bin/true` invocation without sudo. The probe must fail if sandbox cannot start, not merely print a warning. No private filesystem, signing keys, Docker privileged flags or host sysctl mutations. This is a lightweight explicit infrastructure probe, not the full package test. We do not represent it as Glycin end-to-end acceptance.

**If native VM can really create required namespaces**: switch native GTK UI, native-package candidate and final-tag package/Mode B native GTK runner to `runs-on: ubuntu-26.04` with no `container`; install build dependencies with `sudo apt-get`; remove `HOME=/root` override; preserve original actual Bwrap+Glycin real helper test and all lint/package verification. Do not change the 24.04 jobs that don't build 26.04 native ABI. All jobs are already constrained to exact trusted source and need no runtime security-privilege expansion. Prefer this over granting Docker `--privileged` or broad seccomp exceptions.
**If native VM cannot start Bubblewrap**: retain release block. Obtain bounded error evidence and evaluate a narrowly scoped AppArmor profile **only with separate security review**; do not globally disable namespace restrictions or switch sandbox off.

## N1–N12 & coverage/mutation inventory

- Positive: Bwrap unprivileged user/PID namespaces on target VM; exact Glycin PNG/JPEG colors; validated 0.9.0-1 ABI package and signed-APT inputs **only after separate final gate**.
- N1 invalid system binary/setup; N2 resource/namespace limits; N3 missing syscall/kernel feature or container restrictions; N4 repeat invocation reproducibility; N5 runner ref and cancellation not bypassed; N6 wrong OS/ABI; N7 no elevated sandbox privilege or release secrets; N8 no clock requirement; N9 failing sandbox stops CI cleanly; N10 no private image contents in logs; N11 not applicable; N12 actual Bwrap+Glycin test cannot be substituted with pure mock.
- Per production owner: `native_desktop/src/preview_helper.rs` and `preview.rs` unchanged (sandbox and data bounds); `native_desktop/src/preview/tests.rs` real decode kept; `native-package-smoke.yml` and `native-tag-publish.yml` target 26.04 ABI jobs; `native-gtk-ui.yml` and `mode-b.yml` GTK physical sandbox runner; candidate probe new infrastructure only.
- Mutation review: remove `--unshare-user` from probe must be detected by source policy/contract review; a missing bwrap must fail probe; removing the actual Glycin decoder assertion still prohibited; changing any release job to host 24.04 must not silently validate target ABI.

## Execution/completion

1. [x] Repository scan + documented diagnostic hypothesis and safe remediation design before code.
2. [ ] Create a **small, credential-free, auto-run-on-its-own-file-only** Ubuntu 26.04 VM probe. Observe actual runner result and logs.
3. [ ] Only on confirmed support, migrate actual ABI native jobs to host VM and add fail-closed namespace preflight in the real GTK path; keep the user-requested manual-only heavy package/release workflow cadence.
4. [ ] Verify final source Preflight, Native GTK and targeted real helper integration plus deliberate package smoke, then use full tag Mode B only on owner-authorized source/tag. Do not claim completion until each actual run passes and outputs real verified .deb.
5. [ ] Update this docs and issue #14 with real evidence; documentation and code commits remain separate.

Rollback: revert diagnostic/migration code commits only; production source, package version, protected master, tag namespace, signing keys and immutable R2 channel unaffected.

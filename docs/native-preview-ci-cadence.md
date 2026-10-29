# Native preview decoder and release validation cadence

Mode A (implementation and targeted regression). Owner-approved decision: 2026-10-29. This contract supersedes the historic requirement for ten **entire** suites in `mode-b-test-plan.md`; coverage and installed-device acceptance remain mandatory and independent.

## Observed failure and root cause

GitHub Actions [native 1.0.0 run 40964296398](https://github.com/morozhenko-apps/rquickshare/actions/runs/40964296398): tag admission and the ten legacy Core/Tauri/frontend/Python loops passed; Native GTK passed eight full attempts, then on attempt nine `preview::tests::given_external_process_failures_when_cancelled_or_malformed_then_owned_children_are_reaped` failed for oversized external stdout. The Python fixture independently exited 0 and emitted `MAX_OUTPUT+1` bytes, but `Decoder::decode` returned `Unavailable` instead of `Rejected`. A prior single-shot failure was recorded on 2026-10-22. No package, release signature, R2 write or GitHub Release occurred.

Cause in `native_desktop/src/preview.rs`: `tokio::try_join!(write, read)` returns the first error of concurrent child stdin/stdout futures; an OS pipe error from writing to a helper that has already completed or closed stdin can win before the bounded reader reports confirmed stdout overflow. The resulting observable error depends on scheduling rather than on verified malformed stdout.

Other examined hypotheses: invalid Python fixture itself (falsified by independent successful execution); absent /usr/bin/python3 or executable permission (falsified in failed job); Bwrap/Glycin unavailable (real helper test passed); global timeout (the failure occurred in milliseconds); unbounded read/unsafe output acceptance (the read uses a bounded `MAX_OUTPUT+1` reader); nondeterministic pipe scheduling (supported by intermittent classification and first-error join). No sleeps, retries or test suppression are permitted.

**Correction:** drive read and write concurrently but do not allow an early stdin write failure to discard the bounded stdout result. Any verified stdout size greater than `MAX_OUTPUT` is `Rejected`, regardless of stdin failure or child exit code. For non-oversized output, preserve existing stdin failure, child status, cancellation, deadline, and strict pixel framing semantics. On all error/cancellation paths terminate and reap the owned child. Avoid content-bearing logs and never widen the output limit.

## Approved CI architecture

- All stable tests **once**, with no omitted tests: Core Rust library excluding its eight exact-name risk probes, Core integrations, Tauri desktop Rust, frontend Vitest, Python publication contracts, Native GTK library + bin except exact-name risk probes.
- Exactly **ten** consecutive passes of each explicitly enumerated race-sensitive probe; fail immediately on red, do not retry to green. The Core list is the existing eight `scripts/ci-core-test-groups.sh` probes. A new `scripts/ci-native-gtk-test-groups.sh` selects the native library subprocess/cancellation/Bwrap tests and the main-bin shutdown/ownership race tests.
- Require precise test inventory presence before exclusion; prevent zero-test false green. Do not use `--ignored`, automatic retries or environment-wide test skipping. Keep the real Bubblewrap user/PID/mount-namespace check and real sandbox helper.
- Routine Preflight and native GTK CI use stable 1× + isolated stress 10× on source changes. Reusable release Mode B executes the same stable 1× and stress 10× once per tagged source, then a single coverage measurement. Signing, private R2, public HTTPS and exact APT acquisition gates remain unchanged.
- Native GTK suite uses Ubuntu 26.04, D-Bus session and Xvfb. Build `rquickshare-preview-helper` before either selected test set.

## Risk, branch inventory, and invariants

| Owner | Positive | N1-N12 / negative / interaction focus | Coverage / mutation gate |
| --- | --- | --- | --- |
| `native_desktop/src/preview.rs` | Valid PNG/JPEG helper gives exact validated pixels | N1 corrupt frame; N2 oversized output; N3 broken pipe/exit; N4 scheduling race; N5 cancellation; N6 missing helper; N7 actual Bwrap; N8 deadline; N9 child reap; N10 redaction; N11 n/a; N12 truncated/extra bytes | Must reject oversized stdout even when write fails first; validate malformed and exited children. |
| `native_desktop/src/preview/tests.rs` | In-range RGBA/real helper | Explicit early-stdin-close/oversized-output regression; retains old fixture assertions, cancellations and owned child check | A mutation restoring `try_join!` or rejecting only after `stdin` must fail the regression or stress gate. |
| `scripts/ci-core-test-groups.sh` | Stable full Core integration once; eight exact stress selectors | Missing/renamed test, zero-test, scheduler-sensitive errors, early red stop | Keep existing list/inventory and assert output counts. |
| `scripts/ci-native-gtk-test-groups.sh` | Entire native GTK stable lib/bin once | N2/N4/N5/N7/N9: selected worker and subprocess races 10×; missing test fails before any suite; both test targets explicitly named | Detect accidental empty filters, exact selected methods, stable pass counts and first failure. |
| `lint.yml`, `native-gtk-ui.yml`, `mode-b.yml` | Fast source checks and protected release gate | No 10× entire-suite regression, no bypass of stable suite, permissions and package/signature gates unchanged | Static policy regression and release job dependency review. |

Mutation review: dropping a selected method, filtering to zero tests, restoring unconditional full 10×, allowing oversized pixels, ignoring write failure on valid output, or skipping child reap must be caught by policy checks or targeted tests. No fabricated mutation execution or coverage percentage is claimed.

## Milestones and rollback

1. Documentation + root-cause evidence (this file; docs-only commit).
2. Scoped production correction, new process regression, exact-name GTK selectors and CI cadence changes in one code/test commit.
3. Final verification via automatic dev Preflight + native GTK workflows, then evaluate failures on their exact source SHA. No direct workflow polling and no additional manual 10× gate during implementation.
4. Separately, only after CI is green, protected master promotion and owner-directed version/tag/release recovery; do not move `native/v1.0.0` from this task. Do not touch current signed archive, secrets or R2.

Rollback: revert scoped code/test commit to restore prior classification and workflows; documentation records previous release failure and this decision. The one-off native tag recovery procedure remains separate. Device-to-device validation and Mode B Stage 12 physical acceptance are still outstanding.

## Implementation checkpoint

The scoped correction, early-stdin-close regression, native GTK exact inventory selector, selected Core/GTK stress jobs, source policy assertions and Node contract regression were implemented on `fix/native-preview-stress-20261029`. The six selected native GTK test names were cross-checked against actual test names recorded in the failed release's earlier passing attempts. Source-level consistency checks passed; no compiled test, source CI, dedicated GTK CI, coverage measurement or new package publication is claimed by this checkpoint.

Next required gate: feature PR against `dev` on the final code SHA; the PR automatically invokes Preflight and native GTK verification. Stop after triggering those jobs; on a completed red job, inspect exact logs and fix the root cause before another full gate. Promotion to protected `master`, tag replacement, signature import and R2 publication remain outside this change.

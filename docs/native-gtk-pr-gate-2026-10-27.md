# Native GTK pull request validation before merge

Date: 2026-10-27, Mode A CI improvement. Baseline `dev` `ce567dddd133a12fbd313cee5d7c64d0af74ee31`. Current `.github/workflows/native-gtk-ui.yml` triggers only on push to `dev` and manually. It does not run on PR #28, where localized GTK code is being reviewed. A passing generic Preflight alone cannot satisfy GTK Clippy, native widget/Xvfb or locale tests before merge.

## Plan

1. Add `pull_request` event for target branch `dev`, with the same limited relevant paths as existing `push`: `native_desktop/**`, `core_lib/**`, and the workflow file itself.
2. Keep existing `push: dev` and `workflow_dispatch` events, single job and read-only token permissions. Preserve exact native GTK prerequisite install, stable Rust Clippy and Xvfb test commands. No additional 10x runs or release publication.
3. Retain `native-gtk-${{ github.ref }}` concurrency cancellation: for pull requests the ref is PR-specific; on dev it remains branch-specific. Never interfere with unrelated owners' runner sessions.
4. Update docs before YAML; commit docs and CI code separately with expected-SHA compare-and-swap on `dev`.
5. Push a small final source/exact-doc change to branch `feature/native-text-review-i18n-20261026` to generate fresh PR checks, verify PR Preflight + Native GTK (including rustfmt, Clippy and widget tests) to completion. Leave PR draft if any gate fails or physical/linguistic review is unresolved.

## Safety and rollback

- Security: `pull_request` event executes untrusted PR code with read-only `contents: read`; no `pull_request_target`, secrets or write privileges. Keep build/test privileges unchanged.
- Resource risk: each distinct feature PR can start one native GTK gate; existing `cancel-in-progress` deduplicates repeated pushes to the same PR ref.
- CI scope: this improves pre-merge evidence; do not claim overall 1.0.0 Mode B, KDE/Pixel acceptance or professional translation validation. Other workflows and release gates retain existing trigger contracts.
- Rollback: independently revert the workflow commit without changing release or production code. Reverting this plan alone is docs-only.
- Validation: review YAML trigger/static policy and read terminal source-exact workflow jobs; do not claim local GTK execution from a GitHub-only agent environment.

**State before YAML:** CI gap verified, implementation not yet applied.

#!/usr/bin/env bash
# Fail-closed PR change classification for required Preflight and GTK checks.
set -euo pipefail

: "${GITHUB_OUTPUT:?GITHUB_OUTPUT must point to the job output file}"

preflight=true
gtk=true

if [[ "${GITHUB_EVENT_NAME:-}" == pull_request ]]; then
  # GitHub's pull_request checkout is a synthetic merge commit. Compare
  # against its first (target-branch) parent, never the untrusted PR base name.
  read -r -a refs <<< "$(git rev-list --parents -n 1 HEAD)"
  [[ "${#refs[@]}" -eq 3 ]] || { echo "Expected a two-parent PR merge commit" >&2; exit 1; }

  changed="$(mktemp)"
  trap 'rm -f -- "$changed"' EXIT
  git diff --name-only -z "${refs[1]}" "${refs[0]}" > "$changed"
  [[ -s "$changed" ]] || { echo "PR merge has no changed files" >&2; exit 1; }

  preflight=false
  gtk=false
  while IFS= read -r -d '' path; do
    # Documentation changes must not start source tests. The whitelist is
    # intentionally narrow: unknown file types default to executable changes.
    case "$path" in
      docs/*|doc/*|*/docs/*|*/doc/*|README.md|*/README.md|AGENT.md|*/AGENT.md|AGENTS.md|*/AGENTS.md|CHANGELOG.md|*/CHANGELOG.md|CONTRIBUTING.md|*/CONTRIBUTING.md|.github/agents/*|.codex.md|*.md|*.mdx)
        continue ;;
    esac

    preflight=true
    case "$path" in
      native_desktop/*|core_lib/*|scripts/ci-native-gtk-test-groups.sh|scripts/ci-pr-scope.sh|scripts/tests/native-ci-cadence.test.mjs|scripts/tests/pr-ci-scope.test.mjs|scripts/native_*.py|.github/workflows/native-*|.github/workflows/lint.yml|.github/workflows/feature-package-smoke.yml)
        gtk=true ;;
    esac
  done < "$changed"
elif [[ "${GITHUB_EVENT_NAME:-}" != workflow_dispatch && "${GITHUB_EVENT_NAME:-}" != push ]]; then
  echo "Unsupported CI event" >&2
  exit 1
fi

printf 'preflight=%s\ngtk=%s\n' "$preflight" "$gtk" >> "$GITHUB_OUTPUT"

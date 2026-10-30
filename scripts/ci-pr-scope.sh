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
  # Keep both sides of renames: moving an executable input into docs does not
  # make removal of the original native/package input a prose-only change.
  git diff --no-renames --name-only -z "${refs[1]}" "${refs[0]}" > "$changed"
  [[ -s "$changed" ]] || { echo "PR merge has no changed files" >&2; exit 1; }

  preflight=false
  gtk=false
  while IFS= read -r -d '' path; do
    # Native/package inputs take precedence over their extension, including
    # packaged README files. Docs also contains live non-prose test fixtures.
    case "$path" in
      core_lib/*|native_desktop/*)
        preflight=true
        gtk=true
        continue ;;
      docs/*.md|doc/*.md|README.md|AGENT.md|AGENTS.md|CHANGELOG.md|CONTRIBUTING.md|.github/agents/*.md|.codex.md)
        continue ;;
    esac

    preflight=true
    case "$path" in
      app/main/*)
        # This legacy frontend/Tauri tree has its own complete Preflight lane.
        ;;
      *) gtk=true ;;
    esac
  done < "$changed"
elif [[ "${GITHUB_EVENT_NAME:-}" != workflow_dispatch && "${GITHUB_EVENT_NAME:-}" != push ]]; then
  echo "Unsupported CI event" >&2
  exit 1
fi

printf 'preflight=%s\ngtk=%s\n' "$preflight" "$gtk" >> "$GITHUB_OUTPUT"

#!/usr/bin/env bash
# Inventory-verified native GTK test partition: stable 1x, exact risk probes 10x.
set -euo pipefail

case "${1:-}" in
  stable|stress|list) group="$1" ;;
  -h|--help) echo "Usage: bash scripts/ci-native-gtk-test-groups.sh stable|stress|list"; exit 0 ;;
  *) echo "Usage: bash scripts/ci-native-gtk-test-groups.sh stable|stress|list" >&2; exit 2 ;;
esac
if (( $# != 1 )); then
  echo "Expected one group argument" >&2
  exit 2
fi

# These are race-sensitive risk probes, not quarantined or ignored failures.
LIB_RACE_SENSITIVE=(
  'preview::tests::given_cancelled_busy_or_missing_decoder_when_requested_then_no_queued_operation_starts'
  'preview::tests::given_external_process_failures_when_cancelled_or_malformed_then_owned_children_are_reaped'
  'preview::tests::given_actual_sandbox_helper_when_decoding_fixture_then_exact_rgba_is_verified'
)
BIN_RACE_SENSITIVE=(
  'lifecycle::tests::given_signal_and_close_racing_when_both_complete_then_shutdown_is_idempotently_cancelled'
  'ingress::tests::given_dropped_image_wait_when_worker_is_cancelled_then_gate_remains_owned_and_shutdown_joins'
  'portal::tests::given_pending_portal_when_gui_wait_drops_or_owner_disappears_then_close_is_owned_and_ingress_shutdown_joins'
)

if [[ "$group" == list ]]; then
  printf 'lib:%s\n' "${LIB_RACE_SENSITIVE[@]}"
  printf 'bin:%s\n' "${BIN_RACE_SENSITIVE[@]}"
  exit 0
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
cargo_cmd=(cargo +stable test --locked --manifest-path native_desktop/Cargo.toml)

run_target() {
  local target="$1" mode="$2" selected_name="$3"
  local -n selected="$selected_name"
  local inventory count expected method occurrences output
  local -a target_args=(--lib)
  if [[ "$target" == bin ]]; then
    target_args=(--bin rquickshare)
  fi
  inventory="$("${cargo_cmd[@]}" "${target_args[@]}" -- --list)"
  count="$(printf '%s\n' "$inventory" | grep -cE ': test$' || true)"
  if (( count <= ${#selected[@]} )); then
    echo "Native GTK $target inventory too small: $count" >&2
    exit 1
  fi
  for method in "${selected[@]}"; do
    occurrences="$(printf '%s\n' "$inventory" | grep -Fxc -- "$method: test" || true)"
    if [[ "$occurrences" != 1 ]]; then
      echo "Native GTK $target selected method missing or duplicated: $method ($occurrences)" >&2
      exit 1
    fi
  done

  if [[ "$mode" == stable ]]; then
    local skips=()
    for method in "${selected[@]}"; do
      skips+=(--skip "$method")
    done
    expected=$((count - ${#selected[@]}))
    echo "Native GTK $target stable: $expected tests once; ${#selected[@]} exact stress methods excluded."
    output="$("${cargo_cmd[@]}" "${target_args[@]}" -- "${skips[@]}" 2>&1)" || {
      printf '%s\n' "$output"
      exit 1
    }
    printf '%s\n' "$output"
    if ! grep -Fq "test result: ok. $expected passed; 0 failed; 0 ignored;" <<< "$output"; then
      echo "Native GTK $target stable count mismatch (expected $expected)" >&2
      exit 1
    fi
    return 0
  fi

  local attempt
  for attempt in $(seq 1 10); do
    echo "Native GTK $target stress attempt $attempt/10"
    for method in "${selected[@]}"; do
      output="$("${cargo_cmd[@]}" "${target_args[@]}" "$method" -- --exact 2>&1)" || {
        printf '%s\n' "$output"
        echo "Native GTK $target stress failure on attempt $attempt: $method" >&2
        exit 1
      }
      printf '%s\n' "$output"
      if ! grep -Fq 'test result: ok. 1 passed; 0 failed; 0 ignored;' <<< "$output"; then
        echo "Native GTK $target stress filter did not run exactly one test: $method" >&2
        exit 1
      fi
    done
  done
}

run_target lib "$group" LIB_RACE_SENSITIVE
run_target bin "$group" BIN_RACE_SENSITIVE
if [[ "$group" == stable ]]; then
  "${cargo_cmd[@]}" --doc
fi
echo "Native GTK $group group completed."

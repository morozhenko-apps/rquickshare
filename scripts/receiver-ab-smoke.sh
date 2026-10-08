#!/usr/bin/env bash
# Hardware A/B probe for the debug-only slot-0 advertising recovery switch.
# Usage: bash scripts/receiver-ab-smoke.sh baseline|skip-deferred
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: bash scripts/receiver-ab-smoke.sh baseline|skip-deferred

Runs one foreground rQuickShare session and captures a timestamped log under
~/Downloads/rquickshare-smoke/ab-logs/. Close the app from its tray before starting each trial. After receiving the
file, select Quit from the rQuickShare tray menu, not Ctrl+C: the graceful
shutdown unregisters its mDNS service so Android does not cache stale peers.
Run three independent cold-start trials of each variant with the same device,
network, visibility and app foreground/background state.
USAGE
}

case "${1:-}" in
  baseline)
    variant=baseline
    refresh=1
    ;;
  skip-deferred)
    variant=skip-deferred
    refresh=0
    ;;
  -h|--help)
    usage
    exit 0
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac

binary=/usr/bin/rquickshare
if [[ ! -x "$binary" ]]; then
  printf 'Error: %s is not installed or executable.\n' "$binary" >&2
  exit 1
fi

# An already-running Tauri instance can consume the command line and keep its
# old environment. Do not kill it: closing the tray app is a user action.
if pgrep -u "$(id -u)" -x rquickshare >/dev/null; then
  printf 'Error: rQuickShare is running. Exit from its tray before the cold trial.\n' >&2
  exit 1
fi

log_dir="${HOME}/Downloads/rquickshare-smoke/ab-logs"
mkdir -p "$log_dir"
stamp="$(date -u +%Y%m%dT%H%M%S.%NZ)"
log_file="${log_dir}/${variant}-${stamp}.log"
printf 'Variant: %s (RQS_DIAG_SLOT0_ADV_REFRESH=%s)\n' "$variant" "$refresh"
printf 'Log: %s\n' "$log_file"
printf 'Set the same visibility/foreground state, share one image from Pixel,\n'
printf 'then select Quit from the rQuickShare tray menu to stop gracefully.\n'
printf 'Do NOT use Ctrl+C here: it can prevent mDNS service unregistration.\n'

# This switch is intentionally ignored by release builds. The diagnostic
# startup line in the log proves that the installed binary is a debug build.
set +e
RQS_DIAG_SLOT0_ADV_REFRESH="$refresh" RQS_LOG=trace "$binary" 2>&1 | tee "$log_file"
app_status=${PIPESTATUS[0]}
set -e

if ! grep -q "diagnostic slot0 deferred advertising refresh enabled=" "$log_file"; then
  printf '\nWarning: debug diagnostic marker missing; do not use this run for A/B results.\n' >&2
fi

# A sudden SIGINT/SIGTERM may leave a stale randomized mDNS endpoint on Android.
# Do not infer a daemon bug until a trial completed with a graceful tray Quit.
if ! grep -q "MDnsServer: service unregistered" "$log_file"; then
  printf '\nWarning: mDNS graceful unregistration was not confirmed.\n' >&2
  printf 'Quit via tray instead of Ctrl+C. This run cannot establish whether stale Android devices were cleaned up.\n' >&2
fi

printf '\nA/B signals from %s:\n' "$log_file"
grep -E 'diagnostic slot0|PassiveBleMonitor|passive FE2C monitor|slot0 read|slot0 requested|deferred advertising refresh|weave notify|weave session|TCP connection|TCP peer|BWU:|state: Some\(Finished\)|state: Some\(Disconnected\)|BleListener stopped' "$log_file" | tail -n 90 || true
printf '\nKeep the complete log for the comparison. App exit code: %s\n' "$app_status"

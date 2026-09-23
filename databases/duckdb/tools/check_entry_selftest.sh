#!/usr/bin/env bash
# Usage: tools/check_entry_selftest.sh
# check.sh runs the same from any directory, by a relative path, and a
# failed setup step exits nonzero (surfaces-review-5: from another folder
# it looked for tools/version.env beside the caller). The setup-only door
# stops check.sh right after its setup, so this runs in a second.
set -euo pipefail
cd "$(dirname "$0")/../.."
out=$(CHECK_SETUP_ONLY=1 bash duckdb/check.sh 2>&1) || {
  echo "FAILED   check.sh by a relative path from databases/ exited nonzero: $out"
  exit 1
}
if [[ "$out" != *"setup ok"* ]]; then
  echo "FAILED   check.sh by a relative path from databases/ did not finish its setup: $out"
  exit 1
fi
echo "ok       check.sh runs its setup from another folder by a relative path"

# Compare the one-bar filter's kept original records, without a second JSON view.
set -e
kept=$(mktemp)
trap 'status=$?; trap - EXIT; rm -f "$kept"; exit "$status"' EXIT
bash -o pipefail show.sh | tee "$kept"
jq -Rsc 'split("\n") | .[:-1]' "$kept" > "$SITE_ANSWERS_FILE"

set -eu
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0) | if . == "unsure" then null else . end)' \
  "$1" > "$SITE_ANSWERS_FILE"

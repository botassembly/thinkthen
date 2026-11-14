set -eu
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[rtrimstr("\n") | if (. == "no line answers" or . == "unsure") then null else . end]' \
  "$1" > "$SITE_ANSWERS_FILE"

set -eu
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(capture("^(?<refund>true|false) \"?(?<topic>[^\" ]+)\"? (?<urgency>[0-9.eE+-]+)$") | {refund: (.refund | fromjson), topic: (if .topic == "unsure" then null else .topic end), urgency: (.urgency | tonumber)})' \
  "$1" > "$SITE_ANSWERS_FILE"

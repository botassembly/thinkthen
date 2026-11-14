set -eu
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0) | split(" ") | {refund: (.[0] | fromjson), topic: (if .[1] == "unsure" then null else .[1] end), urgency: (.[2] | tonumber)})' \
  "$1" > "$SITE_ANSWERS_FILE"

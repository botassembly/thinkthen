set -eu
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '$case[0].records as $records | split("\n") | map(select(length > 0))
  | map(capture("^(?<rank>[0-9]+) (?<text>.*)$") // error("invalid printed rank row"))
  | if (map(.text) | sort) != ($records | sort)
    then error("printed rank records differ from case records")
    else sort_by(.text as $text | $records | index($text)) | map(.rank | tonumber)
    end' \
  "$1" > "$SITE_ANSWERS_FILE"

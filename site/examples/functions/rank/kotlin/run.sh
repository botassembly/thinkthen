. ../../../lang-kotlin/run.sh
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '$case[0].records as $records | split("\n") | map(select(length > 0))
  | map(capture("^(?<rank>[0-9]+) (?<text>.*)$") // error("invalid printed rank row"))
  | map(.text |= fromjson)
  | if (map(.text) | sort) != ($records | sort)
    then error("printed rank records differ from case records")
    else sort_by(.text as $text | $records | index($text)) | map(.rank | tonumber)
    end' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"

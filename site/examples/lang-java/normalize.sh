# Normalize the displayed JVM answers against the shared command case.
function_name=$(cat "$entry/function-name")
case "$function_name" in
  annotate)
    jq -s -c 'sort_by(.position) | map(.answer | map_values(.answer))' "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  choose)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0))' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  decide)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(fromjson)' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  filter)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0))' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  find)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(if . == "no line answers" then null else . end)' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  rank)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '$case[0].records as $records | split("\n") | map(select(length > 0)) | map(capture("^(?<rank>[0-9]+) (?<text>.*)$")) | sort_by(.text as $text | $records | index($text)) | map(.rank | tonumber)' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  recognize)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[split("\n") | map(select(length > 0)) | map(capture("^(?<kind>[^:]+): (?<text>.*)$") | {text, kind})]' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  relate)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[split("\n") | map(select(length > 0)) | map(capture("^(?<source>.*) --(?<relation>[^ ]+)--> (?<target>.*)$"))]' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  score)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(fromjson)' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  tag)
    jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(ltrimstr("[") | rtrimstr("]") | split(", ") | map(select(length > 0)) | sort)' \
      "$work/output.txt" > "$SITE_ANSWERS_FILE"
    ;;
  *) echo "Unknown prepared JVM function: $function_name" >&2; exit 2 ;;
esac

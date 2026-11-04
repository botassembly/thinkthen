. ../../../lang-typescript/run.sh
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[split("\n") | map(select(length > 0) | capture("^(?<kind>[^ ]+) (?<text>.*)$") | {text, kind})]' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"

. ../../../lang-typescript/run.sh
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[rtrimstr("\n") | if . == "no line answers" then null else . end]' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"

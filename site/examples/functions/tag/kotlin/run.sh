. ../../../lang-kotlin/run.sh
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0) | ltrimstr("[") | rtrimstr("]") | if length == 0 then [] else split(", ") | sort end)' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"

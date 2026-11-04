. ../../../lang-ruby/run.sh
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[fromjson | sort]' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"

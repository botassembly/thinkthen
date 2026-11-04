. ../../../lang-typescript/run.sh
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[split("\n") | map(select(length > 0)) | map(. as $line | $case[0].rules[0].relation as $relation | $line | split(" " + $relation + " ") | {source: .[0], relation: $relation, target: .[1]})]' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"

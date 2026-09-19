#!/bin/sh
# Turn the object arm's saved answers into one row per case, so analyse.sh can
# read the two arms side by side. It opens no connection.
set -eu
cd -- "$(dirname -- "$0")"

: >object-rows.jsonl
while IFS= read -r case; do
	id=$(printf '%s\n' "$case" | jq -r .id)
	jq -c --argjson case "$case" \
		'{input: $case, p: .answers.q1.noul, usage}' "answers/$id.json" \
		>>object-rows.jsonl
done <cases.jsonl
printf 'wrote %s rows to object-rows.jsonl\n' "$(wc -l <object-rows.jsonl | tr -d ' ')"

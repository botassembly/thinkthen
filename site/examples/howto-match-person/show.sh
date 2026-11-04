jq -c 'select(.same == true) | .pair' matched.jsonl \
  > same.jsonl
jq -c 'select(.same == null or .same == "unsure") | .pair' \
  matched.jsonl > person.jsonl
cat person.jsonl

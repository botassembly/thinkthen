by_title='.input = {
  id: (.input.input | split("\nText: ") | last)
}'
jq -c "$by_title" ../11-audit/rows.jsonl > a.jsonl
jq -c "$by_title" ../11-audit/rows-context.jsonl > b.jsonl

thinkthen diff a.jsonl b.jsonl \
  --threshold 0.5 \
  --key ../11-audit/key.jsonl |
tail -n 1 |
jq '.summary | {changed, right_a, right_b, lost}'

by_title='.input = {
  id: (.input.input | split("\nText: ") | last)
}'
jq -c "$by_title" rows.jsonl > memory.jsonl
jq -c "$by_title" rows-context.jsonl > context.jsonl

thinkthen diff memory.jsonl context.jsonl \
  --threshold 0.5 \
  --key key.jsonl |
tail -n 1 |
jq '.summary | {changed, right_a, right_b, lost}'

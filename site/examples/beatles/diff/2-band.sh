thinkthen diff memory.jsonl context.jsonl \
  --threshold 0.2:0.8 \
  --key key.jsonl |
tail -n 1 |
jq '.summary | {changed, right_a, right_b, lost}'

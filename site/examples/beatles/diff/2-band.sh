thinkthen diff a.jsonl b.jsonl \
  --threshold 0.2:0.8 \
  --key ../11-audit/key.jsonl |
tail -n 1 |
jq '.summary | {changed, right_a, right_b, lost}'

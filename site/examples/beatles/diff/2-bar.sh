thinkthen diff memory.jsonl context.jsonl \
  --threshold 0.5 \
  --key key.jsonl |
jq -c 'if .summary
  then .summary | {changed, right_a, right_b, lost}
  else [.id, .from, .to, .effect]
  end'

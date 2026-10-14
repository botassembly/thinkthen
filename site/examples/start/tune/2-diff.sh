thinkthen runs diff shown.jsonl \
  --threshold 0.5 \
  --compare-threshold 0.78 \
  --key shown-key.jsonl |
jq -c 'if .summary
  then .summary | {changed, right_a, right_b}
  else [.id, .from, .to]
  end'

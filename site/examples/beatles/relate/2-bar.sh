thinkthen relate @rules.json \
  --jsonl \
  --input names.jsonl \
  --threshold 0.8 \
  --replay recording |
jq -c '[.source.name, .relation,
  .target.name, .probability]'

thinkthen relate @rules.json \
  --jsonl \
  --input names.jsonl \
  --replay recording |
jq -c '[.source.name, .relation,
  .target.name, .probability]'

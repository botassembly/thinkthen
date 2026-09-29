thinkthen audit shown.jsonl shown-key.jsonl \
  --threshold 0.2:0.8 |
jq '{
  songs: .rows,
  right,
  wrong,
  not_sure: .unsure
}'

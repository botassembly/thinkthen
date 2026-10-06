thinkthen runs audit shown.jsonl shown-key.jsonl \
  --threshold 0.78 |
jq '{
  songs: .rows,
  right,
  wrong_yes: .false_yes,
  missed_yes: .false_no
}'

thinkthen audit rows.jsonl key.jsonl \
  --id /input \
  --threshold 0.78 |
jq '{
  songs: .rows,
  right,
  wrong_yes: .false_yes,
  missed_yes: .false_no,
  suggested_bar: .suggested.cut
}'

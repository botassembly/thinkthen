thinkthen audit rows.jsonl key.jsonl \
  --id /input \
  --threshold 0.2:0.8 |
jq '{
  songs: .rows,
  right,
  wrong,
  not_sure: .unsure,
  suggested_bar: .suggested.cut
}'

thinkthen audit rows.jsonl key.jsonl \
  --id /input |
jq '{
  right,
  wrong_yes: .false_yes,
  missed_yes: .false_no,
  suggested_bar: .suggested.cut
}'

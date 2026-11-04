thinkthen decide --field /text --threshold 0.8 \
  --into strict 'Does the customer request a refund?' \
  answered.jsonl > strict.jsonl
jq -s -c '{
  wrong_yes: map(select(.strict == true
    and .label == false)) | length,
  missed_yes: map(select(.strict == false
    and .label == true)) | length
}' strict.jsonl

thinkthen decide --field /text --json \
  --low 0.2 --high 0.8 \
  'Does the customer request a refund?' messages.jsonl |
  jq -r 'select(.status == "unsure") | .id'

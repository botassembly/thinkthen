thinkthen decide --field /text --json \
  'Does the customer request a refund?' messages.jsonl more.jsonl |
  jq '{position, id, answer}'

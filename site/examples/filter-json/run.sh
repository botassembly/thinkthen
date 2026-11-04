thinkthen filter --field /text --json \
  'Does the customer request a refund?' messages.jsonl |
  jq .

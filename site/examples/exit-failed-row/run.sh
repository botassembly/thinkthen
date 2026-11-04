thinkthen decide --json --from jsonl --field /text \
  'Does the customer request a refund?' broken.jsonl |
  jq .

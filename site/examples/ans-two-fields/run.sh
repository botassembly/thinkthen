jq -c '{id, message: {subject, body}}' two-fields.jsonl |
  thinkthen decide --from jsonl --field /message --json \
    'Does the customer request a refund?' |
  jq '{id, answer, probability}'

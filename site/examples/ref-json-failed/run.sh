echo '{"id":"a4"}' |
  thinkthen decide --from jsonl --field /text --json \
    'Does the customer request a refund?' | jq .

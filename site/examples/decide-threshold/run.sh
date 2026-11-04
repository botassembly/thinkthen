printf '%s' 'Please refund my order.' |
  thinkthen decide --threshold 0.9 --json \
    'Does the customer request a refund?' |
  jq .

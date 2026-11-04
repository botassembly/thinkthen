echo 'Please refund my order.' |
  thinkthen decide --json \
    'Does the customer request a refund?' |
  jq -c '.source | {backend, model}'

echo 'I was charged twice for one invoice.' |
  thinkthen decide --json \
    'Does the customer request a refund?' |
  jq -c '{status, answer, probability}'

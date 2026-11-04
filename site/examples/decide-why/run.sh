echo 'I was charged twice for one invoice.' |
  thinkthen decide --low 0.2 --high 0.8 --json \
    'Does the customer request a refund?' |
  jq -c '{status, probability}'

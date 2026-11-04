printf '%s' 'I was charged twice for one invoice.' |
  thinkthen decide --check --low 0.2 --high 0.8 \
    'Does the customer request a refund?'

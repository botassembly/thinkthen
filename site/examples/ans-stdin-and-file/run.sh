printf '%s\n' 'I was charged twice for one invoice.' |
  thinkthen decide --from lines --json \
    'Does the customer request a refund?' saved.txt - |
  jq '{position, answer}'

thinkthen decide --low 0.2 --high 0.8 --json \
  'Does the customer request a refund?' messages.txt |
  jq -c '{item, status, answer}'

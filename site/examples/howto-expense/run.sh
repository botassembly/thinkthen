thinkthen decide --json --low 0.3 --high 0.7 \
  'Does the expense follow the policy?' expenses.jsonl |
  jq -c '{id, status, answer, probability}'

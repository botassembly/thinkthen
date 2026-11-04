thinkthen decide --field /pair --json \
  --low 0.2 --high 0.8 \
  'Do left and right name the same company at the same place?' pairs.jsonl |
  jq -c '{id, status, answer, probability}'

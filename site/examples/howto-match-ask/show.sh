thinkthen decide --field /pair --low 0.2 --high 0.8 \
  --into same 'Do left and right name the same company?' \
  pairs.jsonl > matched.jsonl
jq -c '{id, same}' matched.jsonl

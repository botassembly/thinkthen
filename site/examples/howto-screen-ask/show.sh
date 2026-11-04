thinkthen annotate --into screen \
  screen.json papers.jsonl > screened.jsonl
jq -c '{id, screen}' screened.jsonl

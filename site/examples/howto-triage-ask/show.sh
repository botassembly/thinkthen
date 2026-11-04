thinkthen annotate --into triage \
  triage.json messages.jsonl > triaged.jsonl
jq -c '{id, triage}' triaged.jsonl

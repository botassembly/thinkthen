head -n 1 messages.jsonl |
  thinkthen annotate --json --from jsonl questions.json |
  jq '.answer'

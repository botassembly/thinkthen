jq -c '.[]' tickets.json |
thinkthen annotate triage.json \
  --jsonl \
  --field /body \
  --jobs 16 |
jq 'del(.body)'

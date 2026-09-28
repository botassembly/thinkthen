jq -c '.[]' tickets.json |
thinkthen annotate triage.json \
  --batch 1 \
  --jsonl \
  --field /body \
  --jobs 16 |
jq 'del(.body)'

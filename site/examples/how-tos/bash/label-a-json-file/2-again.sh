jq -c '.[]' tickets.json |
thinkthen annotate triage.json \
  --jsonl \
  --field /body \
  --jobs 16 \
  --details |
head -n 1 |
jq .

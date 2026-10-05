set -euo pipefail
thinkthen choose @files/question.json \
  --jsonl --field /claim --field /source \
  --input files/cases.jsonl --batch 10 \
  --replay files/recording \
  --url https://api.typesafe.ai/v1 \
  --max-requests-total 0 > results.jsonl
jq -c '
  {id: .input.id, verdict: .value,
   review: (if .value == "supports"
     then "Person must review source before action"
     else "No acceptance" end)}' results.jsonl

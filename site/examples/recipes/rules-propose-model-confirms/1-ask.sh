set -euo pipefail
python3 files/candidates.py files/cases.jsonl \
  --adapt > candidates.jsonl
selected_totals=$(thinkthen choose @files/question.json \
  --jsonl \
  --field /text --options /options --batch 1 \
  --replay files/recording \
  --url http://127.0.0.1:41449 \
  --input candidates.jsonl)
printf '%s\n' "$selected_totals" |
  jq -c '
    .value as $choice |
    {id: .input.id,
     total: (if $choice == null or $choice == "none"
             then null else
             [.input.candidates[] |
              select(.label == $choice) | .value][0]
             end)}'

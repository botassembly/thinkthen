question="Does this sentence hedge?"

jq -cR '{line: input_line_number, text: .}' draft.txt |
thinkthen filter "$question" \
  --batch 1 \
  --jsonl \
  --field /text |
jq -r '"\(.line): \(.text)"'

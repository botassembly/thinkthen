question="Does this change what the code does when it runs?"

jq -cRnf hunks.jq change.diff |
thinkthen decide "$question" \
  --batch 1 \
  --jsonl \
  --field /hunk |
jq '{file: .input.file, at: .input.at, changes: .value}'

thinkthen rank --field /text --json \
  --level cold --level warm --level hot --top 2 \
  'How ready is this lead to buy?' leads.jsonl |
  jq -c '{rank: .answer, id}'

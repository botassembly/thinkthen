thinkthen rank --field /text --json \
  --level routine --level urgent --top 2 \
  'How urgent is the request?' messages.jsonl |
  jq .

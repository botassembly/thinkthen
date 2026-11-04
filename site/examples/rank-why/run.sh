thinkthen rank --json --level routine --level urgent \
  'How urgent is the request?' messages.txt |
  jq -c '{position, answer, levels: .detail.levels}'

thinkthen rank 'How urgent is the request?' \
  --level routine --level urgent --from csv \
  --column text --into urgency_rank < messages.csv

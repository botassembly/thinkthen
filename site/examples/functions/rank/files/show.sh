thinkthen rank 'How urgent is the request?' messages.csv \
  --level routine --level urgent \
  --column message --into urgency_rank
cat messages.thinkthen.csv

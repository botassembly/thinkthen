# cut keeps the first five columns: id, text, the answer, its status
# and its probability. The error, detail and source columns follow.
thinkthen rank --column text --into urgency_rank \
  --level routine --level urgent --top 2 \
  --from csv 'How urgent is the request?' < messages.csv |
  cut -d, -f 1-5

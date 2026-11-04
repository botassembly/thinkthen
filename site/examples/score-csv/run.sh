# cut keeps the first five columns: id, text, the answer, its status
# and its probability. The error, detail and source columns follow.
thinkthen score --column text --into urgency \
  --level routine --level urgent \
  --from csv 'How urgent is the request?' < messages.csv |
  cut -d, -f 1-5

# cut keeps the first five columns: id, text, the answer, its status
# and its probability. The error, detail and source columns follow.
thinkthen filter --column text --into refund \
  --from csv 'Does the customer request a refund?' < messages.csv |
  cut -d, -f 1-5

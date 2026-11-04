thinkthen decide --column text --into refund \
  --from tsv 'Does the customer request a refund?' < messages.tsv |
  cut -f 1-5

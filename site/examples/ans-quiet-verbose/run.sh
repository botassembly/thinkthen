thinkthen decide --field /text --verbose \
  'Does the customer request a refund?' messages.jsonl 2>&1 |
  sed -E 's/[0-9]+\.[0-9]+s$/TIME/'

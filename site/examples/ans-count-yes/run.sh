thinkthen decide --field /text --json \
  'Does the customer request a refund?' messages.jsonl |
  jq -s '{records: length, yes: map(select(.answer == true)) | length}'

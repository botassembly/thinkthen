thinkthen decide --field /text --json \
  'Does the customer request a refund?' messages.jsonl |
  jq -rs '["id", "status", "answer", "probability"],
    (.[] | [.id, .status, .answer, .probability]) | @csv' |
  tee refunds.csv

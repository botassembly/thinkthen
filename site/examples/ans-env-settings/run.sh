export THINKTHEN_FIELD=/text
export THINKTHEN_JSON=true
thinkthen decide --json 'Does the customer request a refund?' messages.jsonl |
  jq '{id, answer}'

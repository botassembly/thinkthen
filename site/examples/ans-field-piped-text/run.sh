#!/usr/bin/env bash
printf '%s\n' '{"id":"a1","text":"Please refund my order."}' |
  thinkthen decide --from jsonl --field /text --json \
    'Does the customer request a refund?' |
  jq '{id, status, answer}'

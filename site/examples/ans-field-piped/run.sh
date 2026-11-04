#!/usr/bin/env bash
printf '%s\n' '{"id":"a1","text":"Please refund my order."}' \
  '{"id":"a2","text":"Where is my parcel?"}' |
  thinkthen decide --field /text --json \
    'Does the customer request a refund?' |
  jq '{id, status, error: .error.code}'

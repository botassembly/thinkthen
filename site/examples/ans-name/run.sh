#!/usr/bin/env bash
printf '%s' 'Please refund my order.' |
  thinkthen decide --json --name refund-check \
    'Does the customer request a refund?' |
  jq '{answer, name: .source.name, backend: .source.backend,
      model: .source.model}'

# Nothing listens on port 1 of this machine.
printf '%s' 'Please refund my order.' |
  THINKTHEN_API_KEY=example-key thinkthen decide --backend systemone \
    --url http://127.0.0.1:1 --json 'Does the customer request a refund?' |
  jq .

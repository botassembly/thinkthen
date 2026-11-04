printf '%s' 'Please refund my order.' |
  THINKTHEN_API_KEY=example-key thinkthen decide --backend systemone \
    --url http://example.com 'Does the customer request a refund?'

# SERVICE_URL is an HTTPS address.
# THINKTHEN_CA_FILE names a certificate file that does not exist.
export THINKTHEN_API_KEY=example-key THINKTHEN_CA_FILE=missing.pem
printf '%s' 'Please refund my order.' |
  thinkthen decide --backend systemone --url "$SERVICE_URL" --json \
    'Does the customer request a refund?' |
  jq .

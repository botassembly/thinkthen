# $port holds the stand-in service's port on this machine.
export THINKTHEN_API_KEY='placeholder key'
printf '%s' 'Please refund my order.' |
  thinkthen decide --backend systemone --url "http://127.0.0.1:$port" \
    --json 'Does the customer request a refund?' |
  jq '{status, error}'

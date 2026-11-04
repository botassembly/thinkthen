# $port holds the stand-in server's port on this machine.
export OPENAI_API_KEY='placeholder key' THINKTHEN_API_KEY='placeholder key'
for backend in openai systemone; do
  printf '%s' 'Please refund my order.' |
    thinkthen decide --backend "$backend" --url "http://127.0.0.1:$port/v1" \
      --model my-model --json 'Does the customer request a refund?' |
    jq '{status, error}'
done

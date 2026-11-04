# A placeholder key and a closed local port: nothing leaves this machine.
printf '%s' 'Please refund my order.' |
  OPENAI_API_KEY=example-key thinkthen decide --backend openai \
    --model my-model --url http://127.0.0.1:1 --json \
    'Does the customer request a refund?'

question="Does the customer ask for a refund?"

printf '%s\n' "I want to send this back." |
thinkthen decide "$question" \
  --backend ollama \
  --url http://localhost:11535/v1 \
  --model tev1

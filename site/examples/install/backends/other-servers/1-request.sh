question="Does the customer ask for a refund?"

printf '%s\n' "I want to send this back." |
thinkthen decide "$question" \
  --url http://localhost:8080/v1 \
  --plan |
head -1 |
jq .request

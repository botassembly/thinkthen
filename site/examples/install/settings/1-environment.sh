question="Does the customer ask for a refund?"
export THINKTHEN_BASE_URL="http://localhost:8080/v1"

printf '%s\n' "I want to send this back." |
thinkthen decide "$question" --plan |
head -1 |
jq .url

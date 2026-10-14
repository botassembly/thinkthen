question="Does the customer ask for a refund?"

printf '%s\n' "I want to send this back." |
thinkthen decide "$question" --plan |
tail -1 |
jq .

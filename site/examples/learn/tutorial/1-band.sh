question="Does the customer ask for a refund?"

printf '%s\n' "I want to send this back." |
thinkthen decide "$question" \
  --threshold 0.2:0.8

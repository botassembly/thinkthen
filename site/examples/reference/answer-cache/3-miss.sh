question="Does the customer ask for a refund?"

printf '%s\n' "My order came a day early." |
thinkthen decide "$question" \
  --replay recording

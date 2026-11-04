printf '%s' 'Please refund my order.' |
  thinkthen choose --json \
    --choice billing --choice shipping --choice other \
    'What is the message about?' |
  jq .

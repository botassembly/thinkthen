echo 'Please refund my order.' |
  thinkthen choose --threshold 0.99 \
    --choice billing --choice shipping --choice other \
    'What is the message about?'

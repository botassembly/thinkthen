printf '%s' 'Please refund my order.' |
  thinkthen decide --check 'Does the customer request a refund?'
echo "exit code: $?"

message='Please refund my order.'
if printf '%s' "$message" | thinkthen decide --check \
    'Does the customer request a refund?' > /dev/null; then
  echo "send to the refunds queue"
fi

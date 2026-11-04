message='Please refund my order.'
answer=$(printf '%s' "$message" |
  thinkthen decide 'Does the customer request a refund?')
echo "answer is: $answer"
if [ "$answer" = true ]; then echo 'send to the refunds queue'; fi

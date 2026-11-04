echo 'Please refund my order.' |
  thinkthen score --json --level routine --level urgent \
    'How urgent is the request?' |
  jq -c '.detail.levels'

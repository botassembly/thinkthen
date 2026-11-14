thinkthen relate --json \
  --rule person organization works_for message.txt |
  jq -c '.detail.names[] | {text, kind, probability}'

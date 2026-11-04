thinkthen relate --json \
  --rule works_for person organization message.txt |
  jq -c '.detail.names[] | {text, kind, probability}'

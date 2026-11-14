thinkthen relate --json \
  --rule person organization works_for message.txt |
  jq '.answer'

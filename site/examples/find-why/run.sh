thinkthen find --json \
  'Which line states the requested action?' message.txt |
  jq -c '{text: .answer.text, probability}'

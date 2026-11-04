thinkthen find --json \
  'Which clause says how to end the agreement?' contract.txt |
  jq '{status, line: .answer.text, probability}'

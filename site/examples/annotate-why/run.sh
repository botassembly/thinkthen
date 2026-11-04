thinkthen annotate --json questions.json messages.txt |
  jq -c '.answer.refund | {status, probability}'

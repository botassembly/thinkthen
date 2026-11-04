thinkthen annotate --json questions.json messages.jsonl |
  jq -c '{id, refund: .answer.refund.answer,
          topic: .answer.topic.answer,
          urgency: .answer.urgency.answer}'

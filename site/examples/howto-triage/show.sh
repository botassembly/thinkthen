thinkthen annotate --json --field /text \
  triage.json messages.jsonl > triaged.jsonl
jq -c '{id, refund: .answer.refund.status, topic: .answer.topic.answer,
  urgency: .answer.urgency.answer}' triaged.jsonl
echo '--- queues, most urgent first'
jq -s -r 'sort_by(-.answer.urgency.answer) | .[]
  | (if .answer.refund.status == "unsure" then "person"
     else .answer.topic.answer end) + " " + .id' triaged.jsonl

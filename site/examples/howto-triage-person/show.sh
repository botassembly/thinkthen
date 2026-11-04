jq -c 'select(any(.triage.refund, .triage.topic;
  . == null or . == "unsure"))' \
  triaged.jsonl > person.jsonl
jq -r .text person.jsonl

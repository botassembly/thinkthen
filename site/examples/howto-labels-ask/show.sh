thinkthen decide --field /text --into answer \
  'Does the customer request a refund?' \
  labeled.jsonl > answered.jsonl
jq -c '{id, label, answer}' answered.jsonl

for file in docs/*.txt; do
  jq -Rsc --arg id "$file" '{id: $id, text: .}' "$file"
done > documents.jsonl
thinkthen relate --from jsonl --field /text --json \
  --rule works_for person organization documents.jsonl |
  jq -r '.id as $id | .answer[]
    | [$id, .source.text, .relation, .target.text, .probability] | @tsv'

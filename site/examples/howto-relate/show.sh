for file in docs/*.txt; do
  jq -Rsc --arg id "$file" '{id: $id, text: .}' "$file"
done > documents.jsonl
thinkthen relate --from jsonl --field /text --json \
  --rule person organization works_for documents.jsonl |
  jq -r '.id as $id | .answer[]
    | [$id, .source.text, .relation, .target.text, .probability] | @tsv'

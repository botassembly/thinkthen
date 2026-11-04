# One JSON record per document, with the file name as its id.
for file in docs/*.txt; do
  jq -Rsc --arg id "$file" '{id: $id, text: .}' "$file"
done > documents.jsonl
head -n 1 documents.jsonl | jq .
echo '--- names'
thinkthen recognize --from jsonl --field /text --json \
  --kind person --kind organization documents.jsonl |
  jq -r '.id as $id | .answer[] | [$id, .kind, .text, .probability] | @tsv'

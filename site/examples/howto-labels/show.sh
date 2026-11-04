thinkthen decide --field /text --json \
  'Does the customer request a refund?' labeled.jsonl > answers.jsonl
# Pair each answer with its label, by id.
jq -s -c 'map({key: .id, value: .label}) | from_entries' labeled.jsonl \
  > labels.json
jq -c --slurpfile labels labels.json \
  '{id, label: $labels[0][.id], answer, probability}' answers.jsonl \
  > compared.jsonl
cat compared.jsonl
echo '--- counts'
jq -s -c '{
  agree: map(select(.answer == .label)) | length,
  wrong_yes: map(select(.answer == true and .label == false)) | length,
  missed_yes: map(select(.answer == false and .label == true)) | length
}' compared.jsonl
echo '--- yes rows at each cut'
for cut in 0.3 0.5 0.7; do
  jq -s -c --argjson cut "$cut" '{cut: $cut,
    wrong_yes: map(select(.probability >= $cut and .label == false)) | length,
    missed_yes: map(select(.probability < $cut and .label == true))
      | length}' compared.jsonl
done

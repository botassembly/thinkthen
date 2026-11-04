thinkthen filter --field /text --json --low 0.3 --high 0.7 \
  'Is the post spam or an insult?' posts.jsonl > judged.jsonl
# Publish the posts that answered no. Every other post waits for a person.
clean=$(jq -s -c \
  '[.[] | select(.status == "answered" and .answer == false) | .id]' \
  judged.jsonl)
jq -c --argjson clean "$clean" 'select(.id as $id | $clean | index($id))' \
  posts.jsonl > publish.jsonl
jq -c 'select(.status != "answered" or .answer == true)
  | {id, status, answer, probability}' judged.jsonl > review.jsonl
echo '--- publish.jsonl'; cat publish.jsonl
echo '--- review.jsonl'; cat review.jsonl

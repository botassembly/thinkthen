jq -s -c '{
  agree: map(select(.answer == .label)) | length,
  wrong_yes: map(select(.answer == true
    and .label == false)) | length,
  missed_yes: map(select(.answer == false
    and .label == true)) | length
}' answered.jsonl

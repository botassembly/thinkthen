jq -s '[.[].answer[]] | group_by(.)
  | map({theme: .[0], answers: length})' themes.jsonl

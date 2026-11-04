thinkthen tag --from lines --json \
  --label price --label support --label bug --label shipping --label praise \
  'Which themes does this survey answer mention?' survey.txt > themes.jsonl
jq -c '{position, themes: .answer}' themes.jsonl

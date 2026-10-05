document_answers=$(thinkthen annotate questions.json \
  --input documents --unit file --model local-1)
printf '%s\n' "$document_answers"

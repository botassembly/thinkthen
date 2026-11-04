echo 'Morgan works for Example Group.' |
  thinkthen recognize --json \
    --kind person --kind organization |
  jq -c '.answer[] | {text, kind, probability}'

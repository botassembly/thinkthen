echo 'Morgan works for Example Group.' |
  thinkthen relate --json \
    --rule works_for person organization |
  jq -c '.answer[] | {relation, probability}'

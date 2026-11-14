echo 'Morgan works for Example Group.' |
  thinkthen relate --json \
    --rule person organization works_for |
  jq -c '.answer[] | {relation, probability}'

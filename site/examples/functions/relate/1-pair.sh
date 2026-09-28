thinkthen relate @relations.json \
  --input entities.json |
jq -c '{relation,source,target}'

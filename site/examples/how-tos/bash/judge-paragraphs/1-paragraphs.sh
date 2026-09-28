awk -v RS= '{gsub(/\n/, " "); print}' paragraphs.txt |
  jq -Rc '{text:.}' |
  thinkthen filter 'Is this a complaint?' \
    --jsonl --field /text --batch 1

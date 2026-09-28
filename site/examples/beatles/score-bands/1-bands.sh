songs='["Her Majesty", "Yesterday", "Revolution 9"]'

thinkthen score @question.json \
  --lines \
  --input titles.txt \
  --replay recording |
jq -c --argjson songs "$songs" \
  'select(.input | IN($songs[]))'

thinkthen audit shown.jsonl shown-key.jsonl \
  --threshold 0.2:0.8 |
jq '{rows, right, wrong, unsure, tied}'

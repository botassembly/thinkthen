thinkthen transform show counts > counts.jq
jq -n -f counts.jq shown.jsonl |
jq '{rows, yes, no, unsure}'

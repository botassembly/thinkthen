set -euo pipefail
choice_audit=$(thinkthen audit files/results.jsonl \
  files/key.jsonl --cases)
printf '%s\n' "$choice_audit" |
  jq -c '{id, said, truth, outcome}'

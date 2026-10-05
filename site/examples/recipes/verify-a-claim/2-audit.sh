set -euo pipefail
thinkthen audit results.jsonl files/key.jsonl --cases |
  jq -c '{id, said, truth, outcome}'

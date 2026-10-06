set -euo pipefail
thinkthen runs audit results.jsonl files/key.jsonl --cases |
  jq -c '{id, said, truth, outcome}'

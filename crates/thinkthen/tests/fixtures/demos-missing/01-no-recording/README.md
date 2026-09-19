# 01 No recording

Status: green

Verbs: `decide if`

This page is green and names a folder it does not hold. The runner stops before it runs a single block, because a page that replays nothing would reach a backend.

```bash
set -euo pipefail

thinkthen decide if 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1/systemone --adapter systemone --model local-1 \
  --min-prob 0.9 --replay recording/ < message.txt | mustmatch like "never run"
```

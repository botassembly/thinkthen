# 01 No recording

Status: green

Verbs: `decide`

This page is green and names a folder it does not hold. The runner stops before it runs a single block, because a page that replays nothing would reach a backend.

```bash
set -euo pipefail

thinkthen decide 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1 --model local-1 \
  --threshold 0.9 --replay recording/ < message.txt | mustmatch like "never run"
```

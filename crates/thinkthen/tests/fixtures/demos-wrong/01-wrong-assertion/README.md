# 01 Wrong assertion

Status: green

Verbs: `decide`

This page is the fixture a failing demo is tested against. It holds one block, and that block expects the wrong exit code, so the runner has to report it.

```bash
set -euo pipefail

thinkthen decide 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1 --adapter systemone --model local-1 \
  --threshold 0.1:0.9 --quiet --replay recording/ \
  < message.txt && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=1"
```

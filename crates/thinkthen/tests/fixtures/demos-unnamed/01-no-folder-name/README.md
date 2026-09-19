# How to hide a folder name in backticks

Status: green

Verbs: `decide`

This page is green and writes `--replay` with the folder name in backticks, so the reader that pulls the name out of the page reads nothing. A silent skip would send the page to a backend, so the runner stops and says what it could not read.

## The gate

```bash
set -euo pipefail

thinkthen decide 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1 --model local-1 \
  --threshold 0.9 --replay `recording/` < message.txt | mustmatch like "never run"
```

## What can go wrong

The runner stops and says it read no folder name.

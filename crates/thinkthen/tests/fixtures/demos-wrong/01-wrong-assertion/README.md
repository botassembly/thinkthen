# 01 Wrong assertion

Status: green

Verbs: `decide if`

This page is not a demo. It is the fixture the demo runner is tested against, written the way a real demo page is written, so `sdlc/scripts/demos` is proved before the first real demo turns green.

The recording under `recording/` holds one exchange. Its file name is the digest of the adapter, the URL, and the request bytes, so the command below finds it with no network. Nothing listens on port 8721, which is how the page shows that a replay opens no connection.

## The gate

```bash
set -euo pipefail

thinkthen decide if 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1/systemone --adapter systemone --model local-1 \
  --min-prob 0.9 --status --replay recording/ \
  < message.txt > /dev/null && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=1"
```

The result says a recording answered, and it names the model the recorded response reported.

```bash
set -euo pipefail

thinkthen decide if 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1/systemone --adapter systemone --model local-1 \
  --min-prob 0.9 --replay recording/ \
  < message.txt \
  | jq -c '{replayed: .meta.replayed, model: .meta.model, status: .assessment.status}' \
  | mustmatch '{"replayed":true,"model":"local-1","status":"accepted"}'
```

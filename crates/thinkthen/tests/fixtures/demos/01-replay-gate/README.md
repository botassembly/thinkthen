# 01 Replay gate

Status: green

Verbs: `decide`

This page is not a demo. It is the fixture the demo runner is tested against, written the way a real demo page is written, so `sdlc/scripts/demos` is proved before the first real demo turns green.

The recording under `recording/` holds one exchange. Its file name is the digest of the wire shape, the URL, and the request bytes, so the command below finds it with no network. Nothing listens on port 8721, which is how the page shows that a replay opens no connection.

## The gate

```bash
set -euo pipefail

thinkthen decide 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1 --model local-1 \
  --threshold 0.1:0.9 --quiet --replay recording/ \
  < message.txt && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
```

The result says a recording answered, and it names the model the recorded response reported.

```bash
set -euo pipefail

thinkthen decide 'the customer asks for money back' \
  --url http://127.0.0.1:8721/v1 --model local-1 \
  --threshold 0.1:0.9 --details --replay recording/ \
  < message.txt \
  | jq -c '{replayed: .meta.replayed, model: .meta.model, value: .value}' \
  | mustmatch '{"replayed":true,"model":"local-1","value":true}'
```

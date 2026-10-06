# Relate a complete entity set

The plan exposes the complete first request and a marked upper bound without a key or connection.

```bash
set -euo pipefail

printf '%s' '[{"name":"gateway","kind":"service"},{"name":"billing","kind":"service"}]' \
  | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate calls=service:service --url https://api.typesafe.ai/v1 \
    --model local-1 --no-cache --plan \
  | sed -n '1p' \
  | jq -c '{schema,backend_profile,framing,fields,entity_count,logical_questions,request_count,method:.relations[0].method,bytes:.requests[0].bytes,digest:.requests[0].digest}' \
  | mustmatch '{"schema":"thinkthen.relate-plan/1","backend_profile":null,"framing":"document","fields":{"name":"/name","kind":"/kind"},"entity_count":2,"logical_questions":2,"request_count":1,"method":"yes_no","bytes":282,"digest":"d59f50a27d4d029d91daf3d3132ebe7341ab84302882158d44aa9d65cb9ed262"}'
```

Replay emits self-contained edges without a key or network.

```bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)

printf '%s' '[{"name":"gateway","kind":"service"},{"name":"billing","kind":"service"}]' \
  | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate calls=service:service --url https://api.typesafe.ai/v1 \
    --model local-1 --no-cache --replay "$root/demos/45-map-relationships/recording" \
  | mustmatch '{"relation":"calls","source":{"name":"gateway","kind":"service"},"target":{"name":"billing","kind":"service"},"probability":0.91}'
```

A replay keeps only good answers, by ADR 0111 section 6. A live run that gets one wrong-kind answer keeps the good one and exits 6. Replaying that run misses the failed question, names its key, and exits 5.

```bash
set -uo pipefail
root=$(git rev-parse --show-toplevel)
out=$(mktemp)
err=$(mktemp)
trap 'rm -f "$out" "$err"' EXIT
status=0

printf '%s' '[{"name":"gateway","kind":"service"},{"name":"billing","kind":"service"}]' \
  | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate calls=service:service --url https://api.typesafe.ai/v1 \
    --model local-1 --no-cache --details --replay "$root/spec/fixtures/relate-partial" \
    >"$out" 2>"$err" || status=$?
printf '%s %s\n' "$status" "$(wc -c <"$out")" | mustmatch '5 0'
cat "$err" | mustmatch 'thinkthen: the relate request: the replay folder holds no answer for question `9be37885a59bf2e6b065016bf2fdf70d294220f1586dec3ed811fc9c04fe0b32`; the key is the SHA-256 of the adapter, address, model, shared state and question as sent'
```

The public command exposes no method control. A malformed relation stops as usage before any key is read.

```bash
set -uo pipefail
status=0
printf '%s' '[{"name":"gateway","kind":"service"}]' \
  | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate calls=service:service:extra --plan >/dev/null 2>&1 || status=$?
printf '%s\n' "$status" | mustmatch '2'
```

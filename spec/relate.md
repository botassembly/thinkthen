# Relate a complete entity set

The dry run exposes the complete plan and exact request count without a key or connection.

```bash
set -euo pipefail

printf '%s' '[{"name":"gateway","kind":"service"},{"name":"billing","kind":"service"}]' \
  | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate calls=service:service --url https://api.typesafe.ai/v1 \
    --model local-1 --no-cache --dry-run \
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

Mixed logical failure preserves good paid answers, marks the failed question, and exits 6.

```bash
set -uo pipefail
root=$(git rev-parse --show-toplevel)
out=$(mktemp)
trap 'rm -f "$out"' EXIT
status=0

printf '%s' '[{"name":"gateway","kind":"service"},{"name":"billing","kind":"service"}]' \
  | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate calls=service:service --url https://api.typesafe.ai/v1 \
    --model local-1 --no-cache --details --replay "$root/spec/fixtures/relate-partial" \
    >"$out" || status=$?
jq -c --argjson status "$status" '{status:$status,value,questions:.answer.questions,failed:.meta.failed_questions}' "$out" \
  | mustmatch '{"status":6,"value":[{"relation":"calls","source":{"name":"gateway","kind":"service"},"target":{"name":"billing","kind":"service"},"probability":0.91}],"questions":[{"relation":"calls","reads":"calls","method":"yes_no","direction":"source_to_target","source":{"name":"gateway","kind":"service"},"target":{"name":"billing","kind":"service"},"probability":0.91,"accepted":true,"request":"d59f50a27d4d029d91daf3d3132ebe7341ab84302882158d44aa9d65cb9ed262"},{"relation":"calls","reads":"calls","method":"yes_no","direction":"source_to_target","source":{"name":"billing","kind":"service"},"target":{"name":"gateway","kind":"service"},"failure":{"kind":"backend","cause":"wrong_kind"},"request":"d59f50a27d4d029d91daf3d3132ebe7341ab84302882158d44aa9d65cb9ed262"}],"failed":1}'
```

The public command exposes no method control. A malformed relation stops as usage before any key is read.

```bash
set -uo pipefail
status=0
printf '%s' '[{"name":"gateway","kind":"service"}]' \
  | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate calls=service:service:extra --dry-run >/dev/null 2>&1 || status=$?
printf '%s\n' "$status" | mustmatch '2'
```

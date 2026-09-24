# How to map relationships in a complete entity set

Status: green

Verbs: `relate`

Use this when names and kinds are already known and one reproducible run should connect them. Replay reads the committed exchange with no network and no key.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen relate @relations.json --url https://api.typesafe.ai/v1 \
  --replay recording/ < entities.json \
  | jq -c '{relation,source,target}' \
  | mustmatch '{"relation":"calls","source":{"name":"gateway","kind":"service"},"target":{"name":"billing","kind":"service"}}'
```

## Input

`entities.json` is one complete entity set. `relations.json` fixes the name and kind pointers, the directed relation, its cut, and the model. Every probability in the recording is illustrative.

## Step 1: inspect the plan

The plan expands the relation and shows the exact request without reading a key or opening a connection. Same-kind directed relations ask both directions.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen relate @relations.json --url https://api.typesafe.ai/v1 \
  --dry-run < entities.json \
  | jq -c '{schema,entity_count,logical_questions,request_count}' \
  | mustmatch '{"schema":"thinkthen.relate-plan/1","entity_count":2,"logical_questions":2,"request_count":1}'
```

## Step 2: keep a self-contained edge

The edge carries endpoint names and kinds. A later consumer needs neither input offsets nor the original entity objects.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen relate @relations.json --url https://api.typesafe.ai/v1 \
  --details --replay recording/ < entities.json \
  | jq -c '{edges:(.value|length),cached:.meta.cached,failed:.meta.failed_questions}' \
  | mustmatch '{"edges":1,"cached":true,"failed":0}'
```

## What can go wrong

- **The complete set validates first.** A missing kind, duplicate identity, malformed rule, or 256th entity exits 2 before any request.
- **Exit 6 carries partial output.** Read the output and the status when some logical relation questions fail.
- **A replay miss is exit 5.** It is neither an empty relation set nor a model answer.
- **A recording holds every entity sent.** Commit only evidence that may be published.

## Related how-tos

- [How to find names in text without a network](../44-recognize-names/)
- [How to test a script with no network](../27-test-with-no-network/)
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)

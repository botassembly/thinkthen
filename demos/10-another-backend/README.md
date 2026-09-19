# 10 Another backend

Status: red

Verbs: `decide if`

A site keeps its shift notes inside the building. The same question runs against a decider server on the local network instead of the hosted one. The command changes by two flags, and the hosted key does not follow the request to the new host.

## Input

`note.txt` is one shift handover note.

## Point it somewhere else

`--url` names the host and `--adapter` names the wire format. The plan shows both before anything is sent.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

TYPESAFE_API_KEY=not-a-real-key thinkthen decide if 'the note says a pallet has not been scanned in' \
  --url http://decider.internal:8099/v1/systemone --adapter systemone \
  --model local-decider-3 --min-prob 0.9 --plan \
  < note.txt > "$work/plan.json"

jq -r '.url' "$work/plan.json" | mustmatch "http://decider.internal:8099/v1/systemone"
jq -r '.model' "$work/plan.json" | mustmatch "local-decider-3"
jq -r '.key_env' "$work/plan.json" | mustmatch "null"
```

That last line is the key rule. `--url` replaced the profile's URL, so the profile's key variable was dropped and the request goes out with no key. A hosted key never crosses hosts on its own.

## Give the new host its own key

A local server that wants a key gets one named for it, and the plan shows the variable name and never a value.

```bash
set -euo pipefail

TYPESAFE_API_KEY=not-a-real-key LOCAL_DECIDER_KEY=also-not-real \
thinkthen decide if 'the note says a pallet has not been scanned in' \
  --url http://decider.internal:8099/v1/systemone --adapter systemone \
  --key-env LOCAL_DECIDER_KEY --min-prob 0.9 --plan \
  < note.txt \
  | mustmatch not like "not-a-real-key"
```

## The answer says which model gave it

A saved result names the model, so two runs against two backends never get mixed up.

```bash
set -euo pipefail

thinkthen decide if 'the note says a pallet has not been scanned in' \
  --url http://decider.internal:8099/v1/systemone --adapter systemone \
  --model local-decider-3 --min-prob 0.9 --replay recording/ \
  < note.txt \
  | jq -c '{model: .meta.model, status: .assessment.status, value: .assessment.value}' \
  | mustmatch '{"model":"local-decider-3","status":"accepted","value":true}'
```

The pass mark does not travel with the command. `0.9` was measured for one model, and pointing the same line at a second model is a new measurement with an old number in it.

The recording under `recording/` does not exist yet.

## What this demo decides

- **`meta.backend` has no value when `--url` replaced the profile.** result.md says `meta` "names the backend", and this run has no named backend. A script that groups saved results by backend gets nothing to group on. Smallest fix: `meta.backend` is `null` when no profile was used, and the result still carries `adapter`, `url`, and `model`. This touches result.md, a settled page. Adding `url` to `meta` is the part that costs a line, and it is what makes two saved results comparable.
- **`--url` should require `--adapter`.** The built-in profile pairs one URL with one adapter. Replacing only the URL leaves the hosted adapter pointed at a server that may not speak it, and the first sign of trouble is a refused reply at exit 4. Smallest fix: `--url` without `--adapter` or `--backend` is a usage error, exit 2. This touches backends.md, a settled page, and it costs one check.
- **The key rule is right and it needs `key_env: null` in the plan to be testable.** Demo 09 asks for the same field for the same reason. Without it, the one line that proves a key was dropped cannot be written.
- **A pass mark belongs to a model, and nothing in the command line says so.** The result names the model and the mark, which is enough to catch it afterwards. Smallest fix: the `--min-prob` line in the help says the mark is a measurement for one model, in the same words backends.md already uses about naming an exact model version.
- **`--model` is the flag that makes a run repeatable, and the demos all pass it.** backends.md says the built-in profile's model is `jev-latest`, which moves. Smallest fix: no change, and the help says to name an exact version in anything saved or replayed.

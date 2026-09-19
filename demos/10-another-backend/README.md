# 10 Another backend

Status: red

Verbs: `decide`, `config`

A site keeps its shift notes inside the building. The same question runs against a decider server on the local network instead of the hosted one. First with three flags, then with a profile in a file, and the hosted key does not follow the request to the new host either way.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`note.txt` is one shift handover note. `site.json` is a configuration file holding one profile.

## Point it somewhere else by hand

`--url` names the host, `--adapter` names the wire format, and `--model` pins the version. The plan shows all three before anything is sent.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

TYPESAFE_API_KEY=not-a-real-key thinkthen decide 'Does the note say a pallet has not been scanned in?' \
  --url http://decider.internal:8099/v1/systemone --adapter systemone \
  --model local-decider-3 --dry-run --input note.txt > "$work/plan.json"

jq -r '.url' "$work/plan.json" | mustmatch "http://decider.internal:8099/v1/systemone"
jq -r '.model' "$work/plan.json" | mustmatch "local-decider-3"
jq -r '.profile' "$work/plan.json" | mustmatch "null"
jq -r '.key_env' "$work/plan.json" | mustmatch "null"
mustmatch not like "not-a-real-key" < "$work/plan.json"
```

Those last three lines are the key rule. The ad-hoc backend has no profile, so it borrows no key variable, and the request goes out with no key. A hosted key never crosses hosts on its own.

## The same backend as a profile

Three flags on every command is three chances to mistype a host. The profile holds the same three values and one word selects it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen config check --config site.json && printf 'config ok\n' | mustmatch "config ok"
thinkthen config path --config site.json | mustmatch "site.json"

printf '%s\n' '{"version":1,"profiles":{"site":{"url":"http://decider.internal:8099/v1/systemone","adapter":"systemone"}}}' \
  > "$work/broken.json"
thinkthen config check --config "$work/broken.json" >/dev/null 2>&1 && bad=0 || bad=$?
printf 'bad=%s\n' "$bad" | mustmatch "bad=5"

TYPESAFE_API_KEY=not-a-real-key thinkthen decide 'Does the note say a pallet has not been scanned in?' \
  --config site.json --profile site --dry-run --input note.txt > "$work/plan.json"

jq -c '{profile, url, model, key_env}' "$work/plan.json" \
  | mustmatch '{"profile":"site","url":"http://decider.internal:8099/v1/systemone","model":"local-decider-3","key_env":null}'
mustmatch not like "not-a-real-key" < "$work/plan.json"
```

The refused file names a profile with no `model`. A profile is complete or it is not a profile, and `check` reports the first problem with its JSON Pointer at exit 5.

`key_env` is `null` in the file and `null` in the plan. The file never holds a key and the profile never borrows one.

`config show` prints one JSON object with the file's own key names and every default filled in. It sends nothing.

```bash
set -euo pipefail

env -u TYPESAFE_API_KEY thinkthen config show --config site.json \
  | jq -c '{profile, jobs, timeout_seconds, key_env: .profiles.site.key_env}' \
  | mustmatch '{"profile":"site","jobs":4,"timeout_seconds":30,"key_env":null}'
```

## The answer says which model gave it

A saved result names the profile and the model, so two runs against two backends never get mixed up.

```bash
set -euo pipefail

thinkthen decide 'Does the note say a pallet has not been scanned in?' \
  --config site.json --profile site --details --input note.txt --replay recording/ \
  | jq -c '{value, profile: .meta.profile, model: .meta.model, url: .meta.url}' \
  | mustmatch '{"value":true,"profile":"site","model":"local-decider-3","url":"http://decider.internal:8099/v1/systemone"}'
```

The same run under the ad-hoc flags reports `null` for the profile and the same three backend facts.

```bash
set -euo pipefail

thinkthen decide 'Does the note say a pallet has not been scanned in?' \
  --url http://decider.internal:8099/v1/systemone --adapter systemone \
  --model local-decider-3 --details --input note.txt --replay recording/ \
  | jq -r '.meta.profile | tostring' | mustmatch "null"
```

A threshold does not travel with a command. A mark measured for one model is an old number pointed at a new one. The result names the model, and that is enough to catch it afterwards.

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms the profile and `meta.profile`.** One word replaces three flags, the file holds no key, and a saved result says which profile answered. `meta.url` is what makes two saved results comparable, and the old surface lacked it.
- **The three ad-hoc flags travel together, and `backends.md` says so.** A URL without an adapter and a model is a usage error, so the old failure mode is gone: a hosted adapter can no longer end up pointed at a server that may not speak it. This page passes all three every time, and demo 12 shows the same three on a backend that is down.
- **`config check`, `path`, and `show` are now a usable triple, and the demo confirms all three.** A lint job runs `check` and reads exit 5. A support thread pastes `show`, and the defaults are filled in, so a reader never has to know which values came from the file.
- **`config show` and a run can disagree and nothing says so.** The block above reads the file alone. A run with `--profile` or `THINKTHEN_PROFILE` on the command line selects something else, and `show` never saw those. The demo asks that `config show` take the same selection flags a run takes, or that its help say it reports the file and not the next run.
- **The two spellings of a backend should not both be everyday.** Both work here and the profile reads better. The demo confirms keeping the ad-hoc flags out of the short help.

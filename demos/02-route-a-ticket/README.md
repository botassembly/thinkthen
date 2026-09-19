# 02 Route a ticket

Status: red

Verbs: `decide which`

One ticket arrives and a script has to put a label on it. The labels are a short list of teams that do not overlap, and a ticket that fits none of them gets `none`. The script then acts on the label, and the `case` has a default branch, so a label the script does not know about stops the run instead of picking a queue at random.

## Input

`ticket.txt` is one message about a failed renewal charge.

## Label it

The option names carry the meaning and `--by` says what decides the pick. `--none` adds an option for a ticket that fits no team. `--min-prob` sets the mark on the winning option, and `--min-gap` refuses a pick that barely beat the runner-up.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide which billing shipping account \
  --by 'the team that owns this request' \
  --none --min-prob 0.8 --min-gap 0.3 --replay recording/ \
  < ticket.txt > "$work/label.json"

jq -c '.assessment | {status, value}' "$work/label.json" \
  | mustmatch '{"status":"accepted","value":"billing"}'
jq -r '.answer.probabilities | keys_unsorted | join(",")' "$work/label.json" \
  | mustmatch "billing,shipping,account,none"
```

The probabilities come back in the order the options were sent, with `none` last. Order matters to the model, so a script that reorders the list is running a different measurement.

## Act on the label

The script reads `assessment.value`, never `answer.pick`. `answer.pick` is what the backend said before policy, and it is filled in even when the pass mark rejected it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide which billing shipping account \
  --by 'the team that owns this request' \
  --none --min-prob 0.8 --min-gap 0.3 --replay recording/ \
  < ticket.txt > "$work/label.json"

label=$(jq -r '.assessment.value // "unsure"' "$work/label.json")

case $label in
  billing)  queue=payments ;;
  shipping) queue=logistics ;;
  account)  queue=identity ;;
  none)     queue=triage ;;
  unsure)   queue=triage ;;
  *) printf 'unknown label: %s\n' "$label" >&2; exit 2 ;;
esac

printf 'queue=%s\n' "$queue" | mustmatch "queue=payments"
```

`// "unsure"` matters. Without it `jq -r` prints the four characters `null` for an unsure ticket and for a ticket with no pass mark, and a `case` that matches on `null` is matching on a spelling accident.

The recording under `recording/` does not exist yet.

## What this demo decides

- **Reuse `--min-prob` for the choice mark.** The script sets one idea, a pass mark, and one word reads well next to `--min-gap`. The cost showed up at once: the same word means the symmetric rule in demo 01 and the winning-option rule here, so a script holding one `MIN_PROB` variable sets two different policies with one number. Smallest fix: keep the name and add `assessment.rule` to the result, with the values `symmetric` and `top`, so a reader of a saved result can tell which rule ran. This bears on the open question in decide.md under `which`.
- **Keep `--none` off by default.** The demo had to type it, and typing it is how the `none` branch got written. A default-on `none` would have put an option in the sent list that the user never saw, and the sent list is the measurement.
- **A script must read `assessment.value` and never `answer.pick`.** The draft says `pick` is "what the backend said, before any policy" and then leaves a policy-free field at the top of the result, where a careless `jq` will find it. Smallest fix: decide.md and the `which` help say the rule in one line.
- **`jq -r '.assessment.value'` prints `null` as text.** Every `which` in a shell script needs `// "unsure"`. Smallest fix: the help for `which` shows the `jq -r '.assessment.value // "unsure"'` idiom beside the example.
- **`--min-gap` earns its place.** Without it a 0.81 against 0.79 would have been accepted at `--min-prob 0.8`, and the measurement says option order alone can move a margin that size.

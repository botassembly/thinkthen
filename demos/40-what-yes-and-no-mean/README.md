# How to say what yes and no mean

Status: green

Verbs: `decide`

A yes/no question has a third answer hiding in it: the evidence never raised the subject. `decide` still prints `false`, and the probability looks like a confident denial. `--true` and `--false` say what each side means, so silence lands where your script needs it.

```bash
set -euo pipefail

thinkthen decide @window.json --replay recording/ < notices/silent.txt \
  | mustmatch "true"
```

The notice never mentions the length. `window.json` counts an unsettled length as a yes, so the claim reaches the branch that asks the author.

## Input

`notices/` holds three made-up maintenance notices about one upgrade window. `longer.txt` says the window runs longer than the plan announced before, `same.txt` says it runs no longer, and `silent.txt` never raises the length at all.

`recording/` holds the six live exchanges this page replays, and `record.sh` made them through `sdlc/scripts/live`.

## Step 1: watch the plain question call silence a denial

```bash
set -euo pipefail

for notice in notices/longer.txt notices/same.txt notices/silent.txt; do
  row=$(
    thinkthen decide 'The notice says the upgrade window runs longer than the plan it announced before.' \
      --details --replay recording/ < "$notice"
  ) && rc=0 || rc=$?
  printf '%s %s ' "$(basename -- "$notice")" "$rc"
  printf '%s' "$row" | jq -r '[.value, (.answer.probability | tostring)] | join(" ")'
done | mustmatch "longer.txt 0 true 0.97
same.txt 1 false 0.03
silent.txt 1 false 0.1"
```

Two notices answered `false`. One of them denies the claim and the other never raises it, and 0.03 against 0.1 puts no usable distance between them. The exit code says the same thing twice.

## Step 2: put silence on the side your script wants

`--true` and `--false` carry one sentence each, and the model reads them beside the question. Either may appear alone.

```bash
set -euo pipefail

for notice in notices/longer.txt notices/same.txt notices/silent.txt; do
  row=$(
    thinkthen decide 'The notice leaves the upgrade window at risk of running longer than the plan it announced before.' \
      --true 'The notice says the window runs longer than the earlier plan, or the notice does not settle the length at all.' \
      --false 'The notice says the window runs no longer than the earlier plan.' \
      --details --replay recording/ < "$notice"
  ) && rc=0 || rc=$?
  printf '%s %s ' "$(basename -- "$notice")" "$rc"
  printf '%s' "$row" | jq -r '[.value, (.answer.probability | tostring)] | join(" ")'
done | mustmatch "longer.txt 0 true 0.96
same.txt 1 false 0.07
silent.txt 0 true 0.81"
```

The silent notice moved from 0.1 to 0.81 and its exit code moved with it. Only the sentence that says what yes means changed. A desk that must chase an unstated length now gets a yes it can branch on, and a desk that wants the other rule writes the other sentence.

## Step 3: keep the two texts in a question file

The two texts belong to the question, not to the command line that ran it. A question file holds them under `true` and `false`, and the request is the same byte for byte.

```bash
set -euo pipefail

cat window.json | jq -r '[.true, .false] | length | tostring' | mustmatch "2"

for notice in notices/longer.txt notices/same.txt notices/silent.txt; do
  thinkthen decide @window.json --replay recording/ < "$notice" || true
done | mustmatch "true
false
true"
```

## What can go wrong

- **A text that argues rather than defines.** The two texts say what each side of one claim looks like. Instructions to the model, extra questions, and policy belong nowhere near them.
- **Moving silence without saying so.** Whoever reads the exit code has to know which rule is in force. Keeping the texts in a question file puts the rule where a reviewer finds it.
- **Expecting the texts to rescue a vague question.** "Is this notice good?" stays vague with two sentences bolted on.
- **Assuming a recording still matches.** A changed text is a changed request and a changed digest, so `--replay` will name the entry it cannot find.
- **A blank text.** An empty `--true` or an empty `true` key is a usage error, not an absent text.
- **Reading 0.81 as a measurement.** Three made-up notices show the direction. A cut is tuned on labeled cases. A confident answer is about what the notice says, never about the world the notice never raises.
- **Two branches when the script wants three.** A pick with `supported`, `contradicted`, and `not_stated` keeps silence as its own label, which has to be typed because nothing adds it.
- **Reading a pick's odds.** `--details` on a pick prints a probability for every option in the order they were sent, so a margin over the runner-up is a rule a desk writes in `jq`. An `ambiguous` label is the model hedging, and exit 3 is the tool saying no option cleared the mark.

## Related how-tos

- [How to tune a question file and use the same file in the gate](../41-tune-a-question-file/) measures a wording against labeled cases.
- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) is the pick that names silence as its own label.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) tunes the cut instead of the wording.
- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the plain form with no texts.

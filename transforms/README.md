# transforms/

A transform is a metric or a policy, as ADR 0015 item 1 defines them. A metric reads a whole run and prints numbers. A policy reads one row and names an action. All seven transforms here are metrics. The names table in [`README.md`](../README.md) holds the four names.

A transform is a folder, as ADR 0012 proposes. It holds a `.jq` file with a header that states what it reads, what arguments it takes, and what it does at every edge, and one short `example.sh` with the pipeline line. The page that teaches it is a green demo, so the gate runs the transform against committed rows and no transform can drift from what it claims.

`thinkthen` runs none of this. The tool obtains the judgments and keeps the evidence. `jq` does the arithmetic.

## The rows every transform reads

`rows/` holds the run these transforms were written against. `cases.jsonl` is forty made-up support messages with a stable `id`, a `body`, and a trusted `label` a person gave. One case carries no label on purpose, because a real case file has one.

`record.sh` judged every case twice through `sdlc/scripts/live`, once with the question in `question.txt` and once with the reworded question in `question-b.txt`, and wrote `runs/run-a.jsonl` and `runs/run-b.jsonl`. Record mode arrived with ticket 0012, after these rows were written, so the loop is the shell's. Each row is the result object of `decide --details` with `input` holding the whole case, which is the record row of `specification/result.md`. The transforms therefore read a record-mode run unchanged.

`recording/` holds the eighty exchanges. A recording holds request bodies and never headers, so no key reaches it.

## The transforms

| Folder | Answers | Page |
| --- | --- | --- |
| `counts/` | How many yes, how many no, how many unresolved | [25](../demos/25-check-the-judge/) |
| `score/` | Accuracy, precision, recall, and F1 at a cut | [25](../demos/25-check-the-judge/) |
| `sweep/` | What every cut would have done, and which one to pick | [13](../demos/13-pick-a-threshold/) |
| `band/` | Accuracy beside coverage for a band | [13](../demos/13-pick-a-threshold/) |
| `calibration/` | Whether a probability of 0.8 means eight in ten | [25](../demos/25-check-the-judge/) |
| `compare/` | What changed between two runs, and why it could have | [41](../demos/41-tune-a-question-file/) |
| `cost/` | The input tokens a run spent and what they cost | [28](../demos/28-what-a-run-cost/) |

## A comparison that hides nothing

`compare.jq` is the one transform whose useful answers are empty lists, and an empty list is easy to believe and easy to get wrong. This block doctors the later run, changing one trusted label and dropping one case, and makes the transform name both. The `spec` rung runs it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

jq -c 'if .input.id == "C-03" then .input.label = false else . end
       | select(.input.id != "C-40")' rows/runs/run-b.jsonl > "$work/doctored.jsonl"

jq -n --slurpfile before rows/runs/run-a.jsonl -f compare/compare.jq "$work/doctored.jsonl" \
  | jq -c '{paired, only_in_before, only_in_after, repeated_ids, mismatched_input, mismatched_label}' \
  | mustmatch '{"paired":39,"only_in_before":["C-40"],"only_in_after":[],"repeated_ids":{"before":[],"after":[]},"mismatched_input":[],"mismatched_label":["C-03"]}'
```

A case in one run alone is listed rather than dropped, so a run that stopped early cannot pass as a smaller run that finished. A changed label is named, because it means somebody moved the target between the two measurements. `repeated_ids` holds the ids a run carries twice, which are repeated trials and are paired in nothing.

## The rules every transform follows

- **Three answers, never two.** true is yes, false is no, and null is unresolved. Every transform tests all three explicitly. `.value // false` turns an unresolved answer into a no, and no transform here uses it.
- **Unresolved rows are counted apart.** They are never scored right or wrong and never folded into no.
- **A metric states its definition.** The header names the label set, what each rate divides by, and what a zero denominator yields, which is null.
- **A case with no label is reported.** It is listed by id and scored in nothing. No row is dropped silently.
- **A cut is an argument.** `--argjson` carries it, so one saved run is read at any cut with no second request.
- **A comparison checks more than an id.** It checks the evidence and the label of every pair, and it reads the rows to say whether the question, the model, or the threshold changed.
- **Counts are exact and rates are rounded.** Four decimals for a rate, six for money.

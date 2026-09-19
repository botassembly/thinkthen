# recipes/

A recipe is a folder, as ADR 0012 proposes. It holds a `.jq` file with a header that states what it reads, what arguments it takes, and what it does at every edge, and one short `example.sh` with the pipeline line. The page that teaches it is a green demo, so the gate runs the recipe against committed rows and no recipe can drift from what it claims.

`thinkthen` runs none of this. The tool obtains the judgments and keeps the evidence. `jq` does the arithmetic.

## The rows every recipe reads

`rows/` holds the run these recipes were written against. `cases.jsonl` is forty made-up support messages with a stable `id`, a `body`, and a trusted `label` a person gave. One case carries no label on purpose, because a real case file has one.

`record.sh` judged every case twice through `sdlc/scripts/live`, once with the question in `question.txt` and once with the reworded question in `question-b.txt`, and wrote `runs/run-a.jsonl` and `runs/run-b.jsonl`. Record mode is not built, so the loop is the shell's. Each row is the result object of `decide --details` with `input` holding the whole case, which is the record row of `specification/result.md`. The recipes therefore keep working when record mode lands.

`recording/` holds the eighty exchanges. A recording holds request bodies and never headers, so no key reaches it.

## The recipes

| Folder | Answers | Page |
| --- | --- | --- |
| `counts/` | How many yes, how many no, how many unresolved | [25](../demos/25-check-the-judge/) |
| `score/` | Accuracy, precision, recall, and F1 at a cut | [25](../demos/25-check-the-judge/) |
| `sweep/` | What every cut would have done, and which one to pick | [13](../demos/13-pick-a-threshold/) |
| `band/` | Accuracy beside coverage for a band | [13](../demos/13-pick-a-threshold/) |
| `calibration/` | Whether a probability of 0.8 means eight in ten | [38](../demos/38-what-a-probability-means/) |
| `compare/` | What changed between two runs, and why it could have | [24](../demos/24-compare-two-runs/) |
| `cost/` | The input tokens a run spent and what they cost | [28](../demos/28-what-a-run-cost/) |

## The rules every recipe follows

- **Three answers, never two.** true is yes, false is no, and null is unresolved. Every recipe tests all three explicitly. `.value // false` turns an unresolved answer into a no, and no recipe here uses it.
- **Unresolved rows are counted apart.** They are never scored right or wrong and never folded into no.
- **A metric states its definition.** The header names the label set, what each rate divides by, and what a zero denominator yields, which is null.
- **A case with no label is reported.** It is listed by id and scored in nothing. No row is dropped silently.
- **A cut is an argument.** `--argjson` carries it, so one saved run is read at any cut with no second request.
- **A comparison checks more than an id.** It checks the evidence and the label of every pair, and it reads the rows to say whether the question, the model, or the threshold changed.
- **Counts are exact and rates are rounded.** Four decimals for a rate, six for money.

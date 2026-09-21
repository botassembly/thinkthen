# transforms/

A transform is a metric or a policy, as ADR 0015 item 1 defines them. A metric reads a whole run and prints numbers. A policy reads one row and names an action. Seven transforms here are metrics, and `triage` is a policy. The names table in [`README.md`](../README.md) holds the four names.

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
| `compare/` | What changed between two scalar or annotated runs, and why it could have | [41](../demos/41-tune-a-question-file/) |
| `cost/` | The input tokens a run spent and what they cost | [28](../demos/28-what-a-run-cost/) |
| `triage/` | Whether a support ticket is drafted, blocked, or reviewed, and why | [16](../demos/16-triage-pipeline/) |

## A comparison that hides nothing

`compare.jq` is the one transform whose useful answers are empty lists, and an empty list is easy to believe and easy to get wrong. This block doctors the later run, changing one trusted label and dropping one case, and makes the transform name both. The `spec` rung runs it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

jq -c 'if .input.id == "C-03" then .input.label = false else . end
       | select(.input.id != "C-40")' rows/runs/run-b.jsonl > "$work/doctored.jsonl"

jq -n --slurpfile before rows/runs/run-a.jsonl -f compare/compare.jq "$work/doctored.jsonl" \
  | jq -c '{paired, compared, only_in_before, only_in_after, repeated_ids, mismatched_input, mismatched_label}' \
  | mustmatch '{"paired":39,"compared":38,"only_in_before":["C-40"],"only_in_after":[],"repeated_ids":{"before":[],"after":[]},"mismatched_input":[],"mismatched_label":["C-03"]}'
```

A case in one run alone is listed rather than dropped, so a run that stopped early cannot pass as a smaller run that finished. A changed label is named, because it means somebody moved the target between the two measurements. `repeated_ids` holds the ids a run carries twice, which are repeated trials and are paired in nothing.

This fixture proves scalar comparison, the comparison partition, digest choice, legacy fallback, and empty-run result. The two mismatched pairs remain in both mismatch lists where applicable, but neither reaches `same`, `changed_values`, or `flips`.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

jq -n -c '
  def row($id; $body; $label; $value):
    {input: {id: $id, body: $body, label: $label}, value: $value,
     question: {text: "same question"}, threshold: 0.5,
     meta: {model: "same", question_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}};
  [row("same"; "same"; true; true),
   row("flip"; "same"; false; false),
   row("pick"; "same"; true; "old"),
   row("input"; "old"; true; true),
   row("both"; "old"; true; false),
   row("gone"; "same"; true; true),
   row("repeat"; "same"; true; true),
   row("repeat"; "same"; true; true)] | .[]' > "$work/before.jsonl"

jq -n -c '
  def row($id; $body; $label; $value):
    {input: {id: $id, body: $body, label: $label}, value: $value,
     question: {text: "same question"}, threshold: 0.5,
     meta: {model: "same", question_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}};
  [row("same"; "same"; true; true),
   row("flip"; "same"; false; true),
   row("pick"; "same"; true; "new"),
   row("input"; "new"; true; true),
   row("both"; "new"; false; true),
   row("new"; "same"; true; true),
   row("repeat"; "same"; true; true),
   row("repeat"; "same"; true; true)] | .[]' > "$work/after.jsonl"

jq -n --slurpfile before "$work/before.jsonl" -f compare/compare.jq "$work/after.jsonl" \
  | jq -c '{paired, compared, mismatched_input, mismatched_label, same, changed_values, flips,
            changes: [.changes[] | {id, before_value, after_value}],
            both_in_flips: ([.flips[]? | .[]] | index("both") != null),
            partition: (.compared == .same + .changed_values),
            changed}' \
  | mustmatch '{"paired":5,"compared":3,"mismatched_input":["both","input"],"mismatched_label":["both"],"same":1,"changed_values":2,"flips":{"no to yes":["flip"]},"changes":[{"id":"flip","before_value":false,"after_value":true},{"id":"pick","before_value":"old","after_value":"new"}],"both_in_flips":false,"partition":true,"changed":{"question":true,"question_by":"digest","model":false,"threshold":false}}'

jq -n -c '{input:{id:"legacy",body:"same",label:true},value:true,
           question:{text:"old question"},threshold:0.5,meta:{model:"same"}}' > "$work/legacy-before.jsonl"
jq -n -c '{input:{id:"legacy",body:"same",label:true},value:true,
           question:{text:"new question"},threshold:0.5,
           meta:{model:"same",question_sha256:"cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"}}' > "$work/legacy-after.jsonl"

jq -n --slurpfile before "$work/legacy-before.jsonl" -f compare/compare.jq "$work/legacy-after.jsonl" \
  | jq -c '{changed}' \
  | mustmatch '{"changed":{"question":true,"question_by":"text","model":false,"threshold":false}}'

jq -n -c '{input:{id:"mixed",body:"same",label:true},value:true,
           question:{text:"same question"},threshold:0.5,
           meta:{model:"same",question_sha256:7}}' > "$work/mixed-before.jsonl"
jq -n -c '{input:{id:"mixed",body:"same",label:true},value:true,
           question:{text:"same question"},threshold:0.5,
           meta:{model:"same",question_sha256:"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"}}' > "$work/mixed-after.jsonl"

jq -n --slurpfile before "$work/mixed-before.jsonl" -f compare/compare.jq "$work/mixed-after.jsonl" \
  | jq -c '{changed}' \
  | mustmatch '{"changed":{"question":false,"question_by":"text","model":false,"threshold":false}}'

jq -n -c '{input:{id:"malformed",body:"same",label:true},value:true,
           question:{text:"old question"},threshold:0.5,
           meta:{model:"same",question_sha256:"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n"}}' > "$work/malformed-before.jsonl"
jq -n -c '{input:{id:"malformed",body:"same",label:true},value:true,
           question:{text:"new question"},threshold:0.5,
           meta:{model:"same",question_sha256:"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"}}' > "$work/malformed-after.jsonl"

jq -n --slurpfile before "$work/malformed-before.jsonl" -f compare/compare.jq "$work/malformed-after.jsonl" \
  | jq -c '{changed}' \
  | mustmatch '{"changed":{"question":true,"question_by":"text","model":false,"threshold":false}}'

: > "$work/empty.jsonl"
jq -n --slurpfile before "$work/before.jsonl" -f compare/compare.jq "$work/empty.jsonl" \
  | jq -c '{changed}' \
  | mustmatch '{"changed":{"question":null,"question_by":"unavailable","model":true,"threshold":true}}'
```

## The rules every transform follows

- **Three yes-or-no answers, never two.** true is yes, false is no, and null is unresolved. `.value // false` turns unresolved into no, and no transform uses it.
- **Unresolved rows are counted apart.** They are never scored right or wrong and never folded into no.
- **A metric states its definition.** The header names the label set, what each rate divides by, and what a zero denominator yields, which is null.
- **A case with no label is reported.** It is listed by id and scored in nothing. No row is dropped silently.
- **A cut is an argument.** `--argjson` carries it, so one saved run is read at any cut with no second request.
- **A comparison checks more than an id.** It checks each pair's evidence and label, then compares scalar values. `changes` always shows changed values. For valid yes-or-no answers it also shows same-value probability movement above the active tolerance. The default 0.08 comes from one limited repeated run. A complete modern run compares question digests; legacy rows fall back to text.
- **An annotation compares by name.** Record pairing happens once. Each shared question then gets its own counts, changes, flips, and probability summary. Added and removed question names are listed without turning every record into a change. Run this transform with `jq -n`; it refuses nonempty ordinary input and `jq -s`. Jq runs no filter for empty ordinary input, so an empty stream prints nothing.
- **Counts are exact and rates are rounded.** Four decimals for a rate, six for money.

# 14 Grade a batch

Status: red

Verbs: `annotate`, `report`

A team has an answering assistant and six saved cases. Each case holds the question asked, the source text, a reference answer a person wrote, the assistant's answer, and a human verdict. The team wants two numbers for every release: how often the assistant is correct, and how often it says something the source text does not support. It also wants to know whether the judge agrees with the humans, because a judge nobody checked is a number nobody should trust.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`cases.jsonl` holds six flat cases with `id`, `input`, `context`, `gold`, `gold_code`, `output`, `output_code`, and `human_correct`. `cases-b.jsonl` holds a second assistant's answers, with `E-04` missing. `checks.json` holds two questions. `correct` sees `/input`, `/gold`, and `/output`. `grounded` sees `/context` and `/output` and never the gold answer, because a grounding check that can read the right answer is grading the wrong thing.

## Prove what each check sees

Several pointers build an evidence object keyed by the last part of each pointer. `--dry-run` shows the first case's request and nothing else.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

env -u TYPESAFE_API_KEY thinkthen annotate checks.json --jsonl --dry-run \
  --input cases.jsonl > "$work/plan.json"

jq -S -c '.request.state | keys' "$work/plan.json" \
  | mustmatch '["gold","input","output"]'
jq -c '.input' "$work/plan.json" \
  | mustmatch '{"framing":"jsonl","on":{"correct":["/input","/gold","/output"],"grounded":["/context","/output"]}}'
```

Two distinct `on` sets means two requests for every case. Twelve requests for six cases, and the plan shows one of them.

## Run it and keep everything

One folder given to `--record` and `--replay` is a cache, so a run that stops halfway is resumed and not repaid.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

thinkthen annotate checks.json --jsonl --details --input cases.jsonl \
  --record "$work/cache" --replay "$work/cache" > "$work/run-a.jsonl"

wc -l < "$work/run-a.jsonl" | tr -d ' ' | mustmatch "6"
jq -r 'select(.input.id == "E-01") | .value | tostring' "$work/run-a.jsonl" \
  | mustmatch '{"correct":true,"grounded":true}'
jq -r '.meta.questions_sha256' "$work/run-a.jsonl" | sort -u | wc -l | tr -d ' ' \
  | mustmatch "1"
jq -r '.answers.correct.request != .answers.grounded.request' "$work/run-a.jsonl" \
  | sort -u | mustmatch "true"
```

One definition digest for the whole run. Two request digests on every row, because the two checks went out separately, and each names its own entry in the cache.

## The enriched spreadsheet

One `jq` line turns the run into a table a reviewer reads, with the probability beside every judgment.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

thinkthen annotate checks.json --jsonl --details --input cases.jsonl \
  --record "$work/cache" --replay "$work/cache" > "$work/run-a.jsonl"

jq -r '[.input.id, (.value.correct|tostring), .answers.correct.answer.probability,
        (.value.grounded|tostring), .answers.grounded.answer.probability,
        (.input.human_correct|tostring)] | @tsv' "$work/run-a.jsonl" \
  | head -1 | cut -f1,2,4,6 | mustmatch "E-01	true	true	true"
```

## Score the run, then score the judge

`report` with no option prints the counts and the run's facts. `--truth correct=/human_correct` scores the `correct` check against the human verdicts. `--truth /gold_code=/output_code` compares two fields of the case exactly and involves no judgment at all. An exact check is keyed by its left pointer, so it prints under `/gold_code`.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

thinkthen annotate checks.json --jsonl --details --input cases.jsonl \
  --record "$work/cache" --replay "$work/cache" > "$work/run-a.jsonl"

env -u TYPESAFE_API_KEY thinkthen report "$work/run-a.jsonl" \
  | jq -c '{rows, tool, checks: (.checks | keys)}' \
  | mustmatch '{"rows":6,"tool":["thinkthen 0.1.0"],"checks":["correct","grounded"]}'

thinkthen report "$work/run-a.jsonl" \
  --truth correct=/human_correct --truth /gold_code=/output_code \
  | jq -c '{judge: (.checks.correct | {coverage, accuracy_resolved, f1}),
            codes: .checks["/gold_code"] | {kind, matched, accuracy}}' \
  | mustmatch '{"judge":{"coverage":1,"accuracy_resolved":1,"f1":1},"codes":{"kind":"exact","matched":5,"accuracy":0.8333333333333334}}'
```

Five of six codes match, and the judge agreed with the humans on every case it resolved.

## Compare two assistants

`--baseline` matches cases by `/id` and says what changed. `E-04` is missing from the second file, and the report names it rather than dropping it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R recording "$work/cache"

for side in a:cases.jsonl b:cases-b.jsonl; do
  thinkthen annotate checks.json --jsonl --details --input "${side#*:}" \
    --record "$work/cache" --replay "$work/cache" > "$work/run-${side%%:*}.jsonl"
done

thinkthen report "$work/run-b.jsonl" --baseline "$work/run-a.jsonl" \
  --truth correct=/human_correct \
  | jq -c '{only_baseline: .baseline.only_baseline, only_run: .baseline.only_run,
            flipped: .baseline.checks.correct.flipped}' \
  | mustmatch '{"only_baseline":["E-04"],"only_run":[],"flipped":{"to_yes":["E-02","E-06"],"to_no":["E-01"]}}'
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **Two commands and one file are the whole eval, and the demo confirms it.** No engine, no `eval` verb, no second question language. `annotate` obtains the judgments, `report` interprets them, and `jq` does everything else.
- **Several pointers on one `on` are what make a flat case work.** The grounding check cannot see the gold answer, and the `--dry-run` block is the proof. That block is the single most valuable one on the page.
- **The shape of a comparison is fixed and the demo reads it.** `report.md` now names `only_baseline`, `only_run`, `flipped`, and `delta`, so the `jq` paths on this page are assertions. The page also confirms that a missing case is reported as missing and never as a change.
- **An exact check is keyed by a pointer and not by a name the user chose.** ADR 0008 keys it by the left pointer, so this page reads `/gold_code`. That key moves the moment somebody renames the field, and a saved dashboard breaks with it. The demo asks for `--truth NAME=/A=/B`, so a script can name the metric it charts.
- **The demo could not show a resumed run resuming.** The cache holds every entry, so a rerun replays all twelve requests and the saving is asserted in prose. ADR 0008 item 5 leans on the resume for its claim that a completed run holds a judgment for every case.

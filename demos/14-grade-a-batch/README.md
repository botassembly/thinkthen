# How to grade an assistant's answers with a rubric

Status: green

Use a saved question set when every assistant reply needs the same factual checks, failure label, and severity score. One detailed JSON line keeps the case, every probability, and the request that produced each answer.

```bash
env -u THINKTHEN_API_KEY thinkthen annotate checks.json --jsonl --details \
  --replay recording --input cases.jsonl \
  | jq -s -c '{rows:length, checks:(.[0].value|keys)}' \
  | mustmatch '{"rows":6,"checks":["complete","correct","failure_kind","grounded","severity"]}'
```

Verbs: `annotate`

## Input

`cases.jsonl` holds six cases with the request, source, reference answer, assistant answer, and a human verdict. `checks.json` holds three decisions, one choice, and one score. `grounded` sees only `/context` and `/output`. The other four checks share `/input`, `/gold`, and `/output`, so mixed question types ride in one request without exposing the reference to the grounding check.

## Prove the disclosure boundary

```bash
env -u THINKTHEN_API_KEY thinkthen annotate checks.json --jsonl --dry-run \
  --input cases.jsonl \
  | jq -c '.on' \
  | mustmatch '{"correct":["/input","/gold","/output"],"grounded":["/context","/output"],"complete":["/input","/gold","/output"],"failure_kind":["/input","/gold","/output"],"severity":["/input","/gold","/output"]}'
```

## Save the complete graded run

```bash
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
for run in a b; do
  env -u THINKTHEN_API_KEY thinkthen annotate checks.json --jsonl --details \
    --replay recording --input cases.jsonl > "$work/run-$run.jsonl"
done
jq -s -c '{rows:length, definition:([.[].meta.questions_sha256]|unique|length), request_groups:([.[0].answers[].request]|unique|length)}' "$work/run-a.jsonl" \
  | mustmatch '{"rows":6,"definition":1,"request_groups":2}'
jq -n --slurpfile before "$work/run-a.jsonl" \
  -f ../../transforms/compare/compare.jq "$work/run-b.jsonl" \
  | jq -c '.questions.correct | {compared,same,changed_values}' \
  | mustmatch '{"compared":6,"same":6,"changed_values":0}'
```

The comparison pairs the six records once, then compares `correct` and every other named check independently. A changed, added, or removed check stays visible without making every record look changed.

## Compare the judge with human labels

```bash
env -u THINKTHEN_API_KEY thinkthen annotate checks.json --jsonl --details \
  --replay recording --input cases.jsonl \
  | jq -s -c '{resolved:[.[]|select(.value.correct != null)]|length, agreements:[.[]|select(.value.correct == .input.human_correct)]|length}' \
  | mustmatch '{"resolved":4,"agreements":4}'
```

The judge resolved four correctness checks and agreed with the human label on all four. It left two cases inside the review band. Those cases need a person; they do not count as agreements or disagreements.

## What can go wrong

Exit 2 means a record already owns a question name or an option cannot act. Exit 4 means a backend request failed. Exit 5 means the set, input, or recording is unusable. Changing one question re-asks its whole group, and near-cut answers can move when their neighbors change. Keep the rubric fixed during a comparison and retain the detailed probabilities.

## Related how-tos

- [Pick a threshold from labeled cases](../13-pick-a-threshold/)
- [Check the judge against human labels](../25-check-the-judge/)
- [Know what a run cost](../28-what-a-run-cost/)

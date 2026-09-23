# How to know what a run cost

Status: green

Verbs: `decide`

Every judged record is a paid request, and a run has no spending cap. Use this page to add up what a finished run spent, to see what one case costs before you scale to a hundred thousand of them, and to confirm that a replayed run spent nothing at all.

## Input

`../../transforms/rows/runs/run-a.jsonl` and `../../transforms/rows/runs/run-b.jsonl` hold forty judged rows each. Every row carries `meta.usage`, the token counts the backend reported, and `meta.replayed`, the older name of `meta.cached`, which says whether a backend or stored exchanges answered.

`../../transforms/rows/recording/` holds the eighty exchanges both runs were made from. A recording keeps request bodies and never headers, so no key is in it.

The transform is `../../transforms/cost/cost.jq`. The price of a million input tokens is an argument, because a price is a fact about a contract and not about a run.

## Add up one run

```bash
set -euo pipefail

jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq \
  ../../transforms/rows/runs/run-a.jsonl | jq -c . \
  | mustmatch '{"rows":40,"no_usage":[],"charged":{"rows":40,"input_tokens":11784,"output_tokens":840},"replayed":{"rows":0,"input_tokens":0,"output_tokens":0},"usd":0.000495,"usd_per_million_input":0.042}'
```

Forty cases cost five hundredths of a cent. `no_usage` is empty, so every row reported its tokens and no count was guessed.

Several runs add up in one call, because `jq` reads as many files as you name.

```bash
set -euo pipefail

jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq \
  ../../transforms/rows/runs/run-a.jsonl ../../transforms/rows/runs/run-b.jsonl \
  | jq -c '{rows, charged, usd}' \
  | mustmatch '{"rows":80,"charged":{"rows":80,"input_tokens":23848,"output_tokens":1680},"usd":0.001002}'
```

## Read the price of one case before you scale

```bash
set -euo pipefail

jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq \
  ../../transforms/rows/runs/run-a.jsonl \
  | jq -c '{tokens_per_case: (.charged.input_tokens / .charged.rows),
            usd_per_100k_cases: ((.charged.input_tokens / .charged.rows) * 100000
                                 / 1000000 * .usd_per_million_input * 1000 | round | . / 1000)}' \
  | mustmatch '{"tokens_per_case":294.6,"usd_per_100k_cases":1.237}'
```

The messages in this case file run about twenty-five words, and a case still costs about 295 input tokens. The question and the wire shape are most of it, so a short message is not a cheap one, and doubling the length of the evidence does not double the bill.

## A replayed row spent nothing

The rows above came from a backend. The same command against the recording opens no connection and reads no key, and the transform keeps its tokens out of the money.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

question=$(cat ../../transforms/rows/question.txt)
example=$(jq -c 'select(.id == "C-01")' ../../transforms/rows/cases.jsonl)

printf '%s' "$(printf '%s' "$example" | jq -r '.body')" \
  | thinkthen decide "$question" --threshold 0.2:0.8 --details \
      --replay ../../transforms/rows/recording/ > "$work/one.json"

jq -c --argjson case "$example" \
  '{schema, value, input: $case, question, answer, threshold, meta}' \
  "$work/one.json" > "$work/one.jsonl"

jq -c '{value, cached: .meta.cached}' "$work/one.jsonl" \
  | mustmatch '{"value":true,"cached":true}'

jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq "$work/one.jsonl" \
  | jq -c . \
  | mustmatch '{"rows":1,"no_usage":[],"charged":{"rows":0,"input_tokens":0,"output_tokens":0},"replayed":{"rows":1,"input_tokens":303,"output_tokens":21},"usd":0,"usd_per_million_input":0.042}'
```

The replayed tokens are reported and never priced. Counting them would put a bill on every gate that replays a recording, and a gate pays nothing.

### A text record with no usage

`--lines` keeps the original text in `input`, so a cost report cannot assume every input has an id. Missing usage stays visible and the text is not echoed.

```bash
set -e
result=$(printf '%s\n' '{"input":"do not echo this text","meta":{}}' | jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq)
test "${result#*do not echo this text}" = "$result"
printf '%s\n' "$result" | jq -c '{rows, no_usage, charged, usd}' \
  | mustmatch '{"rows":1,"no_usage":["with no id"],"charged":{"rows":1,"input_tokens":0,"output_tokens":0},"usd":0}'
```

## The same line, kept as a file

```bash
set -euo pipefail

sh ../../transforms/cost/example.sh | jq -c '{rows, usd}' | mustmatch '{"rows":40,"usd":0.000495}'
```

## What can go wrong

- **`jq` is missing.** The `install` rung names it.
- **A transform stops with exit 5.** `jq` exits 5 for a line it cannot parse and for an error the transform raises.
- **A missing entry in the recording is exit 5 too, from the tool.** `thinkthen` names the entry it wanted. A body that differs by one byte from the recorded one is a different entry, so evidence read through a pipe has to arrive exactly as it did when the exchange was recorded.
- **Assuming a zero for a missing count.** A backend that reports no usage leaves `meta.usage` absent. Those rows are listed in `no_usage` and add nothing, so a total is never quietly short.
- **Pricing output tokens with the input price.** They are reported beside the input tokens and never converted. A decider model answers with numbers, so output is small, and only the input side is worth watching.
- **Reading the price from this page.** 0.042 for a million input tokens is what the hosted service charged on 2026-09-19. Another address has another price, and the argument exists so nobody hard-codes one.
- **A loop over files has no budget.** Each file is its own run, and nothing stops at a cap. `sdlc/scripts/live` is the door for a paid call in this repository, and it refuses at the ledger's limit.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to tune a question file](../41-tune-a-question-file/)
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)
- [How to gate a script step on a yes/no answer](../01-refund-gate/)

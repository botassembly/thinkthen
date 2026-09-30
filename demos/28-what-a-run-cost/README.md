# How to know what a run cost

Status: green

Verbs: `decide`

A request may judge several records. Estimate a finished run at your prices, check one case before scaling, and confirm replay's zero new spend. No price-based dollar cap is enforced.

## Input

`../../transforms/rows/runs/run-a.jsonl` and `../../transforms/rows/runs/run-b.jsonl` each hold forty judged rows with reported `meta.usage` and legacy `meta.replayed` (now `meta.cached`). `../../transforms/rows/recording/` holds their eighty request and response bodies, without headers or keys. The input-only transform `../../transforms/cost/cost.jq` takes your contract's input price as an argument.

## Add up one run

```bash
set -euo pipefail
jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq \
  ../../transforms/rows/runs/run-a.jsonl | jq -c . \
  | mustmatch '{"rows":40,"no_usage":[],"charged":{"rows":40,"input_tokens":11784,"output_tokens":840},"replayed":{"rows":0,"input_tokens":0,"output_tokens":0},"usd":0.000495,"usd_per_million_input":0.042}'
```

These forty cases cost five hundredths of a cent. `no_usage` is empty: no token count was guessed. `jq` can add multiple run files in one call.

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

For roughly twenty-five-word messages, `run-a.jsonl` measured 294.6 input tokens per case. The question and wire shape add cost; evidence length alone does not predict it.

## A replayed row spent nothing

The recording opens no connection and reads no key. The transform excludes replayed tokens from cost.

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

mkdir -p "$work/config/thinkthen"
printf '%s\n' '{"schema":"thinkthen.config/1","usd_per_million_input":"0.042","usd_per_million_output":"0.168"}' \
  > "$work/config/thinkthen/config.json"
printf '%s' "$(printf '%s' "$example" | jq -r '.body')" \
  | XDG_CONFIG_HOME="$work/config" thinkthen decide "$question" --threshold 0.2:0.8 --details \
      --replay ../../transforms/rows/recording/ --facts > "$work/priced.json" 2> "$work/priced.stderr"
tail -n 1 "$work/priced.stderr" | jq -c '{requests_sent, estimated_cost_usd}' \
  | mustmatch '{"requests_sent":0,"estimated_cost_usd":"0.000000"}'
```

Replay reports tokens but no new spend. Configured `--facts` prices new sends with your input and output prices; this replay shows zero. The earlier transform prices input only.

### A text record with no usage

`--lines` keeps text in `input`, which need not have an id. Missing usage stays visible without echoing the text.

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
- **Mistaking the input-only transform for a two-price estimate.** It uses one caller-supplied input price; configured `--facts` uses separate input and output prices. Neither is a provider bill.
- **Copying this page's price.** The sample input price was observed on 2026-09-19; use the prices in your own contract.
- **Assuming a dollar cap.** Request and estimated-input limits can stop sends, but do not cap a provider bill. Paid calls here go through `sdlc/scripts/live` and its ledger limit.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to tune a question file](../41-tune-a-question-file/)
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)
- [How to gate a script step on a yes/no answer](../01-refund-gate/)

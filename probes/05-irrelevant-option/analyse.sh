#!/bin/sh
# Read probe 5: what one added label that fits nothing moved, against the five
# labels of probe 2.
#
# shift.jq lives in probe 4 and is read from there, because one file answers
# both questions and two copies would drift.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}
added=$(cat added.txt)

printf '=== the added label against the five ===\n'
jq -n --slurpfile before ../02-confidence/runs/run.jsonl -f ../04-option-order/shift.jq \
	"$rows/added.jsonl"

printf '\n=== what the added label itself was given ===\n'
jq -s --arg added "$added" '
  map(.answer.probabilities[$added])
  | {label: $added,
     rows: length,
     exactly_zero: (map(select(. == 0)) | length),
     above_zero: (map(select(. > 0)) | length),
     max: max}' "$rows/added.jsonl"

printf '\n=== cost, through transforms/cost/cost.jq ===\n'
jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq "$rows/added.jsonl" |
	jq -c '{rows, requests: .charged.rows, input_tokens: .charged.input_tokens, usd}'

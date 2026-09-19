#!/bin/sh
# Read probe 2: the sweep on each of the two numbers, and how far apart the two
# numbers are on the same row.
#
# None of the recipes in recipes/ reads a `choice` row. They read `value` as
# true, false, or null, and a pick is a string, so sweep-two.jq is written here
# beside the probe. cost.jq is the one recipe that fits any row.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}

printf '=== accuracy at coverage on each number ===\n'
jq -n -f sweep-two.jq "$rows/run.jsonl"

printf '\n=== how far the two numbers sit apart on one row ===\n'
jq -s 'map({p: (.answer.probabilities | to_entries | map(.value) | max),
            c: .answer.confidence})
       | {rows: length,
          equal: (map(select(.p == .c)) | length),
          confidence_below: (map(select(.c < .p)) | length),
          confidence_above: (map(select(.c > .p)) | length),
          gap: (map(.p - .c | fabs) | {min: min, max: (max * 10000 | round / 10000)}),
          mean_gap: ((map(.p - .c | fabs) | add) / length * 10000 | round / 10000)}' \
	"$rows/run.jsonl"

printf '\n=== cost, through recipes/cost/cost.jq ===\n'
jq -n --argjson usd_per_million_input 0.042 -f ../../recipes/cost/cost.jq "$rows/run.jsonl" |
	jq -c '{rows, requests: .charged.rows, input_tokens: .charged.input_tokens, usd}'

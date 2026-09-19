#!/bin/sh
# Read probe 4: what the reversed list and the shuffled list moved, against the
# forward list of probe 2.
#
# recipes/compare/compare.jq cannot read these rows. Its `verdict` accepts true,
# false, and null, and a pick is a string, so it stops on the first row.
# shift.jq beside this script is the `choice` shape of the same idea.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}

for run in reversed shuffled; do
	printf '=== %s against the forward list ===\n' "$run"
	jq -n --slurpfile before ../02-confidence/runs/run.jsonl -f shift.jq "$rows/$run.jsonl"
	printf '\n'
done

printf '=== cost, through recipes/cost/cost.jq ===\n'
for run in reversed shuffled; do
	printf '%s: ' "$run"
	jq -n --argjson usd_per_million_input 0.042 -f ../../recipes/cost/cost.jq "$rows/$run.jsonl" |
		jq -c '{rows, requests: .charged.rows, input_tokens: .charged.input_tokens, usd}'
done

#!/bin/sh
# Read probe 3: the agreement numbers from analyse.py and the cost of each run.
#
# No recipe in transforms/ reads a `score` row or a `choice` row, and jq has no
# rank correlation, so the agreement arithmetic is the Python script beside this
# one. cost.jq is the one recipe that fits any row.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}

printf '=== agreement, within one level, and rank correlation ===\n'
python3 analyse.py "$rows"

printf '\n=== cost, through transforms/cost/cost.jq ===\n'
for run in score choose; do
	printf '%s: ' "$run"
	jq -n --argjson usd_per_million_input 0.042 -f ../../transforms/cost/cost.jq "$rows/$run.jsonl" |
		jq -c '{rows, requests: .charged.rows, input_tokens: .charged.input_tokens, usd}'
done

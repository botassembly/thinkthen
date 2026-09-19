#!/bin/sh
# Add up what a run spent.
# The page is demos/28-what-a-run-cost/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n --argjson usd_per_million_input 0.042 -f cost.jq ../rows/runs/run-a.jsonl

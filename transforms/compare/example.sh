#!/bin/sh
# Compare two runs over the same cases.
# The page is demos/41-tune-a-question-file/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n --argjson probability_tolerance 0.08 \
  --slurpfile before ../rows/runs/run-a.jsonl -f compare.jq ../rows/runs/run-b.jsonl

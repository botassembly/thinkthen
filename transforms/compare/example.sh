#!/bin/sh
# Compare two runs over the same cases.
# The page is demos/41-tune-a-question-file/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n --slurpfile before ../rows/runs/run-a.jsonl -f compare.jq ../rows/runs/run-b.jsonl

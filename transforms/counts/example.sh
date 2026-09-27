#!/bin/sh
# Count the yes, no, and not sure answers of a run.
# The page is demos/25-check-the-judge/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n -f counts.jq ../rows/runs/run-a.jsonl

#!/bin/sh
# Score a run against the trusted labels at one cut.
# The page is demos/25-check-the-judge/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n --argjson cut 0.5 -f score.jq ../rows/runs/run-a.jsonl

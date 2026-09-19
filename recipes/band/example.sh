#!/bin/sh
# Read accuracy beside coverage for a band.
# The page is demos/13-pick-a-threshold/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n --argjson band "[0.2,0.8]" -f band.jq ../rows/runs/run-a.jsonl

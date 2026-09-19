#!/bin/sh
# Ask every cut what it would have done, and pick one.
# The page is demos/13-pick-a-threshold/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n -f sweep.jq ../rows/runs/run-a.jsonl

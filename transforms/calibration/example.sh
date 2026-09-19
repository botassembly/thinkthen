#!/bin/sh
# Set each probability band beside the share of cases that were truly yes.
# The page is demos/38-what-a-probability-means/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n -f calibration.jq ../rows/runs/run-a.jsonl

#!/bin/sh
# Average repeated observations once per case before running a metric.
set -eu
cd -- "$(dirname -- "$0")"

jq -n -f trials.jq ../../probes/07-true-and-false-texts/runs/plain.jsonl

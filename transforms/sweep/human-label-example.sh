#!/bin/sh
# Fit the named correctness answer against its human field.
# The page is demos/14-grade-a-batch/README.md.
set -eu
cd -- "$(dirname -- "$0")"

jq -n --argjson truth '{"correct":"/input/human_correct"}' \
  -f sweep.jq ../../probes/annotate-0015/howto-14.jsonl

#!/bin/sh
# Fit one cut for each record group.
# The page is demos/13-pick-a-threshold/README.md.
set -eu
cd -- "$(dirname -- "$0")"

rows=$(mktemp)
trap 'rm -f -- "$rows"' EXIT
jq -c '.input.document_type = if .input.id <= "C-20" then "email" else "chat" end' \
  ../rows/runs/run-a.jsonl > "$rows"
jq -n --arg group /input/document_type -f sweep.jq "$rows"

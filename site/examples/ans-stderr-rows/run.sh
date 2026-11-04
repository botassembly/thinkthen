#!/usr/bin/env bash
# Work in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.jsonl "$work" && cd "$work"
# First the run the page shows above.
thinkthen decide --field /text --json \
  'Does the customer request a refund?' messages.jsonl \
  > rows.jsonl 2> errors.txt || true
bash "$here/show.sh"

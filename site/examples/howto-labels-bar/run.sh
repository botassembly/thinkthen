#!/usr/bin/env bash
set -eu
# Work in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
cp labeled.jsonl "$work"
cd "$work"
# First the step before, which writes answered.jsonl.
thinkthen decide --field /text --into answer \
  'Does the customer request a refund?' \
  labeled.jsonl > answered.jsonl
bash -e "$here/show.sh"

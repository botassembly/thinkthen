#!/usr/bin/env bash
set -eu
# Work in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
cp papers.jsonl screen.json screen.jq "$work"
cd "$work"
# First the step before, which writes screened.jsonl.
thinkthen annotate --into screen \
  screen.json papers.jsonl > screened.jsonl
bash -e "$here/show.sh"

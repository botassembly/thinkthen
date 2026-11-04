#!/usr/bin/env bash
set -eu
# Work in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
cp messages.jsonl triage.json route.jq "$work"
cd "$work"
# First the step before, which writes triaged.jsonl.
thinkthen annotate --into triage \
  triage.json messages.jsonl > triaged.jsonl
bash -e "$here/show.sh"

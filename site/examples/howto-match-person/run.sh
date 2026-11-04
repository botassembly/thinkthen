#!/usr/bin/env bash
set -eu
# Work in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
cp pairs.jsonl "$work"
cd "$work"
# First the step before, which writes matched.jsonl.
thinkthen decide --field /pair --low 0.2 --high 0.8 \
  --into same 'Do left and right name the same company?' \
  pairs.jsonl > matched.jsonl
bash -e "$here/show.sh"

#!/usr/bin/env bash
set -eu
# Work in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
cp pairs.jsonl "$work"
cd "$work"
bash -e "$here/show.sh"

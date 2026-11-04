#!/usr/bin/env bash
# Work in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.jsonl "$work" && cd "$work"
bash "$here/show.sh"

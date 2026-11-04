#!/usr/bin/env bash
# Work on a copy in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.csv "$work" && cd "$work"
bash "$here/show.sh"

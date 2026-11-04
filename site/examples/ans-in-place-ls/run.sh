#!/usr/bin/env bash
# Work on a copy in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.csv "$work" && cd "$work"
# First the in-place run the page shows above.
thinkthen decide --column text --into refund --in-place \
  'Does the customer request a refund?' messages.csv > /dev/null 2>&1
bash "$here/show.sh"

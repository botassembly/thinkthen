#!/usr/bin/env bash
# Work on a copy in a scratch folder, so the example can run again.
here=$PWD
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp messages.csv "$work" && cd "$work"
# First the run the page shows above, which writes messages.thinkthen.csv.
thinkthen decide --column text --into refund \
  'Does the customer request a refund?' messages.csv > /dev/null
bash "$here/show.sh"

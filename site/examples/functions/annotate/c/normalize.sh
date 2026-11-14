#!/bin/sh
# Normalize the displayed native answer for the shared function case.
set -eu
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" 'split("\n") | map(select(length > 0)) | map(split(" ") | {refund: (.[0] | fromjson), topic: .[1], urgency: (.[2] | fromjson)})' \
  "$1" > "$SITE_ANSWERS_FILE"

#!/bin/sh
# Normalize the displayed native answer for the shared function case.
set -eu
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[split("\n") | map(select(length > 0)) | map(capture("^(?<kind>[^:]+): (?<text>.*)$") | {text, kind})]' \
  "$1" > "$SITE_ANSWERS_FILE"

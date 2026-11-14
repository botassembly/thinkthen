#!/bin/sh
# Normalize the displayed native answer for the shared function case.
set -eu
jq -R -s -c '[split("\n") | map(select(length > 0)) | sort]' \
  "$1" > "$SITE_ANSWERS_FILE"

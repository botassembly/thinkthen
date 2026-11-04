#!/bin/sh
# Compile the shown source against the site's staged installed package.
set -eu
root=$(cd ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
trap 'exit 1' HUP INT TERM
export PKG_CONFIG_PATH="$packages/c/lib/pkgconfig"
cc -std=c11 -Wall -Wextra -Werror show.c \
  $(pkg-config --cflags --libs thinkthen-c) \
  -Wl,-rpath,"$packages/c/lib" -o "$work/example"
"$work/example" > "$work/output.txt"
cat "$work/output.txt"
jq -R -s -c --slurpfile case "$SITE_CASE_FILE" '[split("\n") | map(select(length > 0)) | map(capture("^(?<kind>[^:]+): (?<text>.*)$") | {text, kind})]' \
  "$work/output.txt" > "$SITE_ANSWERS_FILE"

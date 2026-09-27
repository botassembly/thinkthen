#!/bin/sh
# Fail the build when any changed hunk breaks the house rule in convention.json.
# The kept hunks are the report. A clean change keeps none and exits 0.
set -eu

cd -- "$(dirname -- "$0")"

kept=$(thinkthen filter @convention.json --jsonl --batch 1 --replay recording/ --input "$1")
[ -n "$kept" ] || exit 0

printf 'the change breaks the house rule in %s hunk(s):\n' "$(printf '%s\n' "$kept" | wc -l | tr -d ' ')"
printf '%s\n' "$kept"
exit 1

#!/bin/sh
# Monitor the page-16 policy against its recorded human decisions.
set -eu
cd -- "$(dirname -- "$0")"

jq -c -f ../triage/triage.jq ../../demos/16-triage-pipeline/fake-rows.jsonl \
	| jq -n -f monitor.jq /dev/stdin

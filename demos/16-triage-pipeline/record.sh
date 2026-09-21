#!/bin/sh
set -eu

REPO=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd -- "$REPO"
output=demos/16-triage-pipeline/live-output
sdlc/scripts/live --max-tokens 5000 \
	demos/16-triage-pipeline/triage demos/16-triage-pipeline/live-output \
	--cache demos/16-triage-pipeline/recording \
	--url https://api.typesafe.ai/v1

jq -e -s 'length == 6
  and any(.[]; .policy.action == "draft")
  and any(.[]; .policy.action == "block")
  and any(.[]; .policy.action == "review")
  and all(.[]; .policy.action == .input.reviewed_action)' \
	"$output"/*.jsonl >/dev/null

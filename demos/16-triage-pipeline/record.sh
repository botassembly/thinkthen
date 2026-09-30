#!/bin/sh
set -eu

REPO=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd -- "$REPO"
. "$REPO/sdlc/scripts/scratch.sh"
scratch_dir work
scratch_dir scratch
output=$work/output

demos/16-triage-pipeline/triage "$output" \
	--cache "$scratch" \
	--url https://api.typesafe.ai/v1

jq -e -s 'length == 6
  and any(.[]; .policy.action == "draft")
  and any(.[]; .policy.action == "block")
  and any(.[]; .policy.action == "review")
  and all(.[]; .policy.action == .input.reviewed_action)' \
	"$output"/*.jsonl >/dev/null

cp -- "$scratch/thinkthen.sqlite" demos/16-triage-pipeline/recording/
thinkthen cache convert demos/16-triage-pipeline/recording/

#!/bin/sh
# Record this demo's four exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

scratch_dir scratch

# The page repairs the third record, whose text sits under `note`. Recording
# the repaired file records every exchange the page replays, because the first
# two records are byte for byte what the unrepaired run sends.
jq -c 'if has("note") then {id, body: .note} else . end' queue.jsonl >"$scratch/fixed.jsonl"

thinkthen decide 'Does the message report a payment failure?' --batch 1 \
	--jsonl --field /body --input "$scratch/fixed.jsonl" --record "$scratch" >/dev/null

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

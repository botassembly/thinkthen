#!/bin/sh
# Record this demo's four exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# The page repairs the third record, whose text sits under `note`. Recording
# the repaired file records every exchange the page replays, because the first
# two records are byte for byte what the unrepaired run sends.
jq -c 'if has("note") then {id, body: .note} else . end' queue.jsonl >fixed.jsonl
trap 'rm -f -- fixed.jsonl' EXIT

thinkthen decide 'Does the message report a payment failure?' \
	--jsonl --field /body --input fixed.jsonl --record recording/ >/dev/null

ls -1 recording/ | wc -l | tr -d ' '

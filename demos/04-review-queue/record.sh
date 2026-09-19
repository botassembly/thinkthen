#!/bin/sh
# Record this demo's five exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# One record run answers every block of the page. A digest names the URL and
# the request body alone, and the request body carries the evidence and the
# question, so --threshold and --details change no digest and every block
# replays these five entries.
thinkthen decide 'Does the customer ask to end their subscription?' \
	--jsonl --field /body --input messages.jsonl --record recording/ >/dev/null

ls -1 recording/ | wc -l | tr -d ' '

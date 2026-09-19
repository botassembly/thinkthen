#!/bin/sh
# Record this demo's three exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# Each step carries its own list of actions, so each record asks its own
# question and writes its own entry. A digest names the address and the request
# body, and the body carries the evidence, the question, and the options, so
# --details and --raw change no digest and every block replays these three.
thinkthen choose 'Which of these actions should be taken next?' \
	--jsonl --field /state --options /actions --input steps.jsonl \
	--record recording/ >/dev/null

ls -1 recording/ | wc -l | tr -d ' '

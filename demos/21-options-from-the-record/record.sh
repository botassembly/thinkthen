#!/bin/sh
# Record this demo's three exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# Each record carries its own list of codes, so each record asks its own
# question and writes its own entry. A digest names the URL and the request
# body, and the body carries the evidence, the question, and the options, so
# --details and --raw change no digest and every block replays these three.
thinkthen choose 'Which of these codes fits the note?' \
	--jsonl --field /note --options /codes --input notes.jsonl \
	--record recording/ >/dev/null

ls -1 recording/ | wc -l | tr -d ' '

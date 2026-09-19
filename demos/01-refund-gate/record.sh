#!/bin/sh
# Record this demo's two exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# Two exchanges answer every block of the page. The request bytes carry the
# evidence and the question, and nothing else, so --threshold, --quiet, and
# --details change no digest and the blocks share these entries.
for evidence in message.txt question.txt; do
	thinkthen decide 'Does the customer ask for money back?' \
		--quiet --record recording/ < "$evidence" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$evidence" "$exit"
done

ls -1 recording/

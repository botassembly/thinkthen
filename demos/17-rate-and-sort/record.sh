#!/bin/sh
# Record one exchange per request against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

for request in requests/*.txt; do
	thinkthen score 'How hard is this request to answer?' \
		'A canned reply answers it.' \
		'One person can answer it after a look at the account.' \
		'It needs a specialist and more than one system.' \
		--record recording/ <"$request" >/dev/null && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$request" "$exit"
done

ls -1 recording/

#!/bin/sh
# Record one exchange per report against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

for report in reports/*.txt; do
	thinkthen score 'How much disruption does this report?' \
		'No disruption; nothing stops working.' \
		'Work continues, because a workaround exists.' \
		'Work is blocked, and no workaround exists.' \
		--record recording/ < "$report" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$report" "$exit"
done

ls -1 recording/

#!/bin/sh
# Record one pick and one yes/no answer per notice against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

claim='the upgrade window runs longer than the original plan'

for notice in notices/*.txt; do
	thinkthen choose "How does this notice treat the claim that $claim?" \
		supported contradicted not_stated ambiguous \
		--quiet --record recording/ < "$notice" && exit=0 || exit=$?
	printf 'recorded pick for %s, exit %s\n' "$notice" "$exit"

	thinkthen decide "Does this notice say that $claim?" \
		--quiet --record recording/ < "$notice" && exit=0 || exit=$?
	printf 'recorded yes/no for %s, exit %s\n' "$notice" "$exit"
done

ls -1 recording/

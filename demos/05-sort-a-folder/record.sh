#!/bin/sh
# Record one exchange per note against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

for note in inbox/*.txt; do
	thinkthen choose 'What kind of work does this note record?' defect process other \
		--quiet --record recording/ < "$note" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$note" "$exit"
done

ls -1 recording/

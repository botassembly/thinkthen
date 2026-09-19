#!/bin/sh
# Record this demo's two exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# Two exchanges answer every block that gets an answer. The third block asks a
# question this folder does not hold, and that miss is the point of it.
for evidence in note.txt hurried.txt; do
	thinkthen decide 'Does the note say the change was tested in staging?' \
		--quiet --record recording/ <"$evidence" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$evidence" "$exit"
done

ls -1 recording/

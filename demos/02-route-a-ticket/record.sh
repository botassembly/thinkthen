#!/bin/sh
# Record this demo's six exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# One exchange answers every block that asks about the ticket. The request
# bytes carry the evidence, the question, and the options, and nothing else,
# so --threshold, --raw, and --details change no digest.
thinkthen choose 'Which team owns this request?' billing shipping account other \
	--quiet --record recording/ < ticket.txt && exit=0 || exit=$?
printf 'recorded ticket.txt, exit %s\n' "$exit"

# One exchange per note answers the closing section, which files a folder.
for note in inbox/*.txt; do
	thinkthen choose 'What kind of work does this note record?' defect process other \
		--quiet --record recording/ < "$note" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$note" "$exit"
done

ls -1 recording/

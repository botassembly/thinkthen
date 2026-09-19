#!/bin/sh
# Record this demo's one exchange against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# One exchange answers every block of the page. The request bytes carry the
# evidence, the question, and the options, and nothing else, so --threshold,
# --raw, and --details change no digest and the blocks share this entry.
thinkthen choose 'Which team owns this request?' billing shipping account other \
	--quiet --record recording/ < ticket.txt && exit=0 || exit=$?
printf 'recorded ticket.txt, exit %s\n' "$exit"

ls -1 recording/

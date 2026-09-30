#!/bin/sh
# Record this demo's two exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

scratch_dir scratch

# Two exchanges answer every block of the page. The request bytes carry the
# evidence and the question, and nothing else, so --threshold, --quiet, and
# --details change no digest and the blocks share these entries.
for evidence in message.txt question.txt; do
	thinkthen decide 'Does the customer ask for money back?' \
		--quiet --record "$scratch" < "$evidence" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$evidence" "$exit"
done

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

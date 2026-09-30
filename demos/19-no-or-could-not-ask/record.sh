#!/bin/sh
# Record this demo's three exchanges against the live backend.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

scratch_dir scratch

# One exchange per proposed command answers every block that gets an answer.
# A digest names the address and the request bytes, and --threshold, --quiet,
# and --details change none of them, so every block replays these three. The
# block that shows a miss asks a question this folder was never given.
for proposed in proposed/list.txt proposed/fetch.txt proposed/wipe.txt; do
	thinkthen decide 'Does this command only read, and leave every file and every setting on the machine unchanged?' \
		--quiet --record "$scratch" <"$proposed" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$proposed" "$exit"
done

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

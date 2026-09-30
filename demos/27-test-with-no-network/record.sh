#!/bin/sh
# Record this demo's one exchange against the live backend, through the same
# script the page tests. Run through sdlc/scripts/live, which holds the key
# check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# vague.txt is left unrecorded on purpose. The page replays it to show what a
# miss looks like.
scratch_dir scratch
sh triage.sh report.txt --record "$scratch" && exit=0 || exit=$?
printf 'recorded report.txt, exit %s\n' "$exit"

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

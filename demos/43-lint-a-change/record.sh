#!/bin/sh
# Record this page's five exchanges against the live backend, one per hunk.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
#
# clean.jsonl holds three of the same five hunks, so its run replays these
# entries and pays for nothing.
set -eu

cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

scratch_dir scratch

thinkthen decide @convention.json --jsonl --batch 1 --details \
	--cache "$scratch" --input hunks.jsonl |
	jq -c '{file: .input.file, yes: .value, p: .answer.probability}'

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

#!/bin/sh
# Record this page's five exchanges against the live backend, one per issue.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

scratch_dir scratch

thinkthen filter 'Does the report give steps that would reproduce a defect?' --batch 1 \
	--csv --field /body --threshold 0.9 --input issues.csv \
	--cache "$scratch" | jq -r '.id'

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

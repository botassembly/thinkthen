#!/bin/sh
# Record the two exchanges this page replays, from the handbook's first cut.
#
#   sdlc/scripts/live --max-tokens 3000 demos/48-first-cut-a-long-list/record.sh
set -eu

cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

scratch_dir scratch
grep '^customer: ' handbook.txt | cut -d ' ' -f 2- >"$scratch/customer.txt"
thinkthen find 'When does a refund reach the customer?' --lines \
	--url https://api.typesafe.ai/v1 --model jev-1.13.0 --max-retries 0 \
	--record "$scratch" --input "$scratch/customer.txt"
thinkthen find 'How long is the manufacturer warranty?' --lines --none \
	--url https://api.typesafe.ai/v1 --model jev-1.13.0 --max-retries 0 \
	--record "$scratch" --input "$scratch/customer.txt" && status=0 || status=$?
test "$status" -eq 3

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

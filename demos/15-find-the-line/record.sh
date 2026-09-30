#!/bin/sh
# Record the two exchanges this page replays.
#
#   sdlc/scripts/live --max-tokens 3000 demos/15-find-the-line/record.sh
set -eu
cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

scratch_dir scratch

thinkthen find 'When does a refund reach the customer?' --lines \
  --url https://api.typesafe.ai/v1 --model jev-1.13.0 --max-retries 0 \
  --cache "$scratch" --input policy.txt
thinkthen find 'How long is the manufacturer warranty?' --lines --none \
  --url https://api.typesafe.ai/v1 --model jev-1.13.0 --max-retries 0 \
  --cache "$scratch" --input policy.txt && status=0 || status=$?
test "$status" -eq 3

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

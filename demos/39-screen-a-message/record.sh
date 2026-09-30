#!/bin/sh
# Record the one exchange this page replays.
#
#   sdlc/scripts/live --max-tokens 5000 demos/39-screen-a-message/record.sh
set -eu
cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

scratch_dir scratch

thinkthen tag @hazards.json --details \
  --url https://api.typesafe.ai/v1 --cache "$scratch" --input message.txt

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

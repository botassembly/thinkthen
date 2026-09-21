#!/bin/sh
# Record the two exchanges this page replays.
#
#   sdlc/scripts/live --max-tokens 3000 demos/15-find-the-line/record.sh
set -eu
cd -- "$(dirname -- "$0")"

thinkthen find 'When does a refund reach the customer?' --lines \
  --url https://api.typesafe.ai/v1 --model jev-1.13.0 --max-retries 0 \
  --cache recording/ --input policy.txt
thinkthen find 'How long is the manufacturer warranty?' --lines --none \
  --url https://api.typesafe.ai/v1 --model jev-1.13.0 --max-retries 0 \
  --cache recording/ --input policy.txt

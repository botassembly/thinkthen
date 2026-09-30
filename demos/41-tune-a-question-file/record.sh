#!/bin/sh
# Record the forty-eight exchanges this page replays: twenty-four claims, asked
# with the draft question file and then with the tuned one.
#
#   sdlc/scripts/live --max-tokens 1000000 demos/41-tune-a-question-file/record.sh
set -eu
cd -- "$(dirname -- "$0")"
REPO=$(CDPATH= cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"

scratch_dir scratch

for file in draft.json receipt.json; do
	printf '%s: ' "$file"
	thinkthen decide @"$file" --jsonl --field /body --details --batch 1 \
		--record "$scratch" --replay "$scratch" <claims.jsonl |
		jq -s -c '{rows: length, yes: (map(select(.value)) | length),
		           digest: (.[0].meta.question_sha256 | .[0:8])}'
done

cp -- "$scratch/thinkthen.sqlite" recording/
thinkthen cache convert recording/

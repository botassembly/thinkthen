#!/bin/sh
# Record this page's five exchanges against the live backend, one per issue.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

thinkthen filter 'Does the report give steps that would reproduce a defect?' \
	--jsonl --field /body --threshold 0.9 --input issues.jsonl \
	--cache recording/ | jq -r '.id'

ls -1 recording/ | wc -l | tr -d ' '

#!/bin/sh
# Record this page's six exchanges against the live backend, one per hit.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

# The query rides in every record, so the question stays fixed for the run and
# one run is one measurement. The page builds the same records the same way.
jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl |
	thinkthen rank 'The passage answers the query.' \
		--jsonl --field /query --field /passage \
		--cache recording/ | jq -r '.path'

ls -1 recording/ | wc -l | tr -d ' '

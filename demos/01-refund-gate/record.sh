#!/bin/sh
# Record this demo's two exchanges against the live backend. Run by hand only.
set -eu

cd -- "$(dirname -- "$0")"

: "${TYPESAFE_API_KEY:?the built-in profile reads this variable, which holds no value}"

# Two exchanges answer all three blocks of the page. The request bytes carry the
# evidence and the condition, and nothing else, so --min-prob and --status change
# no digest and the blocks share these entries.
for evidence in message.txt question.txt; do
	thinkthen decide if 'the customer asks for money back' \
		--min-prob 0.9 --status --record recording/ \
		<"$evidence" >/dev/null && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$evidence" "$exit"
done

ls -1 recording/

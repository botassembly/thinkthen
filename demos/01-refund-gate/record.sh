#!/bin/sh
# Record this demo's two exchanges against the live backend. Run by hand only.
set -eu

cd -- "$(dirname -- "$0")"

: "${TYPESAFE_API_KEY:?the built-in profile reads this variable, which holds no value}"

# Two exchanges answer every block of the page. The request bytes carry the
# evidence and the question, and nothing else, so --threshold, --quiet, and
# --details change no digest and the blocks share these entries.
for evidence in message.txt question.txt; do
	thinkthen decide 'Does the customer ask for money back?' \
		--quiet --record recording/ < "$evidence" && exit=0 || exit=$?
	printf 'recorded %s, exit %s\n' "$evidence" "$exit"
done

ls -1 recording/

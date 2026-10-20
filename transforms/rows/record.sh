#!/bin/sh
# Record a new provider run for every case twice, once with each question.
# The committed runs are labeled synthetic fixtures; this script replaces them
# with new provider results and requires updating their provenance prose.
# Run through sdlc/scripts/live, which holds the key check and the spend ledger.
#
# The shell loop keeps each trusted case beside its result.
# Each row is shaped the way specification/result.md gives for a record row:
# the result object with `input` holding the whole case, the trusted label
# included. The label never leaves the machine, because only the body goes to
# standard input.
set -eu

cd -- "$(dirname -- "$0")"

: "${THINKTHEN_API_KEY:?the tool reads this variable, and it holds no value}"

mkdir -p recording runs

judge() {
	question=$(cat -- "$1")
	out=$2
	: >"$out"
	while IFS= read -r example; do
		body=$(printf '%s\n' "$example" | jq -r '.body')
		printf '%s' "$body" | thinkthen decide "$question" \
			--threshold 0.2:0.8 --details --record recording/ >row.json &&
			exit=0 || exit=$?
		# 0 is yes, 1 is no, and 3 is not sure. Anything else is a failure
		# about the run rather than an answer about a case, so the job stops.
		case "$exit" in
		0 | 1 | 3) ;;
		*)
			printf 'record: %s stopped at %s with exit %s\n' "$out" "$example" "$exit" >&2
			return "$exit"
			;;
		esac
		jq -c --argjson case "$example" \
			'. + {input: $case}' \
			row.json >>"$out"
	done <cases.jsonl
	rm -f row.json
	printf 'wrote %s rows to %s\n' "$(wc -l <"$out" | tr -d ' ')" "$out"
}

judge question.txt runs/run-a.jsonl
judge question-b.txt runs/run-b.jsonl

printf 'recording holds %s entries\n' "$(ls -1 recording/ | wc -l | tr -d ' ')"

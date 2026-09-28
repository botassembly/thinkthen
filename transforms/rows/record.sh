#!/bin/sh
# Judge every case in cases.jsonl twice, once with each question, and write one
# row per case. Run through sdlc/scripts/live, which holds the key check and the
# spend ledger.
#
# Record mode arrived after these rows were written, so the loop is the shell's.
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
			'{schema, value, input: $case, question, answer, threshold, meta}' \
			row.json >>"$out"
	done <cases.jsonl
	rm -f row.json
	printf 'wrote %s rows to %s\n' "$(wc -l <"$out" | tr -d ' ')" "$out"
}

judge question.txt runs/run-a.jsonl
judge question-b.txt runs/run-b.jsonl

printf 'recording holds %s entries\n' "$(ls -1 recording/ | wc -l | tr -d ' ')"

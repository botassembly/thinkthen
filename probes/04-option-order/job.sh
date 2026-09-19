#!/bin/sh
# Probe 4: does the order of the options move the pick?
#
# The sixty cases of probe 2 run again over the same five labels in two other
# orders. The question, the evidence, the model, and the label set are the same,
# so the option order is the only thing that changed.
#
#   reversed.jsonl  the labels of probe 2 back to front.
#   shuffled.jsonl  one fixed shuffle, written down in shuffled.txt before any
#                   call went out and never changed since.
#
#   sdlc/scripts/live probes/04-option-order/job.sh       records
#   OUT=DIR sh probes/04-option-order/job.sh --replay     replays, no network
set -eu

cd -- "$(dirname -- "$0")"

mode=${1:-record}
out=${OUT:-runs}
rec=${REC:-recording}
case "$mode" in
record) folder="--record $rec/" ;;
--replay) folder="--replay $rec/" ;;
*)
	printf 'job: the one argument is --replay\n' >&2
	exit 2
	;;
esac

mkdir -p "$rec" "$out"
question=$(cat ../02-confidence/question.txt)
reversed=$(tac ../02-confidence/labels.txt | tr '\n' ' ')
shuffled=$(tr '\n' ' ' <shuffled.txt)

judge() {
	target=$1
	shift
	: >"$target"
	while IFS= read -r case; do
		body=$(printf '%s\n' "$case" | jq -r '.body')
		# shellcheck disable=SC2086
		printf '%s' "$body" | thinkthen choose "$question" "$@" \
			--details $folder >row.json && exit=0 || exit=$?
		# 0 is a label and 3 is unresolved. choose never exits 1.
		case "$exit" in
		0 | 3) ;;
		*)
			printf 'job: %s stopped with exit %s\n' \
				"$(printf '%s\n' "$case" | jq -r .id)" "$exit" >&2
			exit "$exit"
			;;
		esac
		jq -c --argjson case "$case" \
			'{schema, value, input: $case, question, answer, threshold, meta}' \
			row.json >>"$target"
	done <../02-confidence/cases.jsonl
	printf 'wrote %s rows to %s\n' "$(wc -l <"$target" | tr -d ' ')" "$target"
}

# shellcheck disable=SC2086
judge "$out/reversed.jsonl" $reversed
# shellcheck disable=SC2086
judge "$out/shuffled.jsonl" $shuffled
rm -f row.json

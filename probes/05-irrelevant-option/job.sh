#!/bin/sh
# Probe 5: does one added label that fits nothing move the pick?
#
# The sixty cases of probe 2 run again over the same five labels in the same
# order, with one label appended that no case is about. The appended label is in
# added.txt and was chosen before any call went out.
#
#   sdlc/scripts/live probes/05-irrelevant-option/job.sh       records
#   OUT=DIR sh probes/05-irrelevant-option/job.sh --replay     replays, no network
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
labels=$(cat ../02-confidence/labels.txt added.txt | tr '\n' ' ')

: >"$out/added.jsonl"
while IFS= read -r case; do
	body=$(printf '%s\n' "$case" | jq -r '.body')
	# shellcheck disable=SC2086
	printf '%s' "$body" | thinkthen choose "$question" $labels \
		--details $folder >row.json && exit=0 || exit=$?
	# 0 is a label and 3 is unresolved. choose never exits 1.
	case "$exit" in
	0 | 3) ;;
	*)
		printf 'job: %s stopped with exit %s\n' "$(printf '%s\n' "$case" | jq -r .id)" "$exit" >&2
		exit "$exit"
		;;
	esac
	jq -c --argjson case "$case" \
		'{schema, value, input: $case, question, answer, threshold, meta}' \
		row.json >>"$out/added.jsonl"
done <../02-confidence/cases.jsonl

rm -f row.json
printf 'wrote %s rows to %s/added.jsonl\n' "$(wc -l <"$out/added.jsonl" | tr -d ' ')" "$out"

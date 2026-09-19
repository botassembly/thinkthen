#!/bin/sh
# Probe 2: does the backend's `confidence` separate right from wrong better
# than the winning option's probability?
#
# One `choose` per case over the five labels of labels.txt, in the order that
# file gives, with no threshold. The sweep happens later over the saved rows,
# so one run answers every cut on either number.
#
# Record mode is not built, so the loop is the shell's. Each row is the record
# row of specification/result.md, with the whole case under `input`.
#
#   sdlc/scripts/live probes/02-confidence/job.sh       records
#   OUT=DIR sh probes/02-confidence/job.sh --replay     replays, no network
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
question=$(cat question.txt)
labels=$(tr '\n' ' ' <labels.txt)

: >"$out/run.jsonl"
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
		row.json >>"$out/run.jsonl"
done <cases.jsonl

rm -f row.json
printf 'wrote %s rows to %s/run.jsonl\n' "$(wc -l <"$out/run.jsonl" | tr -d ' ')" "$out"

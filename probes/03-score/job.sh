#!/bin/sh
# Probe 3: how well does `score` place a text on five levels, and how does
# `choose` do over the same five labels on the same texts?
#
# Two runs over the forty cases in cases.jsonl.
#
#   score.jsonl   one `score` per case, levels lowest first.
#   choose.jsonl  one `choose` per case, the same five words as labels in the
#                 same order, so the two runs differ in the verb alone.
#
# Record mode is not built, so the loop is the shell's. Each row is the record
# row of specification/result.md, with the whole case under `input`.
#
#   sdlc/scripts/live --max-tokens 1000000 probes/03-score/job.sh records
#   OUT=DIR sh probes/03-score/job.sh --replay      replays, no network
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
levels=$(tr '\n' ' ' <levels.txt)

: >"$out/score.jsonl"
: >"$out/choose.jsonl"
while IFS= read -r case; do
	body=$(printf '%s\n' "$case" | jq -r '.body')
	id=$(printf '%s\n' "$case" | jq -r '.id')

	# shellcheck disable=SC2086
	printf '%s' "$body" | thinkthen score "$question" $levels \
		--details $folder >row.json || {
		exit=$?
		printf 'job: %s score stopped with exit %s\n' "$id" "$exit" >&2
		exit "$exit"
	}
	jq -c --argjson case "$case" \
		'{schema, value, input: $case, question, answer, threshold, meta}' \
		row.json >>"$out/score.jsonl"

	# shellcheck disable=SC2086
	printf '%s' "$body" | thinkthen choose "$question" $levels \
		--details $folder >row.json && exit=0 || exit=$?
	# 0 is a label and 3 is unresolved. choose never exits 1.
	case "$exit" in
	0 | 3) ;;
	*)
		printf 'job: %s choose stopped with exit %s\n' "$id" "$exit" >&2
		exit "$exit"
		;;
	esac
	jq -c --argjson case "$case" \
		'{schema, value, input: $case, question, answer, threshold, meta}' \
		row.json >>"$out/choose.jsonl"
done <cases.jsonl

rm -f row.json
printf 'score %s rows, choose %s rows\n' \
	"$(wc -l <"$out/score.jsonl" | tr -d ' ')" \
	"$(wc -l <"$out/choose.jsonl" | tr -d ' ')"

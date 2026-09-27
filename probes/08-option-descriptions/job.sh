#!/bin/sh
# Probe 8: does a description under each option pick better than a bare label?
#
# Two runs over the sixty picks in cases.jsonl, one question, five labels in one
# order.
#
#   bare.jsonl       the five labels alone, each sent with nothing under it.
#   described.jsonl  the same five labels, each with one sentence from
#                    question.json saying what the label covers.
#
#   sdlc/scripts/live --max-tokens 1000000 probes/08-option-descriptions/job.sh records
#   OUT=DIR sh probes/08-option-descriptions/job.sh --replay replays, no network
set -eu

cd -- "$(dirname -- "$0")"

mode=${1:-record}
out=${OUT:-runs}
rec=${REC:-recording}
case "$mode" in
# Recording and replaying one folder pays for a case once. A case the
# folder already holds is answered from it, and only a new case is sent.
record) folder="--record $rec/ --replay $rec/" ;;
--replay) folder="--replay $rec/" ;;
*)
	printf 'job: the one argument is --replay\n' >&2
	exit 2
	;;
esac

mkdir -p "$rec" "$out"
question=$(cat question.txt)

judge() {
	target=$1
	shift
	: >"$target"
	while IFS= read -r case; do
		printf '%s' "$(printf '%s\n' "$case" | jq -r .text)" |
			thinkthen choose "$@" --details --model jev-latest $folder >row.json && exit=0 || exit=$?
		# 0 is a label. 3 is unresolved, which `choose` gives on an exact tie
		# of the top two options, with or without a cut. Anything else stops.
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
	done <cases.jsonl
	printf 'wrote %s rows to %s\n' "$(wc -l <"$target" | tr -d ' ')" "$target"
}

judge "$out/bare.jsonl" "$question" billing shipping account product other
judge "$out/described.jsonl" @question.json
rm -f row.json

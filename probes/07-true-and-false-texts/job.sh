#!/bin/sh
# Probe 7: does saying what true and what false mean move the answer?
#
# Two runs over the forty cases in cases.jsonl, one question, one cut.
#
#   plain.jsonl  the question alone, which is the request the tool always sent.
#   texts.jsonl  the same question with `true` and `false` from question.json.
#
# Both runs put the whole case under `input`, so the two rows for one case carry
# the same `input` and the same `input.label`, which is what transforms/ reads.
#
#   sdlc/scripts/live --max-tokens 1000000 probes/07-true-and-false-texts/job.sh records
#   OUT=DIR sh probes/07-true-and-false-texts/job.sh --replay replays, no network
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
			thinkthen decide "$@" --details --model jev-latest $folder >row.json && exit=0 || exit=$?
		# 0 is yes and 1 is no. There is no band here, so 3 cannot arise.
		case "$exit" in
		0 | 1) ;;
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

judge "$out/plain.jsonl" "$question"
judge "$out/texts.jsonl" @question.json
rm -f row.json

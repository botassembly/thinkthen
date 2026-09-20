#!/bin/sh
# Probe 9, the text arm: evidence built from two pointers, sent the way the
# tool sends it today, as a JSON object flattened into one string.
#
# The object arm is object.sh, which posts the same request with `state` as a
# real JSON object. It opens its own connection, so it keeps no recording and
# lives outside runs/.
#
#   sdlc/scripts/live --max-tokens 1000000 probes/09-evidence-shape/job.sh records
#   OUT=DIR sh probes/09-evidence-shape/job.sh --replay  replays, no network
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
target=$out/text.jsonl
: >"$target"

while IFS= read -r case; do
	printf '%s\n' "$case" | thinkthen decide "$question" \
		--jsonl --field /subject --field /body \
		--details $folder >row.json && exit=0 || exit=$?
	if [ "$exit" != 0 ]; then
		printf 'job: %s stopped with exit %s\n' \
			"$(printf '%s\n' "$case" | jq -r .id)" "$exit" >&2
		exit "$exit"
	fi
	jq -c '{schema, value, input, question, answer, threshold, meta}' \
		row.json >>"$target"
done <cases.jsonl
rm -f row.json
printf 'wrote %s rows to %s\n' "$(wc -l <"$target" | tr -d ' ')" "$target"

#!/bin/sh
# Probe 1: does one request over many units pick as well as one request per unit?
#
# Three runs over the same twenty documents in docs.jsonl.
#
#   rank.jsonl       one `decide` per line, evidence is the line alone. A local
#                    sort over `answer.probability` is `rank --top 1`.
#   find.jsonl       one `choose` per document, evidence is the numbered
#                    document, options are the line ids.
#   find-none.jsonl  the same with a `none` option appended.
#
# Record mode is not built, so the loop is the shell's. Each row is the record
# row of specification/result.md: the result object with `input` holding the
# whole case. The rank rows carry `input.id` and a boolean `input.label`, so the
# yes/no recipes read them unchanged.
#
#   sdlc/scripts/live probes/01-find-vs-rank/job.sh      records
#   OUT=DIR sh probes/01-find-vs-rank/job.sh --replay    replays, no network
set -eu

cd -- "$(dirname -- "$0")"

mode=${1:-record}
out=${OUT:-runs}
docs=${DOCS:-docs.jsonl}
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
: >"$out/rank.jsonl"
: >"$out/find.jsonl"
: >"$out/find-none.jsonl"

# 0 is yes, 1 is no, 3 is unresolved. Anything else is a failure about the run.
answered() {
	case "$1" in
	0 | 1 | 3) return 0 ;;
	*) return 1 ;;
	esac
}

while IFS= read -r doc; do
	id=$(printf '%s\n' "$doc" | jq -r '.id')
	decide=$(printf '%s\n' "$doc" | jq -r '.decide')
	choose=$(printf '%s\n' "$doc" | jq -r '.choose')
	answer=$(printf '%s\n' "$doc" | jq -r '.answer // "none"')
	count=$(printf '%s\n' "$doc" | jq -r '.lines | length')

	# One decide per line, which is what rank asks of each record.
	line=1
	while [ "$line" -le "$count" ]; do
		unit=$(printf 'L%02d' "$line")
		text=$(printf '%s\n' "$doc" | jq -r --argjson n "$line" '.lines[$n - 1]')
		# shellcheck disable=SC2086
		printf '%s' "$text" | thinkthen decide "$decide" \
			--details $folder >row.json && exit=0 || exit=$?
		answered "$exit" || {
			printf 'job: %s %s stopped with exit %s\n' "$id" "$unit" "$exit" >&2
			exit "$exit"
		}
		jq -c --arg doc "$id" --arg unit "$unit" --arg text "$text" \
			--argjson label "$([ "$answer" = "$unit" ] && echo true || echo false)" \
			--argjson hard "$(printf '%s\n' "$doc" | jq '.hard')" \
			'{schema, value, input: {id: ($doc + "/" + $unit), doc: $doc, unit: $unit,
			   text: $text, label: $label, hard: $hard},
			  question, answer, threshold, meta}' \
			row.json >>"$out/rank.jsonl"
		line=$((line + 1))
	done

	# One choose over the whole numbered document, which is what find asks.
	numbered=$(printf '%s\n' "$doc" |
		jq -r '.lines | to_entries | map("L\(.key + 1 | tostring | if length == 1 then "0" + . else . end): \(.value)") | join("\n")')
	units=$(printf '%s\n' "$doc" |
		jq -r '.lines | to_entries | map("L\(.key + 1 | tostring | if length == 1 then "0" + . else . end)")[]')

	pick() {
		target=$1
		shift
		# shellcheck disable=SC2086
		printf '%s' "$numbered" | thinkthen choose "$choose" "$@" \
			--details $folder >row.json && exit=0 || exit=$?
		answered "$exit" || {
			printf 'job: %s choose stopped with exit %s\n' "$id" "$exit" >&2
			exit "$exit"
		}
		jq -c --argjson case "$doc" \
			'{schema, value, input: $case, question, answer, threshold, meta}' \
			row.json >>"$target"
	}

	# shellcheck disable=SC2086
	pick "$out/find.jsonl" $units
	# shellcheck disable=SC2086
	pick "$out/find-none.jsonl" $units none
done <"$docs"

rm -f row.json
printf 'rank %s rows, find %s rows, find-none %s rows\n' \
	"$(wc -l <"$out/rank.jsonl" | tr -d ' ')" \
	"$(wc -l <"$out/find.jsonl" | tr -d ' ')" \
	"$(wc -l <"$out/find-none.jsonl" | tr -d ' ')"

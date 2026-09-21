#!/bin/sh
set -eu

REPO=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

row() {
	id=$1 value=$2 answer=$3
	jq -n -c --arg id "$id" --argjson value "$value" --argjson answer "$answer" \
		'{input:{id:$id,body:"same",label:true},value:$value,
		  question:{text:"same"},threshold:0.5,meta:{model:"same"}} +
		 if $answer == null then {} else {answer:$answer} end'
}

{
	row z-number 1 '{"kind":"score"}'
	row a-null null null
	row m-string '"old"' '{"kind":"choice"}'
	row b-no-to-yes false '{"kind":"yes_no","probability":0.1}'
	row c-no-to-unresolved false '{"kind":"yes_no","probability":0.2}'
	row d-unresolved-to-no null '{"kind":"yes_no","probability":0.5}'
	row e-unresolved-to-yes null '{"kind":"yes_no","probability":0.5}'
	row f-yes-to-no true '{"kind":"yes_no","probability":0.9}'
	row g-yes-to-unresolved true '{"kind":"yes_no","probability":0.8}'
} > "$work/before.jsonl"

{
	row z-number 2 '{"kind":"score"}'
	row a-null null null
	row m-string '"new"' '{"kind":"choice"}'
	row b-no-to-yes true '{"kind":"yes_no","probability":0.17}'
	row c-no-to-unresolved null '{"kind":"yes_no","probability":0.28}'
	row d-unresolved-to-no false '{"kind":"yes_no","probability":0.59}'
	row e-unresolved-to-yes true '{"kind":"yes_no","probability":0.5}'
	row f-yes-to-no false '{"kind":"yes_no","probability":0.81}'
	row g-yes-to-unresolved null '{"kind":"yes_no","probability":0.8}'
} > "$work/after.jsonl"

jq -n --slurpfile before "$work/before.jsonl" -f "$REPO/transforms/compare/compare.jq" "$work/after.jsonl" \
	| jq -c '{compared,same,changed_values,flips,ids:[.changes[].id],partition:(.compared == .same + .changed_values)}' \
	| mustmatch '{"compared":9,"same":1,"changed_values":8,"flips":{"no to yes":["b-no-to-yes"],"no to unresolved":["c-no-to-unresolved"],"unresolved to no":["d-unresolved-to-no"],"unresolved to yes":["e-unresolved-to-yes"],"yes to no":["f-yes-to-no"],"yes to unresolved":["g-yes-to-unresolved"]},"ids":["b-no-to-yes","c-no-to-unresolved","d-unresolved-to-no","e-unresolved-to-yes","f-yes-to-no","g-yes-to-unresolved","m-string","z-number"],"partition":true}'

{
	row below true '{"kind":"yes_no","probability":0.1}'
	row at true '{"kind":"yes_no","probability":0.2}'
	row above true '{"kind":"yes_no","probability":0.3}'
	row same-at true '{"kind":"yes_no","probability":0.4}'
	row same-above false '{"kind":"yes_no","probability":0.5}'
	row still false '{"kind":"yes_no","probability":0.6}'
} > "$work/prob-before.jsonl"
{
	row below false '{"kind":"yes_no","probability":0.17}'
	row at false '{"kind":"yes_no","probability":0.28}'
	row above false '{"kind":"yes_no","probability":0.39}'
	row same-at true '{"kind":"yes_no","probability":0.48}'
	row same-above false '{"kind":"yes_no","probability":0.59}'
	row still false '{"kind":"yes_no","probability":0.61}'
} > "$work/prob-after.jsonl"

jq -n --slurpfile before "$work/prob-before.jsonl" -f "$REPO/transforms/compare/compare.jq" "$work/prob-after.jsonl" \
	| jq -c '{changes,yes_no_probability}' \
	| mustmatch '{"changes":[{"id":"above","before_value":true,"after_value":false,"before_probability":0.3,"after_probability":0.39,"probability_delta":0.09,"probability_delta_over_tolerance":true},{"id":"at","before_value":true,"after_value":false,"before_probability":0.2,"after_probability":0.28,"probability_delta":0.08,"probability_delta_over_tolerance":false},{"id":"below","before_value":true,"after_value":false,"before_probability":0.1,"after_probability":0.17,"probability_delta":0.07,"probability_delta_over_tolerance":false},{"id":"same-above","before_value":false,"after_value":false,"before_probability":0.5,"after_probability":0.59,"probability_delta":0.09,"probability_delta_over_tolerance":true}],"yes_no_probability":{"tolerance":0.08,"compared":6,"changed":6,"summarized_same_value":2,"largest_summarized_delta":0.08}}'

jq -n --argjson probability_tolerance 0 --slurpfile before "$work/prob-before.jsonl" \
	-f "$REPO/transforms/compare/compare.jq" "$work/prob-after.jsonl" \
	| jq -c '{ids:[.changes[].id],yes_no_probability}' \
	| mustmatch '{"ids":["above","at","below","same-above","same-at","still"],"yes_no_probability":{"tolerance":0,"compared":6,"changed":6,"summarized_same_value":0,"largest_summarized_delta":null}}'

for bad in null -0.1 1.1 '"0.08"' true; do
	if jq -n --argjson probability_tolerance "$bad" --slurpfile before "$work/prob-before.jsonl" \
		-f "$REPO/transforms/compare/compare.jq" "$work/prob-after.jsonl" > "$work/out" 2> "$work/error"
	then
		printf 'compare accepted invalid probability tolerance %s\n' "$bad" >&2
		exit 1
	fi
	grep -F 'compare: probability_tolerance must be a number from 0 through 1' "$work/error" > /dev/null
done

jq -n -c '{input:{id:"hostile probability id",body:"hostile probability body",label:true},value:true,
           answer:{kind:"yes_no",probability:{secret:"hostile probability answer"}},
           question:{text:"same"},threshold:0.5,meta:{model:"same"}}' > "$work/bad-before.jsonl"
row malformed true '{"kind":"yes_no","probability":0.5}' > "$work/bad-after.jsonl"
if jq -n --slurpfile before "$work/bad-before.jsonl" -f "$REPO/transforms/compare/compare.jq" \
	"$work/bad-after.jsonl" > "$work/out" 2> "$work/error"
then
	printf '%s\n' 'compare accepted a malformed yes_no answer' >&2
	exit 1
fi
grep -F 'compare: yes_no probability must be a number from 0 through 1' "$work/error" > /dev/null
for marker in 'hostile probability id' 'hostile probability body' 'hostile probability answer'; do
	if grep -F "$marker" "$work/error" > /dev/null; then
		printf 'compare echoed malformed answer marker: %s\n' "$marker" >&2
		exit 1
	fi
done

for kind in missing array object; do
	if [ "$kind" = missing ]; then
		jq -n -c --arg id "hostile $kind id" --arg body "hostile $kind body" \
			'{input:{id:$id,body:$body,label:true},question:{text:"same"},threshold:0.5,meta:{model:"same"}}' > "$work/bad-before.jsonl"
	else
		if [ "$kind" = array ]; then bad='[]'; else bad='{}'; fi
		jq -n -c --arg id "hostile $kind id" --arg body "hostile $kind body" --argjson value "$bad" \
			'{input:{id:$id,body:$body,label:true},value:$value,question:{text:"same"},threshold:0.5,meta:{model:"same"}}' > "$work/bad-before.jsonl"
	fi
	row bad true null > "$work/bad-after.jsonl"
	if jq -n --slurpfile before "$work/bad-before.jsonl" -f "$REPO/transforms/compare/compare.jq" \
		"$work/bad-after.jsonl" > "$work/out" 2> "$work/error"
	then
		printf 'compare accepted %s value\n' "$kind" >&2
		exit 1
	fi
	sed 's/^jq: error (at [^)]*): //' "$work/error" \
		| mustmatch 'compare: value must be null, boolean, string, or number'
	for marker in "hostile $kind id" "hostile $kind body"; do
		if grep -F "$marker" "$work/error" > /dev/null; then
			printf 'compare echoed invalid value marker: %s\n' "$marker" >&2
			exit 1
		fi
	done
done

printf '%s\n' 'compare transform tests pass'

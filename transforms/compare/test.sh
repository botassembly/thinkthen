#!/bin/sh
set -eu

REPO=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
. "$REPO/sdlc/scripts/scratch.sh"
scratch_dir work

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

# The transform has no meaningful primary input. A slurped or ordinary input
# otherwise looks like a valid but empty later run.
printf '%s\n' null > "$work/null.json"
for invocation in slurp ordinary literal-null; do
	case "$invocation" in
		slurp) command="jq -s --slurpfile before $work/before.jsonl -f $REPO/transforms/compare/compare.jq $work/after.jsonl" ;;
		ordinary) command="jq --slurpfile before $work/before.jsonl -f $REPO/transforms/compare/compare.jq $work/after.jsonl" ;;
		literal-null) command="jq --slurpfile before $work/before.jsonl -f $REPO/transforms/compare/compare.jq $work/null.json" ;;
	esac
	if sh -c "$command" > "$work/out" 2> "$work/error"; then
		printf 'compare accepted %s primary input\n' "$invocation" >&2
		exit 1
	fi
	mustmatch 'compare: run with jq -n' < "$work/error"
done

head -n 2 "$work/after.jsonl" > "$work/two-rows.jsonl"
if jq --slurpfile before "$work/before.jsonl" -f "$REPO/transforms/compare/compare.jq" \
	"$work/two-rows.jsonl" > "$work/out" 2> "$work/error"
then
	printf '%s\n' 'compare accepted two-row ordinary input' >&2
	exit 1
fi
[ "$(grep -Fxc 'compare: run with jq -n' "$work/error")" -eq 1 ]

annotate_rows() {
	phase=$1
	jq -n -c --arg phase "$phase" '
def question($text): {text:$text};
def answer($kind; $value; $probability):
  {value:$value, question:question($kind + " question"), threshold:(if $kind == "yes_no" then 0.5 else null end),
   answer:({kind:$kind} + if $kind == "yes_no" then {probability:$probability} else {} end)};
def row($id; $body; $label; $decide; $probability; $choose; $tag; $score):
  {input:{id:$id,body:$body,label:$label},
   value:{decide:$decide,choose:$choose,tag:$tag,score:$score},
   answers:{decide:answer("yes_no";$decide;$probability),
            choose:answer("choice";$choose;null),
            tag:answer("tag";$tag;null),
            score:answer("score";$score;null)},
   meta:{model:(if $phase == "before" then "old-model" else "new-model" end),
         questions_sha256:(if $phase == "before"
           then "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
           else "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" end)}};
if $phase == "before" then
  [row("a";"same";true;false;0.10;"old";["a","b"];1),
   row("b";"same";true;false;0.20;"same";["same"];2),
   row("c";"same";true;null;0.50;"same";["same"];3),
   row("d";"same";true;null;0.50;"same";["same"];4),
   row("e";"same";true;true;0.90;"same";["same"];5),
   row("f";"same";true;true;0.80;"same";["same"];6),
   row("input";"old";true;true;0.9;"old";["old"];7),
   row("label";"same";true;true;0.9;"old";["old"];8),
   row("gone";"same";true;true;0.9;"old";["old"];9),
   row("repeat";"same";true;true;0.9;"old";["old"];10),
   row("repeat";"same";true;true;0.9;"old";["old"];10)]
else
  [row("a";"same";true;true;0.17;"new";["b","a"];11),
   row("b";"same";true;null;0.28;"same";["same"];2),
   row("c";"same";true;false;0.59;"same";["same"];3),
   row("d";"same";true;true;0.50;"same";["same"];4),
   row("e";"same";true;false;0.81;"same";["same"];5),
   row("f";"same";true;null;0.80;"same";["same"];6),
   row("input";"new";true;false;0.1;"new";["new"];70),
   row("label";"same";false;false;0.1;"new";["new"];80),
   row("new";"same";true;true;0.9;"new";["new"];90),
   row("repeat";"same";true;true;0.9;"new";["new"];100),
   row("repeat";"same";true;true;0.9;"new";["new"];100)]
end | .[]'
}

annotate_rows before > "$work/annotate-before.jsonl"
annotate_rows after > "$work/annotate-after.jsonl"

jq -n --slurpfile before "$work/annotate-before.jsonl" -f "$REPO/transforms/compare/compare.jq" "$work/annotate-after.jsonl" \
	| jq -c '{mode,before,after,changed,repeated_ids,only_in_before,only_in_after,paired,compared,
	          mismatched_input,mismatched_label,questions_only_in_before,questions_only_in_after,
	          question_names:(.questions|keys),
	          decide:(.questions.decide|{compared,same,changed_values,flips,ids:[.changes[].id],yes_no_probability}),
	          choose:(.questions.choose|{compared,same,changed_values,ids:[.changes[].id],yes_no_probability}),
	          tag:(.questions.tag|{compared,same,changed_values,ids:[.changes[].id],yes_no_probability}),
	          score:(.questions.score|{compared,same,changed_values,ids:[.changes[].id],yes_no_probability})}' \
	| mustmatch '{"mode":"annotate","before":{"rows":11,"question_sets":["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],"models":["old-model"]},"after":{"rows":11,"question_sets":["bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"],"models":["new-model"]},"changed":{"question_set":true,"model":true},"repeated_ids":{"before":["repeat"],"after":["repeat"]},"only_in_before":["gone"],"only_in_after":["new"],"paired":8,"compared":6,"mismatched_input":["input"],"mismatched_label":["label"],"questions_only_in_before":[],"questions_only_in_after":[],"question_names":["choose","decide","score","tag"],"decide":{"compared":6,"same":0,"changed_values":6,"flips":{"no to yes":["a"],"no to unresolved":["b"],"unresolved to no":["c"],"unresolved to yes":["d"],"yes to no":["e"],"yes to unresolved":["f"]},"ids":["a","b","c","d","e","f"],"yes_no_probability":{"tolerance":0.08,"compared":6,"changed":4,"summarized_same_value":0,"largest_summarized_delta":null}},"choose":{"compared":6,"same":5,"changed_values":1,"ids":["a"],"yes_no_probability":{"tolerance":0.08,"compared":0,"changed":0,"summarized_same_value":0,"largest_summarized_delta":null}},"tag":{"compared":6,"same":5,"changed_values":1,"ids":["a"],"yes_no_probability":{"tolerance":0.08,"compared":0,"changed":0,"summarized_same_value":0,"largest_summarized_delta":null}},"score":{"compared":6,"same":5,"changed_values":1,"ids":["a"],"yes_no_probability":{"tolerance":0.08,"compared":0,"changed":0,"summarized_same_value":0,"largest_summarized_delta":null}}}'

jq -n --slurpfile before "$work/annotate-before.jsonl" -f "$REPO/transforms/compare/compare.jq" "$work/annotate-after.jsonl" \
	| jq -c '[.questions.decide.changes[] | select(.id == "a" or .id == "b" or .id == "c")
	          | {id,probability_delta,probability_delta_over_tolerance}]' \
	| mustmatch '[{"id":"a","probability_delta":0.07,"probability_delta_over_tolerance":false},{"id":"b","probability_delta":0.08,"probability_delta_over_tolerance":false},{"id":"c","probability_delta":0.09,"probability_delta_over_tolerance":true}]'

# Each named question reports definition and threshold changes independently.
jq -c '(.answers.decide.question.text) = "revised question" | (.answers.decide.threshold) = 0.6' \
	"$work/annotate-after.jsonl" > "$work/annotate-after-revised.jsonl"
jq -n --slurpfile before "$work/annotate-before.jsonl" -f "$REPO/transforms/compare/compare.jq" "$work/annotate-after-revised.jsonl" \
	| jq -c '{decide:.questions.decide.changed,choose:.questions.choose.changed}' \
	| mustmatch '{"decide":{"question":true,"threshold":true},"choose":{"question":false,"threshold":false}}'

# Question names present in one whole run are reported once and compared nowhere.
jq -c 'del(.value.score,.answers.score)' "$work/annotate-after.jsonl" > "$work/annotate-after-without-score.jsonl"
jq -n --slurpfile before "$work/annotate-before.jsonl" -f "$REPO/transforms/compare/compare.jq" "$work/annotate-after-without-score.jsonl" \
	| jq -c '{questions_only_in_before,questions_only_in_after,question_names:(.questions|keys)}' \
	| mustmatch '{"questions_only_in_before":["score"],"questions_only_in_after":[],"question_names":["choose","decide","tag"]}'

# One empty side adopts annotate mode and makes the question-set change unknown.
: > "$work/empty.jsonl"
jq -n --slurpfile before "$work/annotate-before.jsonl" -f "$REPO/transforms/compare/compare.jq" "$work/empty.jsonl" \
	| jq -c '{mode,changed,questions_only_in_before,questions}' \
	| mustmatch '{"mode":"annotate","changed":{"question_set":null,"model":true},"questions_only_in_before":["choose","decide","score","tag"],"questions":{}}'

# Every malformed annotate diagnostic is fixed and echoes no row field.
for mutation in \
	'del(.value)' \
	'del(.answers)' \
	'del(.answers.score)' \
	'del(.answers.decide.threshold)' \
	'.answers.score.value = 99' \
	'.input.id = 7' \
	'.meta.model = null' \
	'.answers.decide.question = "hostile question text"' \
	'.answers.decide.threshold = []' \
	'.answers.decide.answer = {kind:"unknown",secret:"hostile answer"}' \
	'.meta.questions_sha256 = "bad"' \
	'.value.tag = ["ok",7]'; do
	jq -c --arg marker 'hostile evidence' ".input.body = \$marker | $mutation" "$work/annotate-before.jsonl" > "$work/bad-annotate.jsonl"
	if jq -n --slurpfile before "$work/bad-annotate.jsonl" -f "$REPO/transforms/compare/compare.jq" \
		"$work/annotate-after.jsonl" > "$work/out" 2> "$work/error"
	then
		printf 'compare accepted malformed annotate rows: %s\n' "$mutation" >&2
		exit 1
	fi
	sed 's/^jq: error (at [^)]*): //' "$work/error" | mustmatch 'compare: invalid annotate row'
	for marker in 'hostile evidence' 'hostile question text' 'hostile answer'; do
		if grep -F "$marker" "$work/error" >/dev/null; then
			printf 'compare echoed malformed annotate marker: %s\n' "$marker" >&2
			exit 1
		fi
	done
done

# A missing nested value cannot masquerade as an explicit top-level null.
jq -c '.value.decide = null | del(.answers.decide.value)' \
	"$work/annotate-before.jsonl" > "$work/bad-annotate.jsonl"
if jq -n --slurpfile before "$work/bad-annotate.jsonl" -f "$REPO/transforms/compare/compare.jq" \
	"$work/annotate-after.jsonl" > "$work/out" 2> "$work/error"
then
	printf '%s\n' 'compare accepted a missing nested null value' >&2
	exit 1
fi
sed 's/^jq: error (at [^)]*): //' "$work/error" | mustmatch 'compare: invalid annotate row'

# Scalar and annotate rows cannot share one invocation.
{ head -n 1 "$work/annotate-before.jsonl"; row scalar true null; } > "$work/mixed.jsonl"
if jq -n --slurpfile before "$work/mixed.jsonl" -f "$REPO/transforms/compare/compare.jq" \
	"$work/annotate-after.jsonl" > "$work/out" 2> "$work/error"
then
	printf '%s\n' 'compare accepted mixed row shapes' >&2
	exit 1
fi
sed 's/^jq: error (at [^)]*): //' "$work/error" | mustmatch 'compare: rows must all be scalar or annotate'

printf '%s\n' 'compare transform tests pass'

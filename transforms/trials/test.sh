#!/bin/sh
set -eu
cd -- "$(dirname -- "$0")"

. ../../sdlc/scripts/scratch.sh
scratch_dir work

yes_row='def row($id;$label;$probability;$threshold;$replayed;$value):
  {schema:"thinkthen.result/1",input:{id:$id,label:$label,body:"fixture"},value:$value,
   question:{verb:"decide",text:"Fixture question"},
   answer:{kind:"yes_no",probability:$probability},threshold:$threshold,
   meta:{tool:"thinkthen 0.0.1",question_sha256:("a"*64),url:"https://example.invalid",model:"fixture",replayed:$replayed,
         usage:{input_tokens:1,output_tokens:1}}};'

jq -n -c "$yes_row
  row(\"many\";true;0.4;0.6;false;false),
  row(\"band-low\";false;0.125;\"0.25:0.75\";false;false),
  row(\"band-inside\";true;0.25;\"0.25:0.75\";false;false),
  row(\"band-high\";true;0.625;\"0.25:0.75\";false;false),
  row(\"one\";false;0.49;0.5;false;false),
  row(\"many\";true;0.6;0.6;true;true),
  row(\"band-low\";false;0.375;\"0.25:0.75\";true;false),
  row(\"band-inside\";true;0.75;\"0.25:0.75\";true;true),
  row(\"band-high\";true;0.875;\"0.25:0.75\";true;true),
  row(\"many\";true;0.8;0.6;false;true)" > "$work/yes.jsonl"

jq -n -c -f trials.jq "$work/yes.jsonl" > "$work/yes-derived.jsonl"
test "$(wc -l < "$work/yes-derived.jsonl")" -eq 5
jq -c '{schema,input,value,question,answer,threshold,trials,meta,has_usage:(.meta|has("usage")),has_replayed:(.meta|has("replayed"))}' \
  "$work/yes-derived.jsonl" > "$work/yes-report.jsonl"
cat > "$work/yes-expected.jsonl" <<'EOF'
{"schema":"thinkthen.trials/1","input":{"id":"many","label":true,"body":"fixture"},"value":true,"question":{"verb":"decide","text":"Fixture question"},"answer":{"kind":"yes_no","probability":0.6},"threshold":0.6,"trials":{"count":3,"live":2,"replayed":1},"meta":{"tool":"thinkthen 0.0.1","question_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","url":"https://example.invalid","model":"fixture"},"has_usage":false,"has_replayed":false}
{"schema":"thinkthen.trials/1","input":{"id":"band-low","label":false,"body":"fixture"},"value":false,"question":{"verb":"decide","text":"Fixture question"},"answer":{"kind":"yes_no","probability":0.25},"threshold":"0.25:0.75","trials":{"count":2,"live":1,"replayed":1},"meta":{"tool":"thinkthen 0.0.1","question_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","url":"https://example.invalid","model":"fixture"},"has_usage":false,"has_replayed":false}
{"schema":"thinkthen.trials/1","input":{"id":"band-inside","label":true,"body":"fixture"},"value":null,"question":{"verb":"decide","text":"Fixture question"},"answer":{"kind":"yes_no","probability":0.5},"threshold":"0.25:0.75","trials":{"count":2,"live":1,"replayed":1},"meta":{"tool":"thinkthen 0.0.1","question_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","url":"https://example.invalid","model":"fixture"},"has_usage":false,"has_replayed":false}
{"schema":"thinkthen.trials/1","input":{"id":"band-high","label":true,"body":"fixture"},"value":true,"question":{"verb":"decide","text":"Fixture question"},"answer":{"kind":"yes_no","probability":0.75},"threshold":"0.25:0.75","trials":{"count":2,"live":1,"replayed":1},"meta":{"tool":"thinkthen 0.0.1","question_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","url":"https://example.invalid","model":"fixture"},"has_usage":false,"has_replayed":false}
{"schema":"thinkthen.trials/1","input":{"id":"one","label":false,"body":"fixture"},"value":false,"question":{"verb":"decide","text":"Fixture question"},"answer":{"kind":"yes_no","probability":0.49},"threshold":0.5,"trials":{"count":1,"live":1,"replayed":0},"meta":{"tool":"thinkthen 0.0.1","question_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","url":"https://example.invalid","model":"fixture"},"has_usage":false,"has_replayed":false}
EOF
cmp "$work/yes-expected.jsonl" "$work/yes-report.jsonl"

jq -n -c '
  def row($id;$probabilities;$threshold;$replayed):
    {schema:"thinkthen.result/1",input:{id:$id,label:"blue"},value:"private",
     question:{verb:"choose",text:"Fixture choice",options:["red","blue","other"]},
     answer:{kind:"choice",pick:"private",probabilities:$probabilities},threshold:$threshold,
     meta:{tool:"thinkthen 0.0.1",question_sha256:("b"*64),url:"https://example.invalid",model:"fixture",replayed:$replayed}};
  row("winner";{red:0.5,blue:0.25,other:0.25};null;false),
  row("tie";{red:0.75,blue:0.25,other:0};null;false),
  row("cut";{red:0.5,blue:0.25,other:0.25};0.6;false),
  row("winner";{red:0.25,blue:0.75,other:0};null;true),
  row("tie";{red:0.25,blue:0.75,other:0};null;true),
  row("cut";{red:0.25,blue:0.75,other:0};0.6;true)
' > "$work/choice.jsonl"
jq -n -c -f trials.jq "$work/choice.jsonl" \
  | jq -c '{id:.input.id,value,answer,threshold,trials}' > "$work/choice-report.jsonl"
cat > "$work/choice-expected.jsonl" <<'EOF'
{"id":"winner","value":"blue","answer":{"kind":"choice","pick":"blue","probabilities":{"red":0.375,"blue":0.5,"other":0.125}},"threshold":null,"trials":{"count":2,"live":1,"replayed":1}}
{"id":"tie","value":null,"answer":{"kind":"choice","pick":"red","probabilities":{"red":0.5,"blue":0.5,"other":0}},"threshold":null,"trials":{"count":2,"live":1,"replayed":1}}
{"id":"cut","value":null,"answer":{"kind":"choice","pick":"blue","probabilities":{"red":0.375,"blue":0.5,"other":0.125}},"threshold":0.6,"trials":{"count":2,"live":1,"replayed":1}}
EOF
cmp "$work/choice-expected.jsonl" "$work/choice-report.jsonl"

jq -n -c '
  def row($probabilities;$replayed):
    {schema:"thinkthen.result/1",input:{id:"score",label:1},value:99,
     question:{verb:"score",text:"Fixture score",levels:["low","middle","high"]},
     answer:{kind:"score",level:"private",probabilities:$probabilities},threshold:null,
     meta:{tool:"thinkthen 0.0.1",question_sha256:("c"*64),url:"https://example.invalid",model:"fixture",replayed:$replayed}};
  row({low:0.5,middle:0.25,high:0.25};false),
  row({low:0.25,middle:0.625,high:0.125};true)
' > "$work/score.jsonl"
jq -n -c -f trials.jq "$work/score.jsonl" \
  | jq -c '{value,answer,trials}' > "$work/score-report.json"
printf '%s\n' '{"value":0.8125,"answer":{"kind":"score","level":"middle","probabilities":{"low":0.375,"middle":0.4375,"high":0.1875}},"trials":{"count":2,"live":1,"replayed":1}}' \
  > "$work/score-expected.json"
cmp "$work/score-expected.json" "$work/score-report.json"

jq -n -c '
  def row($id;$provenance):
    {schema:"thinkthen.result/1",input:{id:$id},value:true,
     question:{verb:"decide",text:"Fixture question"},
     answer:{kind:"yes_no",probability:0.9},threshold:0.5,
     meta:({tool:"thinkthen 0.0.1",question_sha256:("e"*64),url:"https://example.invalid",model:"fixture"} + $provenance)};
  row("new-stored";{cached:true}),
  row("new-live";{cached:false}),
  row("old-stored";{replayed:true}),
  row("old-live";{replayed:false}),
  row("both-stored-wins";{cached:true,replayed:false}),
  row("both-live-wins";{cached:false,replayed:true}),
  row("legacy-ignored";{cached:true,replayed:"junk"})
' > "$work/provenance.jsonl"
jq -n -c -f trials.jq "$work/provenance.jsonl" \
  | jq -c '{id:.input.id,trials}' > "$work/provenance-report.jsonl"
cat > "$work/provenance-expected.jsonl" <<'EOF'
{"id":"new-stored","trials":{"count":1,"live":0,"replayed":1}}
{"id":"new-live","trials":{"count":1,"live":1,"replayed":0}}
{"id":"old-stored","trials":{"count":1,"live":0,"replayed":1}}
{"id":"old-live","trials":{"count":1,"live":1,"replayed":0}}
{"id":"both-stored-wins","trials":{"count":1,"live":0,"replayed":1}}
{"id":"both-live-wins","trials":{"count":1,"live":1,"replayed":0}}
{"id":"legacy-ignored","trials":{"count":1,"live":0,"replayed":1}}
EOF
cmp "$work/provenance-expected.jsonl" "$work/provenance-report.jsonl"

jq -n -c '
  [range(0;255) | "option-\(.)"] as $options
  | reduce $options[] as $name ({}; .[$name]=0)
  | .[$options[0]]=1 | .[$options[1]]=0.01000000000002
  | {schema:"thinkthen.result/1",input:{id:"tolerance"},value:null,
     question:{verb:"choose",text:"Fixture choice",options:$options},
     answer:{kind:"choice",pick:$options[0],probabilities:.},threshold:null,
     meta:{tool:"thinkthen 0.0.1",question_sha256:("d"*64),url:"https://example.invalid",model:"fixture",replayed:false}}
' > "$work/tolerance.jsonl"
jq -n -c -f trials.jq "$work/tolerance.jsonl" \
  | jq -e '.trials.count == 1 and .answer.pick == "option-0"' > /dev/null

expect_failure() {
	name=$1
	expected=$2
	file=$3
	if jq -n -c -f trials.jq "$file" > "$work/out" 2> "$work/error"; then
		echo "trials accepted $name" >&2
		exit 1
	fi
	sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
	printf '%s\n' "$expected" > "$work/expected-message"
	if ! cmp "$work/expected-message" "$work/message"; then
		echo "trials returned the wrong message for $name" >&2
		exit 1
	fi
	if grep -q 'private' "$work/error"; then
		echo "trials echoed hostile row data for $name" >&2
		exit 1
	fi
}

private_row=$(head -n 1 "$work/yes.jsonl")
printf '%s\n' '[]' > "$work/bad.jsonl"
expect_failure container 'trials: every row must be an object' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .schema="private"' > "$work/bad.jsonl"
expect_failure schema 'trials: every row must use schema thinkthen.result/1' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c 'del(.input.id)' > "$work/bad.jsonl"
expect_failure missing-id 'trials: every row must carry a string input id' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id=7' > "$work/bad.jsonl"
expect_failure id 'trials: every row must carry a string input id' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .question.text=7' > "$work/bad.jsonl"
expect_failure question 'trials: every row must carry a valid question' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .answer.probability=2' > "$work/bad.jsonl"
expect_failure probability 'trials: every row must carry valid probabilities' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .threshold="private"' > "$work/bad.jsonl"
expect_failure threshold 'trials: every row must carry a valid threshold' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .meta.model=7' > "$work/bad.jsonl"
expect_failure metadata 'trials: every row must carry valid metadata' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .meta.cached=null' > "$work/bad.jsonl"
expect_failure canonical-null 'trials: every row must carry valid metadata' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .meta.cached="yes"' > "$work/bad.jsonl"
expect_failure canonical-string 'trials: every row must carry valid metadata' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | del(.meta.replayed)' > "$work/bad.jsonl"
expect_failure no-provenance 'trials: every row must carry valid metadata' "$work/bad.jsonl"
printf '%s\n' "$private_row" | jq -c '.input.id="private" | .answer.kind="tag"' > "$work/bad.jsonl"
expect_failure kind 'trials: answer kind must be yes_no, choice, or score and match the question verb' "$work/bad.jsonl"
head -n 1 "$work/choice.jsonl" | jq -c '.input.id="private" | .question.options[0]=" "' > "$work/bad.jsonl"
expect_failure choice-name 'trials: every row must carry valid probabilities' "$work/bad.jsonl"
head -n 1 "$work/choice.jsonl" | jq -c '.input.id="private" | .answer.probabilities.private=0' > "$work/bad.jsonl"
expect_failure choice-keys 'trials: every row must carry valid probabilities' "$work/bad.jsonl"
head -n 1 "$work/choice.jsonl" | jq -c '.input.id="private" | .answer.probabilities={red:0.4,blue:0.4,other:0}' > "$work/bad.jsonl"
expect_failure choice-total 'trials: every row must carry valid probabilities' "$work/bad.jsonl"
head -n 1 "$work/score.jsonl" | jq -c '.input.id="private" | .question.levels=["same","same"] | .answer.probabilities={same:1}' > "$work/bad.jsonl"
expect_failure score-levels 'trials: every row must carry valid probabilities' "$work/bad.jsonl"
printf '%s\n%s\n' "$private_row" "$(printf '%s\n' "$private_row" | jq -c '.answer.probability=0.7 | .question.text="private"')" > "$work/bad.jsonl"
expect_failure facts 'trials: rows for one id must share their case, question, threshold, and run facts' "$work/bad.jsonl"
printf '%s\n' "$private_row" > "$work/mixed.jsonl"
head -n 1 "$work/choice.jsonl" >> "$work/mixed.jsonl"
expect_failure mixed 'trials: one input must carry one answer kind' "$work/mixed.jsonl"

if jq -s -c -f trials.jq "$work/yes.jsonl" > "$work/out" 2> "$work/error"; then
	echo 'trials accepted ordinary input' >&2
	exit 1
fi
printf '%s\n' 'trials: run with jq -n' > "$work/expected-message"
cmp "$work/expected-message" "$work/error"

duplicate_guard='metric: repeated case ids; run trials.jq first'
for metric in counts calibration; do
	if jq -n -f "../$metric/$metric.jq" "$work/yes.jsonl" > "$work/out" 2> "$work/error"; then
		echo "$metric accepted repeated ids" >&2
		exit 1
	fi
	sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
	printf '%s\n' "$duplicate_guard" > "$work/expected-message"
	cmp "$work/expected-message" "$work/message"
done
if jq -n --argjson cut 0.5 -f ../score/score.jq "$work/yes.jsonl" > "$work/out" 2> "$work/error"; then exit 1; fi
sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
printf '%s\n' "$duplicate_guard" > "$work/expected-message"
cmp "$work/expected-message" "$work/message"

jq -n -c "$yes_row row(7;true;0.6;0.5;false;true), row(7;true;0.7;0.5;false;true)" \
  > "$work/numeric-duplicate.jsonl"
if jq -n -f ../counts/counts.jq "$work/numeric-duplicate.jsonl" > "$work/out" 2> "$work/error"; then
	echo 'counts accepted a repeated numeric id' >&2
	exit 1
fi
sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
cmp "$work/expected-message" "$work/message"
if jq -n --argjson band '[0.2,0.8]' -f ../band/band.jq "$work/yes.jsonl" > "$work/out" 2> "$work/error"; then exit 1; fi
sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
cmp "$work/expected-message" "$work/message"
if jq -n -f ../sweep/sweep.jq "$work/yes.jsonl" > "$work/out" 2> "$work/error"; then exit 1; fi
sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
cmp "$work/expected-message" "$work/message"

for metric in counts calibration; do
	jq -n -f "../$metric/$metric.jq" "$work/yes-derived.jsonl" > /dev/null
done
jq -n --argjson cut 0.5 -f ../score/score.jq "$work/yes-derived.jsonl" > /dev/null
jq -n --argjson band '[0.2,0.8]' -f ../band/band.jq "$work/yes-derived.jsonl" > /dev/null
jq -n -f ../sweep/sweep.jq "$work/yes-derived.jsonl" > /dev/null

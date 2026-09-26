#!/bin/sh
set -eu
cd -- "$(dirname -- "$0")"

. ../../sdlc/scripts/scratch.sh
scratch_dir work

jq -n -f sweep.jq ../rows/runs/run-a.jsonl > "$work/decision.json"
cmp decision-expected.json "$work/decision.json"
: > "$work/empty.jsonl"
jq -n -f sweep.jq "$work/empty.jsonl" > "$work/decision-empty.json"
cmp decision-empty-expected.json "$work/decision-empty.json"
sh grouped-test.sh
sh human-label-test.sh

jq -n -c '
  def row($id; $label; $value; $pick; $red; $blue):
    {input:{id:$id,label:$label}, value:$value,
     question:{verb:"choose",options:["red","blue"]},
     answer:{kind:"choice",pick:$pick,probabilities:{red:$red,blue:$blue}}};
  row("above";"red";"red";"red";0.8;0.2),
  row("tie";"blue";null;"red";0.5;0.5),
  row("below";"blue";"blue";"blue";0.45;0.55),
  row("wrong";"red";null;"blue";0.1;0.9),
  row("unlabeled";"private label";"red";"red";0.95;0.05)
' > "$work/choose.jsonl"

jq -n -f sweep.jq "$work/choose.jsonl" \
  | jq -c '{mode,rows,labeled,unlabeled,options,has_pick:has("pick"),
            at_half:(.sweep[]|select(.cut==0.5)),
            at_fifty_five:(.sweep[]|select(.cut==0.55)),
            at_six:(.sweep[]|select(.cut==0.6)),
            at_ninety_five:(.sweep[]|select(.cut==0.95)),
            tied_neighbors:([.sweep[]|select(.cut==0.6 or .cut==0.65)|del(.cut)]|.[0]==.[1])}' \
  > "$work/choose-report.json"
cat > "$work/choose-expected.json" <<'EOF'
{"mode":"choose","rows":5,"labeled":4,"unlabeled":["unlabeled"],"options":["red","blue"],"has_pick":false,"at_half":{"cut":0.5,"resolved":3,"unresolved":1,"ties":1,"coverage":0.75,"accuracy_resolved":0.6667,"accuracy_unresolved":null},"at_fifty_five":{"cut":0.55,"resolved":3,"unresolved":1,"ties":1,"coverage":0.75,"accuracy_resolved":0.6667,"accuracy_unresolved":null},"at_six":{"cut":0.6,"resolved":2,"unresolved":2,"ties":1,"coverage":0.5,"accuracy_resolved":0.5,"accuracy_unresolved":1},"at_ninety_five":{"cut":0.95,"resolved":0,"unresolved":4,"ties":1,"coverage":0,"accuracy_resolved":null,"accuracy_unresolved":0.6667},"tied_neighbors":true}
EOF
cmp "$work/choose-expected.json" "$work/choose-report.json"

jq -n -c '
  def row($id; $label; $value):
    {input:{id:$id,label:$label}, value:$value,
     question:{verb:"score",levels:["low","medium","high"]},
     answer:{kind:"score"}};
  row("low";0;0.4), row("middle";1;1), row("high";2;2),
  row("wrong";0;2), row("unlabeled";3;1)
' > "$work/score.jsonl"

jq -n -f sweep.jq "$work/score.jsonl" \
  | jq -c '{mode,rows,labeled,unlabeled,levels,has_pick:has("pick"),sweep}' \
  > "$work/score-report.json"
cat > "$work/score-expected.json" <<'EOF'
{"mode":"score","rows":5,"labeled":4,"unlabeled":["unlabeled"],"levels":["low","medium","high"],"has_pick":false,"sweep":[{"cut":1,"accuracy":0.75,"precision":0.6667,"recall":1,"f1":0.8},{"cut":2,"accuracy":0.75,"precision":0.5,"recall":1,"f1":0.6667}]}
EOF
cmp "$work/score-expected.json" "$work/score-report.json"

expect_failure() {
  name=$1
  expected=$2
  filter=$3
  if jq -n -c "$filter" > "$work/bad.jsonl" && jq -n -f sweep.jq "$work/bad.jsonl" > "$work/out" 2> "$work/error"; then
    echo "sweep accepted $name" >&2
    exit 1
  fi
  sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
  printf '%s\n' "$expected" > "$work/expected-message"
  cmp "$work/expected-message" "$work/message"
  if grep -q 'private' "$work/error"; then
    echo "sweep echoed hostile row data for $name" >&2
    exit 1
  fi
}

choice='{"input":{"id":"private id","label":"red"},"value":"red","question":{"options":["red","blue"]},"answer":{"kind":"choice","pick":"red","probabilities":{"red":0.8,"blue":0.2}}}'
score='{"input":{"id":"private id","label":0},"value":1,"question":{"levels":["low","high"]},"answer":{"kind":"score"}}'
decision='{"input":{"id":"private id","label":true},"value":true,"question":{},"answer":{"kind":"yes_no","probability":0.8}}'

expect_failure top-level 'sweep: every row must be an object' '[]'
expect_failure answer 'sweep: every row must carry an answer object' "$choice | .answer=[]"
expect_failure decision-input 'sweep: decision rows must carry an input object' "$decision | .input=[]"
expect_failure decision-question 'sweep: decision rows must carry a question object' "$decision | .question=[]"
expect_failure decision-probability 'sweep: decision rows must carry a numeric probability' "$decision | del(.answer.probability)"
expect_failure choice-input 'sweep: choice rows must carry an input object' "$choice | .input=[]"
expect_failure choice-question 'sweep: choice rows must carry a question object' "$choice | .question=[]"
expect_failure score-input 'sweep: score rows must carry an input object' "$score | .input=[]"
expect_failure score-question 'sweep: score rows must carry a question object' "$score | .question=[]"
expect_failure mixed 'sweep: one run must carry one answer kind' "$choice, ($score | .input.id=\"other\")"
expect_failure options 'sweep: choice rows must carry one shared ordered option list' "$choice, ($choice | .input.id=\"other\" | .question.options=[\"red\",\"private\"] | .answer.probabilities={red:0.8,private:0.2})"
expect_failure keys 'sweep: choice probabilities must have exactly the option keys' "$choice | .answer.probabilities += {private:0}"
expect_failure missing-key 'sweep: choice probabilities must have exactly the option keys' "$choice | del(.answer.probabilities.blue)"
expect_failure member 'sweep: choice probabilities must be numbers from zero through one' "$choice | .answer.probabilities.red=\"private\""
expect_failure member-low 'sweep: choice probabilities must be numbers from zero through one' "$choice | .answer.probabilities.red=-0.01"
expect_failure member-high 'sweep: choice probabilities must be numbers from zero through one' "$choice | .answer.probabilities.red=1.01"
expect_failure leader 'sweep: a choice pick must be the first option at the maximum probability' "$choice | .answer.pick=\"blue\""
expect_failure value 'sweep: a non-null choice value must equal its pick' "$choice | .value=\"blue\""
expect_failure levels 'sweep: score rows must carry one shared ordered level list' "$score, ($score | .input.id=\"other\" | .question.levels=[\"low\",\"private\"])"
expect_failure score-value 'sweep: score values must be numbers from zero through the last level' "$score | .value=\"private\""
expect_failure score-range 'sweep: score values must be numbers from zero through the last level' "$score | .value=2"
expect_failure score-low 'sweep: score values must be numbers from zero through the last level' "$score | .value=-0.01"

jq -n -f sweep.jq ../../probes/02-confidence/runs/run.jsonl \
  | jq -c '{rows,labeled,half:(.sweep[]|select(.cut==0.5)),high:(.sweep[]|select(.cut==0.95))}' \
  > "$work/choice-probe.json"
cat > "$work/choice-probe-expected.json" <<'EOF'
{"rows":60,"labeled":60,"half":{"cut":0.5,"resolved":60,"unresolved":0,"ties":0,"coverage":1,"accuracy_resolved":0.9667,"accuracy_unresolved":null},"high":{"cut":0.95,"resolved":53,"unresolved":7,"ties":0,"coverage":0.8833,"accuracy_resolved":0.9623,"accuracy_unresolved":1}}
EOF
cmp "$work/choice-probe-expected.json" "$work/choice-probe.json"

jq -c '.input.label = .input.level' ../../probes/03-score/runs/score.jsonl > "$work/score-probe.jsonl"
jq -n -f sweep.jq "$work/score-probe.jsonl" \
  | jq -c '{rows,labeled,cut:(.sweep[]|select(.cut==2))}' \
  > "$work/score-probe.json"
cat > "$work/score-probe-expected.json" <<'EOF'
{"rows":40,"labeled":40,"cut":{"cut":2,"accuracy":0.925,"precision":1,"recall":0.875,"f1":0.9333}}
EOF
cmp "$work/score-probe-expected.json" "$work/score-probe.json"

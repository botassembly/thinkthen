#!/bin/sh
set -eu
cd -- "$(dirname -- "$0")"

. ../../sdlc/scripts/scratch.sh
scratch_dir work
: > "$work/empty.jsonl"
repo=$(CDPATH= cd -- ../.. && pwd)
case ${CARGO_TARGET_DIR:-target} in
  /*) native_bin="${CARGO_TARGET_DIR}/debug/thinkthen" ;;
  *) native_bin="$repo/${CARGO_TARGET_DIR:-target}/debug/thinkthen" ;;
esac

# Exercise the current native producer, rather than the historical saved
# details whose question presentation predates authored on pointers.
env -i PATH="$PATH" HOME="$work/home" XDG_CONFIG_HOME="$work/config" \
  XDG_CACHE_HOME="$work/cache" XDG_STATE_HOME="$work/state" \
  "$native_bin" annotate \
  ../../demos/14-grade-a-batch/checks.json --jsonl --batch 1 --details \
  --replay ../../demos/14-grade-a-batch/recording \
  --input ../../demos/14-grade-a-batch/cases.jsonl > "$work/native.jsonl"
jq -n --argjson truth '{"correct":"/input/human_correct"}' \
  -f sweep.jq "$work/native.jsonl" \
  | jq -c '.questions.correct | {rows,labeled,cut:.pick.cut,accuracy:.pick.accuracy,f1:.pick.f1}' \
  > "$work/native-summary.json"
printf '%s\n' '{"rows":6,"labeled":6,"cut":0.6,"accuracy":1,"f1":1}' \
  > "$work/native-expected.json"
cmp "$work/native-expected.json" "$work/native-summary.json"

# Reuse captured native questions for declarations, numeric batches and
# structured readings; the report still depends on the replay's stored odds.
jq -c --slurpfile native ../../libraries/python/tests/fixtures/complete.json '
  .answers.correct.question = $native[0].native_presentation.questions[4]
' "$work/native.jsonl" > "$work/presentation.jsonl"
jq -n --argjson truth '{"correct":"/input/human_correct"}' \
  -f sweep.jq "$work/presentation.jsonl" > "$work/presentation-report.json"
jq -c --slurpfile native ../../libraries/python/tests/fixtures/complete.json '
  .answers.correct.question = ($native[0].results[]
    | select(.type=="DecideResult") | .result.question)
' "$work/native.jsonl" > "$work/readings.jsonl"
jq -n --argjson truth '{"correct":"/input/human_correct"}' \
  -f sweep.jq "$work/readings.jsonl" > "$work/readings-report.json"
jq -n --argjson truth '{"correct":"/input/human_correct"}' \
  -f sweep.jq "$work/native.jsonl" > "$work/native-report.json"
cmp "$work/native-report.json" "$work/presentation-report.json"
cmp "$work/native-report.json" "$work/readings-report.json"
jq -c --slurpfile native ../../libraries/python/tests/fixtures/complete.json '
  $native[0].native_presentation.questions[0] as $authored
  | .answers.correct.question += {model:$authored.model,profile:$authored.profile,
                                  batch:$authored.batch,on:$authored.on}
' "$work/presentation.jsonl" > "$work/authored.jsonl"
jq -n --argjson truth '{"correct":"/input/human_correct"}' \
  -f sweep.jq "$work/authored.jsonl" > "$work/authored-report.json"
cmp "$work/native-report.json" "$work/authored-report.json"

jq -n -c '
  def row($id; $truth; $billing; $urgent):
    {input:{id:$id,human_tags:$truth},
     value:(["billing","urgent"] | map(select(({billing:$billing,urgent:$urgent})[.] >= 0.5))),
     question:{verb:"tag",text:"Which topics?",labels:["billing","urgent"]},threshold:0.5,
     answer:{kind:"tag",probabilities:{billing:$billing,urgent:$urgent}}};
  row("one";["billing"];0.9;0.1),
  row("two";["urgent"];0.8;0.7),
  row("three";[];0.2;0.3),
  row("four";null;0.6;0.4)
' > "$work/tag.jsonl"

jq -n --arg truth /input/human_tags -f sweep.jq "$work/tag.jsonl" > "$work/tag-report.json"
jq -c '{keys:(keys_unsorted),mode,truth,rows,
        labels:[.labels[]|{keys:(keys_unsorted),label,rows,labeled,unlabeled,pick}]}' \
  "$work/tag-report.json" > "$work/tag-summary.json"
cat > "$work/tag-expected.json" <<'EOF'
{"keys":["mode","truth","rows","labels"],"mode":"tag","truth":"/input/human_tags","rows":4,"labels":[{"keys":["label","rows","labeled","unlabeled","pick","sweep"],"label":"billing","rows":4,"labeled":3,"unlabeled":["four"],"pick":{"cut":0.9,"accuracy":1,"f1":1,"tied_cuts":[0.85,0.9],"rule":"the highest F1, and the middle cut of the cuts that tie"}},{"keys":["label","rows","labeled","unlabeled","pick","sweep"],"label":"urgent","rows":4,"labeled":3,"unlabeled":["four"],"pick":{"cut":0.55,"accuracy":1,"f1":1,"tied_cuts":[0.35,0.4,0.45,0.5,0.55,0.6,0.65,0.7],"rule":"the highest F1, and the middle cut of the cuts that tie"}}]}
EOF
cmp "$work/tag-expected.json" "$work/tag-summary.json"
jq -e '.labels[] | select(.label == "billing")
       | .sweep[] | select(.cut == 0.95)
       | .precision == null and .recall == 0 and .f1 == null' \
  "$work/tag-report.json" > /dev/null

# Escaped pointer tokens resolve against the whole saved row.
jq -c '.input["human/tags"] = .input.human_tags | del(.input.human_tags)' \
  "$work/tag.jsonl" > "$work/escaped-tag.jsonl"
jq -n --arg truth /input/human~1tags -f sweep.jq "$work/escaped-tag.jsonl" \
  | jq -e '.labels[0].labeled == 3' > /dev/null

# Build detailed annotate rows from the committed page-14 replay, then map one
# choice and one synthetic tag beside the existing decision.
jq -c '
  .input.human_failure = (if .input.human_correct then "none" else "wrong_fact" end)
  | .input.human_tags = (if .input.human_correct then ["safe"] else ["wrong"] end)
  | .value.topics = (if .input.human_correct then ["safe"] else ["wrong"] end)
  | .answers.topics = {
      value:.value.topics,
      question:{verb:"tag",text:"Which topics?",labels:["safe","wrong"]},
      answer:{kind:"tag",probabilities:(if .input.human_correct then {safe:0.9,wrong:0.1} else {safe:0.1,wrong:0.9} end)},
      threshold:0.5,request:"synthetic"
    }
' ../../probes/annotate-0015/howto-14.jsonl > "$work/annotate.jsonl"

truth='{"correct":"/input/human_correct","failure_kind":"/input/human_failure","topics":"/input/human_tags"}'
jq -s -e 'all(.[]; .answers.failure_kind.threshold == null)' "$work/annotate.jsonl" > /dev/null
jq -n --argjson truth "$truth" -f sweep.jq "$work/annotate.jsonl" > "$work/annotate-report.json"
jq -c '{keys:(keys_unsorted),mode,truth,rows,
        questions:(.questions|to_entries|map({name:.key,keys:(.value|keys_unsorted),
          value:(.value | if .verb=="tag" then {verb,rows,labels:[.labels[]|{label,rows,labeled,unlabeled}]}
            elif .verb=="choose" then {verb,rows,labeled,unlabeled,options}
            else {verb,rows,labeled,unlabeled,pick} end)}))}' \
  "$work/annotate-report.json" > "$work/annotate-summary.json"
cat > "$work/annotate-expected.json" <<'EOF'
{"keys":["mode","truth","rows","questions"],"mode":"annotate","truth":{"correct":"/input/human_correct","failure_kind":"/input/human_failure","topics":"/input/human_tags"},"rows":6,"questions":[{"name":"correct","keys":["verb","rows","labeled","unlabeled","pick","sweep"],"value":{"verb":"decide","rows":6,"labeled":6,"unlabeled":[],"pick":{"cut":0.6,"accuracy":1,"f1":1,"tied_cuts":[0.55,0.6],"rule":"the highest F1, and the middle cut of the cuts that tie"}}},{"name":"failure_kind","keys":["verb","rows","labeled","unlabeled","options","sweep"],"value":{"verb":"choose","rows":6,"labeled":6,"unlabeled":[],"options":["none","wrong_fact","unsupported","incomplete"]}},{"name":"topics","keys":["verb","rows","labels"],"value":{"verb":"tag","rows":6,"labels":[{"label":"safe","rows":6,"labeled":6,"unlabeled":[]},{"label":"wrong","rows":6,"labeled":6,"unlabeled":[]}]}}]}
EOF
cmp "$work/annotate-expected.json" "$work/annotate-summary.json"

# Mapping order is public and partial maps are deliberate.
jq -n --argjson truth '{"topics":"/input/human_tags","correct":"/input/human_correct"}' \
  -f sweep.jq "$work/annotate.jsonl" \
  | jq -e '.questions | keys_unsorted == ["topics","correct"]' > /dev/null

expect_failure() {
  name=$1
  expected=$2
  shift 2
  if jq "$@" > "$work/out" 2> "$work/error"; then
    echo "sweep accepted $name" >&2
    exit 1
  fi
  sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
  printf '%s\n' "$expected" > "$work/expected-message"
  cmp "$work/expected-message" "$work/message"
  if grep -q 'private' "$work/error"; then
    echo "sweep echoed hostile data for $name" >&2
    exit 1
  fi
}

# Malformed semantic fields and recognized presentation types retain one
# data-free refusal. Declarations are structural here; Rust owns their grammar.
for change in \
  '.text=null' '.text=7' '.text=""' '.text="private\n"' \
  '.true=7' '.false=false' '.true=""' '.false="private\n"' \
  '.options=[]' '.label_details=[]' '.private="private"' \
  '.on=null' '.on="private"' '.on=[7]' '.model=null' '.profile=7' \
  '.batch=null' '.batch=0' '.batch=1.5' '.batch="private"' \
  '.name=7' '.name="Invalid"' '.wording_version=null' '.wording_version=0' \
  '.wording_version=1.5' '.wording_version=4294967296' \
  '.item_schema=null' '.context_schema=[]'; do
  jq -c ".answers.correct.question |= ($change)" "$work/presentation.jsonl" > "$work/bad.jsonl"
  expect_failure "decision-question $change" 'sweep: mapped decision question is malformed' \
    -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
done

expect_failure tag-no-truth 'sweep: tag rows require --arg truth POINTER' \
  -n -f sweep.jq "$work/tag.jsonl"
expect_failure annotate-no-truth 'sweep: annotate rows require --argjson truth MAP' \
  -n -f sweep.jq "$work/annotate.jsonl"
expect_failure both 'sweep: group and truth cannot be used together' \
  -n --arg group /input/group --arg truth /input/human_tags -f sweep.jq "$work/tag.jsonl"
expect_failure scalar-truth 'sweep: truth accepts tag or annotate rows only' \
  -n --arg truth /input/label -f sweep.jq ../rows/runs/run-a.jsonl
expect_failure empty-truth 'sweep: truth requires a nonempty run' \
  -n --arg truth /input/label -f sweep.jq "$work/empty.jsonl"

jq -c '.input.human_tags=["private"]' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure bad-tag-truth 'sweep: tag truth must contain unique known labels' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"
jq -c '.value=[]' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure bad-tag-value 'sweep: a tag value must equal labels at or above its threshold' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"
jq -c '.meta.questions_sha256="private"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure bad-digest 'sweep: annotate rows must carry one valid question-set digest' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.value.correct = false' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure outer-value 'sweep: an annotate answer must equal its outer value' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
expect_failure empty-map 'sweep: annotate truth must be a nonempty object of JSON Pointers' \
  -n --argjson truth '{}' -f sweep.jq "$work/annotate.jsonl"
expect_failure unknown-name 'sweep: annotate truth names an unknown question' \
  -n --argjson truth '{"private":"/input/human_correct"}' -f sweep.jq "$work/annotate.jsonl"
expect_failure score-name 'sweep: annotate truth accepts decide, choose, or tag answers only' \
  -n --argjson truth '{"severity":"/input/human_correct"}' -f sweep.jq "$work/annotate.jsonl"

jq -c 'if .input.id=="two" then .question.text="private" else . end' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure changed-tag-question 'sweep: tag rows must carry one shared ordered label list' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"
jq -c '.threshold=1.1' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure tag-threshold 'sweep: tag rows must carry one shared numeric threshold' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"
jq -c '.answer.probabilities += {private:0}' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure tag-keys 'sweep: tag probabilities must have exactly the label keys' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"
jq -c '.answer.probabilities.billing=1.1' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure tag-probability 'sweep: tag probabilities must be numbers from zero through one' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"
jq -c '.input.id=7' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure tag-id 'sweep: tag rows must carry a string case id' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"
jq -c '.input.id="same"' "$work/tag.jsonl" > "$work/bad.jsonl"
expect_failure tag-duplicate 'sweep: tag case ids must be unique' \
  -n --arg truth /input/human_tags -f sweep.jq "$work/bad.jsonl"

jq -c 'if .input.id=="E-01" then del(.input.human_correct) else . end' \
  "$work/annotate.jsonl" > "$work/missing-truth.jsonl"
jq -n --argjson truth '{"correct":"/input/human_correct"}' \
  -f sweep.jq "$work/missing-truth.jsonl" \
  | jq -e '.questions.correct.labeled == 5 and .questions.correct.unlabeled == ["E-01"]' > /dev/null
jq -c '.input.human_correct="private"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure decision-truth 'sweep: decision truth must be boolean' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.input.human_failure="private"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-truth 'sweep: choice truth must name a listed option' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.input.human_tags=["private"]' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure annotate-tag-truth 'sweep: tag truth must contain unique known labels' \
  -n --argjson truth '{"topics":"/input/human_tags"}' -f sweep.jq "$work/bad.jsonl"
jq -c 'if .input.id=="E-02" then .answers.correct.question.text="private" else . end' \
  "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure changed-definition 'sweep: a mapped annotate answer must keep one definition' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.correct.question.verb="choose"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure verb-kind 'sweep: an annotate question verb must match its answer kind' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.input.id=7' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure annotate-id 'sweep: annotate rows must carry a string case id' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.input.id="same"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure annotate-duplicate 'sweep: annotate case ids must be unique' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"

jq -c '.answers.correct.answer.probability=2' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure decision-probability 'sweep: mapped decision probability must be a number from zero through one' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c 'del(.answers.correct.threshold)' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure missing-decision-threshold 'sweep: mapped decision threshold must be one legal rule' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.correct.threshold="0.8:0.2"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure invalid-decision-threshold 'sweep: mapped decision threshold must be one legal rule' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c 'del(.answers.correct.question.text)' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure decision-question 'sweep: mapped decision question is malformed' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.correct.value=(.answers.correct.value|not) | .value.correct=.answers.correct.value' \
  "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure decision-derived-value 'sweep: mapped decision value must follow its probability and threshold' \
  -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/bad.jsonl"
jq -c 'del(.answers.failure_kind.threshold)' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-threshold 'sweep: mapped choice threshold must be present and null or one cut' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.failure_kind.threshold=0' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-zero-threshold 'sweep: mapped choice threshold must be present and null or one cut' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.failure_kind.answer.probabilities="private-marker"' \
  "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-probabilities-object 'sweep: choice probabilities must have exactly the option keys' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c 'del(.answers.failure_kind.answer.probabilities.none)' \
  "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-probabilities-keys 'sweep: choice probabilities must have exactly the option keys' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.failure_kind.answer.probabilities.none="private-marker"' \
  "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-probability-member 'sweep: choice probabilities must be numbers from zero through one' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.failure_kind.answer.pick="private-marker"' \
  "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-pick 'sweep: a choice pick must be the first option at the maximum probability' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.failure_kind.answer.probabilities={none:0.4,wrong_fact:0.3,unsupported:0.2,incomplete:0.1}
       | .answers.failure_kind.answer.pick="none" | .answers.failure_kind.value="none"
       | .answers.failure_kind.threshold=0.5 | .value.failure_kind="none"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-below-cut 'sweep: mapped choice value must follow its probabilities and threshold' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"
jq -c '.answers.failure_kind.answer.probabilities={none:0.5,wrong_fact:0.5,unsupported:0,incomplete:0}
       | .answers.failure_kind.answer.pick="none" | .answers.failure_kind.value="none"
       | .value.failure_kind="none"' "$work/annotate.jsonl" > "$work/bad.jsonl"
expect_failure choice-tie 'sweep: mapped choice value must follow its probabilities and threshold' \
  -n --argjson truth '{"failure_kind":"/input/human_failure"}' -f sweep.jq "$work/bad.jsonl"

# A probability at a band's low edge is not sure (specification/threshold.md),
# so a mapped decision at the edge with a null value is accepted.
jq -c 'if .input.id=="E-03" then .answers.correct.answer.probability=0.2 else . end' \
  "$work/annotate.jsonl" > "$work/edge.jsonl"
jq -n --argjson truth '{"correct":"/input/human_correct"}' -f sweep.jq "$work/edge.jsonl" \
  | jq -e '.questions.correct.labeled == 6' > /dev/null

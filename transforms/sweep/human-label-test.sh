#!/bin/sh
set -eu
cd -- "$(dirname -- "$0")"

work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
: > "$work/empty.jsonl"

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

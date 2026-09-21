#!/bin/sh
set -eu
cd -- "$(dirname -- "$0")"

work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

jq -n -c '
  def row($id; $group; $label; $probability):
    {cohort:$group,
     input:{id:$id, document_type:$group, "type/name":$group,
            "type~name":$group, label:$label},
     value:($probability >= 0.5), question:{verb:"decide"},
     answer:{kind:"yes_no",probability:$probability}};
  row("b-1";"beta";true;0.4),
  row("a-1";"alpha";true;0.9),
  row("a-2";"alpha";true;0.7),
  row("b-u";"beta";null;0.8),
  row("a-3";"alpha";false;0.6),
  row("b-2";"beta";false;0.3),
  row("a-4";"alpha";false;0.1)
' > "$work/grouped.jsonl"

jq -n --arg group /input/document_type -f sweep.jq "$work/grouped.jsonl" \
  > "$work/grouped-report.json"
jq -c '{mode,group,rows,has_pick:has("pick"),
        groups:[.groups[]|{value,rows,labeled,unlabeled,pick}]}' \
  "$work/grouped-report.json" > "$work/grouped-summary.json"
cat > "$work/grouped-summary-expected.json" <<'EOF'
{"mode":"grouped_decide","group":"/input/document_type","rows":7,"has_pick":false,"groups":[{"value":"alpha","rows":4,"labeled":4,"unlabeled":[],"pick":{"cut":0.7,"accuracy":1,"f1":1,"tied_cuts":[0.65,0.7],"rule":"the highest F1, and the middle cut of the cuts that tie"}},{"value":"beta","rows":3,"labeled":2,"unlabeled":["b-u"],"pick":{"cut":0.4,"accuracy":1,"f1":1,"tied_cuts":[0.35,0.4],"rule":"the highest F1, and the middle cut of the cuts that tie"}}]}
EOF
cmp "$work/grouped-summary-expected.json" "$work/grouped-summary.json"

jq -c '.groups[] | select(.value == "alpha")' "$work/grouped-report.json" \
  > "$work/alpha-before.json"
jq -n -c '
  range(1; 31) as $n
  | {cohort:"beta",
     input:{id:("extra-" + ($n|tostring)),document_type:"beta",
            "type/name":"beta","type~name":"beta",label:($n % 2 == 0)},
     value:($n % 3 == 0),question:{verb:"decide"},
     answer:{kind:"yes_no",probability:(($n % 10) / 10)}}
' >> "$work/grouped.jsonl"
jq -n --arg group /input/document_type -f sweep.jq "$work/grouped.jsonl" \
  | jq -c '.groups[] | select(.value == "alpha")' > "$work/alpha-after.json"
cmp "$work/alpha-before.json" "$work/alpha-after.json"

for pointer in /cohort /input/type~1name /input/type~0name; do
  jq -n --arg group "$pointer" -f sweep.jq "$work/grouped.jsonl" \
    | jq -e '.groups | map(.value) == ["alpha","beta"]' > /dev/null
done

jq -n -c '{"":"empty member",input:{id:"empty-key",label:true},value:true,
           question:{verb:"decide"},answer:{kind:"yes_no",probability:0.9}}' \
  > "$work/empty-member.jsonl"
jq -n --arg group / -f sweep.jq "$work/empty-member.jsonl" \
  | jq -e '.group == "/" and (.groups | map(.value)) == ["empty member"]' \
  > /dev/null

: > "$work/empty.jsonl"
jq -n --arg group /input/document_type -f sweep.jq "$work/empty.jsonl" \
  | jq -c . > "$work/grouped-empty.json"
printf '%s\n' '{"mode":"grouped_decide","group":"/input/document_type","rows":0,"groups":[]}' \
  > "$work/grouped-empty-expected.json"
cmp "$work/grouped-empty-expected.json" "$work/grouped-empty.json"

choice='{"input":{"id":"private id","label":"red","document_type":"group"},"value":"red","question":{"options":["red","blue"]},"answer":{"kind":"choice","pick":"red","probabilities":{"red":0.8,"blue":0.2}}}'
score='{"input":{"id":"private id","label":0,"document_type":"group"},"value":1,"question":{"levels":["low","high"]},"answer":{"kind":"score"}}'
decision='{"input":{"id":"private id","label":true,"document_type":"group"},"value":true,"question":{},"answer":{"kind":"yes_no","probability":0.8}}'

expect_group_failure() {
  name=$1
  pointer=$2
  expected=$3
  filter=$4
  jq -n -c "$filter" > "$work/bad.jsonl"
  if jq -n --arg group "$pointer" -f sweep.jq "$work/bad.jsonl" \
    > "$work/out" 2> "$work/error"; then
    echo "grouped sweep accepted $name" >&2
    exit 1
  fi
  sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
  printf '%s\n' "$expected" > "$work/expected-message"
  cmp "$work/expected-message" "$work/message"
  if grep -q 'private' "$work/error"; then
    echo "grouped sweep echoed hostile row data for $name" >&2
    exit 1
  fi
}

expect_group_failure relative input/document_type \
  'sweep: group must be an RFC 6901 JSON Pointer' "$decision"
expect_group_failure malformed /input/type~2name \
  'sweep: group must be an RFC 6901 JSON Pointer' "$decision"
expect_group_failure empty '' \
  'sweep: every row must resolve group to a string' "$decision"
expect_group_failure missing /input/missing \
  'sweep: every row must resolve group to a string' "$decision"
expect_group_failure null /input/document_type \
  'sweep: every row must resolve group to a string' "$decision | .input.document_type=null"
expect_group_failure non-string /input/document_type \
  'sweep: every row must resolve group to a string' "$decision | .input.document_type=7"
expect_group_failure choice-kind /input/document_type \
  'sweep: grouped mode accepts decision rows only' "$choice"
expect_group_failure score-kind /input/document_type \
  'sweep: grouped mode accepts decision rows only' "$score"
expect_group_failure mixed-kind /input/document_type \
  'sweep: grouped mode accepts decision rows only' \
  "($decision), ($choice | .input.id=\"other\")"
expect_group_failure repeated-id /input/document_type \
  'metric: repeated case ids; run trials.jq first' "($decision), ($decision)"
expect_group_failure malformed-row /input/document_type \
  'sweep: every row must be an object' '[]'

jq -n -c "$decision | del(.input.id) | .input.label=null" > "$work/missing-id.jsonl"
jq -n --arg group /input/document_type -f sweep.jq "$work/missing-id.jsonl" \
  | jq -e '.groups[0].unlabeled == ["with no id"]' > /dev/null
jq -n -c "$decision | .input.id=7 | .input.label=null" > "$work/non-string-id.jsonl"
jq -n --arg group /input/document_type -f sweep.jq "$work/non-string-id.jsonl" \
  | jq -e '.groups[0].unlabeled == [7]' > /dev/null

expect_invocation_failure() {
  name=$1
  shift
  if jq "$@" > "$work/out" 2> "$work/error"; then
    echo "sweep accepted ordinary input for $name" >&2
    exit 1
  fi
  sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
  printf '%s\n' 'sweep: read rows from files with jq -n' > "$work/expected-message"
  cmp "$work/expected-message" "$work/message"
}
expect_invocation_failure ungrouped -s -f sweep.jq "$work/grouped.jsonl"
expect_invocation_failure grouped -s --arg group /input/document_type \
  -f sweep.jq "$work/grouped.jsonl"

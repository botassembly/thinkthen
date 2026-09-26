#!/bin/sh
set -eu
cd -- "$(dirname -- "$0")"

. ../../sdlc/scripts/scratch.sh
scratch_dir work

jq -n -c '
  def row($id; $action; $reviewed):
    {input:({id:$id, body:"private"}
            + (if $reviewed == "absent" then {} else {reviewed_action:$reviewed} end)),
     policy:{action:$action, reason:"private"}};
  row("D-1";"draft";"draft"),
  row("D-2";"draft";"block"),
  row("D-3";"draft";null),
  row("B-1";"block";"block"),
  row("B-2";"block";"review"),
  row("B-3";"block";"absent"),
  row("R-1";"review";"review"),
  row("R-2";"review";"draft"),
  row("R-3";"review";null)
' > "$work/rows.jsonl"

jq -n -c -f monitor.jq "$work/rows.jsonl" > "$work/report.json"
printf '%s\n' '{"rows":9,"reviewed":6,"unreviewed":3,"review_coverage":0.6667,"agreed":3,"overturned":3,"agreement_rate":0.5,"actions":{"draft":{"rows":3,"reviewed":2,"review_coverage":0.6667,"agreed":1,"overturned":1,"agreement_rate":0.5},"block":{"rows":3,"reviewed":2,"review_coverage":0.6667,"agreed":1,"overturned":1,"agreement_rate":0.5},"review":{"rows":3,"reviewed":2,"review_coverage":0.6667,"agreed":1,"overturned":1,"agreement_rate":0.5}},"changes":[{"id":"D-2","action":"draft","reviewed_action":"block"},{"id":"B-2","action":"block","reviewed_action":"review"},{"id":"R-2","action":"review","reviewed_action":"draft"}]}' > "$work/expected.json"
cmp "$work/expected.json" "$work/report.json"

: > "$work/empty.jsonl"
jq -n -c -f monitor.jq "$work/empty.jsonl" > "$work/empty-report.json"
printf '%s\n' '{"rows":0,"reviewed":0,"unreviewed":0,"review_coverage":null,"agreed":0,"overturned":0,"agreement_rate":null,"actions":{"draft":{"rows":0,"reviewed":0,"review_coverage":null,"agreed":0,"overturned":0,"agreement_rate":null},"block":{"rows":0,"reviewed":0,"review_coverage":null,"agreed":0,"overturned":0,"agreement_rate":null},"review":{"rows":0,"reviewed":0,"review_coverage":null,"agreed":0,"overturned":0,"agreement_rate":null}},"changes":[]}' > "$work/empty-expected.json"
cmp "$work/empty-expected.json" "$work/empty-report.json"

printf '%s\n' '{"input":{"id":"one","reviewed_action":null},"policy":{"action":"draft"}}' > "$work/unreviewed.jsonl"
jq -n -c -f monitor.jq "$work/unreviewed.jsonl" \
  | jq -e '.actions.draft == {rows:1,reviewed:0,review_coverage:0,agreed:0,overturned:0,agreement_rate:null}' >/dev/null

expect_failure() {
	name=$1
	expected=$2
	file=$3
	if jq -n -c -f monitor.jq "$file" > "$work/out" 2> "$work/error"; then
		printf 'monitor accepted %s\n' "$name" >&2
		exit 1
	fi
	test ! -s "$work/out"
	sed -n 's/^jq: error (at .*): //p' "$work/error" > "$work/message"
	printf '%s\n' "$expected" > "$work/expected-message"
	cmp "$work/expected-message" "$work/message"
	test "$(wc -l < "$work/message")" -eq 1
	if grep -q private "$work/error"; then
		printf 'monitor echoed row data for %s\n' "$name" >&2
		exit 1
	fi
}

printf '%s\n' '"private"' > "$work/bad.jsonl"
expect_failure container 'monitor: every row must be an object' "$work/bad.jsonl"
printf '%s\n' '{"input":"private","policy":{"action":"draft"}}' > "$work/bad.jsonl"
expect_failure input-container 'monitor: every row must carry an object input with a string id' "$work/bad.jsonl"
printf '%s\n' '{"input":{"id":7,"private":true},"policy":{"action":"draft"}}' > "$work/bad.jsonl"
expect_failure id 'monitor: every row must carry an object input with a string id' "$work/bad.jsonl"
printf '%s\n' '{"input":{"id":"x","private":true},"policy":"private"}' > "$work/bad.jsonl"
expect_failure policy-container 'monitor: every row must carry an object policy with action draft, block, or review' "$work/bad.jsonl"
printf '%s\n' '{"input":{"id":"x","private":true},"policy":{"action":"private"}}' > "$work/bad.jsonl"
expect_failure policy-action 'monitor: every row must carry an object policy with action draft, block, or review' "$work/bad.jsonl"
printf '%s\n' '{"input":{"id":"x","reviewed_action":"private"},"policy":{"action":"draft"}}' > "$work/bad.jsonl"
expect_failure reviewed-action 'monitor: reviewed_action must be draft, block, review, null, or absent' "$work/bad.jsonl"
printf '%s\n' '{"input":{"id":"x","reviewed_action":7,"private":true},"policy":{"action":"draft"}}' > "$work/bad.jsonl"
expect_failure reviewed-type 'monitor: reviewed_action must be draft, block, review, null, or absent' "$work/bad.jsonl"
printf '%s\n%s\n' \
  '{"input":{"id":"same","private":true},"policy":{"action":"draft"}}' \
  '{"input":{"id":"same","private":true},"policy":{"action":"review"}}' > "$work/bad.jsonl"
expect_failure duplicate 'monitor: input ids must be unique' "$work/bad.jsonl"

if printf '%s\n' '{}' | jq -s -c -f monitor.jq > "$work/out" 2> "$work/error"; then
	printf '%s\n' 'monitor accepted ordinary input' >&2
	exit 1
fi
test ! -s "$work/out"
printf '%s\n' 'monitor: run with jq -n' > "$work/expected-message"
cmp "$work/expected-message" "$work/error"

jq -c -f ../triage/triage.jq ../../demos/16-triage-pipeline/fake-rows.jsonl > "$work/triage.jsonl"
jq -n -c -f monitor.jq "$work/triage.jsonl" \
  | jq -e '.rows == 6
           and .reviewed == 6 and .unreviewed == 0 and .review_coverage == 1
           and .agreed == 6 and .overturned == 0 and .agreement_rate == 1
           and .actions.draft.rows == 2 and .actions.block.rows == 1
           and .actions.review.rows == 3 and .changes == []' >/dev/null

printf '%s\n' 'monitor: report, refusals, and page-16 rows pass'

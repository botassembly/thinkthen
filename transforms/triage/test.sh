#!/bin/sh
set -eu

HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT HUP INT TERM

jq -c -f "$HERE/triage.jq" "$HERE/cases.jsonl" > "$work/routed.jsonl"
jq -c '{id: .input.id, action: .policy.action, reason: .policy.reason}' \
	"$work/routed.jsonl" > "$work/actual.jsonl"
cmp "$HERE/expected.jsonl" "$work/actual.jsonl"

jq -ce 'select(.input.id == "P-08") | .extra == "kept" and (.input.body | contains("$(touch no)"))' \
	"$work/routed.jsonl" >/dev/null
jq -S -c 'select(.input.id == "P-08") | del(.policy)' "$work/routed.jsonl" > "$work/preserved.json"
jq -S -c 'select(.input.id == "P-08")' "$HERE/cases.jsonl" > "$work/original.json"
cmp "$work/original.json" "$work/preserved.json"

check_bad() {
	name=$1
	row=$2
	if printf '%s\n' "$row" | jq -c -f "$HERE/triage.jq" > "$work/$name.out" 2> "$work/$name.err"; then
		printf 'triage test: malformed case succeeded: %s\n' "$name" >&2
		exit 1
	fi
	test ! -s "$work/$name.out"
}

check_bad missing '{"value":{"credential_request":false,"queue":"billing"}}'
check_bad credential '{"value":{"credential_request":"false","queue":"billing","urgency":0}}'
check_bad queue '{"value":{"credential_request":false,"queue":"sales","urgency":0}}'
check_bad urgency '{"value":{"credential_request":false,"queue":"billing","urgency":3}}'
check_bad unknown '{"value":{"credential_request":false,"queue":"billing","urgency":"low"}}'

printf '%s\n' 'triage policy: 8 routes and 5 refusals pass'

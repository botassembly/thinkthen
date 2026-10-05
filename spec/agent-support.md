# Validate support candidates before selecting evidence

The official skill's shell example finishes the producer before extraction. Invalid output cannot reach a judgment; a sole candidate still needs to answer the question. These saved answers are synthetic fixtures, not measurements. Existing CLI tests own the individual verb, framing, request-count and limit contracts.

```bash
set -euo pipefail
top=$(git rev-parse --show-toplevel)
recording="$top/spec/fixtures/agent-support"
# Run the actual example, so an edit changes the behavior tested here.
source <(awk '/^```bash$/ {inside=1; next} inside && /^```$/ {exit} inside {print}' "$top/skills/thinkthen/SKILL.md")
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
producer() { printf '%s' "$payload"; return "$producer_exit"; }
check() {
  expected=$1; reason=$2; expected_output=$3
  select_support producer > "$work/out" 2> "$work/err" && rc=0 || rc=$?
  printf '%s\n' "$rc" | mustmatch "$expected"
  if [ -n "$reason" ]; then
    grep -F "$reason" "$work/err" | mustmatch like "$reason"
  else
    test ! -s "$work/err"
  fi
  if [ -n "$expected_output" ]; then
    cat "$work/out" | mustmatch "$expected_output"
  else
    test ! -s "$work/out"
  fi
}
payload='{"results":[]}'; producer_exit=9
check 9 'producer failed: 9' ''
producer_exit=0
for payload in '{' '{}' '{"results":null}' '{"results":{}}' '{"results":[null]}' '{"results":[{}]}' '{"results":[{"body":false}]}' '{"results":[{"body":" "}]}' '{"results":[]} {"results":[]}'; do
  check 2 'invalid producer JSON or candidate shape' ''
done
payload='{"results":[]}'
check 0 '' ''
payload='{"results":[{"id":"shipping","body":"Shipping takes three days."}]}'
check 1 '' ''
payload='{"results":[{"id":"account","body":"The account page has changed."}]}'
check 3 '' ''
payload='{"results":[{"id":"reset","body":"Use the reset link to set a new password."}]}'
check 0 '' '{"id":"reset","body":"Use the reset link to set a new password."}'
payload='{"results":[{"id":"reset","body":"Use the reset link to set a new password."},{"id":"shipping","body":"Shipping takes three days."}]}'
check 0 '' '{"id":"reset","body":"Use the reset link to set a new password."}'
payload='{"results":[{"id":"shipping","body":"Shipping takes three days."},{"id":"account","body":"The account page has changed."}]}'
check 3 '' ''
payload=$(jq -cn '{results:[range(255)|{body:"support"}]}')
check 2 'candidate bounds exceeded' ''
payload=$(jq -cn '{results:[{body:("x"*16777216)}]}')
check 2 'candidate bounds exceeded' ''
```

An explicit catch-all is an ordinary label, while a tied judgment remains unresolved. Replay the routing command shown in the skill with the same explicit caps.

```bash
set -euo pipefail
recording="$(git rev-parse --show-toplevel)/spec/fixtures/agent-support"
for input in 'Can you recommend a good lunch?' 'I need help with my account charge.'; do
  routing_team=$(printf '%s' "$input" | thinkthen choose 'Which team owns this request?' \
    billing shipping account 'none of these' --threshold 0.8 \
    --max-requests-total 4 --max-estimated-input-tokens-total 12000 --max-retries 0 \
    --replay "$recording") && rc=0 || rc=$?
  case $input in
    'Can you recommend a good lunch?')
      printf '%s\n' "$rc" | mustmatch '0'
      printf '%s\n' "$routing_team" | mustmatch '"none of these"' ;;
    *)
      printf '%s\n' "$rc" | mustmatch '3'
      printf '%s\n' "$routing_team" | mustmatch 'null' ;;
  esac
done
```

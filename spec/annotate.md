# Annotate a record with a question set

The examples on `specification/annotate.md` run here against hand-built `local-1` entries, with no key and no network. The set's `open` question reads `on` at `/body`, and the other two read the whole record. One record makes two requests, one for each `on` group.

```bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cd "$work"
cat > triage.json <<'SET'
{
  "version": 1,
  "threshold": "0.1:0.9",
  "profile": "jev",
  "questions": {
    "open": {"decide": "Is this still open?", "true": "The report names a failure that is still happening.", "false": "Anything else.", "threshold": "0.1:0.9", "on": "/body"},
    "kind": {"choose": "Which kind of request is this?", "options": {"bug": "Reports broken behavior.", "feature": "Asks for new behavior.", "other": "Neither fits."}, "threshold": 0.8},
    "impact": {"score": "How much disruption does this report?", "levels": ["None.", "Work continues with a workaround.", "Work is blocked."]}
  }
}
SET
printf '%s\n' '{"id":"T-91","body":"Payouts have failed for 3 days."}' > issue.json
printf '%s\n' '{"id":"T-91","body":"Payouts have failed for 3 days."}' \
  '{"id":"T-92","body":"Please add a dark theme."}' > issues.jsonl
unset THINKTHEN_API_KEY THINKTHEN_BASE_URL
offline=(--url https://api.typesafe.ai/v1 --model local-1 --no-cache)

# One JSON document: the object gains the answers.
thinkthen annotate triage.json "${offline[@]}" --replay "$root/spec/fixtures/annotate" < issue.json \
  | mustmatch '{"id":"T-91","body":"Payouts have failed for 3 days.","open":true,"kind":"bug","impact":1.6}'

# A JSONL stream with no --field: `on` reads /body inside each record.
thinkthen annotate triage.json --jsonl "${offline[@]}" --replay "$root/spec/fixtures/annotate" < issues.jsonl \
  | jq -c 'select(.kind == "bug")' \
  | mustmatch '{"id":"T-91","body":"Payouts have failed for 3 days.","open":true,"kind":"bug","impact":1.6}'

# The dry run prints the first request a live run sends and counts every request.
thinkthen annotate triage.json --dry-run "${offline[@]}" < issue.json \
  | jq -c '{on, request_count, group_requests, state: .request.state}' \
  | mustmatch '{"on":{"open":["/body"],"kind":[""],"impact":[""]},"request_count":2,"group_requests":[1,1],"state":"Payouts have failed for 3 days."}'
```

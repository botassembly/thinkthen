# Result shapes

This page replays each command's demo through the installed binary with `--details`. Every replayed row and documented example must use result/2 and match its generated schema.

```bash
set -euo pipefail
export HOME="$(mktemp -d)"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL
top="$(git rev-parse --show-toplevel)"
page="$top/specification/result.md"
demos="$top/demos"
row() { head -n 1 | jq -c --arg command "$1" '{from: "replayed", command: $command, row: .}'; }
{
  (cd "$demos/01-refund-gate" && thinkthen decide 'Does the customer ask for money back?' --details --replay recording/ < message.txt | row decide)
  (cd "$demos/03-grep-for-meaning" && thinkthen filter 'Does the report give steps that would reproduce a defect?' --csv --field /body --threshold 0.9 --batch 1 --details --replay recording/ < issues.csv | row filter)
  printf '%s\n' '{"version":1,"questions":{"query":{"decide":"The passage answers the query."}}}' > "$HOME/rank-set.json"
  (cd "$demos/06-top-search-hits" && jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl | thinkthen rank @"$HOME/rank-set.json" --batch 1 --jsonl --field /query --field /passage --top 3 --details --replay recording/ | row rank-set)
  (cd "$demos/06-top-search-hits" && jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl | thinkthen rank 'The passage answers the query.' --batch 1 --jsonl --field /query --field /passage --top 3 --details --replay recording/ | row rank)
  (cd "$demos/02-route-a-ticket" && thinkthen choose 'Which team owns this request?' billing shipping account other --details --replay recording/ < ticket.txt | row choose)
  (cd "$demos/39-screen-a-message" && thinkthen tag @hazards.json --details --replay recording --input message.txt | row tag)
  (cd "$demos/17-rate-and-sort" && thinkthen score 'How hard is this request to answer?' 'A canned reply answers it.' 'One person can answer it after a look at the account.' 'It needs a specialist and more than one system.' --details --replay recording/ < requests/tax-split.txt | row score)
  (cd "$demos/15-find-the-line" && thinkthen find 'When does a refund reach the customer?' --lines --model jev-1.13.0 --input policy.txt --details --replay recording/ | row find)
  (cd "$demos/14-grade-a-batch" && thinkthen annotate checks.json --jsonl --batch 1 --details --replay recording --input cases.jsonl | row annotate)
  fixture="$top/specification/fixtures/recognize/caller-defined"; cat "$fixture/text.txt" | thinkthen recognize @"$fixture/question.json" --url "$(cat "$fixture/url.txt")" --replay "$fixture/recording" --no-cache --details | row recognize
  (cd "$demos/45-map-relationships" && thinkthen relate @relations.json --url https://api.typesafe.ai/v1 --details --replay recording/ < entities.json | row relate)
  awk '/^```/ { if (fenced) { fenced = 0 } else { fenced = 1; label = $2 }; next }
       fenced && /"schema":"thinkthen\.result\/2"/ { print "{\"from\":\"example\",\"command\":\"" label "\",\"row\":" $0 "}" }' "$page"
} > "$HOME/rows.jsonl"
jq -r 'select(.from == "replayed" and .command == "choose") | .row |
  [(.value == "billing"), (.answer.probabilities.billing == 1),
   (.answer.confidence == 1), (.threshold == null),
   (.meta.requests | length == 1), (.meta.requests_sent == 0)] | all' "$HOME/rows.jsonl" | mustmatch "true"

python3 - "$HOME/rows.jsonl" "$top/specification/result.schema.json" <<'PY_SCHEMA' | mustmatch ""
import json, sys
from jsonschema import Draft202012Validator
schema = json.load(open(sys.argv[2]))
names = {"decide":"completeDecide", "choose":"completeChoose", "tag":"completeTag",
         "score":"completeScore", "filter":"completeFilter", "rank":"completeRank",
         "find":"completeFind", "annotate":"completeAnnotation", "recognize":"completeRecognition",
         "relate":"completeRelation"}
seen = set()
for line in open(sys.argv[1]):
    case = json.loads(line)
    row = case["row"]
    assert row["schema"] == "thinkthen.result/2", case["command"]
    if row["schema"] == "thinkthen.result/2":
        command = case["command"].split("-")[0]
        if case["from"] == "replayed":
            seen.add(command)
        Draft202012Validator({"$ref":"#/$defs/"+names[command], "$defs":schema["$defs"]}).validate(row)
        if command == "rank":
            assert isinstance(row["value"], int) and not isinstance(row["value"], bool) and row["value"] > 0
assert seen == set(names), seen
PY_SCHEMA
```

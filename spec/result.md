# Result shapes

[specification/result.md](../specification/result.md) names each command's detailed row in its compatibility table. This page holds that table to rows the binary writes. It replays one command from each command's demo with `--details`, so no key and no network are needed. It then holds every `thinkthen.result/1` example row on the contract page to the same table. Each example row's fence names its command as the second word of the info string.

A finding names where the row came from, its command, and the member or verb that disagrees. The page expects no findings.

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
  (cd "$demos/03-grep-for-meaning" && thinkthen filter 'Does the report give steps that would reproduce a defect?' --csv --field /body --threshold 0.9 --details --replay recording/ < issues.csv | row filter)
  (cd "$demos/06-top-search-hits" && jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl | thinkthen rank 'The passage answers the query.' --jsonl --field /query --field /passage --top 3 --details --replay recording/ | row rank)
  (cd "$demos/02-route-a-ticket" && thinkthen choose 'Which team owns this request?' billing shipping account other --details --replay recording/ < ticket.txt | row choose)
  (cd "$demos/39-screen-a-message" && thinkthen tag @hazards.json --details --replay recording --input message.txt | row tag)
  (cd "$demos/17-rate-and-sort" && thinkthen score 'How hard is this request to answer?' 'A canned reply answers it.' 'One person can answer it after a look at the account.' 'It needs a specialist and more than one system.' --details --replay recording/ < requests/tax-split.txt | row score)
  (cd "$demos/15-find-the-line" && thinkthen find 'When does a refund reach the customer?' --lines --model jev-1.13.0 --input policy.txt --details --replay recording/ | row find)
  (cd "$demos/14-grade-a-batch" && thinkthen annotate checks.json --jsonl --details --replay recording --input cases.jsonl | row annotate)
  (cd "$demos/44-recognize-names" && tr -d '\n' < message.txt | thinkthen recognize --url https://api.typesafe.ai/v1 --replay recording/ --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' --details | row recognize)
  (cd "$demos/45-map-relationships" && thinkthen relate @relations.json --url https://api.typesafe.ai/v1 --details --replay recording/ < entities.json | row relate)
  awk '/^```/ { if (fenced) { fenced = 0 } else { fenced = 1; label = $2 }; next }
       fenced && /"schema":"thinkthen\.result\/1"/ { print "{\"from\":\"example\",\"command\":\"" label "\",\"row\":" $0 "}" }' "$page"
} > "$HOME/rows.jsonl"
wc -l < "$HOME/rows.jsonl" | mustmatch "19"
sed -n '/^## Compatibility$/,/^## Record rows$/p' "$page" | grep '^| `' \
  | jq -R -c 'split("|") | {command: (.[1] | gsub("[` ]"; "")), verb: (.[2] | gsub("[` ]"; "")),
      members: [.[3] | scan("`([^`]+)`")[0]], meta: [.[4] | scan("`([^`]+)`")[0]]}' > "$HOME/table.jsonl"
wc -l < "$HOME/table.jsonl" | mustmatch "10"
jq -r --slurpfile table "$HOME/table.jsonl" '
  def compare($where; $held; $listed):
    ($held - ($listed | map(rtrimstr("?"))) | .[] | "\($where) \(.) is not in the table"),
    (($listed | map(select(endswith("?") | not))) - $held | .[] | "\($where) \(.) is missing");
  .command as $c | .row as $row | "\(.from) \(if $c == "" then "(no command)" else $c end):" as $who
  | [$table[] | select(.command == $c)] as $match
  | if ($match | length) != 1 then "\($who) no table row"
    else $match[0] as $t
    | compare("\($who) member"; $row | keys_unsorted; $t.members),
      compare("\($who) meta member"; $row.meta | keys_unsorted; $t.meta),
      ((if $row | has("question") then $row.question.verb else "none" end) as $verb
       | select($verb != $t.verb) | "\($who) question.verb is \($verb), the table says \($t.verb)")
    end' "$HOME/rows.jsonl" | mustmatch ""
```

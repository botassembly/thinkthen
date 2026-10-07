# Rank sets take turns in saved member order

Rank accepts an ordered saved decide set on the CLI and through Rust's additive `RankSet` and `SetRanked` API. Each member ranks its own list by yes probability with stable ties. The merge visits one depth across members in saved order. A duplicate consumes that visit; the next member runs without a refill. Top follows first appearances of original input identities. Backend boundary tests independently declare `[0,1,2]` and `[0,2,1]`, expecting `[0,1,2]` selected by `[first,first,second]`.

This example reuses a saved individual question's recording. Repeating it under two names sends nothing; both keep the same cache keys, and the first member selects every original. Default bytes and top match ordinary rank. The complete details use result/2 numeric final positions. SQL, C and language set adoption remains with the owning family tickets.

```bash
set -euo pipefail
export HOME="$(mktemp -d)"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL
root="$(git rev-parse --show-toplevel)"
printf '%s\n' '{"version":1,"questions":{"first":{"decide":"The passage answers the query."},"second":{"decide":"The passage answers the query."}}}' > "$HOME/search.json"
cd "$root/demos/06-top-search-hits"
jq -c '{query: "Why is signing in slow or failing?", path, passage: .body}' hits.jsonl > "$HOME/input.jsonl"
thinkthen rank 'The passage answers the query.' --batch 1 --jsonl --field /query --field /passage --top 3 --replay recording/ < "$HOME/input.jsonl" > "$HOME/ordinary"
thinkthen rank @"$HOME/search.json" --batch 1 --jsonl --field /query --field /passage --top 3 --replay recording/ < "$HOME/input.jsonl" > "$HOME/set"
cmp "$HOME/ordinary" "$HOME/set"
wc -l < "$HOME/set" | mustmatch '3'
thinkthen rank @"$HOME/search.json" --batch 1 --jsonl --field /query --field /passage --top 3 --details --replay recording/ < "$HOME/input.jsonl" > "$HOME/details"
jq -sr '[.[] | [.question_name, .question.verb, .answer.kind, .threshold, .value]] == [["first","decide","yes_no",null,1],["first","decide","yes_no",null,2],["first","decide","yes_no",null,3]]' "$HOME/details" | mustmatch 'true'
```

Authored cuts and pointers are refused before normalization. A set owns its meanings; CLI overrides cannot replace them. These exact diagnostic examples validate admission; counted loopback tests prove the no-send claim at runtime.

```bash
set -euo pipefail
export HOME="$(mktemp -d)"
printf '%s\n' '{"version":1,"questions":{"first":{"decide":"Ready?"}}}' > "$HOME/search.json"
printf '%s\n' '{"version":1,"questions":{"first":{"decide":"Ready?","on":""}}}' > "$HOME/invalid.json"
set +e
printf 'a\n' | thinkthen rank @"$HOME/invalid.json" --no-cache > "$HOME/out" 2> "$HOME/err"
status=$?
set -e
printf '%s\n' "$status" | mustmatch '5'
cat "$HOME/out" | mustmatch ''
cat "$HOME/err" | mustmatch 'thinkthen: `questions.first.on`: rank question sets take no threshold or on'
set +e
printf 'a\n' | thinkthen rank @"$HOME/search.json" --true 'Override' --no-cache > "$HOME/out" 2> "$HOME/err"
status=$?
set -e
printf '%s\n' "$status" | mustmatch '2'
cat "$HOME/out" | mustmatch ''
cat "$HOME/err" | mustmatch 'thinkthen: `rank` with a question set takes no --true or --false; put meanings in each member'
```

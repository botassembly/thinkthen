#!/usr/bin/env bash
# The offline suite: every function answers, the three-valued semantics
# hold, the '@file' spelling carries its threshold, and the six error
# kinds reach SQL with their words. Runs on the null backend; no network.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
EXT="$ROOT/build/release/thinkthen.duckdb_extension"
CLI="$ROOT/duckdb-bin/duckdb"

run() {
  ENGINE_NULL=1 "$CLI" -unsigned -noheader -list -c "LOAD '$EXT'; $1" 2>&1
}

expect() {
  local name=$1 want=$2 got=$3
  if [[ "$got" == *"$want"* ]]; then
    echo "ok       $name"
  else
    echo "FAILED   $name: want '$want', got '$got'"
    exit 1
  fi
}

printf '{"decide": "Is this a complaint?", "threshold": 0.9}\n' > "$ROOT/tools/null-cut.json"

expect "decide yes"        "true"  "$(run "SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today');")"
expect "decide no"         "false" "$(run "SELECT thinkthen_decide('Is this a complaint?', 'thanks for the help');")"
expect "band unsure null"  ""      "$(run "SELECT coalesce(thinkthen_decide('{\"decide\":\"Refund?\",\"threshold\":\"0.2:0.8\"}', 'maybe later'), 'NULL');")"
expect "null text"         "NULL"  "$(run "SELECT thinkthen_decide('Is this a complaint?', NULL);")"
expect "at-file threshold" "true"  "$(run "SELECT thinkthen_decide('@$ROOT/tools/null-cut.json', 'I demand a refund today');")"
expect "probability"       "0.97"  "$(run "SELECT thinkthen_probability('Is this a complaint?', 'I demand a refund today');")"
expect "choose picks"      "the refund desk" "$(run "SELECT thinkthen_choose('Which team?', 'refund now', ['the refund desk','other desk']);")"
expect "score position"    "1.05"  "$(run "SELECT thinkthen_score('How strong?', 'maybe later', ['low','mid','high']);")"
expect "tag holds"         "[refund]" "$(run "SELECT thinkthen_tag('What is here?', 'the refund and the shipping', ['refund','shipping']);")"
expect "details digest"    "0b3e8345de7cbf9087b061df9dfaa4a3a467f3c6dc2f84d59b534497ed858996" \
  "$(run "SELECT thinkthen_details('Is this a complaint?', 'I demand a refund today').digest;")"
expect "details score nearest" "mid" "$(run "SELECT thinkthen_details('{\"score\":\"How strong?\",\"levels\":[\"low\",\"mid\",\"high\"]}', 'maybe later').nearest;")"
expect "annotate object"   '{"band":null,"spam":true}' \
  "$(run "SELECT thinkthen_annotate('{\"version\":1,\"questions\":{\"spam\":{\"decide\":\"Is this spam?\"},\"band\":{\"decide\":\"Refund?\",\"threshold\":\"0.2:0.8\"}}}', 'maybe later');")"
expect "warm count"        "2"     "$(run "SELECT thinkthen_warm('Is this a complaint?', t) FROM (SELECT unnest(['refund now','refund again']) t);")"
expect "usage rows"        "requests" "$(run "SELECT metric FROM thinkthen_usage() LIMIT 1;")"
expect "usage error kind"  "thinkthen usage" \
  "$(run "SELECT thinkthen_decide('   ', 'anything');")"
expect "backend error kind" "thinkthen backend" \
  "$(run "SELECT thinkthen_decide('Is this a complaint?', 'this malformed line');")"
expect "list-refusal kind" "thinkthen usage" \
  "$(run "SELECT thinkthen_choose('@$ROOT/tools/null-cut.json', 'text', ['a','b']);")"

# The warm poison raises once, then clears: one failed warm must not fail
# those pairs forever. One process, piped so the CLI continues past the
# raised error, then the same pair's read answers.
poison_out=$(printf "LOAD '%s';\nSELECT thinkthen_warm('Poison probe?', t) FROM (SELECT unnest(['refund now','this malformed line']) t);\nSELECT thinkthen_decide('Poison probe?', 'refund now');\nSELECT thinkthen_decide('Poison probe?', 'refund now');\n" "$EXT" | ENGINE_NULL=1 "$CLI" -unsigned -noheader -list 2>&1 || true)
poison_count=$(printf '%s\n' "$poison_out" | grep -c "thinkthen backend" || true)
poison_last=$(printf '%s\n' "$poison_out" | tail -1)
expect "poison raises once" "1" "$poison_count"
expect "poison clears and the pair answers" "true" "$poison_last"

#!/usr/bin/env bash
# The row-mapping suite: every answering function must put each row's own
# answer on that row when texts repeat or a NULL appears. The bug this
# pins (review finding 1): decide, probability, and score wrote the k-th
# distinct text's answer to the k-th row, so 10,000 rows alternating two
# texts returned 5 true and 9,995 false, and repeated single texts
# answered only their first row. choose, tag, details, and annotate were
# already correct; their cases stand as regressions.
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
  if [[ "$got" == "$want" ]]; then
    echo "ok       $name"
  else
    echo "FAILED   $name: want '$want', got '$got'"
    exit 1
  fi
}

expect "decide alternating 10,000 rows split exactly" "5000|5000" "$(run "
SELECT count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = true)
    || '|' || count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = false)
FROM (SELECT CASE WHEN i % 2 = 0 THEN 'I demand a refund today' ELSE 'thanks for the help' END AS t
      FROM range(10000) r(i));")"

expect "decide one text repeated answers every row" "4|0" "$(run "
SELECT count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = true)
    || '|' || count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = false)
FROM (SELECT 'I demand a refund today' AS t FROM range(4));")"

expect "decide NULL rows stay NULL and consume no answer" "2|1|2" "$(run "
CREATE TABLE m_null(i INTEGER, t VARCHAR);
INSERT INTO m_null VALUES (1,'I demand a refund today'), (2,NULL), (3,'thanks for the help'), (4,NULL), (5,'I demand a refund today');
SELECT count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = true)
    || '|' || count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = false)
    || '|' || count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) IS NULL)
FROM m_null;")"

expect "decide three-text cycle keeps every row's answer" "6|3|0" "$(run "
SELECT count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = true)
    || '|' || count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) = false)
    || '|' || count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t) IS NULL)
FROM (SELECT (['I demand a refund today','thanks for the help','maybe later'])[i % 3 + 1] AS t
      FROM range(9) r(i));")"

expect "probability one text repeated answers every row" "4|0" "$(run "
SELECT count(*) FILTER (WHERE abs(thinkthen_probability('Is this a complaint?', t) - 0.97) < 0.000001)
    || '|' || count(*) FILTER (WHERE thinkthen_probability('Is this a complaint?', t) IS NULL)
FROM (SELECT 'I demand a refund today' AS t FROM range(4));")"

expect "probability NULL row stays NULL beside a repeat" "2|1" "$(run "
SELECT count(*) FILTER (WHERE abs(thinkthen_probability('Is this a complaint?', t) - 0.97) < 0.000001)
    || '|' || count(*) FILTER (WHERE thinkthen_probability('Is this a complaint?', t) IS NULL)
FROM (SELECT unnest([NULL, 'I demand a refund today', 'I demand a refund today']) AS t);")"

expect "score one text repeated answers every row" "4|0" "$(run "
SELECT count(*) FILTER (WHERE abs(thinkthen_score('How strong?', t, ['low','mid','high']) - 1.05) < 0.0000001)
    || '|' || count(*) FILTER (WHERE thinkthen_score('How strong?', t, ['low','mid','high']) IS NULL)
FROM (SELECT 'maybe later' AS t FROM range(4));")"

expect "score two texts alternating answer per row" "3|0" "$(run "
SELECT count(*) FILTER (WHERE abs(thinkthen_score('How strong?', t, ['low','mid','high']) - 1.05) < 0.0000001)
    || '|' || count(*) FILTER (WHERE thinkthen_score('How strong?', t, ['low','mid','high']) IS NULL)
FROM (SELECT CASE WHEN i % 2 = 0 THEN 'maybe later' ELSE 'thanks for the help' END AS t FROM range(6) r(i))
WHERE t = 'maybe later';")"

expect "choose repeated text answers every row" "4" "$(run "
SELECT count(*) FILTER (WHERE thinkthen_choose('Which team?', t, ['the refund desk','other desk']) = 'the refund desk')
FROM (SELECT 'refund now' AS t FROM range(4));")"

expect "tag repeated text answers every row" "4" "$(run "
SELECT count(*) FILTER (WHERE thinkthen_tag('What is here?', t, ['refund','shipping']) = ['refund'])
FROM (SELECT 'the refund and the shipping' AS t FROM range(4));")"

expect "details repeated text answers every row" "3|1" "$(run "
SELECT count(*) FILTER (WHERE thinkthen_details('Is this a complaint?', t).digest = '0b3e8345de7cbf9087b061df9dfaa4a3a467f3c6dc2f84d59b534497ed858996')
    || '|' || count(*) FILTER (WHERE thinkthen_details('Is this a complaint?', t).digest IS NULL)
FROM (SELECT unnest(['I demand a refund today', 'I demand a refund today', NULL, 'I demand a refund today']) AS t);")"

expect "annotate repeated text answers every row" "3|1" "$(run "
SELECT count(*) FILTER (WHERE thinkthen_annotate('{\"version\":1,\"questions\":{\"spam\":{\"decide\":\"Is this spam?\"},\"band\":{\"decide\":\"Refund?\",\"threshold\":\"0.2:0.8\"}}}', t) = '{\"spam\":true,\"band\":null}')
    || '|' || count(*) FILTER (WHERE thinkthen_annotate('{\"version\":1,\"questions\":{\"spam\":{\"decide\":\"Is this spam?\"},\"band\":{\"decide\":\"Refund?\",\"threshold\":\"0.2:0.8\"}}}', t) IS NULL)
FROM (SELECT unnest(['maybe later', 'maybe later', NULL, 'maybe later']) AS t);")"

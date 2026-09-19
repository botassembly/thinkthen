#!/bin/sh
# Read probe 1: the hit rate, the cost, and the behaviour on a document with no
# answering line, for each of the two shapes.
#
# `rank --top 1` is the first row of the rank rows for a document once they are
# sorted by the probability of yes. jq's sort is stable, so an exact tie keeps
# input order, which is the tie rule rank.md states.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}

printf '=== hit rate and no-answer behaviour ===\n'
jq -n --slurpfile rank "$rows/rank.jsonl" \
	--slurpfile find "$rows/find.jsonl" \
	--slurpfile none "$rows/find-none.jsonl" '
  def top: sort_by(- .answer.probability) | .[0];
  ($rank | group_by(.input.doc) | map({doc: .[0].input.doc, top: top})
   | INDEX(.doc)) as $r
  | ($find | INDEX(.input.id)) as $f
  | ($none | INDEX(.input.id)) as $n
  | [$f | keys[] as $doc
     | ($f[$doc].input.answer) as $truth
     | {doc: $doc,
        truth: ($truth // "none"),
        hard: $f[$doc].input.hard,
        rank_pick: $r[$doc].top.input.unit,
        rank_p: $r[$doc].top.answer.probability,
        find_pick: $f[$doc].answer.pick,
        find_p: ($f[$doc].answer.probabilities | to_entries | map(.value) | max),
        find_conf: $f[$doc].answer.confidence,
        none_pick: $n[$doc].answer.pick,
        none_p: ($n[$doc].answer.probabilities | to_entries | map(.value) | max)}]
  | sort_by(.doc)
  | . as $all
  | ($all | map(select(.truth != "none"))) as $answerable
  | ($all | map(select(.truth == "none"))) as $blank
  | {
      documents: ($all | length),
      answerable: ($answerable | length),
      no_answer: ($blank | length),
      hard: ($all | map(select(.hard)) | length),
      rank_top1_hits: ($answerable | map(select(.rank_pick == .truth)) | length),
      find_hits: ($answerable | map(select(.find_pick == .truth)) | length),
      find_none_hits: ($answerable | map(select(.none_pick == .truth)) | length),
      rank_top1_hits_hard: ($answerable | map(select(.hard and .rank_pick == .truth)) | length),
      find_hits_hard: ($answerable | map(select(.hard and .find_pick == .truth)) | length),
      hard_answerable: ($answerable | map(select(.hard)) | length),
      no_answer_rows: $blank,
      rank_top_p_on_answerable: {
        min: ($answerable | map(.rank_p) | min),
        max: ($answerable | map(.rank_p) | max)
      },
      find_none_says_none: ($blank | map(select(.none_pick == "none")) | length),
      find_none_false_none: ($answerable | map(select(.none_pick == "none")) | length),
      disagreements: ($all | map(select(.rank_pick != .find_pick))
                      | map({doc, truth, rank_pick, find_pick}))
    }'

printf '\n=== cost, through recipes/cost/cost.jq ===\n'
for run in rank find find-none; do
	printf '%s: ' "$run"
	jq -n --argjson usd_per_million_input 0.042 -f ../../recipes/cost/cost.jq "$rows/$run.jsonl" |
		jq -c '{rows, requests: .charged.rows, input_tokens: .charged.input_tokens, usd}'
done

printf '\n=== the rank rows read as yes/no rows, through recipes/score/score.jq ===\n'
jq -n --argjson cut 0.5 -f ../../recipes/score/score.jq "$rows/rank.jsonl"

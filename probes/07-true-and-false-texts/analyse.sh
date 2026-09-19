#!/bin/sh
# Read probe 7: what the two texts did to the answer and to the probability.
set -eu
cd -- "$(dirname -- "$0")"

rows=${1:-runs}

printf '=== each arm, through transforms/score/score.jq at the default cut ===\n'
for arm in plain texts; do
	printf '%s: ' "$arm"
	jq -n --argjson cut 0.5 -f ../../transforms/score/score.jq "$rows/$arm.jsonl" |
		jq -c '{rows, accuracy, true_positive, false_positive,
		        true_negative, false_negative}'
done

printf '\n=== how far the probability of yes moved ===\n'
jq -n --slurpfile plain "$rows/plain.jsonl" '
  def round4: . * 10000 | round | . / 10000;
  ($plain | INDEX(.input.id)) as $before
  | [inputs
     | {id: .input.id,
        label: .input.label,
        plain_p: $before[.input.id].answer.probability,
        texts_p: .answer.probability}
     | .move = (.texts_p - .plain_p | round4)
     | .toward_the_label = (if .label then .move > 0 else .move < 0 end)]
  | {rows: length,
     unmoved: (map(select(.move == 0)) | length),
     moved_toward_the_label: (map(select(.move != 0 and .toward_the_label)) | length),
     moved_away: (map(select(.move != 0 and (.toward_the_label | not))) | length),
     mean_absolute_move: ((map(.move | fabs) | add) / length | round4),
     max_absolute_move: (map(.move | fabs) | max | round4),
     crossed_the_half: (map(select((.plain_p >= 0.5) != (.texts_p >= 0.5))
                            | {id, label, plain_p, texts_p})),
     biggest: (sort_by(- (.move | fabs)) | .[0:5]
               | map({id, label, plain_p, texts_p, move}))}' \
	"$rows/texts.jsonl"

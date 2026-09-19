# sweep-two.jq — accuracy at coverage for a cut on each of two numbers.
#
# Reads: the rows of one `choose` run, one JSON object per line. Run it with
#   `jq -n -f sweep-two.jq ROWS`.
# Policies:
#   - A row is right when `answer.pick` equals `input.label`. The pick is read
#     rather than `value`, because the run applied no threshold and every cut
#     in the grid is tried here over the saved distribution.
#   - The two numbers are the winning option's probability, which is the
#     largest entry of `answer.probabilities`, and the backend's own
#     `confidence`. A row with no `confidence` is listed in `no_confidence`
#     and enters no rate on that side.
#   - A row is covered at a cut when its number reaches the cut. `accuracy` is
#     over the covered rows, `accuracy_refused` over the rest, and a zero
#     denominator yields null rather than zero.
#   - The grid is the 19 cuts from 0.05 to 0.95, the grid transforms/sweep/sweep.jq
#     uses.
#   - Counts are exact and rates are rounded to four decimals.

def grid: [range(5; 100; 5) | . / 100];
def round4: . * 10000 | round | . / 10000;

def side($rows; number):
  [grid[] as $cut
   | ($rows | map(select(number >= $cut))) as $covered
   | ($rows | map(select(number < $cut))) as $refused
   | {cut: $cut,
      covered: ($covered | length),
      coverage: (($covered | length) / ($rows | length) | round4),
      right: ($covered | map(select(.right)) | length),
      accuracy: (if ($covered | length) == 0 then null
                 else ($covered | map(select(.right)) | length) / ($covered | length) | round4 end),
      accuracy_refused: (if ($refused | length) == 0 then null
                         else ($refused | map(select(.right)) | length) / ($refused | length) | round4 end)}];

[inputs
 | {id: .input.id,
    label: .input.label,
    hard: .input.hard,
    pick: .answer.pick,
    right: (.answer.pick == .input.label),
    p: (.answer.probabilities | to_entries | map(.value) | max),
    confidence: .answer.confidence}] as $rows
| ($rows | map(select(.confidence != null))) as $with
| {
    rows: ($rows | length),
    right: ($rows | map(select(.right)) | length),
    wrong: ($rows | map(select(.right | not)) | map({id, label, pick, p, confidence, hard})),
    hard: ($rows | map(select(.hard)) | length),
    right_hard: ($rows | map(select(.hard and .right)) | length),
    no_confidence: [$rows[] | select(.confidence == null) | .id],
    spread: {
      p_right: ($rows | map(select(.right) | .p) | {min: min, max: max}),
      p_wrong: ($rows | map(select(.right | not) | .p) | {min: min, max: max}),
      confidence_right: ($with | map(select(.right) | .confidence) | {min: min, max: max}),
      confidence_wrong: ($with | map(select(.right | not) | .confidence) | {min: min, max: max})
    },
    probability: side($rows; .p),
    confidence: side($with; .confidence)
  }

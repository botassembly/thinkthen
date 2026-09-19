# shift.jq — what moved between two `choose` runs over the same cases.
#
# Reads: the later run's rows on the command line, one JSON object per line, and
#   the earlier run's rows through --slurpfile. Run it with
#   `jq -n --slurpfile before BASE_ROWS -f shift.jq OTHER_ROWS`.
# Arguments:
#   $before  the earlier run's rows, as --slurpfile binds them.
# Policies:
#   - Rows are paired by `input.id`. An id that appears twice in either run is
#     listed in `repeated_ids` and paired in nothing.
#   - `answer.pick` is read rather than `value`, because neither run applied a
#     threshold and the pick is what an option list moves.
#   - The winning probability is the largest entry of `answer.probabilities`.
#     The two runs may send different option lists, so the winner is found in
#     each run on its own and never assumed to be the same label.
#   - `p_on_the_base_pick` follows one label across the two runs: the label the
#     earlier run picked, and what the later run gave that same label. A label
#     absent from the later run's option list yields null for that row.
#   - `accuracy` is against `input.label` and is reported for both runs, so a
#     move that changed a wrong answer into a right one is visible.
#   - Counts are exact and rates are rounded to four decimals.

def round4: . * 10000 | round | . / 10000;
def winner: .answer.probabilities | to_entries | max_by(.value);
def repeated: group_by(.) | [.[] | select(length > 1) | .[0]];

[inputs] as $after
| ([$before[].input.id] | repeated) as $before_repeated
| ([$after[].input.id] | repeated) as $after_repeated
| ([$before[] | select(.input.id as $id | $before_repeated | index($id) | not)]
   | INDEX(.input.id)) as $old
| ([$after[] | select(.input.id as $id | $after_repeated | index($id) | not)]
   | INDEX(.input.id)) as $new
| [$new | keys[] as $id | select($old | has($id))
   | {id: $id,
      label: $old[$id].input.label,
      hard: $old[$id].input.hard,
      before_pick: ($old[$id] | winner | .key),
      after_pick: ($new[$id] | winner | .key),
      before_p: ($old[$id] | winner | .value),
      after_p: ($new[$id] | winner | .value),
      base_label_p_after: ((($old[$id] | winner | .key) as $k
                            | $new[$id].answer.probabilities[$k]))}] as $pairs
| ($pairs | map(select(.before_pick != .after_pick))) as $moved
| {
    repeated_ids: {before: $before_repeated, after: $after_repeated},
    only_in_before: [$old | keys[] as $id | select($new | has($id) | not) | $id],
    only_in_after: [$new | keys[] as $id | select($old | has($id) | not) | $id],
    paired: ($pairs | length),
    options: {
      before: ($before[0].question.options),
      after: ($after[0].question.options)
    },
    pick_changed: ($moved | length),
    pick_changed_rows: ($moved | map({id, label, hard, before_pick, after_pick,
                                      before_p, after_p})),
    accuracy: {
      before: ($pairs | map(select(.before_pick == .label)) | length),
      after: ($pairs | map(select(.after_pick == .label)) | length),
      of: ($pairs | length)
    },
    winning_probability_move: {
      mean_absolute: (($pairs | map(.after_p - .before_p | fabs) | add)
                      / ($pairs | length) | round4),
      max_absolute: ($pairs | map(.after_p - .before_p | fabs) | max | round4),
      rows_unmoved: ($pairs | map(select(.after_p == .before_p)) | length),
      rows_moved_over_a_tenth: ($pairs | map(select(.after_p - .before_p | fabs > 0.1))
                                | map({id, before_p, after_p}))
    },
    same_label_probability_move: {
      rows_with_the_label_present: ($pairs | map(select(.base_label_p_after != null)) | length),
      mean_absolute: (($pairs | map(select(.base_label_p_after != null)
                                    | .base_label_p_after - .before_p | fabs) | add)
                      / ($pairs | map(select(.base_label_p_after != null)) | length)
                      | round4),
      max_absolute: ($pairs | map(select(.base_label_p_after != null)
                                  | .base_label_p_after - .before_p | fabs) | max | round4)
    }
  }

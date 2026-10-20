# score.jq — accuracy, precision, recall, and F1 of a run against the labels.
#
# Reads: the rows of one run, one JSON object per line. The trusted label is
#   `input.label` and the probability of yes is `answer.probability`. Run it
#   with `jq -n --argjson cut 0.5 -f score.jq ROWS`.
# Arguments:
#   $cut  a number for a single cut, or a pair [low, high] for a band. The cut
#         is applied to the stored probability, so no request is made and the
#         run's own threshold does not bind.
# Policies:
#   - The label set is yes and no. A row whose `input.label` is neither true
#     nor false is listed by id in `unlabeled` and is scored in nothing. No
#     row is dropped silently.
#   - Under a band, a row between the two sides is not sure. Those rows
#     are counted apart, and they are never scored right or wrong. The test is
#     an explicit three-way `if`.
#   - `coverage` is the share of labeled rows that resolved. Every other rate
#     is over the resolved, labeled rows alone.
#   - A zero denominator yields null. Precision is null with no yes answer,
#     recall is null with no true label, and F1 is null when either is null or
#     when both are zero.
#   - Counts are exact. Every rate is rounded to four decimals.
#   - A row with no probability stops the transform.
#   - Repeated case ids must pass through trials.jq before this metric.

def round4: if . == null then null else (. * 10000 | round) / 10000 end;

def verdict($p):
  if ($cut | type) == "array" then
    if $p >= $cut[1] then "yes" elif $p < $cut[0] then "no" else "unsure" end
  elif ($cut | type) == "number" then
    if $p >= $cut then "yes" else "no" end
  else error("cut is a number, or a pair [low, high]")
  end;

def has_repeated_ids($rows):
  [$rows[] | select((.input? | type) == "object" and (.input | has("id")))
   | .input.id]
  | group_by(.) | any(.[]; length > 1);

[inputs] as $rows
| if has_repeated_ids($rows)
  then error("metric: repeated case ids; run trials.jq first")
  else reduce $rows[] as $row (
  {rows: 0, unlabeled: [], unsure: 0, tp: 0, fp: 0, tn: 0, fn: 0};
  .rows += 1
  | ($row.input.id // "with no id") as $id
  | $row.answer.probability as $p
  | if ($p | type) != "number" then error("row \($id) carries no probability") else . end
  | $row.input.label as $label
  | if $label != true and $label != false then .unlabeled += [$id]
    else
      verdict($p) as $said
      | if $said == "unsure" then .unsure += 1
        elif $said == "yes" and $label then .tp += 1
        elif $said == "yes" then .fp += 1
        elif $label then .fn += 1
        else .tn += 1
        end
    end
  )
| (.tp + .fp + .tn + .fn) as $resolved
| ($resolved + .unsure) as $labeled
| (if .tp + .fp == 0 then null else .tp / (.tp + .fp) end) as $precision
| (if .tp + .fn == 0 then null else .tp / (.tp + .fn) end) as $recall
| {
    cut: $cut,
    rows,
    labeled: $labeled,
    unlabeled,
    unsure,
    true_positive: .tp,
    false_positive: .fp,
    true_negative: .tn,
    false_negative: .fn,
    coverage: (if $labeled == 0 then null else $resolved / $labeled | round4 end),
    accuracy: (if $resolved == 0 then null else (.tp + .tn) / $resolved | round4 end),
    precision: ($precision | round4),
    recall: ($recall | round4),
    f1: (
      if $precision == null or $recall == null or $precision + $recall == 0 then null
      else 2 * $precision * $recall / ($precision + $recall) | round4
      end
    )
  }
  end

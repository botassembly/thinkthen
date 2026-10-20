# band.jq — what a band bought and what it cost: accuracy beside coverage.
#
# Reads: the rows of one run, one JSON object per line. Run it with
#   `jq -n --argjson band '[0.2,0.8]' -f band.jq ROWS`.
# Arguments:
#   $band  the pair [low, high]. A probability at or above high is yes, below
#          low is no, and anything from low up to high is not sure, which is
#          the rule in specification/threshold.md. The band is applied to
#          the stored probability, so no request is made.
# Policies:
#   - Unsure rows are counted apart. They are never scored right or wrong,
#     and they never enter `accuracy_resolved`.
#   - `coverage` is the share of labeled rows that resolved. Accuracy without
#     coverage beside it says nothing: a band that refuses half the run buys
#     its accuracy with the rows it would not answer.
#   - `accuracy_unsure` is what the refused rows would have scored at a
#     plain cut of 0.5. It answers the one question a band raises: were those
#     rows worth refusing, or would the model have got them right anyway.
#   - A row whose `input.label` is neither true nor false is listed by id in
#     `unlabeled` and is scored in nothing. It still appears in `refused` when
#     the band refused it, because a refused row is a row a person must read.
#   - A zero denominator yields null.
#   - A row with no probability stops the transform. jq reads a missing number as
#     below the low side, so an unguarded band would score it a no.
#   - Counts are exact. Every rate is rounded to four decimals.
#   - Repeated case ids must pass through trials.jq before this metric.

def round4: if . == null then null else (. * 10000 | round) / 10000 end;

def verdict($p):
  if $p >= $band[1] then "yes" elif $p < $band[0] then "no" else "unsure" end;

def has_repeated_ids($rows):
  [$rows[] | select((.input? | type) == "object" and (.input | has("id")))
   | .input.id]
  | group_by(.) | any(.[]; length > 1);

[inputs] as $rows
| if has_repeated_ids($rows)
  then error("metric: repeated case ids; run trials.jq first")
  else reduce $rows[] as $row (
  {rows: 0, unlabeled: [], refused: [], right: 0, wrong: 0, refused_right: 0, refused_wrong: 0};
  .rows += 1
  | ($row.input.id // "with no id") as $id
  | $row.answer.probability as $p
  | if ($p | type) != "number" then error("row \($id) carries no probability") else . end
  | $row.input.label as $label
  | verdict($p) as $said
  | (if $said == "unsure" then .refused += [{id: $id, label: $label, probability: $p}] else . end)
  | if $label != true and $label != false then .unlabeled += [$id]
    elif $said == "unsure" then
      if ($p >= 0.5) == $label then .refused_right += 1 else .refused_wrong += 1 end
    elif ($said == "yes") == $label then .right += 1
    else .wrong += 1
    end
  )
| (.right + .wrong) as $resolved
| (.refused_right + .refused_wrong) as $unsure
| {
    band: $band,
    rows,
    labeled: ($resolved + $unsure),
    unlabeled,
    resolved: $resolved,
    unsure: $unsure,
    coverage: (
      if $resolved + $unsure == 0 then null
      else $resolved / ($resolved + $unsure) | round4
      end
    ),
    accuracy_resolved: (if $resolved == 0 then null else .right / $resolved | round4 end),
    accuracy_unsure: (
      if $unsure == 0 then null else .refused_right / $unsure | round4 end
    ),
    refused
  }
  end

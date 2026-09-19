# calibration.jq — each probability band beside the share of cases that were
# truly yes.
#
# Reads: the rows of one run, one JSON object per line. Run it with
#   `jq -n -f calibration.jq ROWS`.
# Arguments: none. The table is ten bands a tenth wide. A band holds the rows
#   at or above its low side and below its high side, and the last band also
#   holds a probability of exactly 1.
# Policies:
#   - A band is honest when `share_truly_yes` sits near `mean_probability`.
#     The pair is the whole point of the table, and neither number alone says
#     anything.
#   - A row whose `input.label` is neither true nor false is listed by id in
#     `unlabeled` and enters no share. It is still counted in the band's
#     `rows`, so a band never hides a case.
#   - `unresolved` counts the rows in the band whose saved `value` is null,
#     the answers the run's own threshold refused. They are counted apart and
#     they still carry a probability, so they stay in the band.
#   - A band with no labeled row yields null for both rates. A small run
#     leaves most bands empty, and an empty band is not evidence.
#   - Counts are exact. Every rate is rounded to four decimals.

def round4: if . == null then null else (. * 10000 | round) / 10000 end;

def slot($p): $p * 10 | floor | if . > 9 then 9 elif . < 0 then 0 else . end;

reduce inputs as $row (
  {
    rows: 0,
    unlabeled: [],
    bands: [range(0; 10) | {rows: 0, unresolved: 0, labeled: 0, truly_yes: 0, total: 0}]
  };
  .rows += 1
  | ($row.input.id // "with no id") as $id
  | $row.answer.probability as $p
  | $row.input.label as $label
  | slot($p) as $i
  | .bands[$i].rows += 1
  | .bands[$i].total += $p
  | (if $row.value == null then .bands[$i].unresolved += 1 else . end)
  | if $label != true and $label != false then .unlabeled += [$id]
    else
      .bands[$i].labeled += 1
      | if $label then .bands[$i].truly_yes += 1 else . end
    end
)
| {
    rows,
    unlabeled,
    bands: [
      range(0; 10) as $i
      | .bands[$i]
      | {
          band: "\($i / 10)-\(($i + 1) / 10)",
          rows,
          unresolved,
          labeled,
          truly_yes,
          share_truly_yes: (if .labeled == 0 then null else .truly_yes / .labeled | round4 end),
          mean_probability: (if .rows == 0 then null else .total / .rows | round4 end)
        }
    ]
  }

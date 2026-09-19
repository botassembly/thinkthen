# sweep.jq — what every cut would have done, and which cut to pick.
#
# Reads: the rows of one run, one JSON object per line. Run it with
#   `jq -n -f sweep.jq ROWS`. The whole run is held in memory, because every
#   cut reads every row.
# Arguments: none. The grid is the 19 cuts from 0.05 to 0.95, a twentieth
#   apart. jq has no optional argument, so a grid given on the command line
#   would have to be given on every call. Another grid is a one-line edit here.
# Policies:
#   - The cut is applied to the stored probability, so the sweep makes no
#     request and the run's own threshold does not bind.
#   - A row whose `input.label` is neither true nor false is listed by id in
#     `unlabeled` and is scored at no cut.
#   - A single cut answers every row, so `unresolved` is zero at every line of
#     the grid. The column stays, because the same arithmetic under a band
#     fills it, and band.jq reads a band.
#   - A zero denominator yields null, and a null F1 never wins the pick.
#   - A row with no probability stops the recipe. jq reads a missing number as
#     below every cut, so an unguarded sweep would score it a no and print a
#     rate nobody could tell from a real one.
#   - `pick` is the cut with the highest F1. When several cuts tie, the pick is
#     the middle one of the tied run, because it sits farthest from both edges
#     of the gap. An even number of ties has two middles, and the pick is the
#     higher of the two. `pick.rule` says so in the output.
#   - Counts are exact. Every rate is rounded to four decimals.
#   - The verdict and the arithmetic are copied from score.jq. jq shares code
#     only through `include` and a search path, and two short copies read
#     better here than a module every command line has to name.

def round4: if . == null then null else (. * 10000 | round) / 10000 end;

def verdict($p; $cut):
  if ($cut | type) == "array" then
    if $p >= $cut[1] then "yes" elif $p <= $cut[0] then "no" else "unresolved" end
  else
    if $p >= $cut then "yes" else "no" end
  end;

def metrics($rows; $cut):
  reduce $rows[] as $row (
    {unlabeled: [], unresolved: 0, tp: 0, fp: 0, tn: 0, fn: 0};
    ($row.input.id // "with no id") as $id
    | $row.answer.probability as $p
    | if ($p | type) != "number" then error("row \($id) carries no probability") else . end
    | $row.input.label as $label
    | if $label != true and $label != false then .unlabeled += [$id]
      else
        verdict($p; $cut) as $said
        | if $said == "unresolved" then .unresolved += 1
          elif $said == "yes" and $label then .tp += 1
          elif $said == "yes" then .fp += 1
          elif $label then .fn += 1
          else .tn += 1
          end
      end
  )
  | (.tp + .fp + .tn + .fn) as $resolved
  | ($resolved + .unresolved) as $labeled
  | (if .tp + .fp == 0 then null else .tp / (.tp + .fp) end) as $precision
  | (if .tp + .fn == 0 then null else .tp / (.tp + .fn) end) as $recall
  | {
      cut: $cut,
      labeled: $labeled,
      unlabeled,
      unresolved,
      coverage: (if $labeled == 0 then null else $resolved / $labeled | round4 end),
      accuracy: (if $resolved == 0 then null else (.tp + .tn) / $resolved | round4 end),
      precision: ($precision | round4),
      recall: ($recall | round4),
      f1: (
        if $precision == null or $recall == null or $precision + $recall == 0 then null
        else 2 * $precision * $recall / ($precision + $recall) | round4
        end
      )
    };

[inputs] as $rows
| [range(1; 20) | . / 20 | . * 100 | round | . / 100] as $cuts
| [$cuts[] | metrics($rows; .)] as $sweep
| ($sweep | map(select(.f1 != null)) | max_by(.f1) | .f1) as $best
| {
    rows: ($rows | length),
    labeled: $sweep[0].labeled,
    unlabeled: $sweep[0].unlabeled,
    pick: (
      if $best == null then null
      else
        [$sweep[] | select(.f1 == $best)] as $tied
        | $tied[$tied | length | . / 2 | floor]
        | {
            cut,
            accuracy,
            f1,
            tied_cuts: [$tied[].cut],
            rule: "the highest F1, and the middle cut of the cuts that tie"
          }
      end
    ),
    sweep: [$sweep[] | {cut, coverage, unresolved, accuracy, precision, recall, f1}]
  }

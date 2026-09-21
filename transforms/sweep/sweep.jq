# sweep.jq — what every meaningful cut would have done.
#
# Reads: one detailed decide, choose, or score run, one JSON object per line.
# Run it with `jq -n -f sweep.jq ROWS`. The whole run is held in memory.
# Arguments: none. The grid is the 19 cuts from 0.05 to 0.95, a twentieth
#   apart. jq has no optional argument, so a grid given on the command line
#   would have to be given on every call. Another grid is a one-line edit here.
# Decide policies:
#   - The cut is applied to the stored probability, so the sweep makes no
#     request and the run's own threshold does not bind.
#   - A row whose `input.label` is neither true nor false is listed by id in
#     `unlabeled` and is scored at no cut.
#   - A single cut answers every row, so `unresolved` is zero at every line of
#     the grid. The column stays, because the same arithmetic under a band
#     fills it, and band.jq reads a band.
#   - A zero denominator yields null, and a null F1 never wins the pick.
#   - A row with no probability stops the transform. jq reads a missing number as
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
# Choice and score policies:
#   - A choice cut is a minimum winning probability. A unique leader at or
#     above it resolves. A lower leader and every exact tie stay unresolved.
#     Ties enter neither accuracy rate because their stored pick is arbitrary.
#   - A score cut is each integer boundary between ordered levels. Stored value
#     and trusted numeric label are positive at or above it.
#   - Choice and score reports make no automatic pick. Coverage and accuracy
#     trade off for a choice, and each score boundary asks a different question.
#   - The grid stays at twentieths. Current backend probabilities have two
#     decimal places, so finer cuts add no resolution to these recorded rows.
#     Neighboring cuts can therefore have identical reports.
#   - One nonempty run carries one answer kind and one ordered option or level
#     list. Fields used by the report are validated with data-free errors.

def round4: if . == null then null else (. * 10000 | round) / 10000 end;

def rate($top; $bottom):
  if $bottom == 0 then null else $top / $bottom | round4 end;

def verdict($p; $cut):
  if ($cut | type) == "array" then
    if $p >= $cut[1] then "yes" elif $p <= $cut[0] then "no" else "unresolved" end
  else
    if $p >= $cut then "yes" else "no" end
  end;

def decision_metrics($rows; $cut):
  reduce $rows[] as $row (
    {unlabeled: [], unresolved: 0, tp: 0, fp: 0, tn: 0, fn: 0};
    $row.answer.probability as $p
    | $row.input.label as $label
    | if $label != true and $label != false then .unlabeled += [($row.input.id // "with no id")]
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
      coverage: rate($resolved; $labeled),
      accuracy: rate(.tp + .tn; $resolved),
      precision: ($precision | round4),
      recall: ($recall | round4),
      f1: (
        if $precision == null or $recall == null or $precision + $recall == 0 then null
        else 2 * $precision * $recall / ($precision + $recall) | round4
        end
      )
    };

def decision_report($rows; $cuts):
  if all($rows[]; (.input | type) == "object")
  then . else error("sweep: decision rows must carry an input object") end
  | if all($rows[]; (.question | type) == "object")
    then . else error("sweep: decision rows must carry a question object") end
  | if all($rows[]; (.answer.probability | type) == "number")
    then . else error("sweep: decision rows must carry a numeric probability") end
  | [$cuts[] | decision_metrics($rows; .)] as $sweep
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
    };

def valid_names:
  type == "array" and length >= 2 and all(.[]; type == "string")
  and (unique | length) == length;

def choice_report($rows; $cuts):
  if all($rows[]; (.input | type) == "object")
  then . else error("sweep: choice rows must carry an input object") end
  | if all($rows[]; (.question | type) == "object")
    then . else error("sweep: choice rows must carry a question object") end
  | ($rows[0].question.options) as $options
  | if ($options | valid_names) and all($rows[]; .question.options == $options)
    then . else error("sweep: choice rows must carry one shared ordered option list") end
  | if all($rows[];
      (.answer.probabilities | type) == "object"
      and ((.answer.probabilities | keys) == ($options | sort)))
    then . else error("sweep: choice probabilities must have exactly the option keys") end
  | if all($rows[];
      .answer.probabilities as $p
      | all($options[]; ($p[.] | type) == "number" and $p[.] >= 0 and $p[.] <= 1))
    then . else error("sweep: choice probabilities must be numbers from zero through one") end
  | if all($rows[];
      .answer.probabilities as $p
      | ($options | map($p[.]) | max) as $maximum
      | .answer.pick == first($options[] | select($p[.] == $maximum)))
    then . else error("sweep: a choice pick must be the first option at the maximum probability") end
  | if all($rows[]; has("value") and (.value == null or .value == .answer.pick))
    then . else error("sweep: a non-null choice value must equal its pick") end
  | [$rows[] | .input.label as $label
             | select(($label | type) != "string" or ($options | index($label)) == null)
             | (.input.id // "with no id")] as $unlabeled
  | [$rows[] | .input.label as $label
             | select(($label | type) == "string" and ($options | index($label)) != null)] as $labeled
  | [$cuts[] as $cut
      | reduce $labeled[] as $row (
          {cut: $cut, resolved: 0, unresolved: 0, ties: 0,
           right_resolved: 0, unique_unresolved: 0, right_unresolved: 0};
          $row.answer.probabilities as $p
          | ($options | map($p[.]) | max) as $maximum
          | [$options[] | select($p[.] == $maximum)] as $leaders
          | if ($leaders | length) > 1 then
              .unresolved += 1 | .ties += 1
            elif $maximum >= $cut then
              .resolved += 1
              | if $row.answer.pick == $row.input.label then .right_resolved += 1 else . end
            else
              .unresolved += 1 | .unique_unresolved += 1
              | if $row.answer.pick == $row.input.label then .right_unresolved += 1 else . end
            end
        )
      | {
          cut,
          resolved,
          unresolved,
          ties,
          coverage: rate(.resolved; $labeled | length),
          accuracy_resolved: rate(.right_resolved; .resolved),
          accuracy_unresolved: rate(.right_unresolved; .unique_unresolved)
        }
    ] as $sweep
  | {mode: "choose", rows: ($rows | length), labeled: ($labeled | length), unlabeled: $unlabeled,
     options: $options, sweep: $sweep};

def score_metrics($rows; $cut):
  reduce $rows[] as $row (
    {tp: 0, fp: 0, tn: 0, fn: 0};
    ($row.value >= $cut) as $said
    | ($row.input.label >= $cut) as $label
    | if $said and $label then .tp += 1
      elif $said then .fp += 1
      elif $label then .fn += 1
      else .tn += 1
      end
  )
  | (if .tp + .fp == 0 then null else .tp / (.tp + .fp) end) as $precision
  | (if .tp + .fn == 0 then null else .tp / (.tp + .fn) end) as $recall
  | {
      cut: $cut,
      accuracy: rate(.tp + .tn; $rows | length),
      precision: ($precision | round4),
      recall: ($recall | round4),
      f1: (if $precision == null or $recall == null or $precision + $recall == 0 then null
           else 2 * $precision * $recall / ($precision + $recall) | round4 end)
    };

def score_report($rows):
  if all($rows[]; (.input | type) == "object")
  then . else error("sweep: score rows must carry an input object") end
  | if all($rows[]; (.question | type) == "object")
    then . else error("sweep: score rows must carry a question object") end
  | ($rows[0].question.levels) as $levels
  | if ($levels | valid_names) and all($rows[]; .question.levels == $levels)
    then . else error("sweep: score rows must carry one shared ordered level list") end
  | ($levels | length) as $count
  | if all($rows[]; (.value | type) == "number" and .value >= 0 and .value <= $count - 1)
    then . else error("sweep: score values must be numbers from zero through the last level") end
  | [$rows[] | select((.input.label | type) != "number"
                      or .input.label != (.input.label | floor)
                      or .input.label < 0 or .input.label >= $count)
             | (.input.id // "with no id")] as $unlabeled
  | [$rows[] | select((.input.label | type) == "number"
                      and .input.label == (.input.label | floor)
                      and .input.label >= 0 and .input.label < $count)] as $labeled
  | {mode: "score", rows: ($rows | length), labeled: ($labeled | length), unlabeled: $unlabeled,
     levels: $levels, sweep: [range(1; $count) | score_metrics($labeled; .)]};

[inputs] as $rows
| [range(1; 20) | . / 20 | . * 100 | round | . / 100] as $cuts
| if ($rows | length) == 0 then decision_report($rows; $cuts)
  elif all($rows[]; type == "object") | not then error("sweep: every row must be an object")
  elif all($rows[]; (.answer | type) == "object") | not then error("sweep: every row must carry an answer object")
  else
    [$rows[].answer.kind] | unique as $kinds
    | if ($kinds | length) != 1 then error("sweep: one run must carry one answer kind")
      elif $kinds[0] == "yes_no" then decision_report($rows; $cuts)
      elif $kinds[0] == "choice" then choice_report($rows; $cuts)
      elif $kinds[0] == "score" then score_report($rows)
      else error("sweep: one run must carry one answer kind")
      end
  end

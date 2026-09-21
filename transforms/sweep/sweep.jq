# sweep.jq — what every meaningful cut would have done.
#
# Reads: one detailed decide, choose, score, tag, or annotate run, one JSON
#   object per line.
# Run it with `jq -n -f sweep.jq ROWS`. The whole run is held in memory.
# Argument: optional `--arg group POINTER`. It fits one decision cut inside
#   each string-valued record group at that RFC 6901 JSON Pointer. Groups are
#   ordered by name. Choice and score rows do not have an automatic cut to fit.
# Argument: optional `--arg truth POINTER` on tag rows, or `--argjson truth
#   MAP` on annotate rows. The pointer or map locates trusted human labels in
#   each whole result row. Tag labels and mapped annotate answers get separate
#   reports through the same decision and choice arithmetic used below.
#   Missing or null truth stays unlabeled. Present malformed truth fails.
#   `group` and `truth` cannot be combined.
# The grid is the 19 cuts from 0.05 to 0.95, a twentieth
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
#   - Repeated case ids must pass through trials.jq before this metric.

def round4: if . == null then null else (. * 10000 | round) / 10000 end;

def rate($top; $bottom):
  if $bottom == 0 then null else $top / $bottom | round4 end;

def has_repeated_ids($rows):
  [$rows[] | select((.input? | type) == "object" and (.input | has("id")))
   | .input.id]
  | group_by(.) | any(.[]; length > 1);

def verdict($p; $cut):
  if ($cut | type) == "array" then
    if $p >= $cut[1] then "yes" elif $p <= $cut[0] then "no" else "unresolved" end
  else
    if $p >= $cut then "yes" else "no" end
  end;

def pointer_path($pointer):
  if $pointer == "" then []
  elif ($pointer | startswith("/")) | not then
    error("sweep: group must be an RFC 6901 JSON Pointer")
  else
    ($pointer | split("/") | .[1:]) as $parts
    | if any($parts[]; test("~([^01]|$)")) then
        error("sweep: group must be an RFC 6901 JSON Pointer")
      else
        [$parts[] | gsub("~1"; "/") | gsub("~0"; "~")]
      end
  end;

def pointer_value($row; $path):
  reduce $path[] as $part ($row;
    if type == "object" then .[$part]
    elif type == "array" and ($part | test("^(0|[1-9][0-9]*)$")) then
      .[$part | tonumber]
    else null
    end
  );

def truth_path($pointer):
  if ($pointer | type) != "string" or ($pointer != "" and (($pointer | startswith("/")) | not)) then
    error("sweep: truth must contain RFC 6901 JSON Pointers")
  elif $pointer == "" then []
  else
    ($pointer | split("/") | .[1:]) as $parts
    | if any($parts[]; test("~([^01]|$)")) then
        error("sweep: truth must contain RFC 6901 JSON Pointers")
      else
        [$parts[] | gsub("~1"; "/") | gsub("~0"; "~")]
      end
  end;

def validate_case_ids($rows; $shape):
  if all($rows[]; (.input | type) == "object" and (.input.id | type) == "string")
  then $rows else error("sweep: \($shape) rows must carry a string case id") end
  | if ([$rows[].input.id] | unique | length) == ($rows | length)
    then $rows else error("sweep: \($shape) case ids must be unique") end;

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

def validate_decision_rows($rows):
  $rows
  | if all(.[]; (.input | type) == "object")
    then . else error("sweep: decision rows must carry an input object") end
  | if all($rows[]; (.question | type) == "object")
    then . else error("sweep: decision rows must carry a question object") end
  | if all($rows[]; (.answer.probability | type) == "number")
    then . else error("sweep: decision rows must carry a numeric probability") end;

def decision_report($rows; $cuts):
  validate_decision_rows($rows) as $rows
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

def grouped_decision_report($rows; $cuts; $pointer):
  pointer_path($pointer) as $path
  | if ($rows | length) == 0 then
      {mode: "grouped_decide", group: $pointer, rows: 0, groups: []}
    else
      validate_decision_rows($rows) as $rows
      | [$rows[] | {value: pointer_value(.; $path), row: .}] as $grouped
      | if all($grouped[]; (.value | type) == "string")
        then . else error("sweep: every row must resolve group to a string") end
      | {
          mode: "grouped_decide",
          group: $pointer,
          rows: ($rows | length),
          groups: [
            $grouped | group_by(.value)[]
            | .[0].value as $value
            | {value: $value} + decision_report([.[].row]; $cuts)
          ]
        }
    end;

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

def validate_tag_rows($rows):
  validate_case_ids($rows; "tag") as $rows
  | if all($rows[]; (.question | type) == "object" and .question.verb == "tag")
    then . else error("sweep: tag rows must carry a tag question") end
  | ($rows[0].question) as $question
  | ($question.labels) as $labels
  | if ($labels | type) == "array" and ($labels | length) > 0
       and all($labels[]; type == "string") and (($labels | unique | length) == ($labels | length))
       and all($rows[]; .question == $question)
    then . else error("sweep: tag rows must carry one shared ordered label list") end
  | ($rows[0].threshold) as $threshold
  | if ($threshold | type) == "number" and $threshold >= 0 and $threshold <= 1
       and all($rows[]; .threshold == $threshold)
    then . else error("sweep: tag rows must carry one shared numeric threshold") end
  | if all($rows[]; (.answer | type) == "object" and .answer.kind == "tag")
    then . else error("sweep: tag rows must carry tag answers") end
  | if all($rows[];
      (.answer.probabilities | type) == "object"
      and ((.answer.probabilities | keys) == ($labels | sort)))
    then . else error("sweep: tag probabilities must have exactly the label keys") end
  | if all($rows[];
      .answer.probabilities as $p
      | all($labels[]; ($p[.] | type) == "number" and $p[.] >= 0 and $p[.] <= 1))
    then . else error("sweep: tag probabilities must be numbers from zero through one") end
  | if all($rows[];
      .answer.probabilities as $p
      | (.value | type) == "array"
      and .value == [$labels[] | select($p[.] >= $threshold)])
    then . else error("sweep: a tag value must equal labels at or above its threshold") end
  | {rows:$rows, labels:$labels};

def tag_report($rows; $cuts; $pointer; $nested):
  truth_path($pointer) as $path
  | validate_tag_rows($rows) as $valid
  | [$valid.rows[] | {row:., truth:(if $nested then .input.label else pointer_value(.; $path) end)}] as $truths
  | if all($truths[];
      .truth as $truth
      | if $truth == null then true
        elif ($truth | type) != "array" then false
        else (($truth | unique | length) == ($truth | length))
             and all($truth[]; . as $label | ($label | type) == "string"
                     and (($valid.labels | index($label)) != null))
        end)
    then . else error("sweep: tag truth must contain unique known labels") end
  | [
      $valid.labels[] as $label
      | [$truths[]
         | .truth as $truth
         | .row
         | .input.label = (if $truth == null then null else ($truth | index($label)) != null end)
         | .answer = {kind:"yes_no", probability:.answer.probabilities[$label]}]
        as $adapted
      | {label:$label} + decision_report($adapted; $cuts)
    ] as $labels
  | if $nested then {rows:($rows | length), labels:$labels}
    else {mode:"tag", truth:$pointer, rows:($rows | length), labels:$labels}
    end;

def validate_annotate_rows($rows):
  validate_case_ids($rows; "annotate") as $rows
  | if all($rows[]; (.value | type) == "object" and (.answers | type) == "object"
                    and ((.value | keys) == (.answers | keys)))
    then . else error("sweep: annotate rows must carry matching value and answer names") end
  | ($rows[0].answers | keys) as $names
  | if all($rows[]; (.answers | keys) == $names)
    then . else error("sweep: annotate rows must carry one shared set of answer names") end
  | if all($rows[]; (.meta.questions_sha256 | type) == "string"
                    and (.meta.questions_sha256 | test("^[0-9a-f]{64}$")))
       and ([$rows[].meta.questions_sha256] | unique | length) == 1
    then . else error("sweep: annotate rows must carry one valid question-set digest") end
  | {rows:$rows, names:$names};

def annotate_answer_rows($rows; $name; $path):
  [$rows[]
   | pointer_value(.; $path) as $truth
   | .answers[$name] as $nested
   | {input:(.input + {label:$truth}), value:$nested.value,
      question:$nested.question, threshold:$nested.threshold, answer:$nested.answer}];

def validate_annotate_answer($rows; $name):
  if all($rows[]; (.answers[$name] | type) == "object"
                  and (.answers[$name].question | type) == "object"
                  and (.answers[$name].answer | type) == "object")
    then . else error("sweep: annotate rows must carry complete mapped answers") end
  | if all($rows[]; .answers[$name].value == .value[$name])
    then . else error("sweep: an annotate answer must equal its outer value") end
  | if ([$rows[].answers[$name].question] | unique | length) == 1
       and ([$rows[].answers[$name].threshold] | unique | length) == 1
       and ([$rows[].answers[$name].answer.kind] | unique | length) == 1
    then . else error("sweep: a mapped annotate answer must keep one definition") end
  | ($rows[0].answers[$name]) as $first
  | if (($first.question.verb == "decide" and $first.answer.kind == "yes_no")
        or ($first.question.verb == "choose" and $first.answer.kind == "choice")
        or ($first.question.verb == "tag" and $first.answer.kind == "tag"))
    then . else error("sweep: an annotate question verb must match its answer kind") end
  | $first;

def annotate_report($rows; $cuts; $truth):
  if ($truth | type) != "object" or ($truth | length) == 0
       or (all($truth[]; type == "string") | not)
    then error("sweep: annotate truth must be a nonempty object of JSON Pointers")
  else . end
  | validate_annotate_rows($rows) as $valid
  | if all(($truth | keys)[]; . as $name | ($valid.names | index($name)) != null)
    then . else error("sweep: annotate truth names an unknown question") end
  | (reduce ($truth | to_entries[]) as $mapping (
      {};
      truth_path($mapping.value) as $path
      | ($valid.rows[0].answers[$mapping.key].answer.kind) as $kind
      | (if (["yes_no","choice","tag"] | index($kind)) == null
         then error("sweep: annotate truth accepts decide, choose, or tag answers only")
         else validate_annotate_answer($valid.rows; $mapping.key)
         end) as $first
      | annotate_answer_rows($valid.rows; $mapping.key; $path) as $adapted
      | .[$mapping.key] = (
          if $first.answer.kind == "yes_no" then
            if all($adapted[]; .input.label == null or (.input.label | type) == "boolean")
            then {verb:"decide"} + decision_report($adapted; $cuts)
            else error("sweep: decision truth must be boolean") end
          elif $first.answer.kind == "choice" then
            if all($adapted[]; .input.label as $label
                 | $label == null or
                   (($label | type) == "string" and (($first.question.options | index($label)) != null)))
            then {verb:"choose"} + (choice_report($adapted; $cuts) | del(.mode))
            else error("sweep: choice truth must name a listed option") end
          elif $first.answer.kind == "tag" then
            {verb:"tag"} + tag_report($adapted; $cuts; $mapping.value; true)
          else error("sweep: annotate truth accepts decide, choose, or tag answers only")
          end
        )
    )) as $questions
  | {mode:"annotate", truth:$truth, rows:($rows | length), questions:$questions};

if . != null then error("sweep: read rows from files with jq -n")
else
  [inputs] as $rows
  | [range(1; 20) | . / 20 | . * 100 | round | . / 100] as $cuts
  | ($ARGS.named | has("group")) as $is_grouped
  | ($ARGS.named | has("truth")) as $has_truth
  | if $is_grouped and $has_truth then error("sweep: group and truth cannot be used together")
    elif $has_truth and ($rows | length) == 0 then error("sweep: truth requires a nonempty run")
    elif ($rows | length) > 0 and (all($rows[]; type == "object") | not) then
      error("sweep: every row must be an object")
    elif $has_truth then
      if all($rows[]; (.answers | type) == "object") then
        if ($ARGS.named.truth | type) == "object" then annotate_report($rows; $cuts; $ARGS.named.truth)
        else error("sweep: annotate truth must be a nonempty object of JSON Pointers") end
      elif all($rows[]; (.answer | type) == "object" and .answer.kind == "tag") then
        if ($ARGS.named.truth | type) == "string" then tag_report($rows; $cuts; $ARGS.named.truth; false)
        else error("sweep: tag truth must be an RFC 6901 JSON Pointer") end
      else error("sweep: truth accepts tag or annotate rows only")
      end
    elif has_repeated_ids($rows) then error("metric: repeated case ids; run trials.jq first")
    elif $is_grouped then
      if ($rows | length) > 0 and (all($rows[]; (.answer | type) == "object") | not) then
        error("sweep: every row must carry an answer object")
      elif ($rows | length) > 0 and
           (([$rows[].answer.kind] | unique) != ["yes_no"])
      then error("sweep: grouped mode accepts decision rows only")
      else grouped_decision_report($rows; $cuts; $ARGS.named.group)
      end
    elif ($rows | length) > 0 and all($rows[]; (.answers | type) == "object") then
      error("sweep: annotate rows require --argjson truth MAP")
    elif ($rows | length) > 0 and (all($rows[]; (.answer | type) == "object") | not) then
      error("sweep: every row must carry an answer object")
    elif ($rows | length) > 0 and all($rows[]; .answer.kind == "tag") then
      error("sweep: tag rows require --arg truth POINTER")
    elif ($rows | length) == 0 then decision_report($rows; $cuts)
    else
      [$rows[].answer.kind] | unique as $kinds
      | if ($kinds | length) != 1 then error("sweep: one run must carry one answer kind")
        elif $kinds[0] == "yes_no" then decision_report($rows; $cuts)
        elif $kinds[0] == "choice" then choice_report($rows; $cuts)
        elif $kinds[0] == "score" then score_report($rows)
        else error("sweep: one run must carry one answer kind")
        end
    end
end

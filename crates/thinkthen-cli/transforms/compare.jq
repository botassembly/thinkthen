# compare.jq — what changed between two runs over the same cases.
#
# Reads: later rows on the command line and earlier rows through --slurpfile.
# Run `jq -n --slurpfile before OLD -f compare.jq NEW`.
# `probability_tolerance` is an optional number from zero through one passed
# with --argjson. It defaults to 0.08.
#
# Cases pair by input.id. Repeated ids, missing cases, changed evidence, and
# changed trusted labels stay at record level. Nonempty primary input is
# refused; jq does not run a filter at all for empty ordinary input. Scalar detailed rows retain the
# original report. Annotate detailed rows compare each shared named answer.
# Saved v2 decide verdicts compare under their run threshold; other values
# compare exactly, including tag array order. Yes-or-no probability
# movement is reported independently per question. Malformed annotate rows
# fail with a fixed message that echoes no row content.

def saved_decision:
  .schema? == "thinkthen.result/2" and .question.verb? == "decide"
  and .answer.kind? == "yes_no" and .threshold? != null;

def decision_verdict:
  .answer.probability as $p
  | (try (
      if (.threshold | type) == "number" then [.threshold]
      elif (.threshold | type) == "string"
      then (.threshold | split(":") | map(tonumber))
      else [] end
    ) catch []) as $cuts
  | if ($cuts | length) == 1 and $cuts[0] > 0 and $cuts[0] <= 1
    then $p >= $cuts[0]
    elif ($cuts | length) == 2 and $cuts[0] >= 0 and $cuts[1] <= 1
         and $cuts[0] < $cuts[1]
    then if $p >= $cuts[1] then true elif $p < $cuts[0] then false else null end
    else error("compare: invalid decision threshold") end;

def scalar_value:
  if has("value") | not
  then error("compare: value must be null, boolean, string, or number")
  elif saved_decision then decision_verdict
  elif ((.value | type) as $type | ["null", "boolean", "string", "number"] | index($type)) == null
  then error("compare: value must be null, boolean, string, or number")
  else .value end;

def verdict:
  if .value == true then "yes"
  elif .value == false then "no"
  elif .value == null then "unsure"
  else null end;

def yes_no_probability:
  if ((has("answer") | not) or (.answer | type) != "object" or .answer.kind? != "yes_no")
  then null
  elif ((.answer.probability? | type) == "number"
        and .answer.probability >= 0 and .answer.probability <= 1)
  then .answer.probability
  else error("compare: yes_no probability must be a number from 0 through 1") end;

def magnitude: if . < 0 then -. else . end;
def probability_delta:
  (((.a | yes_no_probability) - (.b | yes_no_probability)) | magnitude
   | . * 1000000000000 | round) / 1000000000000;
def repeated: group_by(.) | [.[] | select(length > 1) | .[0]];
def input_mismatch: ((.b.input | del(.label)) != (.a.input | del(.label)));
def label_mismatch: (.b.input.label != .a.input.label);
def comparable: (input_mismatch | not) and (label_mismatch | not);
def valid_digest:
  if (.meta.question_sha256? | type) == "string"
  then (.meta.question_sha256 | test("\\A[0-9a-f]{64}\\z")) else false end;
def scalar_facts:
  {rows:length, questions:([.[].question.text] | unique),
   models:([.[].meta.model] | unique), thresholds:([.[].threshold] | unique)};

def value_stats($pairs; $tolerance):
  ([$pairs[] | select((.b | yes_no_probability) != null
                      and (.a | yes_no_probability) != null)]) as $probability_pairs
  | ([$pairs[]
      | select(.b.value != .a.value
               or (.b.value == .a.value
                   and (.b | yes_no_probability) != null
                   and (.a | yes_no_probability) != null
                   and probability_delta > $tolerance))
      | {id, before_value:.b.value, after_value:.a.value}
        + (if ((.b | yes_no_probability) != null and (.a | yes_no_probability) != null)
           then {before_probability:(.b | yes_no_probability),
                 after_probability:(.a | yes_no_probability),
                 probability_delta:probability_delta,
                 probability_delta_over_tolerance:(probability_delta > $tolerance)}
           else {} end)] | sort_by(.id)) as $changes
  | ([$probability_pairs[]
      | select(.b.value == .a.value and probability_delta > 0
               and probability_delta <= $tolerance)
      | probability_delta]) as $summarized
  | {compared:($pairs | length),
     same:([$pairs[] | select(.b.value == .a.value)] | length),
     changed_values:([$pairs[] | select(.b.value != .a.value)] | length),
     flips:(reduce ($pairs[]
                    | select((.b | verdict) != null and (.a | verdict) != null
                             and (.b | verdict) != (.a | verdict))) as $pair
                   ({}; .["\($pair.b | verdict) to \($pair.a | verdict)"] += [$pair.id])),
     changes:$changes,
     yes_no_probability:{tolerance:$tolerance,
       compared:($probability_pairs | length),
       changed:([$probability_pairs[] | select(probability_delta > 0)] | length),
       summarized_same_value:($summarized | length),
       largest_summarized_delta:($summarized | max)}};

def row_shape:
  if has("answers")
     or ((.meta? | type) == "object" and (.meta | has("questions_sha256")))
  then "annotate" else "scalar" end;

def valid_named_answer($value):
  try (
    type == "object"
    and has("value")
    and (.value == $value)
    and (.question | type) == "object"
    and has("threshold")
    and ((.threshold | type) as $type
         | ["null", "number", "string"] | index($type) != null)
    and (.answer | type) == "object"
    and (if .answer.kind? == "yes_no"
         then (($value | type) == "boolean" or $value == null)
              and ((.answer.probability? | type) == "number")
              and .answer.probability >= 0 and .answer.probability <= 1
         elif .answer.kind? == "choice"
         then (($value | type) == "string" or $value == null)
         elif .answer.kind? == "tag"
         then (($value | type) == "array" and all($value[]; type == "string"))
         elif .answer.kind? == "score"
         then (($value | type) == "number")
         else false end)
  ) catch false;

def valid_annotate_row:
  try (
    type == "object"
    and (.input | type) == "object" and (.input.id | type) == "string"
    and (.meta.model | type) == "string"
    and (.meta.questions_sha256 | type) == "string"
    and (.meta.questions_sha256 | test("\\A[0-9a-f]{64}\\z"))
    and (.value | type) == "object" and (.answers | type) == "object"
    and ((.value | keys) == (.answers | keys))
    and (. as $row
         | [$row.value | keys[] | . as $name
            | $row.answers[$name] | valid_named_answer($row.value[$name])]
         | all)
  ) catch false;
def valid_annotate_side:
  all(.[]; valid_annotate_row)
  and (([.[] | .value | keys] | unique | length) <= 1);
def side_question_names: if length == 0 then [] else .[0].value | keys end;
def annotate_facts:
  {rows:length, question_sets:([.[].meta.questions_sha256] | unique),
   models:([.[].meta.model] | unique)};
def named_question_facts:
  {questions:([.[].question] | unique), thresholds:([.[].threshold] | unique)};

def question_report($before_rows; $after_rows; $record_pairs; $name; $tolerance):
  ($before_rows | map(.answers[$name])) as $before_answers
  | ($after_rows | map(.answers[$name])) as $after_answers
  | ([$record_pairs[] | {id, b:.b.answers[$name], a:.a.answers[$name]}]) as $answer_pairs
  | ($before_answers | named_question_facts) as $bf
  | ($after_answers | named_question_facts) as $af
  | {before:$bf, after:$af,
     changed:{question:($bf.questions != $af.questions),
              threshold:($bf.thresholds != $af.thresholds)}}
    + value_stats($answer_pairs; $tolerance);

def paired_rows($before_rows; $after_rows):
  ([$before_rows[].input.id] | repeated) as $before_repeated
  | ([$after_rows[].input.id] | repeated) as $after_repeated
  | ([$before_rows[]
      | select(.input.id as $id | $before_repeated | index($id) | not)]
     | INDEX(.input.id)) as $old
  | ([$after_rows[]
      | select(.input.id as $id | $after_repeated | index($id) | not)]
     | INDEX(.input.id)) as $new
  | {before_repeated:$before_repeated, after_repeated:$after_repeated,
     old:$old, new:$new,
     pairs:[$new | keys[] as $id
            | select($old | has($id))
            | {id:$id, b:$old[$id], a:$new[$id]}]};

def scalar_report($before_rows; $after_rows; $tolerance):
  ($before_rows | map(yes_no_probability) | length) as $validated_before_probability
  | ($after_rows | map(yes_no_probability) | length) as $validated_after_probability
  | ($before_rows | map(.value = scalar_value)) as $before_rows
  | ($after_rows | map(.value = scalar_value)) as $after_rows
  | paired_rows($before_rows; $after_rows) as $joined
  | ($before_rows | scalar_facts) as $bf
  | ($after_rows | scalar_facts) as $af
  | ($before_rows | [.[].meta.question_sha256] | unique) as $bdigests
  | ($after_rows | [.[].meta.question_sha256] | unique) as $adigests
  | (if $bf.rows == 0 or $af.rows == 0
     then {question:null, question_by:"unavailable"}
     elif ($before_rows | all(.[]; valid_digest))
          and ($after_rows | all(.[]; valid_digest))
     then {question:($bdigests != $adigests), question_by:"digest"}
     else {question:($bf.questions != $af.questions), question_by:"text"} end) as $question
  | ([$joined.pairs[] | select(input_mismatch) | .id]) as $mismatched_input
  | ([$joined.pairs[] | select(label_mismatch) | .id]) as $mismatched_label
  | ([$joined.pairs[] | select(comparable)]) as $comparable
  | value_stats($comparable; $tolerance) as $stats
  | {before:$bf, after:$af,
     changed:{question:$question.question, question_by:$question.question_by,
              model:($bf.models != $af.models), threshold:($bf.thresholds != $af.thresholds)},
     repeated_ids:{before:$joined.before_repeated, after:$joined.after_repeated},
     only_in_before:[$joined.old | keys[] as $id
                     | select($joined.new | has($id) | not) | $id],
     only_in_after:[$joined.new | keys[] as $id
                    | select($joined.old | has($id) | not) | $id],
     paired:($joined.pairs | length), compared:$stats.compared,
     mismatched_input:$mismatched_input, mismatched_label:$mismatched_label,
     same:$stats.same, changed_values:$stats.changed_values, flips:$stats.flips,
     changes:$stats.changes, yes_no_probability:$stats.yes_no_probability};

def annotate_report($before_rows; $after_rows; $tolerance):
  if (($before_rows | valid_annotate_side)
      and ($after_rows | valid_annotate_side)) | not
  then error("compare: invalid annotate row")
  else
    paired_rows($before_rows; $after_rows) as $joined
    | ([$joined.pairs[] | select(input_mismatch) | .id]) as $mismatched_input
    | ([$joined.pairs[] | select(label_mismatch) | .id]) as $mismatched_label
    | ([$joined.pairs[] | select(comparable)]) as $comparable
    | ($before_rows | side_question_names) as $before_names
    | ($after_rows | side_question_names) as $after_names
    | ([$before_names[] as $name
        | select($after_names | index($name) | not) | $name]) as $only_before
    | ([$after_names[] as $name
        | select($before_names | index($name) | not) | $name]) as $only_after
    | ([$before_names[] as $name
        | select($after_names | index($name)) | $name]) as $shared
    | ($before_rows | annotate_facts) as $bf
    | ($after_rows | annotate_facts) as $af
    | {mode:"annotate", before:$bf, after:$af,
       changed:{question_set:(if $bf.rows == 0 or $af.rows == 0
                              then null else $bf.question_sets != $af.question_sets end),
                model:($bf.models != $af.models)},
       repeated_ids:{before:$joined.before_repeated, after:$joined.after_repeated},
       only_in_before:[$joined.old | keys[] as $id
                       | select($joined.new | has($id) | not) | $id],
       only_in_after:[$joined.new | keys[] as $id
                      | select($joined.old | has($id) | not) | $id],
       paired:($joined.pairs | length), compared:($comparable | length),
       mismatched_input:$mismatched_input, mismatched_label:$mismatched_label,
       questions_only_in_before:$only_before, questions_only_in_after:$only_after,
       questions:(reduce $shared[] as $name ({};
         .[$name] = question_report($before_rows; $after_rows; $comparable;
                                    $name; $tolerance)))}
  end;

if input_filename != null
then "compare: run with jq -n\n" | halt_error(5)
else . end
| ($ARGS.named
   | if has("probability_tolerance") then .probability_tolerance else 0.08 end
   | if type == "number" and . >= 0 and . <= 1 then .
     else error("compare: probability_tolerance must be a number from 0 through 1") end) as $tolerance
| [inputs] as $after
| (($before + $after) | map(row_shape) | unique) as $shapes
| if ($shapes | length) > 1
  then error("compare: rows must all be scalar or annotate")
  elif $shapes == ["annotate"]
  then annotate_report($before; $after; $tolerance)
  else scalar_report($before; $after; $tolerance) end

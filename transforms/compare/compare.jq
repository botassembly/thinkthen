# compare.jq — what changed between two runs over the same cases.
#
# Reads: the rows of the later run on the command line, one JSON object per
#   line, and the rows of the earlier run through --slurpfile. Run it with
#   `jq -n --slurpfile before OLD_ROWS -f compare.jq NEW_ROWS`.
# Arguments:
#   $before  the earlier run's rows, as --slurpfile binds them.
#   probability_tolerance  optional number from 0 through 1, passed with
#     --argjson. It defaults to 0.08.
# Policies:
#   - Cases are paired by `input.id`. An id that appears twice in one run is
#     listed in `repeated_ids` and paired in nothing, because two rows under
#     one id are repeated trials and averaging them is another transform.
#   - A pair is checked beyond its id. `mismatched_input` lists the ids whose
#     evidence differs, and `mismatched_label` lists the ids whose trusted
#     label differs. `compared` counts only pairs with matching evidence and
#     labels. `same` and every flip use those comparable pairs alone. One id
#     can appear in both mismatch lists.
#   - A case in one run alone is listed and never silently dropped.
#   - `value` may be null, boolean, string, or number. `same` counts exact
#     equality, `changed_values` counts the rest, and `changes` lists every
#     changed pair in lexical id order. `flips` retains the six directions for
#     boolean and null values and does not include strings or numbers.
#   - Probability movement is available only when both rows have a valid
#     `answer.kind` of `yes_no` and a probability from zero through one. Every
#     value change stays visible. A same-value movement is listed only when it
#     exceeds the tolerance. Deltas are rounded to twelve decimal places.
#     Legacy rows with no answer remain comparable. A malformed present
#     yes-no probability fails without printing its contents.
#   - `changed.question` uses the unique valid `meta.question_sha256` values
#     when both runs are nonempty and every row has a 64-character lowercase
#     hexadecimal digest. Otherwise a nonempty comparison uses the unique
#     printed `question.text` values and says `question_by: text`. An empty
#     run makes question change null and its identity `unavailable`. The
#     model and threshold checks keep their existing rules.

def scalar_value:
  if has("value") | not
  then error("compare: row \(.input.id // "with no id") value must be null, boolean, string, or number")
  elif ((.value | type) as $type | ["null", "boolean", "string", "number"] | index($type)) == null
  then error("compare: row \(.input.id // "with no id") value must be null, boolean, string, or number")
  else .value
  end;

def verdict:
  if .value == true then "yes"
  elif .value == false then "no"
  elif .value == null then "unresolved"
  else null
  end;

def yes_no_probability:
  if ((has("answer") | not) or (.answer | type) != "object" or .answer.kind? != "yes_no")
  then null
  elif ((.answer.probability? | type) == "number"
        and .answer.probability >= 0 and .answer.probability <= 1)
  then .answer.probability
  else error("compare: yes_no probability must be a number from 0 through 1")
  end;

def magnitude: if . < 0 then -. else . end;

def probability_delta:
  (((.a | yes_no_probability) - (.b | yes_no_probability)) | magnitude
   | . * 1000000000000 | round) / 1000000000000;

def repeated: group_by(.) | [.[] | select(length > 1) | .[0]];

def facts:
  {
    rows: length,
    questions: [.[].question.text] | unique,
    models: [.[].meta.model] | unique,
    thresholds: [.[].threshold] | unique
  };

def input_mismatch:
  ((.b.input | del(.label)) != (.a.input | del(.label)));

def label_mismatch: (.b.input.label != .a.input.label);

def comparable: (input_mismatch | not) and (label_mismatch | not);

def valid_digest:
  if (.meta.question_sha256? | type) == "string"
  then (.meta.question_sha256 | test("\\A[0-9a-f]{64}\\z"))
  else false
  end;

($ARGS.named
 | if has("probability_tolerance") then .probability_tolerance else 0.08 end
 | if type == "number" and . >= 0 and . <= 1
   then .
   else error("compare: probability_tolerance must be a number from 0 through 1")
   end) as $tolerance
| [inputs] as $after
| ($before | map(scalar_value) | length) as $validated_before
| ($after | map(scalar_value) | length) as $validated_after
| ($before | map(yes_no_probability) | length) as $validated_before_probability
| ($after | map(yes_no_probability) | length) as $validated_after_probability
| ([$before[].input.id] | repeated) as $before_repeated
| ([$after[].input.id] | repeated) as $after_repeated
| ([$before[] | select(.input.id as $id | $before_repeated | index($id) | not)] | INDEX(.input.id)) as $old
| ([$after[] | select(.input.id as $id | $after_repeated | index($id) | not)] | INDEX(.input.id)) as $new
| [$new | keys[] as $id | select($old | has($id)) | {id: $id, b: $old[$id], a: $new[$id]}] as $pairs
| ($before | facts) as $bf
| ($after | facts) as $af
| ($before | [.[].meta.question_sha256] | unique) as $bdigests
| ($after | [.[].meta.question_sha256] | unique) as $adigests
| (if $bf.rows == 0 or $af.rows == 0
   then {question: null, question_by: "unavailable"}
   elif ($before | all(.[]; valid_digest))
        and ($after | all(.[]; valid_digest))
   then {question: ($bdigests != $adigests), question_by: "digest"}
   else {question: ($bf.questions != $af.questions), question_by: "text"}
   end) as $question
| ([$pairs[] | select(input_mismatch) | .id]) as $mismatched_input
| ([$pairs[] | select(label_mismatch) | .id]) as $mismatched_label
| ([$pairs[] | select(comparable)]) as $comparable
| ([$comparable[]
    | select((.b | yes_no_probability) != null and (.a | yes_no_probability) != null)]) as $probability_pairs
| ([$comparable[]
    | select(.b.value != .a.value
             or ((.b.value == .a.value)
                 and (.b | yes_no_probability) != null
                 and (.a | yes_no_probability) != null
                 and probability_delta > $tolerance))
    | {id, before_value: .b.value, after_value: .a.value}
      + (if ((.b | yes_no_probability) != null and (.a | yes_no_probability) != null)
         then {before_probability: (.b | yes_no_probability),
               after_probability: (.a | yes_no_probability),
               probability_delta: probability_delta}
         else {}
         end)] | sort_by(.id)) as $changes
| ([$probability_pairs[]
    | select(.b.value == .a.value and probability_delta > 0
             and probability_delta <= $tolerance)
    | probability_delta]) as $summarized
| {
    before: $bf,
    after: $af,
    changed: {
      question: $question.question,
      question_by: $question.question_by,
      model: ($bf.models != $af.models),
      threshold: ($bf.thresholds != $af.thresholds)
    },
    repeated_ids: {before: $before_repeated, after: $after_repeated},
    only_in_before: [$old | keys[] as $id | select($new | has($id) | not) | $id],
    only_in_after: [$new | keys[] as $id | select($old | has($id) | not) | $id],
    paired: ($pairs | length),
    compared: ($comparable | length),
    mismatched_input: $mismatched_input,
    mismatched_label: $mismatched_label,
    same: ([$comparable[] | select(.b.value == .a.value)] | length),
    changed_values: ([$comparable[] | select(.b.value != .a.value)] | length),
    flips: (
      reduce ($comparable[]
              | select((.b | verdict) != null and (.a | verdict) != null
                       and (.b | verdict) != (.a | verdict))) as $pair (
        {};
        .["\($pair.b | verdict) to \($pair.a | verdict)"] += [$pair.id]
      )
    ),
    changes: $changes,
    yes_no_probability: {
      tolerance: $tolerance,
      compared: ($probability_pairs | length),
      changed: ([$probability_pairs[] | select(probability_delta > 0)] | length),
      summarized_same_value: ($summarized | length),
      largest_summarized_delta: ($summarized | max)
    }
  }

# compare.jq — what changed between two runs over the same cases.
#
# Reads: the rows of the later run on the command line, one JSON object per
#   line, and the rows of the earlier run through --slurpfile. Run it with
#   `jq -n --slurpfile before OLD_ROWS -f compare.jq NEW_ROWS`.
# Arguments:
#   $before  the earlier run's rows, as --slurpfile binds them.
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
#   - The verdict read is `value`: true is yes, false is no, and null is
#     unresolved. The test is an explicit three-way `if`, so a flip into or
#     out of unresolved is one of the six directions in `flips` and is never
#     folded into a flip to no.
#   - `changed.question` uses the unique valid `meta.question_sha256` values
#     when both runs are nonempty and every row has a 64-character lowercase
#     hexadecimal digest. Otherwise a nonempty comparison uses the unique
#     printed `question.text` values and says `question_by: text`. An empty
#     run makes question change null and its identity `unavailable`. The
#     model and threshold checks keep their existing rules.

def verdict:
  if has("value") | not then error("row \(.input.id // "with no id") carries no value")
  elif .value == true then "yes"
  elif .value == false then "no"
  elif .value == null then "unresolved"
  else error("row \(.input.id // "with no id") carries a value that is not true, false, or null")
  end;

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

[inputs] as $after
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
    same: ([$comparable[] | select((.b | verdict) == (.a | verdict))] | length),
    flips: (
      reduce ($comparable[] | select((.b | verdict) != (.a | verdict))) as $pair (
        {};
        .["\($pair.b | verdict) to \($pair.a | verdict)"] += [$pair.id]
      )
    )
  }

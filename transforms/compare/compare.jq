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
#     label differs. A changed label means the two runs measured different
#     things, and every number below it is worthless.
#   - A case in one run alone is listed and never silently dropped.
#   - The verdict read is `value`: true is yes, false is no, and null is
#     unresolved. The test is an explicit three-way `if`, so a flip into or
#     out of unresolved is one of the six directions in `flips` and is never
#     folded into a flip to no.
#   - `changed` says why a value could have moved: the question text, the
#     model that answered, or the threshold the run applied. All three come
#     from the rows themselves. When every one of them is false, the model
#     answered the same question the same way twice and the flips are the
#     model's own variance.

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

[inputs] as $after
| ([$before[].input.id] | repeated) as $before_repeated
| ([$after[].input.id] | repeated) as $after_repeated
| ([$before[] | select(.input.id as $id | $before_repeated | index($id) | not)] | INDEX(.input.id)) as $old
| ([$after[] | select(.input.id as $id | $after_repeated | index($id) | not)] | INDEX(.input.id)) as $new
| [$new | keys[] as $id | select($old | has($id)) | {id: $id, b: $old[$id], a: $new[$id]}] as $pairs
| ($before | facts) as $bf
| ($after | facts) as $af
| {
    before: $bf,
    after: $af,
    changed: {
      question: ($bf.questions != $af.questions),
      model: ($bf.models != $af.models),
      threshold: ($bf.thresholds != $af.thresholds)
    },
    repeated_ids: {before: $before_repeated, after: $after_repeated},
    only_in_before: [$old | keys[] as $id | select($new | has($id) | not) | $id],
    only_in_after: [$new | keys[] as $id | select($old | has($id) | not) | $id],
    paired: ($pairs | length),
    mismatched_input: [
      $pairs[] | select((.b.input | del(.label)) != (.a.input | del(.label))) | .id
    ],
    mismatched_label: [$pairs[] | select(.b.input.label != .a.input.label) | .id],
    same: ([$pairs[] | select((.b | verdict) == (.a | verdict))] | length),
    flips: (
      reduce ($pairs[] | select((.b | verdict) != (.a | verdict))) as $pair (
        {};
        .["\($pair.b | verdict) to \($pair.a | verdict)"] += [$pair.id]
      )
    )
  }

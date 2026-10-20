# counts.jq — how many rows a run answered yes, no, and not sure.
#
# Reads: the rows of one run, one JSON object per line, as `decide --details`
#   writes them. Run it with `jq -n -f counts.jq ROWS`.
# Arguments: none.
# Policies:
#   - The answer is `value`, the verdict the run's own threshold gave. true is
#     yes, false is no, and null is not sure. The test is an explicit
#     three-way `if`, because `.value // false` turns a not sure answer into no.
#   - Unsure rows are counted apart and are never folded into no.
#   - A row that carries no `value` stops the transform. A missing answer is a
#     broken run rather than a not sure answer.
#   - `thresholds` and `questions` report what the file holds, so a file that
#     concatenates two runs cannot pass as one.
#   - Repeated case ids must pass through trials.jq before this metric.

def has_repeated_ids($rows):
  [$rows[] | select((.input? | type) == "object" and (.input | has("id")))
   | .input.id]
  | group_by(.) | any(.[]; length > 1);

[inputs] as $rows
| if has_repeated_ids($rows)
  then error("metric: repeated case ids; run trials.jq first")
  else reduce $rows[] as $row (
  {rows: 0, yes: 0, no: 0, unsure: 0, thresholds: [], questions: []};
  .rows += 1
  | if $row | has("value") | not then
      error("row \($row.input.id // "with no id") carries no value")
    elif $row.value == true then .yes += 1
    elif $row.value == false then .no += 1
    elif $row.value == null then .unsure += 1
    else error("row \($row.input.id // "with no id") carries a value that is not true, false, or null")
    end
  | .thresholds = (.thresholds + [$row.threshold] | unique)
  | .questions = (.questions + [$row.question.text] | unique)
  )
  end

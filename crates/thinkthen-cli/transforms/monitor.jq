# monitor.jq — report policy actions, review coverage, and reviewer changes.
#
# Reads: JSONL audit rows after a policy transform. Run it with
#   `jq -n -f monitor.jq ROWS`.
# Arguments: none.
# Policies:
#   - Policy actions and non-null reviewed actions are draft, block, or review.
#   - A missing or null input.reviewed_action means nobody reviewed the row.
#   - Coverage divides reviewed rows by all rows. Agreement divides agreements
#     by reviewed rows. A zero denominator yields null. Rates use four decimals.
#   - Changes retain input order and expose only id, action, and reviewed_action.
#   - Nonempty ordinary input usually means `jq -s` was used by mistake. It is
#     refused before a plausible wrong report can print.

def rate($numerator; $denominator):
  if $denominator == 0 then null
  else (($numerator / $denominator * 10000) | round) / 10000
  end;

def action_report($rows; $action):
  [$rows[] | select(.policy.action == $action)] as $selected
  | [$selected[] | select(.input.reviewed_action != null)] as $reviewed
  | ([$reviewed[] | select(.input.reviewed_action == .policy.action)] | length) as $agreed
  | {rows:($selected | length),
     reviewed:($reviewed | length),
     review_coverage:rate(($reviewed | length); ($selected | length)),
     agreed:$agreed,
     overturned:(($reviewed | length) - $agreed),
     agreement_rate:rate($agreed; ($reviewed | length))};

if input_filename != null
then "monitor: run with jq -n\n" | halt_error(5)
else . end
| [inputs] as $rows
| if all($rows[]; type == "object") | not
  then error("monitor: every row must be an object")
  elif all($rows[];
           (.input? | type) == "object" and (.input.id? | type) == "string") | not
  then error("monitor: every row must carry an object input with a string id")
  elif all($rows[];
           (.policy? | type) == "object"
           and (.policy.action? as $action
                | (["draft", "block", "review"] | index($action)) != null)) | not
  then error("monitor: every row must carry an object policy with action draft, block, or review")
  elif all($rows[];
           (.input | has("reviewed_action") | not)
           or .input.reviewed_action == null
           or (.input.reviewed_action as $reviewed_action
               | (["draft", "block", "review"] | index($reviewed_action)) != null)) | not
  then error("monitor: reviewed_action must be draft, block, review, null, or absent")
  elif ([$rows[].input.id] | length) != ([$rows[].input.id] | unique | length)
  then error("monitor: input ids must be unique")
  else [$rows[] | select(.input.reviewed_action != null)] as $reviewed
    | ([$reviewed[] | select(.input.reviewed_action == .policy.action)] | length) as $agreed
    | {rows:($rows | length),
       reviewed:($reviewed | length),
       unreviewed:(($rows | length) - ($reviewed | length)),
       review_coverage:rate(($reviewed | length); ($rows | length)),
       agreed:$agreed,
       overturned:(($reviewed | length) - $agreed),
       agreement_rate:rate($agreed; ($reviewed | length)),
       actions:{draft:action_report($rows; "draft"),
                block:action_report($rows; "block"),
                review:action_report($rows; "review")},
       changes:[$reviewed[]
                | select(.input.reviewed_action != .policy.action)
                | {id:.input.id,
                   action:.policy.action,
                   reviewed_action:.input.reviewed_action}]}
  end

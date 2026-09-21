# Reads one `annotate --details` row with credential_request, queue, and
# urgency values. Adds a policy object and preserves every existing member.
# Takes no arguments. Refuses missing, malformed, or unknown values.

def refuse($message): error("triage policy: " + $message);
def has_value($name): .value | has($name);

if type != "object" or (.value | type) != "object" then
  refuse("the row has no value object")
elif (has_value("credential_request") | not)
  or (has_value("queue") | not)
  or (has_value("urgency") | not) then
  refuse("the row is missing a required answer")
elif (.value.credential_request != null
      and (.value.credential_request | type) != "boolean") then
  refuse("credential_request is not true, false, or null")
elif (.value.queue as $queue
      | $queue != null
        and (["billing", "shipping", "account", "other"]
             | index($queue)) == null) then
  refuse("queue is not billing, shipping, account, other, or null")
elif (.value.urgency != null
      and ((.value.urgency | type) != "number"
           or .value.urgency < 0
           or .value.urgency > 2)) then
  refuse("urgency is not a number from zero through two or null")
elif ([.value.credential_request, .value.queue, .value.urgency]
      | any(. == null)) then
  . + {policy: {action: "review", reason: "unresolved"}}
elif .value.credential_request then
  . + {policy: {action: "block", reason: "credential_request"}}
elif .value.queue == "other" then
  . + {policy: {action: "review", reason: "out_of_scope"}}
elif .value.urgency >= 1 then
  . + {policy: {action: "review", reason: "urgent"}}
else
  . + {policy: {action: "draft", reason: "routine"}}
end

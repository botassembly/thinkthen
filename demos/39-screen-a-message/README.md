# How to screen one message for several hazards

Status: green

Use one question set when one message needs several independent checks before a person or script acts. This example checks for a secret, a destructive request, and urgent wording in one request.

```bash
env -u THINKTHEN_API_KEY thinkthen annotate hazards.json --replay recording \
  --input message.txt | jq -c 'keys' \
  | mustmatch '["destructive","secret","urgent"]'
```

Verbs: `annotate`

## Input

`message.txt` contains one support message. `hazards.json` gives three narrow yes-or-no questions the backend sees beside that same message. Their shared evidence means they ride in one request.

## Screen the message

```bash
env -u THINKTHEN_API_KEY thinkthen annotate hazards.json --details \
  --replay recording --input message.txt \
  | jq -c '{value, one_request: ([.answers[].request] | unique | length)}' \
  | mustmatch '{"value":{"secret":false,"destructive":false,"urgent":true},"one_request":1}'
```

The policy sends any unresolved check to a person and permits automatic handling only when every required answer is `false`.

```bash
env -u THINKTHEN_API_KEY thinkthen annotate hazards.json --replay recording \
  --input message.txt \
  | jq -r 'if any(.[]; . == null) then "review" elif any(.[]; . == true) then "hold" else "continue" end' \
  | mustmatch 'hold'
```

## What can go wrong

Exit 2 means the input object already owns a question name or the command line is invalid. Exit 4 means the backend failed. Exit 5 means the question set or recording is missing or malformed. A new question changes the whole grouped request and asks it again. Keep the set fixed while comparing runs, and inspect `--details` when an answer sits near its threshold.

## Related how-tos

- [Test a script with no network](../27-test-with-no-network/)
- [Gate a risky command and fail closed](../19-no-or-could-not-ask/)
- [Grade an assistant's answers with a rubric](../14-grade-a-batch/)

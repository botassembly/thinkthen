# How to screen one message for several hazards

Status: green

Use `tag` when one record can have zero, one, or several labels. Descriptions make each label concrete while all labels ride in one request.

```bash
env -u THINKTHEN_API_KEY thinkthen tag @hazards.json --replay recording \
  --input message.txt | mustmatch '["urgent"]'
```

Verbs: `tag`

## Input

`message.txt` contains one support message. `hazards.json` names three labels and describes what evidence makes each one apply.

## Screen the message

The bare result is one JSON array in label order. Raising the local cut above every recorded probability shows the successful empty answer without another backend call.

```bash
env -u THINKTHEN_API_KEY thinkthen tag @hazards.json --threshold .99 \
  --replay recording --input message.txt | mustmatch '[]'
```

Use `--details` before acting on a label near the cut. This recording measured all three probabilities and put `urgent` well above the default cut.

```bash
env -u THINKTHEN_API_KEY thinkthen tag @hazards.json --details \
  --replay recording --input message.txt \
  | jq -c '{value, probabilities: .answer.probabilities}' \
  | mustmatch '{"value":["urgent"],"probabilities":{"secret":0.03,"destructive":0.03,"urgent":0.95}}'
```

## What can go wrong

Exit 2 means the labels, threshold, or command line are invalid. Exit 4 means the backend failed. Exit 5 means the question file or recording is missing or malformed. Adding, removing, describing, or reordering one label changes the whole request and cache key. The rerun asks every label again, and an answer near the cut can move.

## Related how-tos

- [Test a script with no network](../27-test-with-no-network/)
- [Gate a risky command and fail closed](../19-no-or-could-not-ask/)
- [Grade an assistant's answers with a rubric](../14-grade-a-batch/)

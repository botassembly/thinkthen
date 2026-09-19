# How to gate a script step on a yes/no answer

Status: green

Verbs: `decide`

Use this when a script has to take one branch or another and the thing that decides is the meaning of some text. A support desk wants the messages that ask for money back to go to the refunds queue. `decide` answers the question and sets the exit code, so the branch is an ordinary shell `if`.

```bash
set -euo pipefail

if thinkthen decide 'Does the customer ask for money back?' \
     --quiet --replay recording/ < message.txt
then
  printf 'refunds\n'
else
  printf 'normal\n'
fi | mustmatch "refunds"
```

## Input

`message.txt` is one customer message that asks for a refund in plain words. `question.txt` is one customer message that asks a product question and mentions no money.

`recording/` holds the two exchanges this page replays, so every command here runs with no network and no key. `record.sh` made them once through `sdlc/scripts/live`. Every probability on this page is illustrative.

## The other branch

`--quiet` drops the answer from standard output, so the command reads like `test` and needs no redirect. The product question takes the `else`.

```bash
set -euo pipefail

if thinkthen decide 'Does the customer ask for money back?' \
     --quiet --replay recording/ < question.txt
then
  printf 'refunds\n'
else
  printf 'normal\n'
fi | mustmatch "normal"
```

## What can go wrong

- **An `if` has two branches, and `decide` has five outcomes.** Exit 0 is yes, 1 is no, 3 is unresolved, 4 is a backend failure and 5 is a local one. An `else` swallows the last three and routes a failed request to the normal queue. [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) reads all five with `case`.
- **Exit 1 is a no.** Under `set -e` a bare `thinkthen decide ...` ends the script on it. Put the command in an `if`, a `case`, or a `&& rc=0 || rc=$?` list.
- **Exit 2 is a usage error, and it goes out before any request.** `--quiet` beside `--details` is one, because the gate wants no output and the audit wants the object.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask for money back?' \
  --quiet --details --replay recording/ < message.txt \
  >/dev/null 2>&1 && bad=0 || bad=$?
printf 'bad=%s\n' "$bad" | mustmatch "bad=2"
```

- **The model's word never runs anything.** The `if` is code. `decide` moved the exit code and nothing else.
- **A single cut never says the model is sure.** With no threshold the cut is 0.5 and nothing is ever unresolved, so a borderline message lands in one of the two queues with no sign that it was close. A probability of 0.48 and one of 0.02 both come back as a no.

## Related how-tos

- [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) adds the band, the audit row, and the failure branches.
- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) is the same shape over more than two queues.
- [How to test a script with no network](../27-test-with-no-network/) makes a recording like the one this page replays.

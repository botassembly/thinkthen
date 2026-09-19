# How to gate a script step on a yes/no answer

Status: green

Verbs: `decide`

Use this when a script has to take one branch or another and the thing that decides is the meaning of some text. A support desk reads customer messages and wants the ones that ask for money back to go to the refunds queue. `decide` answers the question and sets the exit code, so the branch is an ordinary shell `if` or `case`.

## Input

`message.txt` is one customer message that asks for a refund in plain words. `question.txt` is one customer message that asks a product question and mentions no money.

`recording/` holds the two exchanges this page replays, so every command here runs with no network and no key. `record.sh` made them once through `sdlc/scripts/live`. Every probability on this page is illustrative.

## Step 1: read the answer as a shell test

`--quiet` drops the answer from standard output, so the command reads like `test` and needs no redirect.

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

The product question takes the other branch.

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

## Step 2: add a third branch for the messages that are not clear

An `if` gives two branches. With no threshold the cut is 0.5 and nothing is ever unresolved, so a borderline message lands in one of the two queues with no sign that it was close. A band adds the third answer. Exit 0 is yes, 1 is no, and 3 is unresolved. `&& rc=0 || rc=$?` is what captures a non-zero code under `set -e`.

```bash
set -euo pipefail

route() {
  thinkthen decide 'Does the customer ask for money back?' \
    --threshold 0.1:0.9 --quiet --replay recording/ < "$1"
}

route message.txt && rc=0 || rc=$?
case $rc in
  0) printf 'refunds\n' ;;
  1) printf 'normal\n' ;;
  3) printf 'review\n' ;;
  *) printf 'gate failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "refunds"
```

## Step 3: keep the answer for the audit

The gate above throws the judgment away. Drop `--quiet`, save the result, and read the exit code from the same run.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the customer ask for money back?' \
  --threshold 0.1:0.9 --details --replay recording/ < message.txt \
  > "$work/result.json" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
jq -c '{value, threshold}' "$work/result.json" \
  | mustmatch '{"value":true,"threshold":"0.1:0.9"}'
jq -r '.question.verb' "$work/result.json" | mustmatch "decide"
jq -r '.answer.probability | type' "$work/result.json" | mustmatch "number"
```

`threshold` comes back as the string `0.1:0.9`, and that string works again on the command line. `answer.probability` is how far the message came, which is what a desk reads when it wants the margin behind a no.

## What can go wrong

- **Exit 1 is a no and exit 3 is unresolved.** Under `set -e` a bare `thinkthen decide ...` ends the script on either one. Put the command in an `if`, a `case`, or a `&& rc=0 || rc=$?` list.
- **Exit 4 is a backend failure and exit 5 is a local one.** Neither is an answer about a customer. The `*` branch of the `case` exists for them. A `case` that only handles 0, 1, and 3 treats a failed request as a normal queue.
- **Exit 2 is a usage error, and it goes out before any request.** `--quiet` beside `--details` is one, because the gate wants no output and the audit wants the object.

```bash
set -euo pipefail

thinkthen decide 'Does the customer ask for money back?' \
  --quiet --details --replay recording/ < message.txt \
  >/dev/null 2>&1 && bad=0 || bad=$?
printf 'bad=%s\n' "$bad" | mustmatch "bad=2"
```

- **The model's word never runs anything.** The `case` is code. `decide` moved the exit code and nothing else.
- **A single cut never says the model is sure.** Exit 1 means the answer did not reach the mark, and a probability of 0.48 and one of 0.02 both come back as a no.

## Related how-tos

- [How to tell "no" from "could not ask"](../19-no-or-could-not-ask/) reads the failure codes in full.
- [How to test a script with no network](../27-test-with-no-network/) makes the recording this page replays.

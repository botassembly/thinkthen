# 01 Refund gate

Status: red

Verbs: `decide`

A support desk wants one branch in a script. A message that plainly asks for money back goes to the refunds queue, a message that plainly does not goes back to the normal queue, and anything in between waits for a person. The question names one fact that is printed in the message, so the model is being asked what it is good at.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`message.txt` is one customer message that asks for a refund in plain words. `question.txt` is one customer message that asks a product question and mentions no money.

## The gate

`decide` is a shell test. `--quiet` drops the answer from standard output, so the command reads like `test` and needs no redirect.

```bash
set -euo pipefail

if thinkthen decide 'Does the customer ask for money back?' \
     --input message.txt --quiet --replay recording/
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
     --input question.txt --quiet --replay recording/
then
  printf 'refunds\n'
else
  printf 'normal\n'
fi | mustmatch "normal"
```

That `if` is a two-way branch and the desk wants three. With no threshold the cut is 0.5 and nothing is ever unresolved, so a borderline message lands in one of the two queues with no sign that it was close.

## Three codes, three branches

A band threshold adds the third answer. Exit 0 is yes, 1 is no, and 3 is unresolved. `&& rc=0 || rc=$?` is what captures a non-zero code under `set -e`.

```bash
set -euo pipefail

route() {
  thinkthen decide 'Does the customer ask for money back?' \
    --input "$1" --threshold 0.1:0.9 --quiet --replay recording/
}

route message.txt && rc=0 || rc=$?
case $rc in
  0) printf 'refunds\n' ;;
  1) printf 'normal\n' ;;
  3) printf 'review\n' ;;
  *) printf 'gate failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "refunds"
```

The `*` branch matters. Exit 4 is a backend failure and exit 5 is a local one, and neither is an answer about a customer.

## Keeping the answer you paid for

The gate above throws the judgment away. A desk that wants to audit its own routing drops `--quiet`, saves the result, and reads the exit code from the same run.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide 'Does the customer ask for money back?' \
  --input message.txt --threshold 0.1:0.9 --details --replay recording/ \
  > "$work/result.json" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
jq -c '{value, threshold}' "$work/result.json" \
  | mustmatch '{"value":true,"threshold":"0.1:0.9"}'
jq -r '.question.verb' "$work/result.json" | mustmatch "decide"
jq -r '.answer.probability | type' "$work/result.json" | mustmatch "number"

thinkthen decide 'Does the customer ask for money back?' \
  --input message.txt --quiet --details --replay recording/ \
  >/dev/null 2>&1 && bad=0 || bad=$?
printf 'bad=%s\n' "$bad" | mustmatch "bad=2"
```

`threshold` comes back as the string `0.1:0.9`, and that string works again on the command line. The saved result says which rule ran.

Nothing here acts on the model's word. The `case` is code, and the model only moved the exit code.

The recording under `recording/` does not exist yet, so this page is red. Every `thinkthen` line carries `--replay recording/`, so a gate touches no network and reads no key. `record.sh` makes the two exchanges once by hand.

## What this demo decides

- **The demo confirms the shell test.** `if thinkthen decide ...` with `--quiet` reads the way `grep -q` reads, and it needs no `> /dev/null`. The three codes are a `case` a shell user already knows how to write.
- **`--quiet` beside `--details` is a usage error, and the demo confirms the rule reads right.** The gate wants no output and the audit wants the object. Asking for both is a mistake the shell should hear about at once.
- **The default threshold of 0.5 gives the two-way gate a silent failure mode.** The first block routes a borderline message with no sign that it was close, because nothing is unresolved under a single cut. That is the documented rule and the demo does not ask to change it. It asks that the `decide` help say in one line that a three-way gate needs a band, next to the warning about `set -e`.
- **`--input FILE` earns its place over a redirect.** The function above takes a path, and `< "$1"` inside a function body would have worked too. `--input` keeps the whole command on one line and puts the file next to the question it is judged against.
- **A single cut has no way to report how close a record came.** Exit 1 means the answer did not reach the mark, and the desk that wants the margin has to drop `--quiet` and read `answer.probability`. The demo does that in the last block. The cost is one saved file per judgment. That is the right price.

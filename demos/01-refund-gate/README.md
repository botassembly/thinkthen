# 01 Refund gate

Status: red

Verbs: `decide if`

A support desk wants one branch in a script. A message that plainly asks for money back goes to the refunds queue, a message that plainly does not goes back to the normal queue, and anything in between waits for a person. The condition names one fact that is printed in the message, so the model is being asked what it is good at.

## Input

`message.txt` is one customer message that asks for a refund in plain words. `question.txt` is one customer message that asks a product question and mentions no money.

## The gate

`--status` turns the judgment into an exit code. It needs `--min-prob`, and the result still prints, so the condition sends it to `/dev/null`. Three exit codes mean three branches, and the `case` names all three.

```bash
set -euo pipefail

asks_for_money_back() {
  thinkthen decide if 'the customer asks for money back' \
    --min-prob 0.9 --status --replay recording/ \
    < "$1" > /dev/null
}

asks_for_money_back message.txt && rc=0 || rc=$?
case $rc in
  0) printf 'refunds\n' ;;
  1) printf 'normal\n' ;;
  3) printf 'review\n' ;;
  *) printf 'gate failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "refunds"
```

The second message never mentions money, so the same gate sends it down the other branch.

```bash
set -euo pipefail

thinkthen decide if 'the customer asks for money back' \
  --min-prob 0.9 --status --replay recording/ \
  < question.txt > /dev/null && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=1"
```

## Keeping the answer you paid for

The gate throws the result away. A desk that wants to audit its own routing keeps the result and reads the exit code from the same run.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide if 'the customer asks for money back' \
  --min-prob 0.9 --status --replay recording/ \
  < message.txt > "$work/result.json" && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
jq -c '.assessment | {status, value}' "$work/result.json" \
  | mustmatch '{"status":"accepted","value":true}'
jq -r '.question.verb' "$work/result.json" | mustmatch "if"
```

Nothing in this demo acts on the model's word. The `case` is code, and the model only moved the exit code.

The recording under `recording/` does not exist yet, so this page is red. Every `thinkthen` line above carries `--replay recording/`, so the page runs in a gate with no network and reads no key. `record.sh` runs the two judged exchanges once against the live backend with `--record recording/`. It is run by hand, and no gate calls it.

## What this demo decides

- **Exit code 3 cannot mean `unassessed` under `--status`.** channels.md gives code 3 to "unsure or unassessed", and decide.md says `--status` needs `--min-prob`. A run with a pass mark is never `unassessed`, so under `--status` code 3 means unsure and nothing else. Smallest fix: channels.md says code 3 means the accepted answer is unsure, and drops `unassessed` from that row. This touches a settled page.
- **`--status` printing the result is right, and the cost is `> /dev/null` on every condition.** The alternative, a quiet `--status`, would throw away a paid answer by default. The demo keeps the current rule and shows the redirect once and the saved file once.
- **An unsure `if` carries no reason.** result.md gives `assessment.reason` to `which` with four named values. An unsure `if` has one possible reason and no field to put it in. Smallest fix: `if` carries `reason: "below_min_prob"` when it is unsure, so one `jq` path reads the reason across verbs.
- **`--replay` is in the specification now.** This demo asked for the flag, and `specification/recording.md` answers it: `--replay DIR` opens no connection, reads no key, and a miss is a local failure at exit 5 naming the entry the folder lacks. Ticket 0004 built it, and the three blocks above run unchanged against a recording.

# How to branch on a label with `choose` and `case`

Status: green

Verbs: `choose`

Use this when a script has to put one ticket on one of a few queues. The labels are a short list of teams that do not overlap, plus one for a ticket that fits none of them. The script acts on the label with a `case`, and the `case` has a default branch, so a label the script does not know about stops the run instead of picking a queue at random.

The recording under `recording/` holds the one live exchange this page replays. The model put every point of probability on `billing`, and no block asserts on a probability.

## Input

`ticket.txt` is one message about a renewal charge that bounced.

## Label the ticket

The question comes first and the options follow it. `--raw` prints the winning label with no quotation marks, which is what a `case` wants.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --raw --replay recording/ < ticket.txt \
  | mustmatch "billing"
```

The fourth option is typed by hand. Nothing adds a catch-all, so the sent list is the list on the command line. Option order moves the odds, so a script that reorders the list is running a different measurement.

Without `--raw` the answer is a JSON string, and a `case` cannot match one.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --replay recording/ < ticket.txt \
  | mustmatch '"billing"'
```

## Act on the label

A script reads the exit code first and the label second, so it takes two `case` blocks. Exit 0 is a label, exit 3 is unresolved, and anything else is a failure that is not an answer about a ticket.

```bash
set -euo pipefail

label=$(
  thinkthen choose 'Which team owns this request?' billing shipping account other \
    --raw --threshold 0.8 --replay recording/ < ticket.txt
) && rc=0 || rc=$?

case $rc in
  0) ;;
  3) label=unresolved ;;
  *) printf 'choose failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac

case $label in
  billing)    queue=payments ;;
  shipping)   queue=logistics ;;
  account)    queue=identity ;;
  other)      queue=triage ;;
  unresolved) queue=triage ;;
  *) printf 'unknown label: %s\n' "$label" >&2; exit 2 ;;
esac

printf 'queue=%s\n' "$queue" | mustmatch "queue=payments"
```

`--raw` prints nothing at all when the answer is unresolved, and an empty string is no label. Only the exit code tells an unresolved pick from a command that crashed, which is why the first `case` reads `$rc` before the second one reads the label.

## Set the mark

`--threshold 0.8` applies to the winning option's probability. A winner under the mark is unresolved and exits 3.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --threshold 0.99 --replay recording/ < ticket.txt \
  | mustmatch "\"billing\""

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --threshold 1 --replay recording/ < ticket.txt \
  > /dev/null && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=0"
```

An exact tie for first place is unresolved with or without a mark, because the order the options were typed is no evidence.

## Check the margin yourself

`--details` carries a probability for every option sent, in the order they were sent. A desk that wants the winner to beat the runner-up by a margin writes that test in `jq` and needs no option for it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --details --threshold 0.8 --replay recording/ < ticket.txt \
  > "$work/label.json"

jq -r '.answer.probabilities | keys_unsorted | join(",")' "$work/label.json" \
  | mustmatch "billing,shipping,account,other"
jq -r '.answer.pick' "$work/label.json" | mustmatch "billing"
jq -r '.question.verb' "$work/label.json" | mustmatch "choose"
jq -e '
  (.answer.probabilities | to_entries | sort_by(-.value) | .[0].value - .[1].value) >= 0.3
' "$work/label.json" > /dev/null && printf 'clear winner\n' | mustmatch "clear winner"
```

The keys come back in the order the options were sent, whatever order the backend answered in. That order is also the evidence that the list the user typed is the list the model saw.

`--details` and `--raw` print two different things, so asking for both is a usage error. So is `--details` beside `--quiet`.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --details --raw --replay recording/ < ticket.txt \
  >/dev/null 2>&1 && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=2"
```

Every `thinkthen` line on this page carries `--replay recording/`, so the page touches no network and reads no key. `record.sh` made the exchange once, through `sdlc/scripts/live`.

## What can go wrong

| Exit code | What happened | What to do |
| --- | --- | --- |
| 0 | A label was returned | Read it |
| 2 | A usage error: a blank option, one option, more than 255, a repeat, a band on `--threshold`, or two views at once | Fix the command line. Nothing was sent |
| 3 | The winner fell under the mark, or the top two tied exactly | Send the ticket to a person |
| 4 | The backend failed, or the adapter refused the reply | Retry or stop. It is not an answer about the ticket |
| 5 | A local failure: the recording folder, standard input | Fix the machine |

- `choose` never exits 1. A pick is not a two-sided decision, so there is no "no".
- Under `set -e` an unresolved answer ends the script, because exit 3 is not zero. Capture the code with `&& rc=0 || rc=$?` as the blocks above do.
- An empty label is not a label. `--raw` prints nothing for an unresolved answer, so `label=$(...)` leaves the variable empty and the exit code is the only thing that says why.
- `--threshold 0.1:0.9` is a usage error on `choose`. The cut falls on one winning probability, and a band over one winner has no meaning.
- Nothing here runs the queue the model named. The `case` is code, and the model only moved a string.

## Related how-tos

- [How to sort files into folders by label](../05-sort-a-folder/) runs this loop over a whole folder.
- [How to tell "not stated" from "false"](../20-not-stated-or-false/) picks labels that a yes/no question cannot tell apart.
- [Refund gate](../01-refund-gate/) is the two-sided decision `choose` is not.

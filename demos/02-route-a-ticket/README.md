# How to branch on a label with `choose` and `case`

Status: green

Verbs: `choose`

Use this when a script has to put one ticket on one of a few queues. The labels are a short list of teams that do not overlap, plus one for a ticket that fits none. `--raw` prints the bare label, which is what a `case` wants.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --raw --replay recording/ < ticket.txt \
  | mustmatch "billing"
```

## Input

`ticket.txt` is one message about a renewal charge that bounced, and `inbox/` holds five short meeting notes for the closing section. `recording/thinkthen.jsonl` holds the answers of the six live exchanges this page replays, and `record.sh` made them once through `sdlc/scripts/live`.

The fourth option is typed by hand: nothing adds a catch-all, and the sent list is the list on the command line. Without `--raw` the answer is the JSON string `"billing"`, which no `case` matches.

## Step 1: act on the label

A script reads the exit code first and the label second, so it takes two `case` blocks. Exit 0 is a label, 3 is not sure, and anything else is a failure. `--threshold 0.8` cuts on the winning option's probability.

```bash
set -euo pipefail

label=$(
  thinkthen choose 'Which team owns this request?' billing shipping account other \
    --raw --threshold 0.8 --replay recording/ < ticket.txt
) && rc=0 || rc=$?

case $rc in
  0) ;;
  3) label=not_sure ;;
  *) printf 'choose failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac

case $label in
  billing)    queue=payments ;;
  shipping)   queue=logistics ;;
  account)    queue=identity ;;
  other)      queue=triage ;;
  not_sure) queue=triage ;;
  *) printf 'unknown label: %s\n' "$label" >&2; exit 2 ;;
esac

printf 'queue=%s\n' "$queue" | mustmatch "queue=payments"
```

`--raw` prints nothing when the answer is not sure, and an empty string is no label. Only the exit code tells a not sure pick from a crash, which is why the first `case` reads `$rc`. `--details` prints a probability for every option instead, in the order they were sent.

## Step 2: run the same shape over a whole folder

The loop is the block above with the queue replaced by a folder. `find` produces NUL-delimited paths, because a filename may hold a newline, and they go to a file first so a failing `find` stops the run.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
cp -R inbox "$work/inbox"
mkdir -p "$work/defect" "$work/process" "$work/other" "$work/review"

find "$work/inbox" -type f -name '*.txt' -print0 | LC_ALL=C sort -z > "$work/paths.nul"
while IFS= read -r -d '' path; do
  label=$(
    thinkthen choose 'What kind of work does this note record?' defect process other \
      --raw --threshold 0.8 --replay recording/ < "$path"
  ) && rc=0 || rc=$?
  case $rc in
    0) dest=$label ;;
    3) dest=review ;;
    *) printf 'choose failed on %s: %d\n' "$path" "$rc" >&2; exit "$rc" ;;
  esac
  mv -- "$path" "$work/$dest/"
done < "$work/paths.nul"

for dir in defect process other review; do
  printf '%s=%d\n' "$dir" "$(find "$work/$dir" -type f | wc -l)"
done | mustmatch "defect=2
process=0
other=0
review=3"
```

Two notes name a broken thing plainly, and the model put every point of probability on `defect` for both. A warehouse call, a budget review, and a hallway chat came nowhere near 0.8. A strict mark on three options sends most of a mixed folder to a person, and that is the cost the desk chooses.

## What can go wrong

| Exit code | What happened | What to do |
| --- | --- | --- |
| 0 | A label was returned | Read it |
| 2 | A usage error: a blank option, one option, more than 255, a repeat, or a band on `--threshold` | Fix the command line. Nothing was sent |
| 3 | The winner fell under the threshold, or the top two tied exactly | Send the ticket to a person |
| 4 | The backend failed, or the adapter refused the reply | Retry or stop. It is no answer about the ticket |
| 5 | A local failure: the recording folder, standard input | Fix the machine |

- `choose` never exits 1. A pick is not a two-sided decision, so there is no "no".
- Under `set -e` a not sure answer ends the script. Capture the code with `&& rc=0 || rc=$?`.
- An empty file is a usage error, not a label, and a real inbox holds one. A loop needs a branch for exit 2 or a `find -size +0` filter.
- `mv` over an existing name overwrites it, so two notes sharing a basename collide. Use `mv -n`.
- Nothing here runs the queue the model named. The `case` is code, and the model moved a string.

## Related how-tos

- [How to say what yes and no mean](../40-what-yes-and-no-mean/) puts silence where a yes/no question cannot.
- [How to choose the next action from a list that changes at every step](../21-options-from-the-record/) takes the options out of each record.
- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the two-sided decision `choose` is not.
- [How to route a request by how hard it is](../17-rate-and-sort/) orders a queue instead of filing it.

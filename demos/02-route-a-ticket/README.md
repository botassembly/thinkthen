# 02 Route a ticket

Status: red

Verbs: `choose`

One ticket arrives and a script has to put a label on it. The labels are a short list of teams that do not overlap, plus one for a ticket that fits none of them. The script then acts on the label, and the `case` has a default branch, so a label the script does not know about stops the run instead of picking a queue at random.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`ticket.txt` is one message about a failed renewal charge.

## Label it

The question comes first and the options follow it. `--raw` prints the winning label with no quotation marks. A `case` wants exactly that.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --raw --input ticket.txt --replay recording/ \
  | mustmatch "billing"
```

The fourth option is typed by hand. Nothing adds a catch-all, so the sent list is the list on the command line, and a script that reorders it is running a different measurement.

## Act on the label

```bash
set -euo pipefail

label=$(
  thinkthen choose 'Which team owns this request?' billing shipping account other \
    --raw --threshold 0.8 --input ticket.txt --replay recording/
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

Two `case` blocks, not one. The first reads the exit code and the second reads the label, because `--raw` prints nothing at all when the answer is unresolved and an empty string is not a label.

## The mark and the tie

`--threshold 0.8` applies to the winning option's probability. A winner under the mark is unresolved and exits 3. An exact tie for first place is unresolved with or without a mark.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --threshold 0.99 --input ticket.txt --replay recording/ \
  > /dev/null && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=3"
```

Without `--raw` the answer is a JSON string. A `jq` pipeline wants that and a `case` does not.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --input ticket.txt --replay recording/ \
  | mustmatch '"billing"'
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms the grammar.** Verb, question, then the options reads as English and puts the question the model receives in front of the user.
- **`--raw` printing nothing for `null` costs the script a second `case`.** The page could not be written with one branch table, because an unresolved answer and a crashed command both leave `label` empty. Exit 3 is the only thing that tells them apart, and reading it requires `&& rc=0 || rc=$?` around a command substitution. The demo argues that the `choose` help show this two-`case` shape as its worked example, and that nothing be added to the surface.
- **The demo could not see the per-option probabilities.** ADR 0007 fixes the `--details` object for a yes/no answer and never says what `answer` holds for `choose`. The old page asserted on the option order coming back in the order it was sent, which was evidence that the list the user typed is the list the model saw. That assertion cannot be written now. The surface should fix the `choose` answer shape.
- **A catch-all option is the user's to type, and typing it is what got the branch written.** The dropped `--none` flag would have put an option in the sent list that the user never saw. The demo confirms the removal.
- **The dropped `--min-gap` is missed here and the demo does not ask for it back.** A 0.81 against a 0.79 clears `--threshold 0.8`, and the measurement says option order alone can move a margin that size. With no per-option probabilities in the result, a script cannot check the margin itself either. That makes the missing `choose` answer shape the stronger of the two findings.

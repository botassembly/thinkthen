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

The page needs two `case` blocks. The first reads the exit code and the second reads the label. `--raw` prints nothing at all when the answer is unresolved, and an empty string is no label.

## The mark and the tie

`--threshold 0.8` applies to the winning option's probability. A winner under the mark is unresolved and exits 3. An exact tie for first place is unresolved with or without a mark.

```bash
set -euo pipefail

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --threshold 0.99 --input ticket.txt --replay recording/ \
  > /dev/null && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=3"
```

Without `--raw` the answer is a JSON string, and a `case` cannot match one.

## Check the margin yourself

`--details` carries a probability for every option sent, in the order they were sent. A desk that wants the winner to beat the runner-up by a margin writes that test in `jq` and needs no option for it.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --details --threshold 0.8 --input ticket.txt --replay recording/ \
  > "$work/label.json"

jq -r '.answer.probabilities | keys_unsorted | join(",")' "$work/label.json" \
  | mustmatch "billing,shipping,account,other"
jq -r '.answer.pick' "$work/label.json" | mustmatch "billing"
jq -e '
  (.answer.probabilities | to_entries | sort_by(-.value) | .[0].value - .[1].value) >= 0.3
' "$work/label.json" > /dev/null && printf 'clear winner\n' | mustmatch "clear winner"
```

The keys come back in the order the options were sent. Order moves the odds, so a script that reorders the list is running a different measurement.

## One line for one record

Under `--jsonl` the run prints one line per record. An unresolved pick prints an empty line, so a `paste` against the input never shifts.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

printf '%s\n' '{"body":"The renewal charge bounced last night."}' '{"body":"Hello."}' \
  > "$work/two.jsonl"

thinkthen choose 'Which team owns this request?' billing shipping account other \
  --raw --threshold 0.8 --jsonl --field /body --input "$work/two.jsonl" --replay recording/ \
  | wc -l | tr -d ' ' | mustmatch "2"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms the grammar.** Verb, question, then the options reads as English and puts the question the model receives in front of the user.
- **`--raw` printing nothing for `null` costs the script a second `case`.** An unresolved answer and a crashed command both leave `label` empty. Exit 3 is the only thing that tells them apart, and reading it takes `&& rc=0 || rc=$?` around a command substitution. The demo asks that the `choose` help show the two-`case` shape and that nothing be added to the surface.
- **The demo confirms the full distribution on a `choose` result.** `answer.probabilities` gives the margin test that the dropped `--min-gap` used to give, in one `jq` line and with no new option. The key order is also the evidence that the list the user typed is the list the model saw.
- **A catch-all option is the user's to type, and typing it is what got the branch written.** The dropped `--none` flag would have put an option in the sent list that the user never saw. The demo confirms the removal.
- **The demo could not read `confidence`.** ADR 0009 says `answer` carries the vendor's `confidence` when one exists and then says the formula is unpublished. A page cannot assert on a field that may be absent and cannot explain one whose meaning nobody knows. The demo cuts on `probabilities` alone and asks that `confidence` stay out of every example until a measurement says what it is for.
- **The empty line for an unresolved record is right and it is silent.** Two records in, two lines out, and the blank one means unresolved. A pipeline that pipes into `grep -v '^$'` drops the record and shifts every later row. The demo asks that the `--raw` help say the blank line is a record.

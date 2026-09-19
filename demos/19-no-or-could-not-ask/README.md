# How to tell "no" from "could not ask" in a script

Status: green

Verbs: `decide`

Use this when a script acts on the answer and a wrong branch costs something. `decide` reports a no by exiting 1, and it reports a failed request by exiting 4 or 5. An `if` with an `else` puts all three in the same branch. This page reads the exit code with `case` and keeps a failure apart from an answer.

## Input

`note.txt` is a change note that says the change ran on staging. `hurried.txt` is a change note that says nothing about staging. `recording/` holds the two exchanges for the question below, and every probability here is illustrative.

## Step 1: read every outcome with `case`

Run the command, then branch on `$?`. Four branches cover everything `decide` can report. This form needs `set -e` off, because a no exits 1 and `set -e` would end the script before the `case` ran. Step 2 gives the form for a script that wants `set -e`.

```bash
set +e
set -uo pipefail

thinkthen decide 'Does the note say the change was tested in staging?' \
  --threshold 0.1:0.9 --quiet --replay recording/ < note.txt
case $? in
  0) printf 'deploy\n' ;;
  1) printf 'hold: not tested\n' ;;
  3) printf 'hold: unclear\n' ;;
  *) printf 'hold: the judge did not answer\n' ;;
esac | mustmatch "deploy"
```

The hurried note takes the second branch. Exit 1 is an answer, and the answer is no.

```bash
set +e
set -uo pipefail

thinkthen decide 'Does the note say the change was tested in staging?' \
  --threshold 0.1:0.9 --quiet --replay recording/ < hurried.txt
case $? in
  0) printf 'deploy\n' ;;
  1) printf 'hold: not tested\n' ;;
  3) printf 'hold: unclear\n' ;;
  *) printf 'hold: the judge did not answer\n' ;;
esac | mustmatch "hold: not tested"
```

## Step 2: keep `case $?` working under `set -e`

Under `set -e` the script ends the moment `decide` exits 1, so the `case` never runs. Capture the code in the same line that runs the command.

```bash
set -euo pipefail

thinkthen decide 'Does the note say the change was tested in staging?' \
  --threshold 0.1:0.9 --quiet --replay recording/ < hurried.txt && rc=0 || rc=$?
case $rc in
  0) printf 'deploy\n' ;;
  1) printf 'hold: not tested\n' ;;
  3) printf 'hold: unclear\n' ;;
  *) printf 'hold: the judge did not answer\n' ;;
esac | mustmatch "hold: not tested"
```

Here is what the bare command does. The script below ends at the no, prints nothing, and leaves exit 1 behind.

```bash
set +e
set -uo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

cat > "$work/gate.sh" <<'SCRIPT'
set -e
thinkthen decide 'Does the note say the change was tested in staging?' \
  --quiet --replay recording/ < hurried.txt
printf 'deploy\n'
SCRIPT

said=$(bash "$work/gate.sh" 2>/dev/null)
printf 'rc=%s said=[%s]\n' "$?" "$said" | mustmatch "rc=1 said=[]"
```

## Step 3: see a failure that is not an answer

`--replay` answers from the folder alone. A request the folder does not hold is a local failure, exit 5. The question below is one this recording never held, so the command fails without a network and without a key.

```bash
set +e
set -uo pipefail

thinkthen decide 'Was the change reviewed by a second person?' \
  --threshold 0.1:0.9 --quiet --replay recording/ < note.txt 2>/dev/null
case $? in
  0) printf 'deploy\n' ;;
  1) printf 'hold: not tested\n' ;;
  3) printf 'hold: unclear\n' ;;
  *) printf 'hold: the judge did not answer\n' ;;
esac | mustmatch "hold: the judge did not answer"
```

The same failure through an `if` with an `else` comes out as the no branch, and nothing in the output says a request failed.

```bash
set +e
set -uo pipefail

if thinkthen decide 'Was the change reviewed by a second person?' \
     --quiet --replay recording/ < note.txt 2>/dev/null
then
  printf 'deploy\n'
else
  printf 'not tested\n'
fi | mustmatch "not tested"
```

## Step 4: word the question so that yes permits the action

Write the question in the form where yes lets the script go ahead. "Does the note say the change was tested in staging?" permits a deploy on yes. Every other outcome, a no, an unresolved answer, a backend failure, and a local failure, leaves the deploy undone, so a failure can never permit anything.

The reversed wording, "Is this change too risky to deploy?", makes no the permitting answer. A failed request exits non-zero, an `if` sends it to the `else`, and the script deploys because the judge was unreachable.

## What can go wrong

- **Exit 1 and exit 4 both fail an `if`.** Exit 1 is a no about the evidence. Exit 4 is a backend failure and exit 5 is a local failure, and neither says anything about the evidence. An `if` with an `else` cannot tell them apart. Use `case`.
- **`set -e` ends the script on a no.** A bare `thinkthen decide` exits 1 on a no and 3 on an unresolved answer, and `set -o pipefail` carries the same code out of a pipeline. Put the command in an `if`, a `case`, or a `&& rc=0 || rc=$?` list.
- **`case $?` reads the last command.** Put nothing between the command and the `case`, not even a `printf`.
- **Exit 2 is a usage error and no request went out.** A mistyped option lands there before the backend is touched.
- **Exit 70 is a defect in the tool.** Treat it like 4 and 5 and report it.
- **A failure message goes to standard error.** The blocks above send it to `/dev/null` to keep the output clean. A real script lets it through, because that text names the missing entry.

## Related how-tos

- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the shorter version, with the two-way `if`.
- [How to test a script with no network](../27-test-with-no-network/) makes the recordings that put exit 5 in reach of a test.

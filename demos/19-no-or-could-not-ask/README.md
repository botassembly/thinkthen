# How to gate a risky command and fail closed

Status: green

Verbs: `decide`

Use this when something proposes a shell command and a script has to decide whether to run it. A no, an unclear answer, and a judge that could not be reached are three different things, and an `if` puts all three in the `else`. `case` keeps them apart, and only yes runs anything.

```bash
set +e
set -uo pipefail
for proposed in proposed/list.txt proposed/fetch.txt proposed/wipe.txt; do
  thinkthen decide 'Does this command only read, and leave every file and every setting on the machine unchanged?' \
    --threshold 0.1:0.8 --quiet --replay recording/ <"$proposed" 2>/dev/null
  case $? in
    0) printf 'run\n' ;;
    1) printf 'hold: it changes something\n' ;;
    3) printf 'hold: unclear, ask a person\n' ;;
    *) printf 'hold: the judge did not answer\n' ;;
  esac
done | mustmatch "run
hold: unclear, ask a person
hold: it changes something"
```

## Input

`proposed/` holds three short notes, each naming a command somebody wants run and the reason given. `list.txt` greps the source tree, `fetch.txt` runs `git fetch`, and `wipe.txt` deletes the build directory. Nothing here runs any of them.

`recording/` holds the three exchanges this page replays, so every command runs with no network and no key. `record.sh` made them through `sdlc/scripts/live`. The judge answered 0.89 for the grep, 0.22 for the fetch, and 0.01 for the wipe, so the band `0.1:0.8` gives one of each answer. `git fetch` writes inside `.git` and touches no working file, which is the kind of question a person settles.

## Step 1: see a judge that could not answer

`--replay` answers from the folder alone. A question the folder never held is a local failure, exit 5, and it reaches the same `*` branch exit 4 does. A failure is never an answer about the command.

```bash
set +e
set -uo pipefail

thinkthen decide 'Is this command reversible?' \
  --threshold 0.1:0.8 --quiet --replay recording/ <proposed/wipe.txt 2>/dev/null
code=$?
case $code in
  0) printf 'run\n' ;;
  1|3) printf 'hold\n' ;;
  *) printf 'hold: the judge did not answer, exit %s\n' "$code" ;;
esac | mustmatch "hold: the judge did not answer, exit 5"
```

## Step 2: keep the answer for the audit

The gate above throws the judgment away. Drop `--quiet`, keep the result, and read the exit code from the same run. `answer.probability` is how far the command came, which is what a reviewer reads when a gate is questioned.

```bash
set -euo pipefail

audit=$(
  thinkthen decide 'Does this command only read, and leave every file and every setting on the machine unchanged?' \
    --threshold 0.1:0.8 --details --replay recording/ <proposed/fetch.txt
) && rc=0 || rc=$?

printf 'rc=%s\n' "$rc" | mustmatch "rc=3"
printf '%s' "$audit" \
  | jq -c '{value, threshold, probability: .answer.probability, verb: .question.verb}' \
  | mustmatch '{"value":null,"threshold":"0.1:0.8","probability":0.22,"verb":"decide"}'
```

`threshold` comes back as the string `0.1:0.8`, and that string works again on the command line. `value` is `null`, the one spelling of unresolved in the bare output, in the object, and beside exit 3.

## Step 3: word the question so that yes permits the action

Write the question so that yes lets the script go ahead. "Does this command only read?" permits a run on yes, and every other outcome leaves the command unrun, so no failure can ever permit anything.

The reversed wording, "Is this command dangerous?", makes no the permitting answer. A failed request exits non-zero, an `if` sends it to the `else`, and the script runs the command because the judge was unreachable.

## What can go wrong

- **`set -e` ends the script on a no.** A bare `thinkthen decide` exits 1 on a no and 3 on an unresolved answer, and `set -o pipefail` carries the code out of a pipeline. The script below ends at the no, prints nothing, and leaves exit 1 behind.

```bash
set +e
set -uo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

cat >"$work/gate.sh" <<'SCRIPT'
set -e
thinkthen decide 'Does this command only read, and leave every file and every setting on the machine unchanged?' \
  --quiet --replay recording/ <proposed/wipe.txt
printf 'run\n'
SCRIPT

said=$(bash "$work/gate.sh" 2>/dev/null)
printf 'rc=%s said=[%s]\n' "$?" "$said" | mustmatch "rc=1 said=[]"
```

Put the command in an `if`, a `case`, or a `&& rc=0 || rc=$?` list instead.

- **Exit 1 and exit 4 both fail an `if`.** Exit 1 is a no about the evidence, while exit 4 and exit 5 say nothing about the command at all. Exit 2 is a usage error with no request sent, and exit 70 is a defect in the tool. Treat every code but 0 as a refusal.
- **`case $?` reads the last command.** Put nothing between it and the `case`, not even a `printf`.
- **A failure message goes to standard error.** The blocks above send it to `/dev/null` to keep the output clean. A real gate lets it through, because it names what went wrong.
- **Nothing here runs a command.** `thinkthen` moved an exit code, and the shell that reads it only prints.

## Related how-tos

- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the shorter version, with the two-way `if`.
- [How to test a script with no network](../27-test-with-no-network/) makes the recordings that put exit 5 in reach of a test.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) says where `0.1:0.8` comes from.

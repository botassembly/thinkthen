# How to test a script with no network

Status: green

Verbs: `decide`

Use this when a script calls `thinkthen` and you want a test for it that runs in a gate. `--replay` answers from a folder of saved exchanges alone, with no connection and no key, so the test gives the same result every time and costs nothing.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  sh triage.sh report.txt --replay recording/ \
  | mustmatch "ready"
```

## Input

`triage.sh` is the script under test, and the block below prints it past its header comment. It reads one bug report and prints the queue it belongs in. Arguments after the file name go straight to `thinkthen`, which is how a test points the script at a recording.

```bash
set -euo pipefail

sed -n '/^set -eu$/,$p' triage.sh | mustmatch "$(cat <<'SCRIPT'
set -eu

report=$1
shift

thinkthen decide 'Does this report say what the person did before the problem appeared?' \
	--quiet "$@" <"$report" && rc=0 || rc=$?
case $rc in
0) printf 'ready\n' ;;
1) printf 'needs detail\n' ;;
*)
	printf 'triage: the judge did not answer, exit %s\n' "$rc" >&2
	exit "$rc"
	;;
esac
SCRIPT
)"
```

`report.txt` is a bug report that says what the person did. `vague.txt` is a bug report that says nothing, and it is left out of the recording on purpose. Every probability here is illustrative.

## Step 1: record the exchange once

Run the script for real, with `THINKTHEN_API_KEY` set, and add `--record` and a scratch folder. The command calls the backend, prints its answer, and stores each answer in `thinkthen.sqlite` in that folder. `thinkthen cache convert` then merges the scratch file into the committed `thinkthen.jsonl`.

```sh
sh triage.sh report.txt --record scratch/
cp scratch/thinkthen.sqlite recording/
thinkthen cache convert recording/
```

`thinkthen.jsonl` holds one line for each shared state, then one line for each answer, sorted, so a review reads it as text. Each answer is stored under its question key: the SHA-256 of the adapter, the address, the model, the shared state and the question as sent. An answer line holds the question and the answer, and never a header, so no key can reach it.

```bash
set -euo pipefail

jq -r 'select(has("key")) | keys_unsorted | join(",")' recording/thinkthen.jsonl \
  | mustmatch "key,url,model,state,question,answer,answered_by,input_tokens,output_tokens,taken_at,origin"
```

`record.sh` in this folder is the script that made the recording. It runs through `sdlc/scripts/live`, the one door for a paid call.

## Step 2: read where the answer came from

`--details` says so. `meta.cached` is `true` for replay. Live attempts are optional detail; replay has none.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --details --replay recording/ < report.txt \
  | jq -c '{value, cached: .meta.cached, attempts_present: (.meta | has("attempts"))}' \
  | mustmatch '{"value":true,"cached":true,"attempts_present":false}'
```

## Step 3: know a replay miss when you see one

`vague.txt` was never recorded, so the folder holds no answer for it. `--replay` opens no connection, so the run ends in a local failure, exit 5, and the message on standard error carries the missing question's key.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  sh triage.sh vague.txt --replay recording/ 2>/dev/null && rc=0 || rc=$?
printf 'rc=%s\n' "$rc" | mustmatch "rc=5"
```

```bash
set -euo pipefail

said=$(env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --quiet --replay recording/ < vague.txt 2>&1 >/dev/null) && rc=0 || rc=$?
printf 'rc=%s %s\n' "$rc" "$said" \
  | mustmatch 'rc=5 thinkthen: the decide request for one document: the replay folder holds no answer for question `bdd549264706825ecdea6fd3dc98f854f0aeb2e50ffe35c00157c53ccd60f70b`; the key is the SHA-256 of the adapter, address, model, shared state and question as sent'
```

Record the missing case and the test passes again. A recording is grown one case at a time.

## What can go wrong

- **A miss is exit 5, and it is not a no.** A script that reads the answer through an `if` with an `else` turns a miss into a no. Read the code with `case`, as `triage.sh` does.
- **Any change to a question makes a new answer.** The key covers the question text, the evidence bytes, the model name, and the address. Change a word in the question or in the input file and the old answer no longer answers. `--threshold`, `--quiet`, and `--details` change nothing that is sent, so they never cost a new answer.
- **A recording holds the evidence.** The request body carries whatever the script read. Record only text that may be kept, and keep a recording of private input out of version control.
- **A failed answer is never recorded.** Only an answer that came back and decoded is written, so a folder never grows an entry that replays an error.
- **`--plan` beside either option is a usage error, exit 2.** A plan sends nothing and reads nothing.

## Related how-tos

- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the shape of the script being tested.
- [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) reads exit 5 and the other failures in full.
- [How to resume a long run that stopped](../12-keep-going/) points `--record` and `--replay` at one folder and pays for the rest alone.

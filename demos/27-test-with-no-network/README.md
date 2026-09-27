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

Run the script for real, with `THINKTHEN_API_KEY` set, and add `--record` and a folder. The command calls the backend, prints its answer, and writes the exchange into the folder.

```sh
sh triage.sh report.txt --record recording/
```

The file name is the digest of the wire shape, the address, and the request bytes, so the same command finds it again. An entry holds the request body and the response body, and never a header, so no key can reach it.

```bash
set -euo pipefail

jq -r 'input_filename, (keys_unsorted | join(","))' \
  recording/4492d4e8f2d047146e41dfe2eeb5ba6c5ab91140bd6be76eca6e66051f4d48e7.json \
  | mustmatch "recording/4492d4e8f2d047146e41dfe2eeb5ba6c5ab91140bd6be76eca6e66051f4d48e7.json
schema,adapter,url,request,response"
```

`record.sh` in this folder is the script that made the recording. It runs through `sdlc/scripts/live`, the one door for a paid call.

## Step 2: read where the answer came from

`--details` says so. `meta.cached` is `true` when a recording answered and `false` when a backend did.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --details --replay recording/ < report.txt \
  | jq -c '{value, cached: .meta.cached}' \
  | mustmatch '{"value":true,"cached":true}'
```

## Step 3: know a replay miss when you see one

`vague.txt` was never recorded, so the folder holds no answer for it. `--replay` opens no connection, so the run ends in a local failure, exit 5, and the message on standard error carries the digest, which is the name the entry would have had.

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
printf 'rc=%s %s\n' "$rc" "$said" | mustmatch like "rc=5 thinkthen: the replay folder holds no entry named"
```

Record the missing case and the test passes again. A recording is grown one case at a time.

## What can go wrong

- **A miss is exit 5, and it is not a no.** A script that reads the answer through an `if` with an `else` turns a miss into a no. Read the code with `case`, as `triage.sh` does.
- **Any change to the request makes a new entry.** The digest covers the question text, the evidence bytes, the model name, and the address. Change a word in the question or add a line to the input file and the old entry no longer answers. `--threshold`, `--quiet`, and `--details` change nothing that is sent, so they never cost a new entry.
- **A recording holds the evidence.** The request body carries whatever the script read. Record only text that may be kept, and keep a recording of private input out of version control.
- **A failed request is never recorded.** Only an exchange that came back and decoded is written, so a folder never grows an entry that replays an error.
- **`--dry-run` beside either option is a usage error, exit 2.** A plan sends nothing and reads nothing.

## Related how-tos

- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the shape of the script being tested.
- [How to gate a risky command and fail closed](../19-no-or-could-not-ask/) reads exit 5 and the other failures in full.
- [How to resume a long run that stopped](../12-keep-going/) points `--record` and `--replay` at one folder and pays for the rest alone.

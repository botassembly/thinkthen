# How to test a script with no network

Status: green

Verbs: `decide`

Use this when a script calls `thinkthen` and you want a test for it that runs in a gate. `--record` saves the exchange the backend answered. `--replay` answers from that folder alone, with no connection and no key, so the test gives the same result every time and costs nothing.

## Input

`triage.sh` is the script under test. It reads one bug report and prints the queue it belongs in. Arguments after the file name go straight to `thinkthen`, which is how a test points the script at a recording.

```sh
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
```

`report.txt` is a bug report that says what the person did. `vague.txt` is a bug report that says nothing, and it is left out of the recording on purpose. Every probability here is illustrative.

## Step 1: record the exchange once

Run the script for real, with `THINKTHEN_API_KEY` set, and add `--record` and a folder. The command calls the backend, prints its answer, and writes the exchange into the folder.

```sh
sh triage.sh report.txt --record recording/
```

The folder now holds one file. Its name is the digest of the wire shape, the address, and the request bytes, so the same command finds it again.

```bash
set -euo pipefail

ls -1 recording/ | mustmatch "01c476cebd5a5b2e7e2bf649d0604516f4bdd555a7cabb4ab1b8def6c840c4dc.json"
```

An entry holds the request body and the response body. It never holds a header, so no key can reach it.

```bash
set -euo pipefail

jq -r 'keys_unsorted | join(",")' \
  recording/01c476cebd5a5b2e7e2bf649d0604516f4bdd555a7cabb4ab1b8def6c840c4dc.json \
  | mustmatch "schema,adapter,url,request,response"
```

`record.sh` in this folder is the script that made the recording. It runs through `sdlc/scripts/live`, the one door for a paid call.

## Step 2: run the test from the recording

Unset the key and both backend variables, then run the same script with `--replay` and the same folder. The answer is the recorded one.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  sh triage.sh report.txt --replay recording/ | mustmatch "ready"
```

`--details` says where the answer came from. `meta.replayed` is `true` when a recording answered and `false` when a backend did.

```bash
set -euo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --details --replay recording/ < report.txt \
  | jq -c '{value, replayed: .meta.replayed}' \
  | mustmatch '{"value":true,"replayed":true}'
```

## Step 3: know a replay miss when you see one

`vague.txt` was never recorded, so the folder holds no answer for it. `--replay` opens no connection, so the run ends in a local failure, exit 5, and the message names the entry it looked for.

```bash
set +e
set -uo pipefail

env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  sh triage.sh vague.txt --replay recording/ 2>/dev/null
printf 'rc=%s\n' "$?" | mustmatch "rc=5"
```

The message on standard error carries the digest, which is the name the entry would have had.

```bash
set +e
set -uo pipefail

said=$(env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen decide 'Does this report say what the person did before the problem appeared?' \
  --quiet --replay recording/ < vague.txt 2>&1 >/dev/null)
printf '%s\n' "$said" | mustmatch like "holds no entry named"
```

Record the missing case and the test passes again. A recording is grown one case at a time.

## What can go wrong

- **A miss is exit 5, and it is not a no.** A script that reads the answer through an `if` with an `else` turns a miss into a no. Read the code with `case`.
- **Any change to the request makes a new entry.** The digest covers the question text, the evidence bytes, the model name, and the address. Change a word in the question or add a line to the input file and the old entry no longer answers. `--threshold`, `--quiet`, and `--details` change nothing that is sent, so they never cost a new entry.
- **A recording holds the evidence.** The request body carries whatever the script read. Record only text that may be kept, and keep a recording of private input out of version control.
- **A failed request is never recorded.** Only an exchange that came back and decoded is written, so a folder never grows an entry that replays an error.
- **`--record` and `--replay` on the same folder is a cache.** An entry that exists is replayed and one that is absent goes to the backend. That is useful while a recording is being grown, and it is the wrong thing for a gate, because a gate must never reach a backend.
- **`--dry-run` beside either option is a usage error, exit 2.** A plan sends nothing and reads nothing.

## Related how-tos

- [How to gate a script step on a yes/no answer](../01-refund-gate/) is the shape of the script being tested.
- [How to tell "no" from "could not ask"](../19-no-or-could-not-ask/) reads exit 5 and the other failures in full.

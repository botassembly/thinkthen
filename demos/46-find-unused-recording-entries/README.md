# How to find unused recording entries

Status: green

Verbs: `cache unused`

Use this after a complete run when the runner has saved every request digest that run used. This one-message example has one used entry. The same folder also holds a separate product-question answer.

```bash
set -euo pipefail

thinkthen cache unused ../01-refund-gate/recording --used used.txt \
  | mustmatch 'unused from supplied digests: 1
unused ab14a1fa02d3c85fe7f97051b043e6276a0cce429370a783b040e1b45305f971.json'
```

## Input

`used.txt` lists the one request digest from demo 01's message run. The two committed entries in `../01-refund-gate/recording/` hold that message's answer and a separate product-question answer. This report says the second entry was unused **by the one-message run**, not that it is unused by demo 01 or by a whole test suite. The command reads the folder and manifest without a key or network request.

## Step 1: keep a complete list

Run the job you want to measure and retain every request digest it actually used. For a single completed replay command, `--details` includes request digests in `meta.requests`. [How to test a script with no network](../27-test-with-no-network/) shows the replay workflow. A suite runner must collect a complete list across all its commands before calling this report.

## What can go wrong

- **An incomplete list makes a false claim.** `filter --details` omits dropped rows; bare and quiet output do not carry printed request digests. Do not turn those partial outputs into a full-suite `used.txt`.
- **Exit 2 refuses a bad manifest.** Each nonblank line must be one lowercase 64-character digest. Duplicate and blank lines are harmless. An empty valid file reports every valid entry as unused from the supplied list.
- **Exit 5 refuses an unsafe or damaged folder.** A malformed digest-named entry produces no partial report. The tool never removes an entry; `cache prune DIR` is the separate removal command.
- **The folder holds evidence and answers.** Whoever can write it can change answers read from it. Keep it private to trusted writers.

## Related how-tos

- [How to test a script with no network](../27-test-with-no-network/)
- [How to gate a script step on a yes/no answer](../01-refund-gate/)

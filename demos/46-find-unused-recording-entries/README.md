# How to find unused recording entries

Status: green

Verbs: `cache unused`

Use this after a complete run when the runner has saved every question key that run used. This one-message example uses one answer. The same folder also holds a separate product-question answer.

```bash
set -euo pipefail

thinkthen cache unused ../01-refund-gate/recording --used used.txt \
  | mustmatch 'unused from supplied keys: 1
unused 0311afcb8eba767b756924bd17cba8bdf973788f9103d8feb9ec00d4a7366474'
```

## Input

`used.txt` lists the one question key from demo 01's message run. The fixture `../01-refund-gate/recording/thinkthen.jsonl` holds that message's answer and a separate product-question answer. This report says the second answer was unused **by the one-message run**, not that it is unused by demo 01 or by a whole test suite. The command reads the folder and the list without a key or network request.

## Step 1: keep a complete list

Run the job you want to measure and retain every question key it actually used. For a single completed replay command, `--details` includes the question keys in `meta.requests`. [How to test a script with no network](../27-test-with-no-network/) shows the replay workflow. A suite runner must collect a complete list across all its commands before calling this report.

## What can go wrong

- **An incomplete list makes a false claim.** `filter --details` omits dropped rows; bare and quiet output do not carry printed question keys. Do not turn those partial outputs into a full-suite `used.txt`.
- **Exit 2 refuses a bad list.** Each nonblank line must be one lowercase 64-character question key. Duplicate and blank lines are harmless. An empty valid file reports every answer as unused from the supplied list.
- **Exit 5 refuses a damaged folder.** A fixture line that does not parse, or a folder holding both `thinkthen.jsonl` and `thinkthen.sqlite`, produces no partial report. The tool never removes an answer; `cache prune DIR` is the separate removal command.
- **The folder holds evidence and answers.** Whoever can write it can change answers read from it. Keep it private to trusted writers.

## Related how-tos

- [How to test a script with no network](../27-test-with-no-network/)
- [How to gate a script step on a yes/no answer](../01-refund-gate/)

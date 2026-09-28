# audit exits 0 when no answer matches the key

Status: Closed 2026-09-26 by ticket 0135.

Filed on 2026-09-25 while writing the Beatles Bench worked-example pages (bench ticket 0010). The build under test was thinkthen 0.0.1 at 02dc0b96. No audit code changed on main since then. No key was used, and no request left the machine.

## What happens

`thinkthen audit` exits 0 when not one answer matches the key. It prints "0 labeled" and a grade of dashes. A script sees the same exit as a run that graded every answer.

The common cause is a wrong `--id`. The key names each record by one field, and the answers carry it under another. Without the right pointer, every answer goes unlabeled.

## Reproduction

Offline, from the committed Beatles Bench files. The answers in `examples/11-audit/rows.jsonl` keep their record as `{"id": "audit-01", "input": "Here Comes the Sun"}`. The key names songs by title, as in `{"id": "Here Comes the Sun", "value": "yes"}`. The right call passes `--id /input`.

    $ cd path/to/beatles-bench/examples/11-audit
    $ thinkthen audit rows.jsonl key.jsonl --id /input --table | head -2
    Is this song on the album Abbey Road?  (decide, 70 rows, 70 labeled, 0 failed, rule as run)
      agreement 0.714 (95% 0.599 to 0.807): 50 right, 20 wrong, 0 unresolved, 0 tied

Leave out `--id /input`:

    $ thinkthen audit rows.jsonl key.jsonl --table | head -3; echo "exit $?"
    Is this song on the album Abbey Road?  (decide, 70 rows, 0 labeled, 0 failed, rule as run)
      agreement - (95% - to -): 0 right, 0 wrong, 0 unresolved, 0 tied
      said yes, key no: 0   said no, key yes: 0   yes recall -   mean p(yes) -   AUC -
    exit 0

The JSON form gives `"labeled": 0` and nulls, also at exit 0.

## What the spec says

- `specification/audit.md`, the key: "A record missing from the key, or a null value, is unlabeled. Unlabeled answers count in `unlabeled` and leave every measure."
- The failure table lists no case for a run where no answer is labeled.
- The diff issue `2026-09-25-diff-exits-0-when-nothing-pairs-and-pairs-different-questions-silently.md` covers the same gap in `diff`. The fix there lives in diff's code, so this issue stands apart.

## Why it matters

The bench's audit page now warns readers to check the labeled count before they read a grade. A command that grades should say when it graded nothing. A CI job that runs audit on a nightly key would pass on an id mix-up.

## A possible fix

When the key is non-empty and a group has no labeled answer, print one line on standard error that names the group and suggests `--id`. Exit with a failure code, as the table does for other unusable input. A run where some answers are labeled keeps exit 0.

Done when a run with a non-empty key and no labeled answer exits nonzero with a line that names `--id`, and the spec's failure table lists it.

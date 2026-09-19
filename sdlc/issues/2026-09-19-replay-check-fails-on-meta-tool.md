# `probes/replay-check.sh` fails on `meta.tool`

Found on 2026-09-19 while running the acceptance checks of ticket 0016.

## What happens

`sh probes/replay-check.sh probes/NN-name` exits 1 on all six probes. Every committed run file is reported as different. Eighteen run files differ across the six probes, and no file is reported as reproduced.

## Why

Ticket 0012 added `meta.tool` to every result object. The probe rows were written by ticket 0011, before that field existed, so a committed row carries no `meta.tool` and a replayed row carries `"thinkthen 0.0.1"`. `replay-check.sh` sets aside `meta.replayed` and compares every other byte, so the added field fails the comparison on every row.

Record 0012 already names the same gap for the rows under `transforms/rows/`, which were also written before `meta.tool`. It concluded that those rows stay as they are. Nobody ran `replay-check.sh` after that change.

## The evidence that nothing else moved

Replaying all six jobs and deleting `meta.tool` from the replayed rows before the comparison reproduces all 639 rows across all 18 run files, with zero real differences. The break is that one field and nothing else.

## What this is not

No recording changed. A recording is keyed by its request, and no request changed. The rename of `recipes/` to `transforms/` in ticket 0016 did not cause this: the failure reproduces on the commit before that rename.

## The levers

1. Teach `replay-check.sh` to set `meta.tool` aside the way it sets `meta.replayed` aside, and say on the page that a row older than the field is compared without it.
2. Rewrite the committed probe rows with the field, which edits the evidence of a landed record.
3. Leave it, and treat `replay-check.sh` as a check that ran once.

Lever 1 is the cheap one and keeps the record's rows untouched. Ticket 0016 did not take it, because that ticket changes words, pages, a license file, and a workflow, and it changes no check.

# The `annotate` spec example fails on its own inputs

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 03, finding 2-1. Blocks 0.1 under goal 4, honest docs.

## What happens

`specification/annotate.md` defines `triage.json`. Its `unresolved` question reads `"on": "/body"` (line 23). The page then runs the set on text input: `thinkthen annotate triage.json < issue.txt` (line 81), and `--jsonl --field /body`, and `--lines`. Each of those three runs exits 2 with `the input is not valid JSON`. Only `--jsonl` with no `--field` works. The error blames the input and not the question.

`spec/` holds no executable `annotate` page, so no gate runs these examples.

## Checked on main

Verified: `annotate.md:23` carries `"on": "/body"`, and line 81 feeds it `issue.txt`. `spec/` has no annotate page. The exit 2 runs come from the report.

## What would fix it

Make the examples match the set. Either give the text examples a set with no `on`, or change their input to JSON objects. Add an executable `spec/annotate.md` that runs every example on the page. The `on` re-parse issue (`2026-09-26-annotate-on-reparses-selected-text-as-json.md`) may change what the right example is, so fix the two together.

## Done when

Every example on `annotate.md` runs from an executable page and exits as the page says.

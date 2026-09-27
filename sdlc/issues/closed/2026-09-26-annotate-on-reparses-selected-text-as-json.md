# `annotate`'s `on` re-parses selected text as JSON

Status: Closed 2026-09-26 by ticket 0161 (`sdlc/records/0161-build-annotate-reads-what-it-names.md`). `on` reads inside the JSON value the record selected and never parses a string. Text refusals name the question. Filed 2026-09-26 by the queue owner from local experiment 273, report 03, finding 2-2. Blocks 0.1: the same set and flags pass some records and stop on others, depending on the data.

## What happens

`on` selects a member of the record. When the selection is text, the command turns it back into bytes and parses them again as a JSON document (`crates/thinkthen/src/cli/annotate.rs:300`, `nested.record(base_evidence.as_text()?.as_bytes())`). So:

- A `/body` string that happens to hold JSON text is parsed, and `on` sends only part of it to the model.
- A `/body` string of plain text stops the run at exit 2 with `the input is not valid JSON`.
- A `--lines` line that happens to be JSON gets members, though `records.md` says "A text line has no members."

The error names neither the question nor `on`.

## Checked on main

Verified by reading `cli/annotate.rs:300`. The three runs come from the report.

## What would fix it

Apply `on` to the selected JSON value, never to its re-serialized text. Refuse `on` over a text selection with a message that names the question and says text has no members. Add an edge-case table: JSON text inside a string, plain text, and a JSON-looking line under `--lines`.

## Done when

A record's result no longer depends on whether a string happens to hold JSON, and the table pins each case.

# A blank line stops a `--lines` run, and the spec does not say so

Status: closed by ticket 0162 on 2026-09-27. Line framing skips blank text before batching while preserving original input positions. Space, CRLF and trailing blank cases pass; blank JSONL remains a usage error.

## What happens

Under `--lines`, one empty or white-space line stops the run at exit 2 with `the evidence is empty or blank`. A trailing `\n\n` does the same after the last real line. A blank line in the middle drops every record after it. The output is a silent prefix, and the reason appears only on standard error.

Report 01 ran `printf 'keep a\nkeep b\n\nkeep c\n' | thinkthen filter 'Q.' --replay ...`. It printed two rows and stopped at record 3, exit 2. Report 11 saw the same with `decide --lines`, with a line of spaces, and with a CRLF blank line.

## What the spec says

`specification/records.md` line 14 says only "Each line is one text record. A trailing newline ends the last record". It says "No blank lines" only for `--jsonl`. `audit` skips blank lines. The spec README says a behavior absent from the spec is absent from the tool.

## Checked on main

Verified: `records.md:14` reads as above, and the message comes from `crates/thinkthen/src/core/text.rs:19`. The runs come from the reports.

## What would fix it

Choose one rule and write it into `records.md` and each verb page that takes `--lines`.

- Skip blank lines and still count them, as `audit` does. A grep user expects this.
- Or keep the refusal, name "blank line" in the message, and document a `grep -v '^[[:space:]]*$'` step.

The queue owner picks the rule and records why.

## Done when

The chosen rule is on the pages, and an edge-case table covers an empty line, a line of spaces, a CRLF blank line and a trailing blank line.

## Resolution

The independently reviewed code at `66da9fb1` and focused passing checks are recorded in `sdlc/records/0162-a-record-stream-ends-cleanly.md`. The coordinator also ran the selected 31-ID command boundary: 25 passed, six deliberate not-run cases, zero failures.

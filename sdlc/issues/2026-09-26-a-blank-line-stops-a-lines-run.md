# A blank line stops a `--lines` run, and the spec does not say so

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, reports 01 (issue 2) and 11 (issue 1). Blocks 0.1: it is the default framing for `filter`, and the spec says nothing.

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

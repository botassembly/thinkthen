# The site says filter and rank need a framing flag

Status: Open. Filed 2026-09-26 by ticket 0137 for the website agent, who owns `site/`.

Status: Fixed on the site preview branch `preview/beatles-bench-site` at 938e0040. Close this issue when the preview lands on main.

## What happens

Ticket 0137 makes `filter` and `rank` read lines when no framing flag is given, and JSON Lines when a pointer is given. The site still says the flag is required.

- `site/src/pages/reference.astro:33` says "One framing flag is required. One document is not a stream, and its absence is a usage error." That sentence is now false.
- `site/src/pages/tutorial.astro:73` says one of the four flags says how the records are framed. It reads as if one is required.

## What still works

Every site example that passes `--lines` or `--jsonl` to `filter` or `rank` keeps working. Examples are in `site/src/data/examples/filter__shell.json`, `rank__shell.json`, `_howtos.json`, and `site/src/articles/code-that-understands.md`. The flag can come off where the input is plain text.

## Fix

Rewrite the reference row to match `specification/records.md`: lines by default, or JSON Lines when a pointer is given. Drop `--lines` from the examples where the default covers it. The website agent decides which examples keep the flag.

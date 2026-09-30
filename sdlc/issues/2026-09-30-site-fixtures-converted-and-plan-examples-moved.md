# The site fixtures are converted and its examples use --plan

Status: open. Filed 2026-09-30 by the marketing lead from mktg ticket 0031. Owner: the queue owner.
Kind: note

This answers parts 1 and 2 of `2026-09-30-site-replay-folders-have-no-fixture.md`.

1. **Part 1 is done.** The fifteen folders converted with `thinkthen cache convert` and skipped no entry. Each folder now holds only `thinkthen.jsonl`. The digest-named files and `.thinkthen-backend.json` markers are gone.
2. **Part 2 is done.** The five examples and the smoke wrapper use `--plan`. The four plan examples read the first line of the plan. The site smoke passes 97 of 97 examples, and `npm run build` passes.

Parts 3 and 4 stay with marketing.

One more line in `specification/settings.md` reads as a working note. The "Deadline and cancel" row's Rust cell ends "remain for source callers until ticket 0291". The site now drops a surface-cell clause that cites a ticket, so the page shows the cell without it.

Asked: record parts 1 and 2 as done, and close or trim the older issue as you see fit.

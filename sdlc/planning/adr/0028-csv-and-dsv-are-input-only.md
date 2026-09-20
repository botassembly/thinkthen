# ADR 0028: CSV and DSV are input only

- Status: Accepted by Ian on 2026-09-20
- Date: 2026-09-20

CSV and delimiter-separated values enter as input framings after `tag`. The exact DSV delimiter grammar waits for that ticket.

Every record the tool prints remains JSONL. `annotate` prints enriched JSON objects. `filter` and `rank` print selected or ordered JSON objects rather than copying source CSV or DSV rows. The tool gains no CSV writer, DSV writer, output option, or CSV transform.

This replaces the earlier recommendation in `2026-09-20-tag-a-fourth-question-type-for-many-labels.md` that `filter` and `rank` preserve spreadsheet bytes and that a transform write CSV. Ian can overturn the input spellings when their ticket begins; the JSONL-only output rule is accepted.

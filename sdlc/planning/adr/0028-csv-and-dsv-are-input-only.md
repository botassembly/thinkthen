# ADR 0028: CSV and TSV are input only

- Status: Accepted by Ian on 2026-09-20
- Date: 2026-09-20

CSV and TSV enter as input framings after `tag`. The exact TSV grammar waits for that ticket.

Every record the tool prints remains JSONL. `annotate` prints enriched JSON objects. `filter` and `rank` print selected or ordered JSON objects rather than copying source CSV or TSV rows. The tool gains no CSV writer, TSV writer, output option, or CSV transform.

This replaces the earlier recommendation in `2026-09-20-tag-a-fourth-question-type-for-many-labels.md` that `filter` and `rank` preserve spreadsheet bytes and that a transform write CSV. Ian can overturn the input spellings when their ticket begins; the JSONL-only output rule is accepted.

## Amendment, 2026-09-20: TSV is the second input spelling

Ian corrected the dictated term before parser work began. The two explicit input framings are CSV and TSV. CSV uses a comma and TSV uses a tab. Both require a header, treat every cell as a string, and turn an empty cell into an empty string. The tool gains no generic delimiter mode.

The amendment replaces the earlier generic delimiter term with TSV. The accepted JSONL-only output rule is unchanged. The input ticket fixes the remaining parser rules.

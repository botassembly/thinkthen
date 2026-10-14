# 0588: Experiment: the one-sheet spreadsheet writer keeps workbooks intact

Status: COMPLETE.

Milestone: 0.3

Reviews: revision a37ca83, accept

## Outcome

Learn whether a writer that copies every part of an `.xlsx` or `.xlsm` file byte for byte, and rewrites only the chosen sheet and the few parts that must agree, adds answer columns without damaging real workbooks.

## Evidence

- Starts from: the marketing repo's `products/thinkthen/0.3/prd/08-spreadsheets.md`; ThinkThen's CSV and TSV readers under `crates/thinkthen/src/`.
- Keeps: no customer or licensed data; every workbook is generated or public with its source and license recorded. No product code changes, no paid model calls, no publishing, and no agent or machine time taken from 0.2 work.
- Changes: `sdlc/records/0588*` only. Prototype and corpus stay in an owned numbered experiment folder with `OWNER` reading `thinkthen/0588`; tools run in `docker run --rm --label pm.repo=thinkthen --label pm.ticket=0588`.
- Method: build a corpus of at least 30 workbooks with tables, charts, pivots, macros, shared and inline strings, shared formulas, merged cells, conditional formats, defined names and several sheets. A Rust prototype appends two value columns to one sheet of each.
- Proof: for every workbook, every other part is byte-identical; Microsoft's free Open XML SDK validator reports no error the original lacked; LibreOffice opens and converts the output without error and shows the new values. A 1,000,000-row sheet is read and written with flat memory, timed against a plain copy. At the end the coordinator gives Ian the outputs to open once in the free Excel for the web, since no machine here has desktop Excel.
- Stop rule: every check passes, or a damage case the prototype cannot fix is named.
- Decision it changes: whether spreadsheets stay in 0.3 or move to 0.4, and Ian's decision 7.
- Machine: the Beelink.
- Defers: the shipped writer goes to a 0.3 product ticket.

## Progress

- 2026-10-14 started
- 2026-10-14 next: Await Ian opening the delivered workbooks in Excel for the web and reporting any repair prompt or missing values; local finding passed fresh review.

Ian ruling, 2026-10-14. Ian opened the three most complex outputs (the macro, chart and pivot workbook; the chart and merged-cells workbook; the other-sheet table workbook) in Google Sheets, and all three looked right. That check replaces the Excel for the web opening for this experiment: the writer passes, and spreadsheets stay in 0.3. The product writer is opened in Excel for the web before the 0.3 release.

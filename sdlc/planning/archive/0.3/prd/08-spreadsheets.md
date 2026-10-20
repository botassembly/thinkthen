---
date: 2026-10-13
status: draft PRD
release: 0.3
source: Ian's rulings in the 0.3 README ("Excel is a must", "Excel is a new tool", "Spreadsheets", 2026-10-12 and 2026-10-13); Ian's idea note on spreadsheets in and out; PRD 00 and PRD 02; experiment 229; ThinkThen 0.2's `specification/annotate.md`, `records.md`, `channels.md`, `roadmap.md`, ADR 0028, and the Polars, pandas, R and DuckDB surfaces. The 25 worked questions and sources are in [08-spreadsheets-questions.md](08-spreadsheets-questions.md)
---

# PRD 08: Spreadsheets

An agent hands ThinkThen an Excel or CSV file, names the columns to read and the columns to fill, and gets a file back with an answer in every cell. It is `annotate` for spreadsheets: grep for Excel.

## Who and what's in it for them

- **The agent working for a person** fills new columns in a local workbook with one command and hands back a file. Proof: a fresh agent, given only `thinkthen help`, fills two columns in a 1,000-row workbook on its first try.
- **The person who opens the file** finds their own workbook, with new columns and nothing else changed: other sheets, formatting, formulas, charts and macros intact. Proof: a corpus round trip in which every other part is byte-identical and every file opens in Excel and LibreOffice without a repair prompt.
- **The cautious user** sees the columns read and added, the rows the store already answers and what the rest will send, before anything is sent. Results go to a new file unless asked; in place keeps a backup. Proof: a demonstrated plan, and a test that a stopped run leaves the original untouched.
- **The user with a big sheet** stops and restarts at no extra cost, because every answered row sits in the store. Proof: a 100,000-row run killed halfway sends only unanswered rows on rerun, with flat memory.
- **The reviewer** corrects answers in Excel and sends the file back as right answers. Proof: edited cells become settled right answers that `eval grade` then uses.

## Use cases

U8 and U9 from PRD 00 in full; U18 to U20 on spreadsheet runs; U26 through a spreadsheet import; U11 as recipes for data frames and DuckDB. U10, Google Sheets inside the sheet, stays later.

## The design

**No new word.** A spreadsheet is a file format, so it is a reader and a writer on functions that already exist. `--excel` joins `--csv` and `--tsv` as a framing. Rows become records keyed by header name, so 0.2's `--field /Body` and a question set's `on` already pick the input columns.

```text
thinkthen annotate triage.json --excel --input intake.xlsx --sheet Intake --plan
thinkthen annotate triage.json --excel --input intake.xlsx --sheet Intake --output intake-triaged.xlsx
thinkthen choose "Which kind of request is this?" bug feature other \
    --excel --input intake.xlsx --sheet Intake --field /Body --into kind --in-place
thinkthen decide "Does the note mention a biopsy?" --csv --input notes.csv --field /Note --into biopsy --output notes-biopsy.csv
```

- **Which functions write cells.** `decide`, `choose`, `score`, `tag` and `annotate`, which give one value per record. Each `annotate` question's name is its column header; a single function names its column with `--into`. The other five functions keep 0.2's JSON output, and a person filters or sorts in Excel on a `decide` column.
- **Where results go.** With no output option, rows print as JSON lines, as 0.2 prints CSV today. `--output FILE` writes a new file in the input's format. `--in-place` first copies the input to a timestamped backup beside it, and refuses while Excel has the file open. Both write a temporary file and rename it only on success, so a stopped run leaves no half-written file; the store keeps every answer, so the rerun is free.
- **The writer edits one sheet and copies the rest.** An `.xlsx` or `.xlsm` file is a zip of parts. ThinkThen copies every part byte for byte and rewrites only the chosen sheet's XML, appending value cells after the last used column, plus the few parts that must agree, such as a table's range. Macros, charts and pivots are never parsed, so they cannot be lost. `--output` and `--in-place` share this writer. Libraries that model the whole workbook, such as umya-spreadsheet and openpyxl, have documented losses of charts, comments and shapes. `.xls`, `.xlsb` and `.ods` are read only and write a new `.xlsx` of values. CSV appends cells to each record's original bytes.
- **Cells.** A yes or no is a boolean cell, a choice or tag is text, with tags joined by "; ", and a score is a number. Not sure is an empty cell. A failure is the text `#FAILED kind`, and the run exits 6. `--details` adds `NAME_probability` and `NAME_id`.
- **Reading.** `--sheet NAME` is required when a workbook has several sheets; the refusal lists them. `--header-row N` defaults to 1. `--rows FIRST:LAST` judges a slice in Excel's numbering. Every cell becomes a string, as 0.2 does for CSV: ISO 8601 dates, shortest-form numbers, and formulas as their saved values. A merged block gives each row its top-left value. An existing output column is refused; `--fill` writes only its empty cells, so a growing sheet is topped up and a reviewer's edits survive.
- **The dry run is `--plan`.** For a spreadsheet it shows the sheet, header, rows, input columns, output columns in order, rows the store already answers, requests and estimated tokens for the rest, the target file and the backup name. Spend caps apply as everywhere.
- **Corrections come back** through PRD 02: `curate settle --import reviewed.xlsx --excel --sheet Intake --column kind` reads `kind_id` and settles each cell the reviewer changed.
- **Libraries** gain no Excel calls. pandas, Polars and R read their own frames and already have `annotate_frame`, `.tt` and `tt_annotate`; DuckDB reads with `read_xlsx` and writes with `COPY ... (FORMAT xlsx)`. Each gets a recipe. The Rust crate exposes the reader and writer, as it exposes `TableReader` today.

## What the writer experiment showed

Spreadsheets stay in 0.3: Ian made them conditional on the writer prototype, and it passed. A Rust prototype streamed one sheet's XML, appended a yes or no column and a text column, and copied every other part of the file unchanged.

- Across 31 workbooks with tables, charts, pivots, a macro project, comments, conditional formats, data validation, merged cells, defined names, frozen panes and shared formulas, all 375 untouched parts stayed byte-identical, and existing cells on the written sheet kept their values.
- Microsoft's Open XML SDK found no errors in any original or output. LibreOffice opened all 62 files, and every original cell kept its value.
- Ian opened the three most complex outputs in Google Sheets on 2026-10-14, and they looked right.
- A million-row sheet took 26 seconds against a plain copy's 0.3, with memory flat at about 14 MB from 10,000 rows to a million.
- A first version damaged an unrelated table on another sheet. The fix ties table edits to the chosen sheet's own relationships, and a regression file keeps it fixed.

The prototype handles one fixed layout. The product writer still needs: worksheet names, any append position, refusing an existing output column, `--fill`, sparse and self-closing rows, namespace-prefixed XML, tables with totals rows, and refusing signed, encrypted or damaged files. Every one gets a corpus file. The workbooks were variations on one template, so the release check adds real workbooks from public sources.

## What must be true

| Risk | What retires it |
| --- | --- |
| The one-sheet writer corrupts real workbooks | Prototype passed, above. The product writer repeats the corpus check, adds real workbooks, and is opened in Excel for the web before release; desktop Excel on Windows and Mac is not available to test |
| Large sheets exhaust memory or time | Prototype passed: a million rows in 26 seconds with flat memory. The product writer repeats it |
| Evidence differs from what the person sees | A fixture of dates, percentages, currency and formula cells checked against Excel's display |
| An agent cannot drive it from help | The fresh-agent test above |
| Corrections cannot round-trip | The PRD 02 import test above |

## Out of 0.3

Google Sheets inside the sheet, which needs a reachable hosted proxy, an Apps Script menu action that writes values, and a per-sheet token; a Sheets user downloads, runs and uploads. Excel formulas such as `=TT.DECIDE(...)`, which experiment 229 showed need a signed add-in. Writing `.xls`. Other CSV delimiters and encodings. Several sheets in one run. Hidden or filtered rows as a selection.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 6, 7 and 14.

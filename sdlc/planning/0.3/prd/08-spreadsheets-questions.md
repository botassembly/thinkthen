---
date: 2026-10-13
status: draft PRD
source: as [08-spreadsheets.md](08-spreadsheets.md), plus the outside sources listed at the end
---

# PRD 08 working file: the hard questions about spreadsheets

This file backs [08-spreadsheets.md](08-spreadsheets.md). Each question gives the options, the reasoning and a recommendation. **Ian** marks the three that need his product direction; the rest the PM can carry into tickets, and Ian can overturn any of them. Claims about 0.2 cite pages in [`specification/`](../../../../specification/) or [`sdlc/`](../../../../sdlc/).

## What 0.2 already gives

- `annotate` asks a saved question set of each record and adds one field per question. Each question's `on` pointer selects the part it sees, and the help says "a table asks different questions of different columns" (`annotate.md`).
- `--csv` and `--tsv` read a table with a required header. Each row becomes a JSON object of string cells keyed by header name, a UTF-8 byte-order mark is ignored, and the `csv-core` grammar owns quoting (`records.md`, "CSV and TSV").
- ADR 0028 makes every output JSON lines: "The tool gains no CSV writer, TSV writer, output option, or CSV transform." The roadmap holds `--output FILE` because "it would write a file", and `channels.md` says the tool "never writes a file the user did not name".
- The libraries already judge frames: Rust Polars `annotate_frame` returns "your frame plus one column per question, then `failed`" (`libraries/polars/README.md`); Python has opt-in `series.tt` namespaces for pandas and Polars (`libraries/python/README.md`); R takes data frames by row (`libraries/r/README.md`). DuckDB has `thinkthen_annotate(set, text)` (`databases/duckdb/README.md`).
- Experiment 229 (2026-09-21) found that Excel formulas need a signed native add-in built on Windows, that Google Sheets needs an endpoint reachable from Google's cloud, and that a sheet of formula cells is a "recalc storm" unless answers are cached and ranges cross in one call. It ruled an empty cell for not sure.

So the gap is small and specific: read Excel, choose a sheet and header, and write the answers back into a file.

## Shape and words

**Q1. A new command, a new function, or a format on existing functions? (Ian)**
Options: (a) a top-level word such as `thinkthen sheet` or `thinkthen excel`; (b) a new function; (c) `--excel` beside `--csv`, plus an output option, on the functions that return one value per record. Ian called it "a new tool" and "annotate for spreadsheets". PRD 00 principle 3 makes a new top-level word Ian's call, and PRD 00 Q33 already recommends (c). A spreadsheet adds no new kind of judgment; it changes only where records come from and where answers go, which is exactly what framing flags do in 0.2. A new word would duplicate every option of `annotate` and `choose` and teach an agent two ways to do one thing. A toolbox tool can still wrap a recurring sheet job by name (`run intake/triage --input intake.xlsx`), which may be what "tool" meant. Recommendation: (c), and ask Ian to confirm.

**Q2. Which functions write cells?**
A cell holds one value. `decide`, `choose`, `score`, `tag` and `annotate` return one value per record. `filter` drops rows and `rank` reorders them; writing either into a workbook would delete or move the person's rows and break their formulas. `find` returns one row from a set; `recognize` and `relate` return lists of spans and edges. Recommendation: the five value functions write cells. "Grep for Excel" becomes a `decide` column that the person filters with Excel's own filter. The other five keep 0.2's JSON output on spreadsheet input, so an agent still can filter or rank a sheet into JSON lines.

**Q3. What names the input columns?**
Options: a new `--columns A,B`, Excel letters, or 0.2's `--field /Header` and a question's `on`. Rows already become objects keyed by header name, so `--field /Body` and `"on": ["/Body", "/Age"]` already select columns and already send several columns as one object (`records.md`). A JSON pointer escapes `/` as `~1`, and spaces are legal, so `--field "/Patient note"` works. Excel letters move when a column is inserted; header names do not. Recommendation: header names through `--field` and `on`, one concept with one name. A header that is blank or duplicated is refused, as 0.2 does for CSV.

**Q4. What names the output columns?**
For `annotate`, each question's name already becomes a field, so it becomes the header. A question name allows only lowercase letters, digits and underscores (`annotate.md`), which suits a column an agent names. A single function needs a name: `--into NAME`. Requiring it, rather than defaulting to `decide`, follows Ian's "name the output columns" and stops two runs colliding on one default. Recommendation: question names for `annotate`, a required `--into` for the others when writing a file. Header text outside the question grammar can be renamed in Excel afterwards.

**Q5. Where do new columns go?**
Inserting a column in the middle shifts every cell reference, formula, chart range and conditional format to its right, which a faithful writer would have to rewrite. Appending after the last used column of the header row touches nothing. Recommendation: always append, in question order. No `--after` option in 0.3.

## Where results go

**Q6. What happens with no output option?**
Principle 5 says 0.2 behavior is the default, and 0.2 prints CSV rows as JSON lines. An agent that pipes the result to `jq` or DuckDB wants exactly that. Writing a file nobody named would break `channels.md`. Recommendation: no `--output` and no `--in-place` prints JSON lines, as today, for Excel too. A file is written only when named.

**Q7. New file or in place, and how is in place made safe?**
Ian ruled a new file by default, and in place only when asked, after a backup. Options for the backup name: `book.xlsx.bak` (Excel will not open it by double-click), `book.backup.xlsx` (a second run overwrites the first backup), or `book.backup-20261013T140512.xlsx` (unique, opens in Excel). Excel on Windows locks an open file; Excel on Mac does not, and saving over a workbook open in Excel on a Mac loses whichever save comes second. Excel creates an owner file named `~$book.xlsx` beside any open workbook. Recommendation: `--in-place` copies to a timestamped backup that opens in Excel, prints its path, and refuses while `~$NAME` exists, naming Excel as the reason. `--plan` shows the backup name. The backup is the one file the tool writes without the user naming it, so `channels.md` needs one sentence of amendment.

**Q8. What happens to a run that stops halfway?**
0.2's channels print finished output before a stop. For a file, a partial file looks finished. The store already holds every answer received, so a rerun costs nothing for those rows. Recommendation: write to a temporary file in the target folder and rename only after the last row, for `--output` and `--in-place` alike. A stopped or failed run leaves the original and any earlier output untouched and prints the rerun command. This also answers the roadmap's held concern, "a demo that must not leave a half-written file behind".

**Q9. What if an output column already exists?**
It happens on a second run of the same command, and on a living sheet where rows are added weekly. Overwriting would erase a reviewer's corrections. Refusing forces a new column every week. Recommendation: refuse by default, naming the column. `--fill` writes only empty cells and `#FAILED` cells of an existing output column. Answered rows come from the store at no cost, new rows are asked, and every cell a person typed survives.

**Q10. Does this break 0.2's ADR 0028 and roadmap? (Ian)**
Yes. ADR 0028, accepted by Ian, says output stays JSON lines and the tool gains no CSV writer or output option. The roadmap holds `--output FILE`, and the ideal state says "Thinkthen will never run a command or take an action on a judgment". Writing answers into a file the user named is not acting on a judgment; it is printing to a different place. Recommendation: amend ADR 0028 so that `--output` and `--in-place` exist for spreadsheet and CSV framings only, and keep JSON lines everywhere else.

## Excel files

**Q11. Which Rust crates read and write workbooks faithfully?**
- calamine 0.36.1 (MIT) reads `.xls`, `.xlsx`, `.xlsm`, `.xlsb` and `.ods`. `worksheet_range` returns each formula's saved value, and `worksheet_formula` the formula text. Dates come back as `ExcelDateTime` with the 1904 flag. It reads merged regions for `.xlsx` and `.xls` only, picks a header with `with_header_row`, reads Excel tables in `.xlsx`, and walks cells one at a time with `worksheet_cells_reader` for `.xlsx` and `.xlsb`. Its maintainer asked for more maintainers in July 2026.
- rust_xlsxwriter 0.99.1 writes new files only: its README says "it cannot be used to modify an existing file". It has a low-memory mode that writes rows in order, and makes `.xlsm` only from a macro part extracted beforehand.
- umya-spreadsheet 3.1.1 opens and saves existing `.xlsx` and `.xlsm`, but it parses the whole workbook into its own model and writes it back. Its tracker has open reports of charts corrupted after open and save, lost comments, dropped pivot caches, external links that make Excel offer a repair, silently lost cells, and over 500 MB of memory for a 10,000-row export. It has no code for external links, custom XML, sparklines, timelines or ActiveX controls, so those cannot survive.
- ooxmlsdk 0.13.2 claims round-tripping with typed parts; whether it keeps parts it does not model is unverified. edit-xlsx has had no release since 2024.
- Python shows the same trade: openpyxl's tutorial warns that shapes are lost when an existing file is opened and saved, and keeps macros only with `keep_vba`.
- Other Rust command lines, qsv and xan, read with calamine and write new files with other crates. Neither edits a workbook in place.
- Recommendation: calamine to read; a ThinkThen one-sheet writer (Q12) to write; rust_xlsxwriter only for a new `.xlsx` from a read-only format. No whole-workbook model library: each one can lose what it does not model, and the ones that exist do.

**Q12. How does the writer keep everything else intact?**
An `.xlsx` is a zip of XML parts: one per sheet, shared strings, styles, tables, charts, drawings, and for `.xlsm` a `vbaProject.bin`. Adding columns of plain values to one sheet changes that sheet's XML, plus a few parts that must agree with it: the sheet's `dimension`, the `ref` and column list of an Excel table that the new columns join, and the auto-filter range over the header. Writing answers as inline strings leaves the shared-string part alone. Writing values, never formulas, leaves the calculation chain valid, which avoids Excel's "Removed Records: Formula from /xl/calcChain.xml" repair that other surgical writers hit. Every other part is copied byte for byte, so charts, pivots, macros, comments and styles cannot be lost: they are never parsed. JavaScript's xlsx-populate takes this approach and says it preserves "styles and other content" for the same reason. What can still go wrong: shared formulas and conditional formats whose ranges end at the old last column, data validation over whole rows, a table followed by other data, and new cells taking the default style. Recommendation: one streaming one-sheet writer for both `--output` and `--in-place`, so a new file is exactly as faithful as an in-place edit. Retire the risk with the corpus prototype before any ticket for the writer. umya-spreadsheet is the fallback only if the prototype fails.

**Q13. What about `.xlsm`, `.xls`, `.xlsb` and `.ods`?**
`.xlsm` is the `.xlsx` package plus a macro part, so the one-sheet writer keeps the macros by copying them and keeps the extension. No mature Rust crate writes `.xls`: the three that claim to appeared in 2026 with under a thousand downloads. Nothing writes `.xlsb`, and `.ods` is a different package. Recommendation: read all five; write `.xlsx` and `.xlsm` faithfully; for `.xls`, `.xlsb` and `.ods`, refuse `--in-place` and require `--output NAME.xlsx`, which writes the chosen sheet's values as a new workbook through rust_xlsxwriter. The plan states that formatting, formulas and other sheets are not carried. `--excel` detects the format from the file's bytes, a zip or an old compound file, not from its name, keeping 0.2's rule that the tool never guesses from a filename.

**Q14. Which sheet, which header, which rows?**
pandas, Polars and DuckDB default to the first sheet, and qsv silently falls back to the first sheet when a named one is missing. For an agent, a wrong sheet is a paid run over the wrong data; a refusal costs one more command. Excel files often carry title rows above the header. Excel's row numbers are what a person says aloud: "rows 2 to 500". Recommendation: one sheet is used silently; several require `--sheet NAME`, and the refusal lists them. `--header-row N`, default 1; rows above it are kept and never read. `--rows FIRST:LAST` in Excel's numbering judges a slice, so an agent can try twenty rows, check, then run all, and the store makes the full run cost no more than if it had started there. Selecting an Excel table by name is a small later addition. One sheet per run.

**Q15. How do Excel values become evidence?**
0.2 makes every CSV cell a string, so the same table gives the same evidence and the same stored answers. Excel holds typed values: a date is a number with a date format, `12%` is 0.12, and a formula has a saved result. Rendering Excel's display format exactly needs Excel's format engine. Recommendation: every cell becomes a string: dates and times as ISO 8601 (honoring the 1904 date system), numbers in shortest round-trip form, booleans as `TRUE` and `FALSE`, error values as their text such as `#N/A`, and formulas as their saved values. A formula with no saved value, which some libraries write, is reported by the plan with a count and the advice to open and save the file in Excel. The prototype checks percentages and currency against what a person sees; showing the formatted text is a later option if it matters.

**Q16. Merged cells?**
A merged block stores its value in the top-left cell only, so a reader sees blanks below it. calamine reports merged regions for `.xlsx` and `.xls`, not for `.xlsb` or `.ods`. People merge a category down many rows and read it as applying to each. Recommendation: in an input column, every row of a merged block gets the top-left value, and the plan counts such cells; for `.xlsb` and `.ods` the plan says merged blocks were not detected. An output column that would land inside a merged block is refused.

**Q17. How big can a sheet be?**
Excel allows 1,048,576 rows by 16,384 columns and 32,767 characters a cell. A dense read builds the whole grid, and calamine's tracker shows a 2 KB file with far-apart cells exhausting memory that way. Recommendation: read cells incrementally with `worksheet_cells_reader`, send records in batches as 0.2 does, keep the answers in the store, then stream the sheet's XML a second time and insert each row's cells. Memory then grows with one row, not the sheet. The prototype times a 1,000,000-row sheet against a plain copy.

## Cells

**Q18. How does each answer look in a cell, and where do probabilities go?**
Excel has booleans, numbers, text and errors, and no lists. Experiment 229 ruled an empty cell for not sure. A failure must never look like not sure (`result.md`: "a failure is never a null"). Recommendation: yes or no as a boolean cell; a choice as its label; a score as its number; tags as labels joined by "; ". Not sure is an empty cell. A failure is the text `#FAILED backend` with the run exiting 6. `--details` adds `NAME_probability`, so an empty answer with a probability reads as not sure and an empty pair reads as not asked, and `NAME_id`, the answer id. CSV uses `TRUE` and `FALSE`, which Excel, pandas and R all read as booleans.

**Q19. How does a reviewer's correction come back?**
PRD 02 keys a right answer on the question id and the evidence, and settles through `curate settle`, with an `--import` for files. The answer id in `NAME_id` leads straight to the stored answer, its question and its evidence, even after the reviewer sorts or filters the sheet. Recommendation: `curate settle --import reviewed.xlsx --excel --sheet Intake --column kind` settles each row whose `kind` cell differs from the stored answer, with the person running it as the actor and "imported" as the stratum. Unchanged cells are not settled, because an unchanged cell does not show that anyone looked. This needs PRD 02 to accept a table import beside its JSONL import.

**Q20. What does the store keep about the file?**
`data.md`'s evidence carries an optional source. 0.2 keeps file positions out of every key (`records.md`). Recommendation: the call record keeps the file path and sheet, and each answer keeps its row as its position. Keys never include them, so a renamed file or a re-sorted sheet still reuses its answers, and `answers list` can still show where an answer came from.

## CSV

**Q21. Which CSV dialects and encodings?**
Excel's "CSV (Comma delimited)" saves in the local code page, and in locales whose decimal mark is a comma Excel uses a semicolon. "CSV UTF-8" is UTF-8 with a byte-order mark. ADR 0028's amendment ruled two spellings, comma and tab, and "no generic delimiter mode". Recommendation: keep 0.2's strict UTF-8 comma and tab. When a header holds semicolons and no commas, or the bytes are not UTF-8, the refusal names the cause and the fix: save as "CSV UTF-8", or run `--excel` on the original workbook. Ian may want a `--delimiter` later (question 3 in the PRD).

**Q22. How is a CSV written back?**
Re-serializing every row would change quoting, line endings and number spelling in cells nobody asked to change. Recommendation: append the delimiter and each new cell to the original bytes of each record, before its line ending, quoting a new cell only when it needs it. A byte-order mark on the input is kept, because Excel needs it to read UTF-8. Untouched bytes stay untouched, which a test can check by stripping the new columns.

## Other surfaces and the future

**Q23. How does a library user do the same?**
pandas, Polars and R users already hold the sheet as a frame and already have `annotate_frame`, `series.tt` and `tt_annotate`. They read Excel with `pd.read_excel`, `pl.read_excel` or readxl and write with their own writers, which write new files. DuckDB reads with `read_xlsx` and writes with `COPY ... TO 'out.xlsx' (FORMAT xlsx)`, new files only and no `.xls`. PRD 00 Q10 keeps tier-two calls to four languages, and Excel calls in 24 languages would be about a hundred more bindings for an edit the command line already does. Recommendation: no new library functions. One recipe page each for pandas, Polars, R and DuckDB. The Rust crate exposes the Excel reader and the one-sheet writer, as it already exposes `TableReader`, so a Rust program and the command share one code path.

**Q24. Do 0.3's choices keep the Google Sheets path open?**
Apps Script custom functions must return within 30 seconds, may use UrlFetch, Cache and Properties, run without the user's authorization, and Google advises taking a range and returning a range in one call. Scripts run from a menu get six minutes and can write values. Experiment 229 showed a cell formula per row is a recalc storm. So the later Sheets path is a menu action, "Fill columns with ThinkThen", that sends the selected rows to a hosted proxy's function door in chunks and writes values, the same as `--output`, with formulas as a small extra. Three 0.3 choices keep it open: the cell mapping in Q18 lives once in the Rust core, so the proxy returns cell-ready values; evidence is the same string row object from CSV, Excel or Sheets, so answers stored by a local run are reused later from the sheet; and every answer carries an id, so Sheets corrections use the same import. Recommendation: build nothing for Sheets in 0.3, and test these three properties in the 0.3 tests.

**Q25. How does this differ from Excel's and Google's own AI functions?**
Microsoft's `=COPILOT()` fills cells from a prompt with one fixed model (gpt-4.1-mini at the time of writing), needs a Microsoft 365 Copilot licence, is rate limited (secondary sources report 100 calls per ten minutes), and Microsoft warns results can change on recalculation and advises pasting values. Google's `=AI()` runs Gemini, generates at most 350 selected cells at a time, offers no model choice and has daily limits. Add-ins such as GPT for Sheets and Excel resell many models. ThinkThen differs in four ways a reader can check: every answer is kept in the user's own database, so a rerun is free and the same; any model works, hosted or local; the plan shows requests and tokens before anything is sent; and answers are bounded choices with probabilities and a "not sure", never generated text, which is what lets a reviewer's corrections grade every later model. "Rent your models, own your decisions" in a spreadsheet.

## Sources

- ThinkThen 0.2: `specification/annotate.md`, `records.md`, `channels.md`, `roadmap.md`, `result.md`; `sdlc/planning/adr/0028-csv-and-dsv-are-input-only.md`; `libraries/polars/README.md`, `libraries/python/README.md`, `libraries/r/README.md`, `databases/duckdb/README.md`.
- Experiment 229's notes, 2026-09-21.
- Ian's idea notes on spreadsheets in and out, and on decorating spreadsheets and CSV and TSV files.
- Microsoft, Excel specifications and limits: https://support.microsoft.com/excel/excel-specifications-and-limits
- Microsoft, COPILOT function: https://support.microsoft.com/en-us/excel/copilot-function; rate limits as reported by https://pureinfotech.com/excel-copilot-function/ and https://redmondmag.com/articles/2025/10/29/the-new-copilot-function-vastly-improves-microsoft-excel.aspx
- Google, AI function in Sheets: https://support.google.com/docs/answer/15877199
- Google, Apps Script custom functions: https://developers.google.com/apps-script/guides/sheets/functions; quotas: https://developers.google.com/apps-script/guides/services/quotas
- DuckDB excel extension: https://duckdb.org/docs/current/core_extensions/excel
- Excel CSV separators by locale: https://ablebits.com/office-addins-blog/change-excel-csv-delimiter
- calamine: https://github.com/tafia/calamine, https://docs.rs/calamine/0.36.1; issues 693 (dense range memory), 698 (call for maintainers), 706 (1904 dates)
- rust_xlsxwriter: https://docs.rs/rust_xlsxwriter and its performance page https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/performance/index.html
- umya-spreadsheet: https://github.com/MathNya/umya-spreadsheet, issues 233, 281, 285, 297, 301, 352, 370
- ooxmlsdk: https://github.com/KaiserY/ooxmlsdk; edit-xlsx: https://github.com/MortalreminderPT/edit-xlsx
- xlsx-populate: https://github.com/dtjohnson/xlsx-populate
- calcChain repair after surgical edits: https://github.com/mini-software/MiniExcel/pull/993
- qsv: https://github.com/dathere/qsv/blob/master/src/cmd/excel.rs; xan: https://github.com/medialab/xan/blob/master/docs/cmd/from.md
- openpyxl: https://openpyxl.readthedocs.io/en/stable/tutorial.html; pandas read_excel: https://pandas.pydata.org/docs/reference/api/pandas.read_excel.html; polars read_excel and write_excel: https://docs.pola.rs/api/python/stable/reference/api/polars.read_excel.html, https://docs.pola.rs/api/python/stable/reference/api/polars.DataFrame.write_excel.html

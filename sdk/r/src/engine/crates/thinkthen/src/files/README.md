# Tabular files

`thinkthen::files` reads UTF-8 CSV, UTF-8 TSV and `.xlsx` field and appends the existing ThinkThen outcomes to a new file. It preserves existing rows and workbook content. It performs file I/O outside the pure engine core.


The command exposes the same operation:

```sh
thinkthen decide 'Ready?' --backend stub requests.csv \
  --column Body --into ready
```

Repeat `--column` to select several named field columns. A single selected column supplies text and replaces any field pointer a saved question carries. Several columns supply an object, and a saved pointer must then be empty or name one of them. Annotate always receives a named object and names answer columns from its questions.

Each question adds one answer column. Yes and no are TRUE and FALSE, numbers stay numbers, and list answers read as `; `-separated text. A row that is not sure or that failed gets a missing value: an empty cell in CSV and TSV, `#N/A` in xlsx. It never reads as FALSE. `FileJob::details(true)` (`--details`) adds status, probability, error, detail and source columns after each answer; the error cell reads `code: message`. The receipt names the first twenty failed rows and counts rows that were not sure. Filter writes only kept rows to CSV and TSV. In xlsx it keeps every row, hides the rows it does not keep, and adds an AutoFilter on the answer column, so formulas that point at rows still work. Rank attaches ranks to original rows without sorting them.

The default creates a `.thinkthen` sibling. `--output` selects another new file; existing destinations are refused. `--in-place` explicitly replaces the source after keeping a distinct backup. An exclusive transient lock serializes in-place operations. Close other spreadsheet writers before replacement. After a killed process, remove a leftover lock only after confirming that its owner has stopped.

A completed engine stream publishes every outcome, including failed annotation entries; the receipt keeps the first twenty original failures. A top-level failed row fails the call after every row is delivered. An answer or detail that Excel cannot represent becomes a failed outcome with a `local.io` error cell; other rows continue. A final call failure, cancellation or file error publishes nothing. Once the output is finished, a file error before it is published, such as a failed backup, a changed source, a failed swap or a destination that appeared, leaves the finished output in a `.thinkthen` sibling of the destination, or under its temporary name beside the destination when that name cannot be made, and the error names it. An error while the output is still being written leaves nothing finished to keep. Cancellation removes provisional backups as well as temporary files. Formulas are read from saved results, never recalculated. Missing formula results and selected merged/error cells require a saved or plain-text field column. Worksheet filters and row spans widen to include new columns. Existing Excel table definitions remain unchanged. On Unix a new output file takes the source file’s permissions, limited by umask; on Windows it uses the caller’s normal creation permissions. On Unix an in-place replacement keeps the source file’s permissions; on Windows it keeps the read-only flag and the access list described below. On Windows, `--in-place` copies the source’s access list to its temporary files before any data is written and to the backup, keeping whether the list follows its folder. It stops before any call if a list exists and can’t be set, or would block replacing the source. A source with no list, as on a drive with no access lists, runs as usual. With details, each answer’s `_detail` column retains detail JSON when the result supplies it.

CSV and TSV output preserves formula-like text literally. A spreadsheet program may interpret that text as a formula when it opens the file. Use text import settings when opening untrusted output in a spreadsheet program.

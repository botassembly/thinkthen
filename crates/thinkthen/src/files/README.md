# Tabular files

`thinkthen::files` reads UTF-8 CSV, UTF-8 TSV and `.xlsx` evidence and appends the existing ThinkThen outcomes to a new file. It preserves existing rows and workbook content. It performs file I/O outside the pure engine core.

```rust,no_run
use thinkthen::{Cancel, Client, SdkRequest};
use thinkthen::files::{FileFormat, FileJob, FileOutput};

let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None)?;
let request = SdkRequest::from_json(
    r#"{"function":"decide","question":{"type":"inline","value":"Ready?"}}"#,
)?;
let job = FileJob::new(
    "requests.csv".into(), FileFormat::Csv, None,
    vec!["Body".into()], Some("ready".into()), FileOutput::NewFile(None),
)?;
let receipt = job.execute(&request, &client, &Cancel::default())?;
// requests.thinkthen.csv now contains the complete input with outcome columns.
// Inspect receipt.first_failure: a completed file may contain failed outcomes.
# Ok::<(), Box<dyn std::error::Error>>(())
```

The command exposes the same operation:

```sh
thinkthen decide 'Ready?' --adapter stub --csv --input requests.csv \
  --column Body --into ready
```

Repeat `--column` to select several named evidence columns. A single selected column supplies text; several supply an object. Annotate always receives a named object and names answer columns from its questions. Every answer column has status, probability, error and detail columns. Nested answers use JSON text. Filter writes judgments and a keep flag without removing rows. Rank attaches ranks to original rows without sorting them.

The default creates a `.thinkthen` sibling. `--output` selects another new file; existing destinations are refused. `--in-place` explicitly replaces the source after keeping a distinct backup. An exclusive transient lock serializes in-place operations. Close other spreadsheet writers before replacement. After a killed process, remove a leftover lock only after confirming that its owner has stopped.

A completed engine stream publishes every outcome, including failed annotation entries; the receipt retains the first original failure. A top-level failed row fails the call after every row is delivered. An answer or detail that Excel cannot represent becomes a failed outcome with a `local.io` error cell; other rows continue. A final call failure, cancellation or file error publishes nothing. Cancellation removes provisional backups as well as temporary files. Formulas are read from saved results, never recalculated. Missing formula results and selected merged/error cells require a saved or plain-text evidence column. Worksheet filters and row spans widen to include new columns. Existing Excel table definitions remain unchanged. New files use the caller’s normal creation permissions. The `--details` flag keeps detail JSON in each answer’s `_detail` column.

CSV and TSV output preserves formula-like text literally. A spreadsheet program may interpret that text as a formula when it opens the file. Use text import settings when opening untrusted output in a spreadsheet program.

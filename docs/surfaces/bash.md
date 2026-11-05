# ThinkThen in Bash

The one-line installer adds its command folder to your shell profiles for new terminals. Set `THINKTHEN_NO_MODIFY_PATH=1` for print-only PATH instructions; the receipt lists any profile changes.

[Where record text goes](service.md) describes keys, service selection, connection settings and [relation rule fields](service.md#relation-rule-fields).

A whole JSON null record, a record without the selected field, or a selected field containing JSON null, such as `{"text":null}` with `/text`, returns a failed `record.missing` row locally and sends no HTTP request for that record. An empty or blank selected value, such as `""`, `{}` or `[]`, returns a failed `record.empty` row the same way. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

ThinkThen judges text with ten functions. Use Bash and install the command with `curl -fsSL https://thinkthen.dev/install.sh | sh`, or run `brew install botassembly/thinkthen/thinkthen`. See the [release downloads](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) for other platforms and runtime requirements.

A normal call uses the default backend and reads its key from `THINKTHEN_API_KEY`. Supply the key through your environment or secret manager. Keep it out of scripts, output and reports.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Keep every collection outcome, including answered false and failed rows.

```bash
printf '%s\n' '["first","second"]' |
  thinkthen decide --from jsonl --json 'Does the customer request a refund?'
thinkthen decide --from jsonl --field /text --json \
  'Does the customer request a refund?' records.jsonl
```

Every function can read one record or a record stream. `--from text` reads one whole text per file; `--from jsonl` reads one record per nonblank line. `--json` writes one result row per record. The command closes its files and exits when processing finishes. A pipeline ends its input when its producer closes standard output.

## Input and results

Write `thinkthen FUNCTION QUESTION FILE`. Quote the question. Omit `FILE` to read standard input, or use `-` among file arguments to select standard input. Question arguments are literal text. `recognize` and `relate` have no question argument; `annotate` takes a toolbox filename.

Use `--from text` for one whole text per file, `--from lines` for one record per nonblank line, or `--from jsonl` for one JSON value per nonblank line. A `.jsonl`, `.csv`, `.tsv` or `.xlsx` filename selects its own format. Otherwise filter, rank and annotate default to lines; other functions default to text. `run` defaults to JSON Lines. State `--from` when a pipeline's format matters.

JSON records can carry a string or integer `id`. A JSON array on one JSON Lines line is one record. Use `--field /text` to judge only the `text` field. The selector is a JSON Pointer: `/` separates keys, `~1` means a literal slash, and `~0` means a literal tilde. Without a selector, ThinkThen uses the whole record.

```bash
printf '%s\n' '{"id":"a1","text":"Please refund my order."}' |
  thinkthen decide --from jsonl --field /text --json \
    'Does the customer request a refund?'
```

`--json` emits one typed result per line. Rows carry `function`, `position`, optional `id`, `source`, and `status`. A position starts at 1. An `answered` row has `answer`. An `unsure` row has no answer. A `failed` row has `error.code` and `error.message`. `probability` appears where the function provides one. `source` identifies the exact question execution. `detail` contains additional function data when available.

A rank row's `answer` is its rank and `position` is its input order.

Text output prints a bare answer for one text input. Streams prefix each answer with its ID or position and a tab; several text files use the file path. Filter prints original kept records, and rank prints original records in rank order, adding a final newline when needed. An unsure answer prints `unsure`. Find prints nothing for one input with no matching line, and an empty answer after the key in a stream. Failed records print no text answer; their diagnostics go to standard error. `-q` suppresses standard output. Use JSON output to retain every filter outcome and to inspect ranks, failures and probabilities. False and unsure answers do not fail an ordinary command. Empty or blank input from a single text source exits 2 before any request; a blank record in a stream fails with `record.empty`, and the other records continue.

A raw text file has no record ID. If your source gives that text an ID, keep it in a JSON envelope. The `jq` command reads the text without changing line breaks and writes the envelope. The following command judges that record:

```bash
jq -n -c --arg id 'a1' --rawfile text message.txt '{id: $id, text: $text}' > message.jsonl
thinkthen decide 'Does the customer request a refund?' message.jsonl --field /text --json
```

Read command JSON output as JSON Lines. Each nonblank line is a separate row; the file is not one JSON array. This example saves every result, counts rows and displays their IDs without another ThinkThen call:

```bash
thinkthen decide 'Does the customer request a refund?' messages.jsonl \
  --from jsonl --field /text --json > outcomes.jsonl
jq -s 'length' outcomes.jsonl
jq -c '{id, status, answer, error}' outcomes.jsonl
```

For text spanning several fields, put those fields under one object and select that object. A pointer selects one value, which can itself be an object. For example, a record can be `{"id":"a1","message":{"subject":"Invoice refund","body":"Please refund my order."}}`; `--field /message` judges both subject and body while preserving the outer ID. Use `jq -c '{id, message: {subject, body}}' source.jsonl > messages.jsonl` to build that shape from existing fields, or repeat `--field`: `--field /subject --field /body` judges an object keyed by each pointer's last name, here `subject` and `body`.

Use the same `{id, text}` shape for a single record and for `messages.jsonl`. Retain `id` in saved result rows and join results by it. A row's position describes one call's input order; ranking changes output order. Spreadsheet output keeps original ID columns beside its answer columns.

## Options and services

`--backend NAME` or `THINKTHEN_BACKEND` chooses `systemone` (the default) or `openai`. `--model NAME` or `THINKTHEN_MODEL` chooses a model. The default models are `jev-1.13.0` for System One, `gpt-6-luna` for OpenAI. See [services and keys](service.md) for obtaining a key and choosing a service.

`--format text|json` chooses answer text or typed JSON Lines; `--json` selects the latter. `--name NAME` sets an optional execution identity for a function call. `--verbose` (`-v`) prints a one-line row summary on standard error; `--quiet` (`-q`) prints no answers, rows, notices or progress, and errors still reach standard error. Run `thinkthen --help` and `thinkthen FUNCTION --help` for the complete current options. `thinkthen completions bash` writes shell completions without calling a backend; the other supported shells are listed by `thinkthen completions --help`.

## The ten functions

These commands assume a text file `message.txt` and a JSON Lines file `messages.jsonl` whose objects have `id` and `text` fields. Each command makes a separate call. The shell settings below stop this script when a command or pipeline fails. Use the JSON envelope above when these text files have source IDs.

```bash
set -euo pipefail

# Boolean answer.
thinkthen decide 'Does the customer request a refund?' message.txt --json

# Keep matching original records in text output.
thinkthen filter 'Does the customer request a refund?' messages.jsonl --field /text

# Exactly one choice, or unsure.
thinkthen choose 'What is the message about?' message.txt --json \
  --choice billing --choice shipping --choice other

# Every applicable label, possibly an empty list.
thinkthen tag 'Which topics appear?' message.txt --json \
  --label billing --label shipping

# A number from 0 to 1 on a scale ordered from low to high.
thinkthen score 'How urgent is the request?' message.txt --json \
  --level routine --level urgent

# Highest scores first; answer is the rank starting at 1.
thinkthen rank 'How urgent is the request?' messages.jsonl --field /text --json \
  --level routine --level urgent --top 5

# The single nonblank line that best answers the question, or null.
thinkthen find 'Which line states the requested action?' message.txt --json

# Named spans grouped by the requested kinds.
thinkthen recognize message.txt --json --kind person --kind organization

# Named entities and directed relationships between their kinds.
thinkthen relate message.txt --json --rule works_for person organization

# Several named questions for each record; see the toolbox below.
thinkthen annotate questions.json messages.jsonl --json
```

Choose requires distinct choices. Score and rank require distinct levels ordered from lowest to highest. A score is the probability-weighted position on that scale. Rank judges the whole input before it emits ranked answers. `--top` limits answered ranks; it does not hide failed records. Find selects a whole nonblank line, so preserve line breaks in its field. Find and recognize spans use Unicode character offsets with an inclusive `start` and exclusive `end`. Relate returns `source` and `target` spans, `relation`, `probability` and optional `either`. Add `--either` when a relation has no direction.

Decide and filter accept `--threshold 0.8` for true at or above the cut and false below it. Their `--low 0.2 --high 0.8` band yields false below the lower bound, unsure inside the band and true at or above the upper bound. Choose accepts the same forms but yields unsure when the winning choice does not reach the upper cut. Tag, find, recognize and relate accept a single numeric `--threshold`. Do not combine `--threshold` with `--low` or `--high`.

Write questions that name the field and the exact meaning you need. Replace an ambiguous phrase such as “currently available” with the allowed status values or the date rule you intend. Use the same wording in a command and its saved toolbox entry.

## Toolboxes, batching and spreadsheets

Save this as `questions.json`. `annotate` accepts this `questions` object, and each entry uses the same function fields as a direct question. Rank cannot be an annotation entry. An entry can override the shared `field` selector.

```json
{
  "field": "/text",
  "questions": {
    "refund": {"function": "decide", "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?", "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?", "levels": ["routine", "urgent"]}
  }
}
```

A reusable saved question entry uses the command's `entries` envelope. Save this as `question-entries.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

Write `thinkthen run TOOLBOX NAME FILE...` to run one named entry. This command runs the `ready` entry over the whole JSON Lines collection:

```bash
thinkthen run question-entries.json ready records.jsonl --json
```

After a toolbox folder, or a toolbox file with several entries, the next word is the entry name. After a file that holds one entry, the next word is that entry's name or the first input file. `--entry NAME` means the same as NAME after the toolbox and leaves every other word as input. `thinkthen run NAME` finds the entry in `./thinkthen.json` or `./thinkthen/`; name the toolbox when both exist. A toolbox mistake, such as a missing file, an unknown entry name or both local toolboxes present, exits 2, and its message starts with `toolbox FILE:` to name the file.

The same question-only entries load in SDK toolboxes. The annotation `questions.json` above supplies an annotation map; richer command entries may also contain shell steps and runs.

An answered annotation row maps each question name to its own outcome. Inspect those nested statuses and errors as well as the outer status.

Pass a whole record stream to one command so the engine can batch its questions. Avoid starting one process per record. `--batch` or `THINKTHEN_BATCH` controls packing; the engine also applies request size and backend limits. One row can require several backend questions. A batch is not a promise about the number of network requests. A backend error status or an unreadable reply answers a whole request, so it fails every record in that batch; a missing or out-of-range answer fails only its own record. Records that fail locally, such as `record.missing`, are never sent.

When a task gives you a call or request-body budget, plan before making calls. Count records and named questions, select the text columns each question needs, and consider how much text each question repeats. Use one annotate command for several questions on the same records or spreadsheet. Keep the complete text needed by find, recognize and relate. Save completed output for later analysis instead of calling again to format a report. Batching reduces request overhead; it does not remove the questions or text bytes the backend must receive. Task budgets belong to the task and do not change product limits.

XLSX date and time formats become ISO 8601 text. Elapsed times become ISO 8601 durations. Date-time and clock values carry no timezone. Fractional seconds round to milliseconds. Excel’s fictitious leap day becomes `1900-02-29 (Excel fictitious leap day)`. A date-time on that day includes its time before the note. Plain numbers and percentages keep their stored values. Locale-specific built-in formats 27–36 and 50–58 keep their stored values.

CSV, TSV and XLSX inputs require at least one `--column`. One column supplies its cell as the text for every function, annotate included, and replaces any field pointer a saved question or toolbox carries, so the `/text` pointer in `questions.json` gives way to `--column text`. Several columns supply an object with the column names as keys; each saved pointer must then name one of those columns, or the command exits 2 before any call. Don't pass `--field` with a single `--column`.

```bash
thinkthen decide 'Does the customer request a refund?' messages.csv \
  --column text --into is_refund
thinkthen annotate questions.json messages.xlsx --sheet Messages --column text \
  --output messages-reviewed.xlsx
```

The first command writes `messages.thinkthen.csv` and prints its path. Table input takes no `--json`. With the table on standard input, add `--from csv` or `--from tsv`, and the new table comes back on standard output. Ordinary functions require `--into` for the answer column; annotate writes one column per question name and rejects `--into`.

Each question adds one answer column, holding a value a spreadsheet reads:

| Function | Cell |
| --- | --- |
| decide, filter | `TRUE` or `FALSE`; a real boolean in xlsx |
| choose | the chosen label |
| tag | the labels joined with `; `, such as `billing; shipping` |
| score, rank | a number |
| find | the text of the found line |
| recognize | `text (kind)` for each name, joined with `; ` |
| relate | `source relation target` for each relation, joined with `; ` |

An empty list, or find with no line, leaves the cell empty. A row that is not sure or failed gets a missing value: an empty cell in CSV and TSV, and `#N/A` in xlsx. An Excel formula on `#N/A` fails instead of reading the row as FALSE. `--details` adds `_status`, `_probability`, `_error`, `_detail` and `_source` columns after each answer, such as `is_refund_status`; the error cell reads `code: message`.

```bash
thinkthen filter 'Does the customer request a refund?' messages.csv \
  --column text --into is_refund --output refunds.csv
```

Filter writes only the kept rows to CSV and TSV, as `csvgrep` does. In xlsx it keeps every row, hides the rows it did not keep, and adds an AutoFilter on the answer column, so formulas that point at rows still work. Rank attaches ranks without sorting spreadsheet rows. Output files must be new. Without `--output`, a file input produces a sibling filename with `.thinkthen` before its extension. `--in-place` replaces the source after keeping a backup. Filter with `--in-place` replaces nothing when any row fails: the source stays byte-for-byte unchanged, no backup is written, and the command exits 6 and says how many rows failed. This holds for CSV, TSV and xlsx. Table choices come only from options, never from environment variables. Use saved formula values or plain text for selected field cells.

A saved entry runs on a table the same way, and a single `--column` replaces its field pointer. Save this one-entry toolbox as `toolbox.json`:

```json
{
  "entries": {
    "review": {
      "question": {
        "function": "annotate",
        "field": "/text",
        "questions": {
          "refund": {"function": "decide", "question": "Does the customer request a refund?"}
        }
      }
    }
  }
}
```

```bash
thinkthen run toolbox.json review messages.jsonl --json
thinkthen run toolbox.json review messages.xlsx --sheet Messages \
  --column text --output messages-reviewed.xlsx
```

A run can instead hold a `run` array of function entries and `{"shell":["executable","argument"]}` steps. Each step reads the previous step's output as one JSON value per line. The last step's output reaches standard output unchanged, so a final shell step may print plain text. Shell steps require explicit permission: run `thinkthen run toolbox.json review records.jsonl --allow-shell` only after inspecting the saved commands. Shell arguments are literal; add an interpreter explicitly for shell syntax. Only single-function entries accept spreadsheet framing.

Call relate alone when you need names and relations from the same field. Its JSON row's `detail.names` contains all admitted names, including names with no returned relation, and `detail.pairs` contains scored relation pairs. The `answer` contains accepted relations. Do not run recognize first for that task.

## Failures and cancellation

Read standard output for results and standard error for diagnostics. Each library diagnostic has a stable dotted code and a message. Record failures, including nested annotation failures, yield exit 6 after the stream completes. If every row failed with a backend error, the command exits 4; this also applies to table files. A `backend.transport` failure before any row is written stops the command at once with exit 4 and nothing on standard output, even with `--json`. Failed rows remain in JSON output. A table call still publishes its file when rows fail, except filter with `--in-place`, which leaves the source unchanged. Standard error names each of the first 20 failed rows by its spreadsheet row number, where the header is row 1, as `thinkthen: CODE: row N: message`, counts any more, and then says how many rows were not sure and how many failed. Add `--details` to keep each error in the table. When the backend refuses a batch after its retries, every row in that batch fails. A call-level failure or cancellation does not publish a partial spreadsheet.

Stop a scripted workload after any nonzero call status. Capture output before inspecting or exporting it, and keep the failed output for diagnosis. This example stops before any later command can make another call:

```bash
set -euo pipefail
if thinkthen annotate questions.json messages.jsonl --json > outcomes.jsonl; then
  printf '%s\n' 'Completed; outcomes.jsonl contains the results.'
else
  status=$?
  printf 'ThinkThen failed with exit %s; inspect outcomes.jsonl and the diagnostic.\n' "$status" >&2
  exit "$status"
fi
```

Apart from that early transport failure, the command finishes its current stream before returning exit 4 or 6. This recipe stops subsequent work after that return. An unsure or false result is an outcome, so neither triggers this stop for an ordinary call. A failed spreadsheet can still leave an output file; a file's existence does not establish success. Do not retry the same failed call blindly.

| Exit | Meaning |
| --- | --- |
| 0 | Completed without failed rows |
| 1 | `--check` found no positive answer |
| 2 | Invalid arguments or setup, or blank text from a single text source |
| 3 | `--check` found unsure and no positive answer |
| 4 | Backend call failure, or no answered records and only backend failures |
| 5 | Local or internal failure |
| 6 | Failed records, except the all-backend case above |
| 130 | Cancelled |
| 141 | Downstream closed the output pipe |

`--check` supports decide, filter, choose and find. Decide, choose and find require one source read as text; with `--field`, that text is one JSON record. Filter can check a stream. Add `-q` to branch on the exit code alone, as with `grep -q`. A positive answer wins over unsure; any failed row wins over both. Capture the command status explicitly in Bash when false or unsure is expected. Use `--check` only when you intend to branch on a verdict; handle exits 1 and 3 explicitly rather than treating them as backend failures. With pipelines, `set -o pipefail` preserves a failed command's status. It also preserves 141: when `head` or another reader closes the pipe before ThinkThen finishes writing, `thinkthen ... | head -1` returns 141 under `pipefail`, and `set -e` stops the script. A shell step in a saved run can return its own nonzero status.

Press Ctrl-C to cancel input, backend work and run children. A second Ctrl-C exits immediately. Streaming output written before a failure or interruption remains available.

Each annotation child has its own status. For decide and filter, probability means the probability of yes, including when the answer is false. For choose, it means the chosen label's probability. Score and tag have no single probability; their distributions remain in detail.

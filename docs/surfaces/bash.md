# ThinkThen in Bash

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

ThinkThen judges evidence with ten functions. This guide covers the 0.2.0 command. You need Bash and the command archive for your operating system and CPU. Extract the supplied archive and put its command directory on `PATH`. With the Linux x86-64 archive, run `mkdir thinkthen-0.2.0`, `tar -xzf thinkthen-0.2.0-linux-x86_64.tar.gz -C thinkthen-0.2.0` and `export PATH="$PWD/thinkthen-0.2.0:$PATH"`.

The archive includes the engine and licenses. Check the archive's runtime requirements for your platform. Use the supplied release candidate archive when evaluating an unpublished release.

A normal call uses the default backend and reads its key from `THINKTHEN_API_KEY`. Supply the key through your environment or secret manager. Keep it out of scripts, output and reports.

Add `--backend stub` for an installation check without a key or network. Stub answers are deterministic synthetic answers; use a live backend for meaningful judgments.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```bash
printf '%s\n' '["first","second"]' |
  thinkthen decide --backend stub --from jsonl --json 'Does the customer request a refund?'
thinkthen decide --backend stub --from jsonl --evidence /text --json \
  'Does the customer request a refund?' records.jsonl
```

Every function can read one record or a record stream. `--from text` reads one whole text per file; `--from jsonl` reads one record per nonblank line. `--json` writes one result row per record. The command closes its files and exits when processing finishes. A pipeline ends its input when its producer closes standard output.

## Input and results

Write `thinkthen FUNCTION QUESTION FILE`. Quote the question. Omit `FILE` to read standard input, or use `-` among file arguments to select standard input. Question arguments are literal text. `recognize` and `relate` have no question argument; `annotate` takes a toolbox filename.

Use `--from text` for one whole text per file, `--from lines` for one record per line, or `--from jsonl` for one JSON value per nonblank line. A `.jsonl`, `.csv`, `.tsv` or `.xlsx` filename selects its own format. Otherwise filter, rank and annotate default to lines; other functions default to text. `run` defaults to JSON Lines. State `--from` when a pipeline's format matters.

JSON records can carry a string or integer `id`. A JSON array on one JSON Lines line is one record. Use `--evidence /text` to judge only the `text` field. The selector is a JSON Pointer: `/` separates keys, `~1` means a literal slash, and `~0` means a literal tilde. Without a selector, ThinkThen uses the whole record.

```bash
printf '%s\n' '{"id":"a1","text":"Please refund my order."}' |
  thinkthen decide --from jsonl --evidence /text --json \
    'Does the customer request a refund?'
```

`--json` emits one typed result per line. Rows carry `function`, `position`, optional `id`, `source`, and `status`. A position starts at 1. An `answered` row has `answer`. An `unsure` row has no answer. A `failed` row has `error.code` and `error.message`. `probability` appears where the function provides one. `source` identifies the exact question execution. `detail` contains additional function data when available.

A rank row's `answer` is its rank and `position` is its input order.

Text output prints answers. Filter prints the original kept records, and rank prints original records in rank order. Use JSON output to retain every filter outcome and to inspect ranks, failures and probabilities. False and unsure answers do not fail an ordinary command.

A raw text file has no record ID. If your source gives that text an ID, keep it in a JSON envelope. The `jq` command reads the text without changing line breaks and writes the envelope. The following command judges that record:

```bash
jq -n -c --arg id 'a1' --rawfile text message.txt '{id: $id, text: $text}' > message.jsonl
thinkthen decide 'Does the customer request a refund?' message.jsonl --evidence /text --json
```

Read command JSON output as JSON Lines. Each nonblank line is a separate row; the file is not one JSON array. This example saves every result, counts rows and displays their IDs without another ThinkThen call:

```bash
thinkthen decide 'Does the customer request a refund?' messages.jsonl \
  --from jsonl --evidence /text --json > outcomes.jsonl
jq -s 'length' outcomes.jsonl
jq -c '{id, status, answer, error}' outcomes.jsonl
```

For evidence spanning several fields, put those fields under one object and select that object. A pointer selects one value, which can itself be an object. For example, a record can be `{"id":"a1","evidence":{"subject":"Invoice refund","body":"Please refund my order."}}`; `--evidence /evidence` judges both subject and body while preserving the outer ID. Use `jq -c '{id, evidence: {subject, body}}' source.jsonl > messages.jsonl` to build that shape from existing fields.

Use the same `{id, text}` shape for a single record and for `messages.jsonl`. Retain `id` in saved result rows and join results by it. A row's position describes one call's input order; ranking changes output order. Spreadsheet output keeps original ID columns beside its outcome columns.

## The ten functions

These commands assume a text file `message.txt` and a JSON Lines file `messages.jsonl` whose objects have `id` and `text` fields. Each command makes a separate call. The shell settings below stop this script when a command or pipeline fails. Use the JSON envelope above when these text files have source IDs.

```bash
set -euo pipefail

# Boolean answer.
thinkthen decide 'Does the customer request a refund?' message.txt --json

# Keep matching original records in text output.
thinkthen filter 'Does the customer request a refund?' messages.jsonl --evidence /text

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
thinkthen rank 'How urgent is the request?' messages.jsonl --evidence /text --json \
  --level routine --level urgent --top 5

# The single nonblank line that best answers the question, or null.
thinkthen find 'Which line states the requested action?' message.txt --json

# Named spans grouped by the supplied kinds.
thinkthen recognize message.txt --json --kind person --kind organization

# Named entities and directed relationships between their kinds.
thinkthen relate message.txt --json --rule works_for person organization

# Several named questions for each record; see the toolbox below.
thinkthen annotate questions.json messages.jsonl --json
```

Choose requires distinct choices. Score and rank require distinct levels ordered from lowest to highest. A score is the probability-weighted position on that scale. Rank judges the whole input before it emits ranked answers. `--top` limits answered ranks; it does not hide failed records. Find selects a whole nonblank line, so preserve line breaks in its evidence. Find and recognize spans use Unicode character offsets with an inclusive `start` and exclusive `end`. Relate returns `source` and `target` spans, `relation`, `probability` and optional `either`. Add `--either` when a relation has no direction.

Decide and filter accept `--threshold 0.8` for true at or above the cut and false below it. Their `--low 0.2 --high 0.8` band yields false below the lower bound, unsure inside the band and true at or above the upper bound. Choose accepts the same forms but yields unsure when the winning choice does not reach the upper cut. Tag, find, recognize and relate accept a single numeric `--threshold`. Do not combine `--threshold` with `--low` or `--high`.

Write questions that name the evidence field and the exact meaning you need. Replace an ambiguous phrase such as “currently available” with the allowed status values or the date rule you intend. Use the same wording in a command and its saved toolbox entry.

## Toolboxes, batching and spreadsheets

Save this as `questions.json`. `annotate` accepts this `questions` object, and each entry uses the same function fields as a direct question. Rank cannot be an annotation entry. An entry can override the shared `evidence` selector.

```json
{
  "evidence": "/text",
  "questions": {
    "refund": {"function": "decide", "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?", "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?", "levels": ["routine", "urgent"]}
  }
}
```

A reusable saved question entry uses the command's `entries` envelope. Save this as `question-entries.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Run one named entry over the whole JSON Lines collection:

```bash
thinkthen run question-entries.json --entry ready records.jsonl --backend stub --json
```

The same question-only entries load in SDK toolboxes. The annotation `questions.json` above supplies an annotation map; richer command entries may also contain shell steps and runs.

An answered annotation row maps each question name to its own outcome. Inspect those nested statuses and errors as well as the outer status.

Pass a whole record stream to one command so the engine can batch its questions. Avoid starting one process per record. `--batch` or `THINKTHEN_BATCH` controls packing; the engine also applies request size and backend limits. One row can require several backend questions. A batch is not a promise about the number of network requests.

When a task gives you a call or request-body budget, plan before making calls. Count records and named questions, select the evidence columns each question needs, and consider how much text each question repeats. Use one annotate command for several questions on the same records or spreadsheet. Keep the complete evidence needed by find, recognize and relate. Save completed output for later analysis instead of calling again to format a report. Batching reduces request overhead; it does not remove the questions or evidence bytes the backend must receive. Task budgets belong to the task and do not change product limits.

CSV, TSV and XLSX inputs require at least one `--column`. A single column supplies text for ordinary functions. Multiple columns supply an object with the column names as keys. Annotate always receives that named object, even for one column. This spreadsheet has a `text` header, so the same toolbox works:

```bash
thinkthen annotate questions.json messages.xlsx --sheet Messages --column text \
  --output messages-reviewed.xlsx
thinkthen decide 'Does the customer request a refund?' messages.csv \
  --column text --into refund --output messages-reviewed.csv
```

Do not add `--json` to table input. The command writes an enriched table and prints its destination path. Ordinary functions require `--into` for the answer column; annotate uses its question names and rejects `--into`. Output retains original rows and appends answers with status, probability, error, detail and source columns. Filter adds judgments without removing spreadsheet rows. Rank attaches ranks without sorting spreadsheet rows. Output files must be new. Without `--output`, a file input produces a sibling filename with `.thinkthen` before its extension. `--in-place` replaces the source with a backup. Use saved formula values or plain text for selected evidence cells.

For a saved command entry, use a separate wrapper file, such as `toolbox.json`:

```json
{
  "entries": {
    "review": {
      "question": {
        "function": "annotate",
        "evidence": "/text",
        "questions": {
          "refund": {"function": "decide", "question": "Does the customer request a refund?"}
        }
      }
    }
  }
}
```

```bash
thinkthen run toolbox.json --entry review messages.jsonl --json
thinkthen run toolbox.json --entry review messages.xlsx --sheet Messages \
  --column text --output messages-reviewed.xlsx
```

A direct file can contain just the entry. `thinkthen run review` finds the entry in `./thinkthen.json` or `./thinkthen/`; select an explicit path if both exist. A run can instead hold a `run` array of function entries and `{"shell":["executable","argument"]}` steps. The next step receives the preceding step's JSON rows. Shell arguments are literal; add an interpreter explicitly for shell syntax. Only single-function entries accept spreadsheet framing.

Call relate alone when you need names and relations from the same evidence. Its JSON row's `detail.names` contains all admitted names, including names with no returned relation, and `detail.pairs` contains scored relation pairs. The `answer` contains accepted relations. Do not run recognize first for that task.

## Failures and cancellation

Read standard output for results and standard error for diagnostics. Each library diagnostic has a stable dotted code and a message. Ordinary record failures, including nested annotation failures, yield exit 6 after the stream completes. Failed rows remain in JSON output. A spreadsheet can publish completed failed outcomes; inspect its status and error columns even when output exists. A call-level failure or cancellation does not publish a partial spreadsheet.

Stop a scripted workload after any nonzero call status. Capture output before inspecting or exporting it, and keep the failed output for diagnosis. This example stops before any later command can make another call:

```bash
set -euo pipefail
if thinkthen annotate questions.json messages.jsonl --json > outcomes.jsonl; then
  printf '%s\n' 'Completed; outcomes.jsonl contains the evidence.'
else
  status=$?
  printf 'ThinkThen failed with exit %s; inspect outcomes.jsonl and the diagnostic.\n' "$status" >&2
  exit "$status"
fi
```

The command finishes its current stream before returning exit 6. This recipe stops subsequent work after that return. An unsure or false result is an outcome, so neither triggers this stop for an ordinary call. A failed spreadsheet can still leave an output file; a file's existence does not establish success. Do not retry the same failed call blindly.

| Exit | Meaning |
| --- | --- |
| 0 | Completed without failed rows |
| 1 | `--check` found no positive answer |
| 2 | Invalid arguments or setup |
| 3 | `--check` found unsure and no positive answer |
| 4 | Call-level backend failure |
| 5 | Local or internal failure |
| 6 | One or more failed records |
| 130 | Cancelled |
| 141 | Downstream closed the output pipe |

`--check` supports decide, filter, choose and find. Decide, choose and find require one text source. Filter can check a stream. A positive answer wins over unsure; any failed row wins over both. Capture the command status explicitly in Bash when false or unsure is expected. Use `--check` only when you intend to branch on a verdict; handle exits 1 and 3 explicitly rather than treating them as backend failures. With pipelines, `set -o pipefail` preserves a failed command's status. A shell step in a saved run can return its own nonzero status.

Press Ctrl-C to cancel input, backend work and run children. A second Ctrl-C exits immediately. Streaming output written before a failure or interruption remains available.

Each annotation child has its own status. For decide and filter, probability means the probability of yes, including when the answer is false. For choose, it means the chosen label's probability. Score and tag have no single probability; their distributions remain in detail.

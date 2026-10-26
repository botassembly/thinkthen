# ThinkThen in SQLite

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Extract the supplied SQLite 0.2.0 archive. Use SQLite 3.41.0 or later with extension loading enabled. Load its extension in your database connection. The first example uses Python’s standard `sqlite3` module; replace the extension path with the extracted file:

Check `README.md` at the archive root for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```python
import sqlite3

with sqlite3.connect(":memory:") as database:
    database.enable_load_extension(True)
    database.load_extension("/absolute/path/to/thinkthen_sqlite")
    database.enable_load_extension(False)
    database.execute("SELECT thinkthen_set('backend', 'stub')")
    array = database.execute("""
        SELECT status FROM thinkthen_decide
        WHERE source_sql = 'SELECT ''["first","second"]'' AS record'
          AND question = 'Does the customer request a refund?' AND format = 'json'
    """).fetchone()
    print("array", array[0])
    database.execute("CREATE TEMP TABLE input_lines(record TEXT)")
    with open("records.jsonl", encoding="utf-8") as source:
        database.executemany("INSERT INTO input_lines VALUES (?)", ((line,) for line in source if line.strip()))
    rows = database.execute("""
        SELECT position, status, answer, error_message FROM thinkthen_decide
        WHERE source_sql = 'SELECT record FROM input_lines ORDER BY rowid'
          AND question = 'Does the customer request a refund?'
          AND format = 'json' AND evidence = '/text'
    """)
    for position, status, answer, message in rows:
        print(position, status, answer, message or "")
```

The archive includes the engine. Set `THINKTHEN_API_KEY` in the SQLite process environment before using the default backend. Keep keys out of SQL and reports. Omit the stub setting for meaningful judgments. Stub answers are synthetic and need no key or network. Settings belong to the connection; thinkthen_set also accepts model, batch and retry settings such as max_retries.

All ten functions are table-valued and take a source SELECT, including a one-row SELECT. Results close with the database statement or cursor. There is no separate single-record scalar API. A JSON array in one source cell is one record when format is json; it does not become multiple rows.

## Records and the ten functions

Create input tables using normal SQL. The source query must be a read-only SELECT and expose a `record` column by default. It may be text, JSON text or NULL. Use `record_column` to select another column and `id_column` to preserve a separate string/integer ID. Order the source explicitly for stable positions. Bind source parameters with the `params` JSON array instead of interpolating values.

```sql
CREATE TABLE documents(doc_id INTEGER PRIMARY KEY, body TEXT);
INSERT INTO documents VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, NULL);
SELECT id, position, status, answer, error_code
FROM thinkthen_decide
WHERE source_sql = 'SELECT doc_id, body AS record FROM documents ORDER BY doc_id'
  AND question = 'Does the customer request a refund?'
  AND id_column = 'doc_id';
```

Use hidden-column constraints for options and lists. Required lists and toolboxes are JSON text:

| Table function | Required constraints beyond source_sql | Answer columns |
| --- | --- | --- |
| thinkthen_decide | question | answer Boolean as 0/1 |
| thinkthen_filter | question | answer and kept |
| thinkthen_choose | question, choices = '["billing","shipping","other"]' | answer text |
| thinkthen_tag | question, labels = '["billing","shipping"]' | answer JSON array |
| thinkthen_score | question, levels = '["routine","urgent"]' | answer number from 0 to 1 |
| thinkthen_rank | question, levels | rank |
| thinkthen_find | question | found_text, found_start, found_end |
| thinkthen_recognize | kinds = '["person","organization"]' | answer JSON names |
| thinkthen_relate | rules JSON array | answer JSON relations |
| thinkthen_annotate | toolbox JSON object | answer JSON named outcomes |

A relation rule has `name`, `source`, `target` and optional `reads`/`either`. Recognize and relate do not take wording:

```sql
SELECT id, status, answer
FROM thinkthen_relate
WHERE source_sql = 'SELECT doc_id, body AS record FROM documents ORDER BY doc_id'
  AND id_column = 'doc_id'
  AND rules = '[{"name":"works_for","source":"person","target":"organization"}]';
```

The annotate toolbox is a flat map of names to function-tagged question objects:

```sql
CREATE TEMP TABLE annotations AS
SELECT id, status, answer
FROM thinkthen_annotate
WHERE source_sql = 'SELECT doc_id, body AS record FROM documents ORDER BY doc_id'
  AND id_column = 'doc_id'
  AND toolbox = '{"refund":{"function":"decide","question":"Does the customer request a refund?"},"topic":{"function":"choose","question":"Which topic?","choices":["billing","shipping","other"]}}';
```

A rank row's `rank` is its rank answer; `position` is its input order. Rank reads the collection, returns highest scores first and retains failures when `top` limits successful ranks. Levels go lowest to highest. Filter returns every outcome, with kept true only for answered true; inspect failures before selecting kept rows.

## Rows, batching and files

Every row includes position, id, status, probability, error_code, error_message, error_http_status, detail and source. Status is answered, unsure or failed. SQL NULL answers occur for unsure and failed outcomes, and for an answered find with no match. Check status to distinguish them. Source and detail are JSON text. Source identifies the question, backend and served model. Details hold label/level probabilities, recognition pieces or relation pairs. Annotation children have their own status and errors; an answered outer row can contain a failed child. Rank is not an annotation question.

Use `format = 'json'` for encoded JSON records, or `evidence = '/text'` to select a field in JSON evidence. Text format treats JSON-looking strings as text. Pointers escape `/` as `~1` and `~` as `~0` inside keys. IDs remain strings or integers; preserve them when exporting. Find selects a whole nonblank line, so keep line breaks. Its offsets count Unicode code points from zero with an exclusive end.

One source SELECT supplies the collection to batch. Do not invoke the table function once per input row. Batch settings also obey byte and backend limits; a record can require several questions. Each execution of the query makes a new call, so materialize results once if several later queries need them:

```sql
CREATE TEMP TABLE outcomes AS
SELECT * FROM thinkthen_decide
WHERE source_sql = 'SELECT doc_id, body AS record FROM documents ORDER BY doc_id'
  AND id_column = 'doc_id'
  AND question = 'Does the customer request a refund?';
SELECT * FROM outcomes WHERE status = 'failed';
```

Save a reusable annotation question map with your application's file IO, or in a database table, and bind it as toolbox. There is no shell toolbox-run API. Load CSV using the SQLite shell's `.import --csv` or your application. After inspecting outcomes, use `.headers on`, `.mode csv` and `.once answers.csv` before a SELECT to write a spreadsheet-compatible CSV. XLSX needs an application library. Preserve IDs and nested failures in exports; JSON columns remain JSON text in CSV cells.

Call `thinkthen_relate` alone when you need names and relations from the same evidence. The JSON `detail` value contains `names` and scored `pairs`; names include admitted names with no returned relation. The `answer` contains accepted relations. Do not run recognize first for that task.

SQL annotation accepts its existing named question map. It does not load the saved `entries` envelope used by SDK toolboxes and command runs. Load file contents with your application, then pass the annotation map through the existing toolbox argument.

## Failures and cancellation

Bad records and backend replies return failed rows, including a one-row source. NULL records fail as record.missing without a backend question. Call-level usage/setup/cancellation problems fail the SQL statement with the library code and message. A failed row does not abort the statement automatically. Inspect failures before saving or running later steps, and inspect annotation children too. Roll back or stop the host workflow when its task requires stopping after failure.

Use SQLite's normal interruption API, sqlite3_interrupt, or interrupt the shell query. Closing the cursor finalizes its operation. SQL cannot set a backend URL or credential source; the process owner configures credentials outside SQL.

Make questions unambiguous: define the evidence criterion, distinguish absence from explicit negatives and use distinct choices. Stub answers check installation, not question quality.

Each annotation child has its own status. For decide and filter, probability means the probability of yes, including when the answer is false. For choose, it means the chosen label's probability. Score and tag have no single probability; their distributions remain in detail.

Materialize the annotation query once as `annotations`. Its `id` holds JSON text, so extract the scalar ID when joining it to the source:

```sql
SELECT d.*, json_extract(a.answer, '$.refund.answer') AS refund,
       json_extract(a.answer, '$.refund.probability') AS refund_probability,
       json_extract(a.answer, '$.topic.answer') AS topic,
       json_extract(a.answer, '$.topic.probability') AS topic_probability
FROM documents d JOIN annotations a ON json_extract(a.id, '$') = d.doc_id;
```

Use `json_extract(a.answer, '$.topic.status')` to inspect a child status. A missing single probability extracts as SQL NULL. Keep the original JSON outcomes for nested errors and distributions.

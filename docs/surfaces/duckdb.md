# ThinkThen in DuckDB

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Use exactly DuckDB 1.4.3. Extract the supplied matching DuckDB 0.2.0 archive. Start `duckdb -unsigned`, then load its extension by absolute path:

Check `README.md` at the archive root for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```sql
LOAD '/absolute/path/to/thinkthen.duckdb_extension';
SET thinkthen_backend = 'stub';
SET thinkthen_batch = 16;
SELECT 'array' AS example, status
FROM thinkthen_decide((SELECT '["first","second"]'::JSON AS record),
  'Does the customer request a refund?');
SELECT position, status, answer, error_code, error_message
FROM thinkthen_decide(
  (SELECT json AS record FROM read_json_objects('records.jsonl', format = 'newline_delimited')),
  'Does the customer request a refund?', evidence := '/text');
```

The extension includes the engine. Set `THINKTHEN_API_KEY` in the DuckDB process environment before using the default backend. Keep keys out of SQL and reports. Omit the stub setting for meaningful judgments. Stub answers are synthetic and need no key or network. Session settings also include thinkthen_model and retry settings such as thinkthen_max_retries.

All ten functions take a table and return rows, including a table with just one row. Database statement/cursor ownership closes the operation. There is no separate scalar API. A JSON array in one source cell is one record; it does not become multiple records.

## Records and the ten functions

The input table must have a VARCHAR or JSON `record` column; `record := 'column_name'` selects another one. VARCHAR stays text, including JSON-looking text. JSON input preserves structured values, or evidence can select a field in encoded JSON. NULL returns a failed missing-record row. Original input columns are returned beside result columns. Their names must not collide with result names such as id, position, answer, rank or status; use aliases such as doc_id.

```sql
CREATE TABLE documents(doc_id INTEGER, body VARCHAR);
INSERT INTO documents VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, NULL);
SELECT doc_id, position, status, answer, error_code
FROM thinkthen_decide(
  (SELECT doc_id, body AS record FROM documents ORDER BY doc_id),
  'Does the customer request a refund?');
```

| Table function | Required arguments after input table | Answer column |
| --- | --- | --- |
| thinkthen_decide | Wording | answer BOOLEAN |
| thinkthen_filter | Wording | answer BOOLEAN, kept BOOLEAN |
| thinkthen_choose | Wording, choices := ['billing','shipping','other'] | answer VARCHAR |
| thinkthen_tag | Wording, labels := ['billing','shipping'] | answer VARCHAR[] |
| thinkthen_score | Wording, levels := ['routine','urgent'] | answer DOUBLE from 0 to 1 |
| thinkthen_rank | Wording, levels | rank UBIGINT |
| thinkthen_find | Wording | found STRUCT with text/start/end |
| thinkthen_recognize | kinds := ['person','organization'] | answer typed names list |
| thinkthen_relate | rules := typed rules list | answer typed relations list |
| thinkthen_annotate | toolbox := named question map as JSON | answer typed annotation STRUCT |

Rules are STRUCT values with name, source, target, reads and either. For example `rules := [{'name':'works_for','source':'person','target':'organization','reads':NULL,'either':false}]`. Recognize and relate have no wording prompt.

```sql
CREATE TEMP TABLE annotations AS
SELECT doc_id, status, answer
FROM thinkthen_annotate(
  (SELECT doc_id, body AS record FROM documents ORDER BY doc_id),
  toolbox := '{"refund":{"function":"decide","question":"Does the customer request a refund?"},"topic":{"function":"choose","question":"Which topic?","choices":["billing","shipping","other"]}}'::JSON);
```

A rank row's `rank` is its rank answer; `position` is its input order. Rank reads the collection, returns highest scores first and keeps failures when `top := 5` limits successful ranks. Score/rank levels go lowest to highest. Filter returns every outcome; kept is true only for answered true.

## Rows, batching and files

Results expose position, id, status, probability, error_code, error_message, error_http_status, source, detail and the function's typed answer. Status and error_code are enums. Source is a typed STRUCT identifying the question, backend and served model. Details hold typed label or level probabilities, recognition pieces or relation pairs. Unsure and failed answers are NULL. Find can be answered with no match. Check status instead of treating NULL as failure. Find selects a whole nonblank line; preserve line breaks. Its offsets count Unicode code points from zero with an exclusive end.

A JSON object record can carry an exact string or integer id. Preserve the returned id or original doc_id column for joins. Use `evidence := '/text'` to select a field; pointers escape `/` as `~1` and `~` as `~0` in keys. Decide, choose and filter accept `threshold := 0.8` or `low := 0.2, high := 0.8`. Tag, find, recognize and relate accept numeric cuts only. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winner to reach the upper cut.

Pass the whole table in one call to batch it. Batch settings also obey byte and backend limits; one record may require multiple questions. Reexecuting a query sends another call. Materialize results once before inspecting, joining or exporting them. An annotation can be answered while a child is unsure or failed; inspect each answer field's status and error. Rank cannot be an annotation entry.

An outer `LIMIT` may wait for other requests or their timeout and may pay for rows it never returns. Put `LIMIT` inside the input subquery to bound input work. While a request is held, DuckDB can buffer one 2048-row input vector separately from the engine iterator's read-ahead bound.

Use read_csv/read_json or host file IO to load records and an annotation question map. The annotation toolbox is a flat named question map; no shell toolbox-run API exists. After checking all outer and nested failures, export a materialized outcome table with `COPY outcomes TO 'answers.csv' (HEADER, DELIMITER ',')`. DuckDB can also export JSON through COPY; CSV cells containing nested values retain their textual representations. XLSX needs an appropriate table library or extension outside this package.

Call `thinkthen_relate` alone when you need names and relations from the same evidence. `detail.names` contains all admitted names, including names with no returned relation, and `detail.pairs` contains scored relation pairs. The `answer` contains accepted relations. Do not run recognize first for that task.

SQL annotation accepts its existing named question map. It does not load the saved `entries` envelope used by SDK toolboxes and command runs. Load file contents with your application, then pass the annotation map through the existing toolbox argument.

## Failures and cancellation

Bad records or backend replies return failed rows, including a one-row table. Call-level errors fail the statement with the library code and message. A failed row does not automatically abort the statement. Stop subsequent work in the host when its task requires stopping, and inspect nested annotation failures too.

Use the normal DuckDB query interrupt in your client or shell. Closing the query releases its operation. SQL cannot set an arbitrary backend URL or key source; the process owner supplies credentials outside SQL.

Make questions unambiguous: define the evidence criterion, distinguish absence from explicit negatives and use distinct choices. The stub checks installation, not judgment quality.

Each annotation child has its own status. For decide and filter, probability means the probability of yes, including when the answer is false. For choose, it means the chosen label's probability. Score and tag have no single probability; their distributions remain in detail.

Materialize the annotation query once as `annotations`, then read its child STRUCT fields:

```sql
SELECT d.*, a.answer.refund.answer AS refund,
       a.answer.refund.probability AS refund_probability,
       a.answer.topic.answer AS topic,
       a.answer.topic.probability AS topic_probability
FROM documents d JOIN annotations a ON a.doc_id = d.doc_id;
```

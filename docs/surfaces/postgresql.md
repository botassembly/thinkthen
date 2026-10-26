# ThinkThen in PostgreSQL

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

The supplied PostgreSQL 0.2.0 archive requires a Linux x86-64 server running PostgreSQL 18 with glibc 2.28 or later. Run `pg_config` and the file-copy commands on that server. A macOS psql client can connect to the remote server; it cannot load this Linux extension locally. An administrator copies `lib/thinkthen_postgres.so` into `pg_config --pkglibdir` and the supplied `share/extension/*` files into the extension directory under `pg_config --sharedir`, then runs:

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```sql
CREATE SCHEMA thinkthen;
CREATE EXTENSION thinkthen SCHEMA thinkthen;
SET thinkthen.allowed_backends = 'stub:*';
SET thinkthen.backend = 'stub';
SELECT 'array', status FROM thinkthen.decide(
  $source$SELECT '["first","second"]'::jsonb AS record$source$,
  'Does the customer request a refund?');
CREATE TEMP TABLE input_lines(line_number bigint GENERATED ALWAYS AS IDENTITY, record text);
\copy input_lines(record) FROM 'records.jsonl' WITH (FORMAT csv, DELIMITER E'\x01', QUOTE E'\x02')
SELECT position, status, answer, error_code, error_message
FROM thinkthen.decide(
  'SELECT record::jsonb AS record FROM input_lines ORDER BY line_number',
  'Does the customer request a refund?', evidence => '/text');
```

The package contains the engine. The administrator grants application roles schema USAGE and EXECUTE on the functions they need. Set `THINKTHEN_API_KEY` in the server process environment for the default backend and configure the allowed backend/model policy for application roles. Keep keys out of SQL and reports. Stub answers are synthetic, need no key or network and only check installation. Session settings include thinkthen.backend, thinkthen.model, thinkthen.batch and retry settings such as thinkthen.max_retries.

All ten functions take a source SELECT and return a table, including a one-row SELECT. PostgreSQL owns and closes the query operation. There is no separate scalar API. A JSON array in one source cell is one record, not multiple records.

## Records and the ten functions

The source query must be a read-only SELECT and expose `record` by default. Text stays text; JSON/JSONB carries structured values. SQL NULL becomes a missing-record row. Use record_column for another column and id_column for a separate exact string/integer ID. Source ordering determines input positions. Source parameters use params JSONB instead of interpolated strings.

```sql
CREATE TABLE documents(doc_id bigint PRIMARY KEY, body text);
INSERT INTO documents VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, NULL);
SELECT id, position, status, answer, error_code
FROM thinkthen.decide(
  'SELECT doc_id, body AS record FROM documents ORDER BY doc_id',
  'Does the customer request a refund?', id_column => 'doc_id');
```

| Function in thinkthen schema | Required arguments after source_sql | Answer columns |
| --- | --- | --- |
| decide | question | answer boolean |
| filter | question | answer boolean and kept |
| choose | question, choices text[] | answer text |
| tag | question, labels text[] | answer text[] |
| score | question, levels text[] | answer float8 from 0 to 1 |
| rank | question, levels text[] | rank bigint |
| find | question | found_text, found_start, found_end |
| recognize | kinds text[] | answer thinkthen.name[] |
| relate | rules thinkthen.rule[] | answer thinkthen.relation[] |
| annotate | toolbox jsonb | answer jsonb named outcomes |

For example choose takes `ARRAY['billing','shipping','other']`; score/rank take levels `ARRAY['routine','urgent']`; recognize takes kinds `ARRAY['person','organization']`. Recognize and relate have no question prompt. A rule is `ROW('works_for','person','organization',NULL,false)::thinkthen.rule`, in name/source/target/reads/either order. Arrays of those rules configure relate.

```sql
CREATE TEMP TABLE annotations AS
SELECT id, status, answer
FROM thinkthen.annotate(
  'SELECT doc_id, body AS record FROM documents ORDER BY doc_id',
  '{"refund":{"function":"decide","question":"Does the customer request a refund?"},"topic":{"function":"choose","question":"Which topic?","choices":["billing","shipping","other"]}}'::jsonb,
  id_column => 'doc_id');
```

A rank row's `rank` is its rank answer; `position` is its input order. Rank reads the collection, returns highest scores first and retains failures when top limits successful ranks. Levels go lowest to highest. Filter returns every outcome, with kept true only for answered true.

## Rows, batching and files

Every function returns position, id JSONB, typed status, probability, typed error_code, error_message, error_http_status, typed source and detail JSONB. Source identifies the question, backend and served model. Details hold label or level probabilities, recognition pieces or relation pairs. Unsure and failed answers are NULL. An answered find with no match also has NULL found fields. Inspect status to distinguish them.

Preserve IDs when exporting or joining. A JSON object record can also carry an ID. Use evidence => '/text' to select a field, escaping `/` as `~1` and `~` as `~0` in keys. Find selects one whole nonblank line, so keep line breaks. Span offsets count Unicode code points from zero with an exclusive end.

Decide, choose and filter accept threshold => 0.8 or low => 0.2, high => 0.8. Tag, find, recognize and relate use cuts only. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winner to reach the upper cut.

One source SELECT supplies the collection for batching. Avoid calling the function once per source row. Batch settings also obey byte and backend limits; a record may need several backend questions. Query reexecution makes another call, so materialize once before several later steps:

```sql
CREATE TEMP TABLE outcomes AS
SELECT * FROM thinkthen.decide(
  'SELECT doc_id, body AS record FROM documents ORDER BY doc_id',
  'Does the customer request a refund?', id_column => 'doc_id');
SELECT * FROM outcomes WHERE status = 'failed';
```

A saved annotation toolbox is a flat named question map in JSONB. Store it in a table or load it through application file IO. There is no shell toolbox-run API. An answered annotation can contain failed or unsure children; inspect each child status/error in answer JSONB. Rank cannot be an annotation child.

Use application file IO or psql \copy to load CSV and save results. After checking outer and nested failures, `\copy outcomes TO 'answers.csv' WITH (FORMAT csv, HEADER true)` writes a spreadsheet-compatible file on the client. JSONB columns retain JSON text in CSV cells. XLSX needs an application library. Preserve IDs and full status/error information in exports.

Call `thinkthen.relate` alone when you need names and relations from the same evidence. The JSONB `detail` value contains `names` and scored `pairs`; names include admitted names with no returned relation. The `answer` contains accepted relations. Do not run recognize first for that task.

SQL annotation accepts its existing named question map. It does not load the saved `entries` envelope used by SDK toolboxes and command runs. Load file contents with your application, then pass the annotation map through the existing toolbox argument.

## Failures and cancellation

Bad records or backend replies return failed rows, including a one-row source. Call-level errors fail the SQL statement with the library code and message. A failed row does not abort SQL automatically. Inspect it before saving or running later steps. Stop or roll back the host workflow when its task requires stopping, and inspect nested annotation failures too.

Use PostgreSQL's normal query cancellation through the driver or psql. A cancelled query releases its operation. SQL cannot choose an arbitrary backend URL or credential source; the server process owner configures credentials outside SQL.

Make questions unambiguous: define the evidence criterion, distinguish absence from explicit negatives and use distinct choices. The stub checks installation, not judgment quality.

Each annotation child has its own status. For decide and filter, probability means the probability of yes, including when the answer is false. For choose, it means the chosen label's probability. Score and tag have no single probability; their distributions remain in detail.

Materialize the annotation query once as `annotations`. Extract child answers and probabilities from JSONB, and join its JSONB ID to the source ID:

```sql
SELECT d.*, (a.answer #>> '{refund,answer}')::boolean AS refund,
       (a.answer #>> '{refund,probability}')::double precision AS refund_probability,
       a.answer #>> '{topic,answer}' AS topic,
       (a.answer #>> '{topic,probability}')::double precision AS topic_probability
FROM documents d JOIN annotations a ON a.id = to_jsonb(d.doc_id);
```

Use `a.answer #>> '{topic,status}'` to inspect a child status. A missing single probability extracts as SQL NULL. Keep the original JSONB outcomes for nested errors and distributions.

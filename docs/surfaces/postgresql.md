# ThinkThen in PostgreSQL

The ThinkThen 0.2.0 extension adds ten set-returning functions to PostgreSQL 18, in the `thinkthen` schema. Each one runs a query you pass as text, asks one question of every row, and returns one row per record with a typed `answer` column.

To judge a file of records, save one JSON value per line as `records.jsonl`:

    {"id":"a1","text":"Please refund my order."}
    {"id":"a2","text":"Where is my parcel?"}
    null
    {"id":"a4"}
    [{"text":"first"},{"text":"second"}]
    {"id":"a6","text":null}

With the extension files installed and `THINKTHEN_API_KEY` set in the server's environment, run this in psql as an administrator, from the folder that holds `records.jsonl`:

```sql
CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;

SELECT 'array', status
FROM thinkthen.decide($source$SELECT '["first","second"]'::jsonb AS record$source$,
                      'Does the customer request a refund?');

CREATE TEMP TABLE input_lines (line_number bigint GENERATED ALWAYS AS IDENTITY, record text);
\copy input_lines(record) FROM 'records.jsonl' WITH (FORMAT csv, DELIMITER E'\x01', QUOTE E'\x02')

SELECT position, status, answer, error_code, error_message
FROM thinkthen.decide('SELECT record::jsonb AS record FROM input_lines ORDER BY line_number',
                      'Does the customer request a refund?', field => '/text');
```

The first query judges `["first","second"]` as one record, because a JSON array in one source cell stays one record. The second reads each line of the file as its own record, in file order, and keeps every row:

- Lines 1 and 2 are judged. Line 2 asks about a parcel, not a refund, so expect `f`.
- Line 3 is `null`, line 4 has no `text` field, and line 6's `text` is JSON null, so all three fail as `record.missing`, and no request is sent for them.
- Line 5 is an array with nothing at `/text`, so it fails, and its message says to judge each value as its own record.

`\copy` is a psql command, not server SQL. It reads the file from psql's working folder, not the server's.

The rest of this page uses a `messages` table. Create it and ask a first question:

```sql
CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;

CREATE TABLE messages (msg_id integer PRIMARY KEY, body text);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.'),
  (4, 'I was charged twice and my parcel is late again.'),
  (5, 'Morgan works for Example Group.
Please refund my order.'),
  (6, 'Morgan works for Example Group.');

SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'Does the customer request a refund?');
```

`is_refund` is `true` or `false` when `status` is `answered`. It is NULL when `status` is `unsure` or `failed`. The other examples on this page use the same `messages` table.

## Install the extension

1. The release archives support Linux x86-64 and arm64 servers running PostgreSQL 18 with glibc 2.28 or later. Download the PostgreSQL 18 archive for your server's architecture from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). [Verify the checksum](../../README.md#verify-a-download), then extract it.
2. Copy `lib/thinkthen_postgres.so` into the folder `pg_config --pkglibdir` prints, and `share/extension/*` into the `extension` folder under `pg_config --sharedir`. This needs write access to the server's installation folders.
3. Set `THINKTHEN_API_KEY` in the server's environment, then restart the server. For the `openai` backend, set `OPENAI_API_KEY`. SQL never sets a key or a backend address.
4. In each database that needs it, run `CREATE SCHEMA thinkthen;` and `CREATE EXTENSION thinkthen SCHEMA thinkthen;` as an administrator.

The extension contains the engine. [Grant access](#grant-access) lets other roles call it.

### Move from a 0.1 build

This path is tested only from a 0.1 test build of the extension, which installed its functions in `public`.

`ALTER EXTENSION thinkthen UPDATE` from that build succeeds, but the updated extension stays recorded in `public` and cannot be moved. A later dump of the whole database then fails to restore, with `extension "thinkthen" must be installed in schema "thinkthen"`. Instead, back up the database first, copy the 0.2.0 files over the old ones, and recreate the extension in each database:

```sql
DROP EXTENSION thinkthen;
CREATE EXTENSION thinkthen;
```

The new extension lives in its fixed `thinkthen` schema. Your own tables and their rows are untouched, and dumps of the database restore. A view or a column that uses the old extension's functions or types blocks the `DROP`. ThinkThen does not drop or recreate them for you, so drop them yourself and recreate them on the ten functions afterward. Never add `CASCADE`, which drops them without asking. Rewrite each 0.1 call as one of the ten, such as `thinkthen_decide(...)` as `thinkthen.decide(source_sql, question)`.

## Write the query

- The first argument, `source_sql`, is one read-only `SELECT` as text. Its `record` column holds the text to judge, as `text`, `json` or `jsonb`. `record_column => 'body'` reads another column.
- A source column named `id`, if there is one, comes back in the result's `id` as `jsonb`. `id_column => 'msg_id'` reads ids from another column. A column you name must exist, `id` included, or the call raises before any request.
- Name the other arguments with `=>`, as `choices => ARRAY['billing', 'shipping', 'other']`.
- Add `ORDER BY` to the source query so `position` follows your order.
- Put any `LIMIT` inside the source query. PostgreSQL runs the whole function before it applies an outer `LIMIT`, so `LIMIT 1` outside can still pay for every row.
- Bind values into the source query with `params`, a `jsonb` array for its `$1`, `$2` slots, instead of pasting them into the text. Strings bind as `text`, numbers as `numeric` and booleans as `boolean`:

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = $1',
                      'Does the customer request a refund?', params => '[1]');
```

## Handle not sure and failures

Save the answers in a table once, then read them as often as you like. Running a ThinkThen query again asks again and is billed again.

```sql
CREATE TABLE refund_answers AS
SELECT (id)::integer AS msg_id, answer AS is_refund, status, error_code, error_message
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3',
                      'Does the customer request a refund?');

SELECT msg_id FROM refund_answers WHERE is_refund IS TRUE;                      -- yes
SELECT msg_id FROM refund_answers WHERE is_refund IS FALSE;                     -- no
SELECT msg_id FROM refund_answers WHERE status = 'unsure';                      -- send to a person
SELECT msg_id, error_code, error_message FROM refund_answers WHERE status = 'failed';
```

- `status` is `answered`, `unsure` or `failed`, of type `thinkthen.status`.
- Not sure has a NULL answer. Test answers with `IS TRUE` and `IS FALSE`, which are both false for NULL. `coalesce(is_refund, false)` and `CASE WHEN is_refund THEN ... ELSE ... END` turn not sure and failed rows into no.
- A failed record is a row with `status = 'failed'` and a NULL answer, and the query keeps going. `error_code` is a `thinkthen.error_code` such as `record.missing`, `error_message` says what went wrong, and `error_http_status` holds the backend's HTTP status when there was one.
- A NULL record fails as `record.missing`, and no request is sent for it.
- When the backend refuses a batch request after its retries, every record in that batch fails.
- When every record failed at the backend, the statement raises that backend's error instead of returning a page of failed rows.

A mistake in the call raises an error before any request is sent. Its `DETAIL` holds the engine's code, such as `usage.question`, and many carry a `HINT` with the fix. [Errors](#errors) lists the SQLSTATE for each code.

## Keep only the matching rows

`thinkthen.filter` returns only the records whose answer is yes. Not-sure records are left out. Join the kept ids back to their text:

```sql
SELECT msg_id, body
FROM messages
WHERE to_jsonb(msg_id) IN (SELECT id FROM thinkthen.filter(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
  'Does the customer request a refund?'))
ORDER BY msg_id;
```

If any record fails, filter judges every record, then raises one error and returns no rows. The message names how many records failed, the first failed position and its code, as in `1 of 4 records failed; the first, at position 4, failed with record.missing: ...`. The error's SQLSTATE is the first failure's. `failures => 'row'` returns each failed record as a row with `status = 'failed'` beside the kept rows instead, so the answers you paid for are kept:

```sql
INSERT INTO messages VALUES (7, NULL);

SELECT id AS msg_id, status, error_code
FROM thinkthen.filter('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id IN (1, 2, 3, 7) ORDER BY msg_id',
                      'Does the customer request a refund?', failures => 'row');
```

Message 7 has no text, so it comes back as a failed `record.missing` row, and no request is sent for it.

To keep every record with its yes, no or not sure, use `thinkthen.decide`.

## Say what the answers mean

`yes_means` and `no_means` say what yes and no mean for a decide or filter question:

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                      'Does the customer request a refund?',
                      yes_means => 'The customer asks for money back.',
                      no_means => 'The customer asks for something else.');
```

The meanings are part of the question, so `(source).question_identity` changes when they do.

`descriptions` describes some or all options of choose and tag, or levels of score and rank. It is a `jsonb` object that maps an option or level name to its text:

```sql
SELECT id AS msg_id, answer AS topic, status
FROM thinkthen.choose('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                      'What is the message about?',
                      choices => ARRAY['billing', 'shipping', 'other'],
                      descriptions => '{"billing": "Invoices, fees and refunds.", "shipping": "Parcels and delivery."}');
```

A key that is not an option or level is refused, and so is a value that is not text or a `descriptions` that is not an object. Each raises before any request is sent, with `usage.question` in its `DETAIL`.

## The ten functions

Each query below asks the same question of the same records as the function's page on thinkthen.dev.

### decide: answer a yes-or-no question

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'Does the customer request a refund?');
```

`answer` is `boolean`. `probability` is the probability of yes, also when the answer is no.

### choose: pick one option

```sql
SELECT id AS msg_id, answer AS topic, status
FROM thinkthen.choose('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'What is the message about?',
                      choices => ARRAY['billing', 'shipping', 'other']);
```

`answer` is `text`. `probability` is the chosen option's probability, and `detail -> 'labels'` maps every option to its probability.

### tag: every label that fits

```sql
SELECT id AS msg_id, answer AS topics, status
FROM thinkthen.tag('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 4',
                   'Which topics appear?',
                   labels => ARRAY['billing', 'shipping']);
```

`answer` is `text[]`, and it can be empty. `detail -> 'labels'` maps every label to its probability.

### score: a number from 0 to 1 along your levels

```sql
SELECT id AS msg_id, answer AS urgency, status
FROM thinkthen.score('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                     'How urgent is the request?',
                     levels => ARRAY['routine', 'urgent']);
```

`answer` is a `float8`. List levels from lowest to highest. `detail -> 'levels'` holds each level's probability in the same order.

### filter: keep the records that pass

```sql
SELECT msg_id, body
FROM messages
WHERE to_jsonb(msg_id) IN (SELECT id FROM thinkthen.filter(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
  'Does the customer request a refund?'))
ORDER BY msg_id;
```

[Keep only the matching rows](#keep-only-the-matching-rows) covers failures.

### rank: order every record

```sql
SELECT answer AS urgency_rank, id AS msg_id, status
FROM thinkthen.rank('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                    'How urgent is the request?',
                    levels => ARRAY['routine', 'urgent'])
ORDER BY urgency_rank NULLS LAST;
```

`answer` is a `bigint` rank from 1, highest first, and `position` keeps the input order. `top => 2` keeps the first two ranks. Failed records come after the ranked rows with a NULL rank, and `top` never drops them. Join `id` back to `messages` to see each record's text.

### find: the line that answers

```sql
SELECT id AS msg_id, (answer).text AS requested_action, status
FROM thinkthen.find('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5',
                    'Which line states the requested action?');
```

`answer` is a `thinkthen.span` with `text`, `start_offset` and `end_offset`, or NULL with `status = 'answered'` when no line answers. Offsets count Unicode code points from zero, and the end is exclusive. Find picks a whole line, so keep the record's line breaks.

### annotate: fill in several questions at once

```sql
SELECT id AS msg_id, (answer ->> 'refund')::boolean AS is_refund,
       answer ->> 'topic' AS topic, (answer ->> 'urgency')::float8 AS urgency, status
FROM thinkthen.annotate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
  toolbox => '{
    "refund": {"function": "decide", "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }');
```

`toolbox` maps each name to one question. Rank cannot be one of them. `answer` is a `jsonb` object with each question's bare answer by name, JSON null when that question was not sure or failed. `detail -> 'refund' ->> 'status'`, `detail -> 'refund' -> 'error'` and the rest hold each question's full outcome. The row's own `status` can be `answered` while one question inside it failed, so check `detail` before you trust a NULL answer.

`toolbox` takes this map of named questions. A toolbox saved with its `entries`, as the command and the packages write it, is not read here: pass its questions as this map.

### recognize: pull out each name

```sql
SELECT r.id AS msg_id, found.text AS name, found.kind, r.status
FROM thinkthen.recognize('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 6',
                         kinds => ARRAY['person', 'organization']) AS r
LEFT JOIN LATERAL unnest(r.answer) AS found ON true;
```

`answer` is an array of `thinkthen.recognized_name`, each with `text`, `kind`, `start_offset`, `end_offset` and `probability`. `LEFT JOIN LATERAL unnest(...)` gives one row per name and keeps records with none.

### relate: link names by the relations you allow

```sql
SELECT r.id AS msg_id, (link.source).text AS person, link.relation,
       (link.target).text AS organization, r.status
FROM thinkthen.relate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5',
       rules => ARRAY[ROW('works_for', 'person', 'organization', NULL, false)::thinkthen.rule]) AS r
LEFT JOIN LATERAL unnest(r.answer) AS link ON true;
```

A rule is a `thinkthen.rule` built by position: `ROW(relation, source kind, target kind, reads, either)`. `reads` is the relation's wording, and NULL derives it from `relation`. `either` is true when the relation has no direction. `answer` is an array of `thinkthen.relation`, each with `source`, `target`, `relation`, `probability` and `either`. `source` and `target` are `thinkthen.endpoint` values with `text`, `kind`, `start_offset` and `end_offset`. `detail` holds `names`, every recognized name, and `pairs`, every allowed pair with its score. Call relate alone when you need both the names and the relations.

## Read JSON records and join on their id

A `json` or `jsonb` record can carry its own `id`. `field => '/text'` selects the field to judge.

```sql
CREATE TABLE inbox (line jsonb);
INSERT INTO inbox VALUES
  ('{"id": "a1", "text": "Please refund my order."}'),
  ('{"id": "a2", "text": "Where is my parcel?"}'),
  ('{"id": "a3", "text": "I was charged twice for one invoice."}');

CREATE TABLE refund_answers_by_ticket AS
SELECT position, id #>> '{}' AS ticket_id, answer AS is_refund, status, error_code, error_message
FROM thinkthen.decide($$SELECT line AS record FROM inbox ORDER BY line ->> 'id'$$,
                      'Does the customer request a refund?', field => '/text');

SELECT * FROM refund_answers_by_ticket ORDER BY position;
```

`id` is `jsonb` on every row, because ids may be numbers or text. `id #>> '{}'` gives the text `a1`, `(id)::integer` gives a number back, and `to_jsonb(msg_id) = id` compares without a cast. A record that is JSON `null`, or has no `text` field, fails as `record.missing`.

A JSON array in one cell is one record, not several. A `text` column is judged as text, even when it looks like JSON. In a pointer, write `/` in a key as `~1` and `~` as `~0`. Dollar quotes, as `$$...$$` above, keep quotes inside the source query readable.

## Grant access

`CREATE EXTENSION` revokes the functions from `PUBLIC`. An administrator grants each role the schema, each function it calls and the tables its queries read, and chooses the backends it may use:

```sql
CREATE ROLE app LOGIN;
GRANT USAGE ON SCHEMA thinkthen TO app;
GRANT EXECUTE ON FUNCTION thinkthen.decide TO app;
GRANT SELECT ON messages TO app;
ALTER ROLE app SET thinkthen.allowed_backends = 'systemone:*';
```

Without `thinkthen.allowed_backends`, a session may use only the default backend, `systemone`, with its default model. The setting lists `backend:model` pairs separated by commas, such as `'systemone:*,openai:*'`, where `*` allows any model. Only an administrator can set it. A malformed list is refused when it is set, with `usage.sql-policy`. A session that selects a backend or model outside the list gets `usage.sql-selection` at its next call.

## Settings

Set these with `SET` for a session, or with `ALTER ROLE` or `ALTER DATABASE`. `SHOW thinkthen.attempt_timeout` reads one, and `pg_settings` describes each.

| Setting | Default | Sets |
| --- | --- | --- |
| `thinkthen.backend` | `systemone` | `systemone` or `openai` |
| `thinkthen.model` | the backend's default | The model |
| `thinkthen.batch` | `0`, the engine's choice | Records per backend request |
| `thinkthen.max_retries` | `3` | Retries after a failed request, 0 to 3. More is refused. |
| `thinkthen.initial_backoff` | `500ms` | The first wait before a retry |
| `thinkthen.attempt_timeout` | `2min` | The time limit for one attempt |
| `thinkthen.allowed_backends` | none | Backends and models a session may use; administrators only |

```sql
SET thinkthen.attempt_timeout = '30s';
SET thinkthen.batch = 0;
```

The two time settings take PostgreSQL units, as `'2min'` or `'500ms'`, and a bare number is milliseconds. `SET thinkthen.batch = 0` hands the batch size back to the engine.

## Control cost and cancel

- One query makes one batched call for every record of its source query. Running the query again sends every question again.
- Save results with `CREATE TABLE ... AS SELECT`, then inspect, join and export the table. In `psql`, `\copy refund_answers TO 'answers.csv' WITH (FORMAT csv, HEADER true)` writes a CSV on the client, with `jsonb` columns as JSON text.
- Cancelling the query, or `statement_timeout`, stops it with SQLSTATE 57014, and the session stays usable. The query's answers are discarded, and the backend still bills requests it completed. Split large jobs into several saved queries.

## Reference

### Arguments

| Argument | Functions | Does |
| --- | --- | --- |
| `source_sql` | all | The query to judge |
| `question` | all but annotate, recognize, relate | The question |
| `choices`, `labels`, `levels`, `kinds` | choose; tag; score and rank; recognize | The options, as `text[]` |
| `rules` | relate | `thinkthen.rule[]` |
| `toolbox` | annotate | Named questions, as `jsonb` |
| `record_column => 'body'` | all | Reads that column instead of `record` |
| `id_column => 'msg_id'` | all | Reads ids from that column, which must exist. Without it, a column named `id` is read if there is one. |
| `field => '/text'` | all | Judges one field of a JSON record |
| `params => '[1]'` | all | Binds values to the source query's `$1` |
| `threshold => 0.8` | decide, filter, choose, tag, find, recognize, relate | Sets the cut an answer must reach |
| `low => 0.2, high => 0.8` | decide, filter, choose | Sets a band; between them is not sure |
| `top => 5` | rank | Keeps the first five ranks |
| `yes_means => '...'`, `no_means => '...'` | decide, filter | Says what yes and no mean for this question |
| `descriptions => '{"billing": "Invoices, fees and refunds."}'` | choose, tag, score, rank | Describes some or all options or levels, as `jsonb`. A key that is not an option or level is refused. |
| `failures => 'row'` | filter | Returns failed records as rows; `'raise'` is the default. Any other value raises `usage.option`. |
| `name => 'refund-check'` | all | Names the question in `source` |

With a band, decide answers no below `low`, yes at or above `high`, and not sure between them. Choose needs its winner to reach the upper cut.

### Result columns

Every function returns these columns in this order:

| Column | Type | Holds |
| --- | --- | --- |
| `position` | `bigint` | The record's place in the source, from 1 |
| `id` | `jsonb` | The source `id`, a JSON record's own `id`, or NULL |
| `answer` | per function | The answer, or NULL when not sure or failed |
| `probability` | `float8` | The answer's probability, when the function has one |
| `status` | `thinkthen.status` | `answered`, `unsure` or `failed` |
| `error_code` | `thinkthen.error_code` | The failure's code |
| `error_message` | `text` | What went wrong |
| `error_http_status` | `integer` | The backend's HTTP status, when there was one |
| `source` | `thinkthen.source` | `question_identity`, `name`, `backend` and `model` |
| `detail` | `jsonb` | Probabilities and pieces behind the answer |

`answer` is `boolean` for decide and filter, `text` for choose, `text[]` for tag, `float8` for score, `bigint` for rank, `thinkthen.span` for find, `jsonb` for annotate, `thinkthen.recognized_name[]` for recognize and `thinkthen.relation[]` for relate.

### Types

| Type | Fields |
| --- | --- |
| `thinkthen.span` | `text`, `start_offset`, `end_offset` |
| `thinkthen.recognized_name` | `text`, `kind`, `start_offset`, `end_offset`, `probability` |
| `thinkthen.endpoint` | `text`, `kind`, `start_offset`, `end_offset` |
| `thinkthen.relation` | `source`, `target`, `relation`, `probability`, `either` |
| `thinkthen.rule` | `relation`, `source`, `target`, `reads`, `either` |
| `thinkthen.source` | `question_identity`, `name`, `backend`, `model` |

In `psql`, `\df+ thinkthen.*` describes each function with an example, and `\dT+ thinkthen.*` each type.

### Errors

A raised error's `DETAIL` is the engine's code, and its SQLSTATE says what kind of fix it needs, so code that retries can tell a backend that is down from a mistake in the call:

| SQLSTATE | Codes | Means |
| --- | --- | --- |
| `22023` | `usage.*`, except below | Fix the call or a setting |
| `42501` | `usage.sql-selection` | Ask an administrator to allow the backend |
| `22000` | `record.*` | A record failed; filter raises this |
| `08001` | `backend.transport` | The server cannot reach the backend |
| `58000` | `backend.status`, `backend.reply`, `local.io`, `local.toolbox` | The backend or the server refused or failed |
| `57014` | `local.cancelled` | The query was cancelled |
| `XX000` | `internal.bug` | A bug in ThinkThen |

The extension's own argument checks use PostgreSQL's codes: `42703` for a source column that does not exist, `42804` for a record column that is not `text`, `json` or `jsonb`, `22004` for a required argument that is NULL, and `22023` for a source that is not one read-only `SELECT` or a `params` value that is not a JSON array of strings, numbers, booleans and nulls. A `failures` value other than `'raise'` or `'row'` raises `22023` with `usage.option` in `DETAIL`, and a `descriptions` value that is not an object of option or level names to text raises `22023` with `usage.question`.

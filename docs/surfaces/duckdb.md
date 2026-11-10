# ThinkThen in DuckDB

The ThinkThen 0.2.0 extension adds ten table functions to DuckDB 2.0.0. Each one reads a query of records, asks one question of every record, and returns one row per record with a typed `answer` column beside your own columns.

Set `THINKTHEN_API_KEY` in your shell, start `duckdb -unsigned`, and run:

```sql
LOAD '/absolute/path/to/thinkthen.duckdb_extension';

CREATE TABLE messages (msg_id INTEGER, body VARCHAR);
INSERT INTO messages VALUES
    (1, 'Please refund my order.'),
    (2, 'Where is my parcel?'),
    (3, 'I was charged twice for one invoice.'),
    (4, 'I was charged twice and my parcel is late again.'),
    (5, 'Morgan works for Example Group.
Please refund my order.'),
    (6, 'Morgan works for Example Group.');

SELECT msg_id, answer AS is_refund, status
FROM thinkthen_decide((SELECT msg_id, body FROM messages WHERE msg_id = 1),
                      'Does the customer request a refund?', record := 'body');
```

`is_refund` is `true` or `false` when `status` is `answered`. It is NULL when `status` is `unsure` or `failed`. The other examples on this page use the same `messages` table.

## Load the extension

1. Download the DuckDB archive for your platform and `SHA256SUMS` from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). [Verify the checksum](../../README.md#verify-a-download), then extract it.
2. Use DuckDB 2.0.0. DuckDB loads an extension only into the version it was built for.
3. The extension is unsigned. Start the command with `duckdb -unsigned`. In Python, open the connection with `duckdb.connect(config={'allow_unsigned_extensions': 'true'})`. Either one turns off DuckDB's signature check for that session, so load only the file you verified.
4. Run `LOAD '/absolute/path/to/thinkthen.duckdb_extension';`.

DuckDB's community extension repository does not carry ThinkThen, so `INSTALL thinkthen FROM community` finds nothing.

The extension contains the engine. It reads `THINKTHEN_API_KEY` from the DuckDB process environment, or `OPENAI_API_KEY` after `SET thinkthen_backend = 'openai'`. SQL can never set a key or a backend address.

## Handle not sure and failures

Save the answers in a table once, then read them as often as you like. A plain rerun can send questions again and incur new charges.

```sql
CREATE TABLE refund_answers AS
SELECT msg_id, answer AS is_refund, status, error_code, error_message
FROM thinkthen_decide((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                      'Does the customer request a refund?', record := 'body');

SELECT m.msg_id, m.body, a.is_refund, a.status
FROM refund_answers a JOIN messages m ON a.msg_id = m.msg_id
ORDER BY m.msg_id;

SELECT msg_id FROM refund_answers WHERE is_refund IS TRUE;                      -- yes
SELECT msg_id FROM refund_answers WHERE is_refund IS FALSE;                     -- no
SELECT msg_id FROM refund_answers WHERE status = 'unsure';                      -- send to a person
SELECT msg_id, error_code, error_message FROM refund_answers WHERE status = 'failed';
```

- `status` is a `thinkthen_status` value: `answered`, `unsure` or `failed`.
- Not sure has a NULL answer. Test answers with `IS TRUE` and `IS FALSE`, which are both false for NULL. `coalesce(is_refund, false)` and `CASE WHEN is_refund THEN ... ELSE ... END` turn not sure and failed rows into no.
- A failed record is a row with `status = 'failed'` and a NULL answer, and the query keeps going. `error_code` is a `thinkthen_error_code` value such as `record.missing`, `error_message` says what went wrong, and `error_http_status` holds the backend's HTTP status when there was one.
- A NULL record fails as `record.missing`. Empty or whitespace-only text fails as `record.empty`. Neither sends a request.
- When the backend refuses a batch request after its retries, every record in that batch fails.
- When every record fails with a backend error, the query raises that backend error instead of returning failed rows.
- A usage mistake, such as a missing `choices` or a repeated choice, raises an `Invalid Input Error` whose message starts with a dotted code such as `usage.question`. It stops the query before any request is sent.

## Keep only the matching rows

`thinkthen_filter` returns only the records whose answer is yes. Not-sure records are left out.

```sql
SELECT msg_id, body
FROM thinkthen_filter((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                      'Does the customer request a refund?', record := 'body');
```

If any record fails, filter raises one error after judging every record. The error names how many records failed and the first error code, and the statement fails. `failures := 'row'` returns each failed record as a row with `status = 'failed'` instead, so the answers you paid for are kept:

```sql
SELECT msg_id, body, status, error_code
FROM thinkthen_filter((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                      'Does the customer request a refund?', record := 'body',
                      failures := 'row');
```

To keep every record with its yes, no or not sure, use `thinkthen_decide`.

## The ten functions

Each query below asks the same question of the same records as the function's page on thinkthen.dev.

### decide: answer a yes-or-no question

```sql
SELECT msg_id, answer AS is_refund, status
FROM thinkthen_decide((SELECT msg_id, body FROM messages WHERE msg_id = 1),
                      'Does the customer request a refund?', record := 'body');
```

`answer` is `BOOLEAN`. `probability` is the probability of yes, also when the answer is no.

### choose: pick one option

```sql
SELECT msg_id, answer AS topic, status
FROM thinkthen_choose((SELECT msg_id, body FROM messages WHERE msg_id = 1),
                      'What is the message about?',
                      choices := ['billing', 'shipping', 'other'], record := 'body');
```

`answer` is `VARCHAR`. `probability` is the chosen option's probability, and `detail.labels` maps every option to its probability.

### tag: every label that fits

```sql
SELECT msg_id, answer AS topics, status
FROM thinkthen_tag((SELECT msg_id, body FROM messages WHERE msg_id = 4),
                   'Which topics appear?',
                   labels := ['billing', 'shipping'], record := 'body');
```

`answer` is `VARCHAR[]`, and it can be an empty list. `detail.labels` maps every label to its probability.

### score: a number from 0 to 1 along your levels

```sql
SELECT msg_id, answer AS urgency, status
FROM thinkthen_score((SELECT msg_id, body FROM messages WHERE msg_id = 1),
                     'How urgent is the request?',
                     levels := ['routine', 'urgent'], record := 'body');
```

`answer` is `DOUBLE`. List levels from lowest to highest. `detail.levels` holds each level's probability in the same order.

### filter: keep the records that pass

```sql
SELECT msg_id, body
FROM thinkthen_filter((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                      'Does the customer request a refund?', record := 'body');
```

[Keep only the matching rows](#keep-only-the-matching-rows) covers failures.

### rank: order every record

```sql
SELECT answer AS urgency_rank, msg_id, body
FROM thinkthen_rank((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                    'How urgent is the request?',
                    levels := ['routine', 'urgent'], record := 'body')
ORDER BY urgency_rank;
```

`answer` is a `BIGINT` rank from 1, highest first, ranked across the whole input, including every branch of a `UNION ALL`. Ties keep input order. `top := 2` keeps the first two ranks. Failed records come after the ranked rows, and `top` never drops them.

Rank returns nothing until it has judged every record. Meanwhile DuckDB keeps each input row's original columns in its own buffer-managed storage, which spills to disk under `memory_limit` and `temp_directory`, as any DuckDB query does. The extension has no rank memory setting of its own.

### find: the line that answers

```sql
SELECT msg_id, answer.text AS requested_action, status
FROM thinkthen_find((SELECT msg_id, body FROM messages WHERE msg_id = 5),
                    'Which line states the requested action?', record := 'body');
```

`answer` is `STRUCT(text VARCHAR, start_offset BIGINT, end_offset BIGINT)`, or NULL with `status = 'answered'` when no line answers. Offsets count Unicode code points from zero, and the end is exclusive. Find picks a whole line, so keep the record's line breaks.

### annotate: fill in several questions at once

```sql
SELECT msg_id, answer.refund AS is_refund, answer.topic AS topic,
       answer.urgency AS urgency, status
FROM thinkthen_annotate((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  record := 'body',
  toolbox := '{
    "refund": {"function": "decide", "question": "Does the customer request a refund?"},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }'::JSON);
```

`toolbox` maps each name to one question. Rank cannot be one of them. `answer` is a `STRUCT` with one field per name, typed for that question, and a field is NULL when that question was not sure or failed. `detail.refund.status`, `detail.refund.error` and the rest hold each question's full outcome. The row's own `status` can be `answered` while one question inside it failed, so check `detail` before you trust a NULL field.

### recognize: pull out each name

```sql
SELECT msg_id, answer AS names, status
FROM thinkthen_recognize((SELECT msg_id, body FROM messages WHERE msg_id = 6),
                         kinds := ['person', 'organization'], record := 'body');
```

`answer` is a list of `STRUCT(text, kind, start_offset, end_offset, probability)`. `detail.names` and `detail.pieces` hold every candidate with its probabilities.

### relate: link names by the relations you allow

```sql
SELECT msg_id, answer AS works_for, status
FROM thinkthen_relate((SELECT msg_id, body FROM messages WHERE msg_id = 5),
                      rules := [{'relation': 'works_for', 'source': 'person', 'target': 'organization'}],
                      record := 'body');
```

Each rule has `relation`, `source` and `target`, plus optional `reads` (the relation's wording) and `either` (true when the relation has no direction). `answer` is a list of relations, each with `relation`, `source`, `target`, `either` and `probability`. `source` and `target` are names with `text`, `kind`, `start_offset` and `end_offset`. `detail.names` lists every recognized name, and `detail.pairs` scores every allowed pair. Call relate alone when you need both the names and the relations.

To read one relation per row, unnest the saved answers:

```sql
CREATE TABLE employment AS
SELECT msg_id, answer AS works_for, status
FROM thinkthen_relate((SELECT msg_id, body FROM messages WHERE msg_id = 5),
                      rules := [{'relation': 'works_for', 'source': 'person', 'target': 'organization'}],
                      record := 'body');

SELECT msg_id, link.source.text AS person, link.target.text AS organization
FROM (SELECT msg_id, unnest(works_for) AS link FROM employment WHERE status = 'answered');
```

## Read JSON records and join on their id

A JSON object record can carry its own `id`. `field := '/text'` selects the field to judge. Put these lines in `records.jsonl`:

```json
{"id":"a1","text":"Please refund my order."}
{"id":"a2","text":"Where is my parcel?"}
{"id":"a3","text":"I was charged twice for one invoice."}
```

```sql
LOAD '/absolute/path/to/thinkthen.duckdb_extension';

CREATE TABLE refund_answers AS
SELECT position, id->>'$' AS ticket_id, answer AS is_refund, status, error_code, error_message
FROM thinkthen_decide(
  (SELECT json AS record FROM read_json_objects('records.jsonl', format = 'newline_delimited')),
  'Does the customer request a refund?', field := '/text');

SELECT * FROM refund_answers ORDER BY position;
```

`id` is `JSON`, because an id can be a string or a number. `id->>'$'` reads it as `VARCHAR`, ready to join to your own key, as in `JOIN tickets t ON t.ticket_id = refund_answers.ticket_id`. An `id` with nothing else is a blank record and fails as `record.empty`. A record without the `text` field, or with `"text": null`, fails as `record.missing`.

A `JSON` record keeps its structure, and a `VARCHAR` record is judged as text, even when it looks like JSON. A JSON array is one record, not several.

## Options

| Option | Functions | What it does |
| --- | --- | --- |
| `record := 'body'` | all | Reads another `VARCHAR` or `JSON` input column. Without it, the column named `record` is read. |
| `field := '/text'` | all | Judges one field of a JSON record, chosen by a JSON pointer. Write `/` in a key as `~1` and `~` as `~0`. |
| `threshold := 0.8` | decide, filter, choose, tag, find, recognize, relate | Sets the cut an answer must reach. |
| `low := 0.2, high := 0.8` | decide, filter | Sets a band. Decide answers no below `low`, yes at or above `high`, and not sure between them. |
| `top := 5` | rank | Keeps the first five ranks. |
| `yes_means := '...'`, `no_means := '...'` | decide, filter | Says what yes and no mean for this question. |
| `descriptions := MAP {'billing': 'Invoices, fees and refunds.'}` | choose, tag, score, rank | Describes some or all options or levels. A key that is not an option or level is refused. |
| `failures := 'row'` | filter | Returns failed records as rows instead of raising. |
| `name := 'refund-check'` | all | Names the question in `source.name`. |

```sql
SELECT msg_id, answer AS topic, status
FROM thinkthen_choose((SELECT msg_id, body FROM messages WHERE msg_id = 1),
                      'What is the message about?',
                      choices := ['billing', 'shipping', 'other'],
                      descriptions := MAP {'billing': 'Invoices, fees and refunds.'},
                      record := 'body');
```

## Result columns

Every function returns your input columns first, then these columns in this order:

| Column | Type | Holds |
| --- | --- | --- |
| `position` | `BIGINT` | The record's place in the input, from 1 |
| `id` | `JSON` | A JSON record's own `id`, or NULL |
| `answer` | per function | The answer, or NULL when not sure or failed |
| `probability` | `DOUBLE` | The answer's probability, when the function has one |
| `status` | `thinkthen_status` | `answered`, `unsure` or `failed` |
| `error_code` | `thinkthen_error_code` | The failure's code |
| `error_message` | `VARCHAR` | What went wrong |
| `error_http_status` | `BIGINT` | The backend's HTTP status, when there was one |
| `source` | `STRUCT` | The backend, model, question name and question identity |
| `detail` | per function | Probabilities and pieces behind the answer |

An input column must not share a name with a result column. Rename it in the input query, as in `SELECT id AS ticket_id, body FROM tickets`.

`thinkthen_status` and `thinkthen_error_code` are named enum types, so you can write `status = 'unsure'::thinkthen_status`.

## Settings

Settings last for the session. `SET GLOBAL` is refused.

| Setting | Default | Sets |
| --- | --- | --- |
| `thinkthen_backend` | `systemone` | The backend: `systemone` or `openai` |
| `thinkthen_model` | the backend's default | The model |
| `thinkthen_batch` | the engine's default | The most questions in one request |
| `thinkthen_max_retries` | 3 | Retries after a failed request, from 0 to 3. `SET` refuses a larger value with `usage.settings`. |
| `thinkthen_initial_backoff_ms` | 500 | The first wait before a retry |
| `thinkthen_attempt_timeout_ms` | 120000 | The time limit for one attempt |

```sql
SET thinkthen_backend = 'openai';
```

DuckDB's own `memory_limit` and `temp_directory` settings govern the memory and disk that rank uses for its input columns.

`RESET thinkthen_model` returns a setting to its default. `SELECT name, description FROM duckdb_settings() WHERE name LIKE 'thinkthen_%'` lists them inside DuckDB, and `duckdb_functions()` describes each function with an example.

## Control cost and cancel

A function invocation owns one engine call over its supplied input. A statement can invoke a function more than once, especially when its arguments depend on an outer row. A plain rerun can send questions again; reading a saved result table does not.

Choose the input before invoking ThinkThen: put `WHERE`, a deterministic `ORDER BY` on a stable unique key, and any input `LIMIT` inside the supplied source. An output `LIMIT` does not bound paid requests. A CTE is not a durable saved result, and batch size controls packing rather than total work. Retries and dependent questions can make several requests for the same input records. SQL does not admit `max_requests`, inherit `THINKTHEN_MAX_REQUESTS`, or report a statement-wide usage total.

The `refund_answers` example above saves answers in a real table before joining them to application data. For a large job, snapshot the source or keep it unchanged, select a fixed range of unique keys for each chunk, save its answers, and commit that chunk before starting the next. Commit the answer rows and a completed-range marker in the same transaction. Restart only ranges that did not commit; do not treat gaps from a failed range as completed work. A single `CREATE TABLE ... AS SELECT` is one statement, not a checkpoint after each row. Cancelling it can discard all of that statement's results even when requests were paid for. Earlier separately committed chunks remain available.

DuckDB can reuse an uncorrelated invocation's rows on the inner side of a join; repeating output rows does not by itself prove repeated requests. Save answers before joining when you need reuse across statements.

Ctrl-C, or your client's interrupt, stops the query and leaves the connection ready for the next one. Cancellation does not refund already sent requests. While a request is out, DuckDB can hold one more 2,048-row input vector in memory. `COPY refund_answers TO 'answers.csv' (HEADER)` exports the saved table.

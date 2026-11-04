# ThinkThen in SQLite

The ThinkThen 0.2.0 extension adds ten table-valued functions to SQLite. Each one runs a query you pass as text, asks one question of every row, and returns one row per record with a typed `answer` column.

Set `THINKTHEN_API_KEY` in your shell, start `sqlite3` in the extracted archive's folder, and run:

```sql
.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode box

CREATE TABLE messages(msg_id INTEGER PRIMARY KEY, body TEXT);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.'),
  (4, 'I was charged twice and my parcel is late again.'),
  (5, 'Morgan works for Example Group.
Please refund my order.'),
  (6, 'Morgan works for Example Group.');

SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'Does the customer request a refund?');
```

<!-- capture: real-backend output of the query above. -->

`is_refund` is 1 or 0 when `status` is `answered`. It is NULL when `status` is `unsure` or `failed`. The other examples on this page use the same `messages` table.

## Load the extension

1. Download the SQLite archive for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). [Verify the checksum](../../README.md#verify-a-download), then extract it.
2. Use SQLite 3.41.0 or later with extension loading on. Loading into an older SQLite fails with a message that names the version.
3. In the `sqlite3` shell, run `.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init` from the extracted folder. `SELECT thinkthen_version();` prints `0.2.0`.

In Python, load it once per connection:

```python
import sqlite3

connection = sqlite3.connect("support.db")
connection.enable_load_extension(True)
connection.load_extension("/absolute/path/to/thinkthen_sqlite")
connection.enable_load_extension(False)
```

The extension contains the engine. It reads `THINKTHEN_API_KEY` from the SQLite process environment, or `OPENAI_API_KEY` after `SELECT thinkthen_set('backend', 'openai');`. SQL can never set a key or a backend address.

## Write the query

- The first argument, `source_sql`, is a read-only query as text, with one statement. Its `record` column holds the text to judge. `record_column = 'body'` reads another column.
- A source column named `id` comes back in the result's `id` with its own SQLite type: INTEGER, TEXT, or even BLOB. `id_column = 'msg_id'` reads ids from another column.
- The arguments after `source_sql` follow in order: `question`, then the function's list, such as `choices`. Any argument can be named in `WHERE` instead, as `WHERE choices = '["billing", "shipping", "other"]'`. Each takes one value with `=`.
- Lists, rules, toolboxes and descriptions are JSON text.
- Add `ORDER BY` to the source query so `position` follows your order.
- Bind values into the source query with `params`, a JSON array for its `?` slots, instead of pasting them into the text:

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = ?',
                      'Does the customer request a refund?')
WHERE params = '[1]';
```

## Handle not sure and failures

Save the answers in a table once, then read them as often as you like. Running a ThinkThen query again asks again and is billed again.

```sql
CREATE TABLE refund_answers AS
SELECT id AS msg_id, answer AS is_refund, status, error_code, error_message
FROM thinkthen_decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3',
                      'Does the customer request a refund?');

SELECT msg_id FROM refund_answers WHERE is_refund IS TRUE;                      -- yes
SELECT msg_id FROM refund_answers WHERE is_refund IS FALSE;                     -- no
SELECT msg_id FROM refund_answers WHERE status = 'unsure';                      -- send to a person
SELECT msg_id, error_code, error_message FROM refund_answers WHERE status = 'failed';
```

- `status` is `answered`, `unsure` or `failed`.
- Not sure has a NULL answer. Test answers with `IS TRUE` and `IS FALSE`, which are both false for NULL. `coalesce(is_refund, 0)` and `CASE WHEN is_refund THEN ... ELSE ... END` turn not sure and failed rows into no.
- A failed record is a row with `status = 'failed'` and a NULL answer, and the query keeps going, unless the backend failed every record. `error_code` is a code such as `record.missing`, `error_message` says what went wrong, and `error_http_status` holds the backend's HTTP status when there was one.
- A NULL record fails as `record.missing`, and a number or blob as `record.invalid`. Neither sends a request.
- A backend that cannot be reached fails each record with `backend.transport`. When the backend refuses a batch request after its retries, every record in that batch fails with `backend.status` and the HTTP status.
- When the backend fails every record in the query, no rows come back. The query raises the first record's error instead, as in `backend.transport: the openai backend could not be reached: connection refused`. This holds for every function, and for filter even with `failures = 'row'`. Once any record is answered, or fails for another reason, the backend failures come back as rows in input order.
- A usage mistake stops the query before any request is sent. Its message starts with a code. The extension's own argument checks use `usage.argument` and name the argument, as in `usage.argument: question is required for thinkthen_decide`. The engine's checks use codes such as `usage.question`.

## Keep only the matching rows

`thinkthen_filter` returns only the records whose answer is yes. Not-sure records are left out. Join the kept ids back to their text:

```sql
SELECT msg_id, body
FROM messages
WHERE msg_id IN (SELECT id FROM thinkthen_filter(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
  'Does the customer request a refund?'))
ORDER BY msg_id;
```

<!-- capture: real-backend output of the query above. -->

If any record fails, filter returns the kept rows, then raises one error. The error names how many records failed, the first failed position and its code, and the statement fails, so `CREATE TABLE ... AS` saves nothing. `failures = 'row'` returns each failed record as a row with `status = 'failed'` instead, so the answers you paid for are kept, unless the backend failed every record:

```sql
SELECT id AS msg_id, status, error_code
FROM thinkthen_filter('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                      'Does the customer request a refund?')
WHERE failures = 'row';
```

To keep every record with its yes, no or not sure, use `thinkthen_decide`.

## The ten functions

Each query below asks the same question of the same records as the function's page on thinkthen.dev.

### decide: answer a yes-or-no question

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'Does the customer request a refund?');
```

`answer` is 1 or 0. `probability` is the probability of yes, also when the answer is no. `yes_means` and `no_means` say what yes and no mean for this question:

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'Does the customer request a refund?')
WHERE yes_means = 'The customer asks for money back.'
  AND no_means = 'Anything else.';
```

### choose: pick one option

```sql
SELECT id AS msg_id, answer AS topic, status
FROM thinkthen_choose('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'What is the message about?')
WHERE choices = '["billing", "shipping", "other"]';
```

`answer` is text. `probability` is the chosen option's probability, and `detail ->> '$.labels'` maps every option to its probability.

### tag: every label that fits

```sql
SELECT id AS msg_id, answer AS topics, status
FROM thinkthen_tag('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 4',
                   'Which topics appear?')
WHERE labels = '["billing", "shipping"]';
```

`answer` is a JSON array, and it can be empty. `detail ->> '$.labels'` maps every label to its probability.

### score: a number from 0 to 1 along your levels

```sql
SELECT id AS msg_id, answer AS urgency, status
FROM thinkthen_score('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                     'How urgent is the request?')
WHERE levels = '["routine", "urgent"]';
```

`answer` is a REAL. List levels from lowest to highest. `detail ->> '$.levels'` holds each level's probability in the same order.

### filter: keep the records that pass

```sql
SELECT msg_id, body
FROM messages
WHERE msg_id IN (SELECT id FROM thinkthen_filter(
  'SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
  'Does the customer request a refund?'))
ORDER BY msg_id;
```

[Keep only the matching rows](#keep-only-the-matching-rows) covers failures.

### rank: order every record

```sql
SELECT answer AS urgency_rank, id AS msg_id, status
FROM thinkthen_rank('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                    'How urgent is the request?')
WHERE levels = '["routine", "urgent"]'
ORDER BY urgency_rank NULLS LAST;
```

`answer` is an INTEGER rank from 1, highest first, and `position` keeps the input order. `top = 2` keeps the first two ranks. Failed records come after the ranked rows with a NULL rank, and `top` never drops them.

### find: the line that answers

```sql
SELECT id AS msg_id, answer ->> '$.text' AS requested_action, status
FROM thinkthen_find('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5',
                    'Which line states the requested action?');
```

`answer` is a JSON object with `text`, `start_offset` and `end_offset`, or NULL with `status = 'answered'` when no line answers. Offsets count Unicode code points from zero, and the end is exclusive. Find picks a whole line, so keep the record's line breaks.

### annotate: fill in several questions at once

```sql
SELECT id AS msg_id, answer ->> '$.refund' AS is_refund, answer ->> '$.topic' AS topic,
       answer ->> '$.urgency' AS urgency, status
FROM thinkthen_annotate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id')
WHERE toolbox = '{
  "refund": {"function": "decide", "question": "Does the customer request a refund?"},
  "topic": {"function": "choose", "question": "What is the message about?",
            "choices": ["billing", "shipping", "other"]},
  "urgency": {"function": "score", "question": "How urgent is the request?",
              "levels": ["routine", "urgent"]}
}';
```

`toolbox` maps each name to one question. Rank cannot be one of them. `answer` is a JSON object with each question's bare answer by name, null when that question was not sure or failed. `detail ->> '$.refund.status'`, `detail ->> '$.refund.error'` and the rest hold each question's full outcome. The row's own `status` can be `answered` while one question inside it failed, so check `detail` before you trust a NULL answer.

`toolbox` takes this map of named questions. A toolbox saved with its `entries`, as the command and the packages write it, is not read here: pass its questions as this map.

### recognize: pull out each name

```sql
SELECT id AS msg_id, answer AS names, status
FROM thinkthen_recognize('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 6')
WHERE kinds = '["person", "organization"]';
```

`answer` is a JSON array of names, each with `text`, `kind`, `start_offset`, `end_offset` and `probability`. `json_each(answer)` gives one row per name.

### relate: link names by the relations you allow

```sql
SELECT id AS msg_id, answer AS works_for, status
FROM thinkthen_relate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5')
WHERE rules = '[{"relation": "works_for", "source": "person", "target": "organization"}]';
```

Each rule has `relation`, `source` and `target`, plus optional `reads` (the relation's wording) and `either` (true when the relation has no direction). `answer` is a JSON array of relations, each with `relation`, `source`, `target`, `either` and `probability`. `source` and `target` are names with `text`, `kind`, `start_offset` and `end_offset`. `detail` holds `names`, every recognized name, and `pairs`, every allowed pair with its score. Call relate alone when you need both the names and the relations.

To read one relation per row, save the answers and expand them with `json_each`:

```sql
CREATE TABLE employment AS
SELECT id AS msg_id, answer AS works_for, status
FROM thinkthen_relate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 5')
WHERE rules = '[{"relation": "works_for", "source": "person", "target": "organization"}]';

SELECT msg_id, link.value ->> '$.source.text' AS person,
       link.value ->> '$.target.text' AS organization
FROM employment, json_each(employment.works_for) AS link
WHERE status = 'answered';
```

## Read JSON records and join on their id

A JSON object record can carry its own `id`. `field = '/text'` selects the field to judge and reads each record as JSON. `format = 'json'` reads records as JSON without selecting a field.

```sql
CREATE TABLE inbox(line TEXT);
INSERT INTO inbox VALUES
  ('{"id":"a1","text":"Please refund my order."}'),
  ('{"id":"a2","text":"Where is my parcel?"}'),
  ('{"id":"a3","text":"I was charged twice for one invoice."}');

CREATE TABLE refund_answers_by_ticket AS
SELECT position, id AS ticket_id, answer AS is_refund, status, error_code, error_message
FROM thinkthen_decide('SELECT line AS record FROM inbox ORDER BY rowid',
                      'Does the customer request a refund?')
WHERE field = '/text';

SELECT * FROM refund_answers_by_ticket ORDER BY position;
```

<!-- capture: real-backend output of the final SELECT above. -->

`id` keeps the JSON id's type: `"a1"` comes back as TEXT `a1`, and a number as INTEGER, so it joins to your own key as it is. An integer too large for SQLite's 64-bit integers comes back as TEXT digits. A record that is JSON `null`, or has no `text` field, fails as `record.missing`.

A JSON array in one cell is one record, not several. Text read without `format = 'json'` or `field` is judged as text, even when it looks like JSON. In a pointer, write `/` in a key as `~1` and `~` as `~0`.

## JSON columns

`answer` for tag, find, annotate, recognize and relate, and `detail` and `source` for every function, carry SQLite's JSON subtype. `json_object('topics', answer)` nests a real array, and `->>` reads a field. In a CSV export they are JSON text.

## Options

| Option | Functions | What it does |
| --- | --- | --- |
| `record_column = 'body'` | all | Reads another source column. Without it, the column named `record` is read. |
| `id_column = 'msg_id'` | all | Reads ids from another source column. Without it, a column named `id` is read if there is one. |
| `format = 'json'` | all | Reads each record as JSON. `'text'`, the default, judges it as text. |
| `field = '/text'` | all | Judges one field of a JSON record, chosen by a JSON pointer. |
| `params = '[1]'` | all | Binds values to the source query's `?` slots. |
| `threshold = 0.8` | decide, filter, choose, tag, find, recognize, relate | Sets the cut an answer must reach. |
| `low = 0.2 AND high = 0.8` | decide, filter, choose | Sets a band. Decide answers no below `low`, yes at or above `high`, and not sure between them. |
| `top = 5` | rank | Keeps the first five ranks. |
| `yes_means = '...'`, `no_means = '...'` | decide | Says what yes and no mean for this question. |
| `descriptions = '{"billing": "Invoices, fees and refunds."}'` | choose, tag, score, rank | Describes some or all options or levels. A key that is not an option or level is refused. |
| `failures = 'row'` | filter | Returns failed records as rows instead of raising, unless the backend failed every record. `'raise'` is the default. |
| `name = 'refund-check'` | all | Names the question in `source`. |

```sql
SELECT id AS msg_id, answer AS topic, status
FROM thinkthen_choose('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = 1',
                      'What is the message about?')
WHERE choices = '["billing", "shipping", "other"]'
  AND descriptions = '{"billing": "Invoices, fees and refunds."}';
```

## Result columns

Every function returns these columns in this order:

| Column | Type | Holds |
| --- | --- | --- |
| `position` | INTEGER | The record's place in the source, from 1 |
| `id` | the id's own type | The source `id`, or a JSON record's own `id`, or NULL |
| `answer` | per function | The answer, or NULL when not sure or failed |
| `probability` | REAL | The answer's probability, when the function has one |
| `status` | TEXT | `answered`, `unsure` or `failed` |
| `error_code` | TEXT | The failure's code |
| `error_message` | TEXT | What went wrong |
| `error_http_status` | INTEGER | The backend's HTTP status, when there was one |
| `source` | JSON | The backend, model, question name and question identity |
| `detail` | JSON | Probabilities and pieces behind the answer |

The arguments are hidden columns, so `SELECT *` shows only these ten.

## Settings

`thinkthen_set(name, value)` changes a setting for the connection and returns 1.

| Setting | Default | Sets |
| --- | --- | --- |
| `backend` | `systemone` | The backend: `systemone` or `openai` |
| `model` | the backend's default | The model |
| `batch` | the engine's default | The most questions in one request |
| `max_retries` | 3 | Retries after a failed request, 0 to 3. More is refused. |
| `initial_backoff_ms` | 500 | The first wait before a retry |
| `attempt_timeout_ms` | 120000 | The time limit for one attempt |

```sql
SELECT thinkthen_set('backend', 'openai');
```

`backend` and `model` take text, and the others take integers. A key or a backend address cannot be set.

## Control cost and cancel

- One ThinkThen call batches every record of its source query. Running the query again sends every question again.
- Save results with `CREATE TABLE ... AS SELECT`, then inspect, join and export the table. In the shell, `.headers on`, `.mode csv` and `.once answers.csv` before a `SELECT` write a CSV.
- Inside one statement, a ThinkThen function that SQLite runs again with the same arguments, such as on the inner side of a join, gives its saved rows and asks nothing new. A correlated call, whose arguments come from the outer row, asks again for each new set of arguments. Save the answers first, then join the table.
- The functions cannot run inside a view or a trigger.
- `sqlite3_interrupt`, or Ctrl-C in the shell, stops the query and leaves the connection ready for the next one. The query's answers are discarded, and the backend still bills requests it completed. Split large jobs into several saved queries.
- Closing the statement or cursor ends its call.

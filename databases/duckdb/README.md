# DuckDB functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

In DuckDB, each ThinkThen function is a table function. You give it a query of records and one question, and it gives back one row per record, with the answer in a column beside your own columns. This page is complete for DuckDB. The same ten functions exist in SQLite and PostgreSQL, and the [shared SQL guide](https://thinkthen.dev/sdks/sql/) shows the three spellings side by side. Set `THINKTHEN_API_KEY` in your shell before you start DuckDB.

Every row is in exactly one of three states, named by its `status` column:

- **`answered`**: there is an answer, and the `answer` column holds it.
- **`unsure`**: the model's number fell between the two bars you set, or no choice was likely enough.
- **`failed`**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Where there is no answer, `answer` is NULL, and NULL is never a yes or a no.** `WHERE answer` keeps only the yeses. `WHERE NOT answer` keeps only the noes. A missing answer passes neither test.
2. **Checking the state never fails.** `status`, `probability`, `error_code` and `error_message` are on every row, beside the answer, in every state.
3. **A failed record is a row, not a failed statement.** One bad record never costs you the others. A statement fails for a record only where you ask it to: in `thinkthen_filter`, and in any call with `raise_if_failed`.

Rule 1 has one trap, and it is SQL's own: `ELSE`, `coalesce` and `NOT IN` all treat NULL as "everything else". `CASE WHEN answer THEN 'refund queue' ELSE 'inbox' END` sends a not-sure message and a failed message to the inbox. Test `status` first, or name the no with `WHEN NOT answer`.

## Load the extension

1. Download the DuckDB archive for your platform and `SHA256SUMS` from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). [Verify the checksum](../../README.md#verify-a-download), then extract it.
2. Use DuckDB 2.0.0. DuckDB loads an extension only into the version it was built for.
3. Start the shell with `duckdb -unsigned`. The extension is unsigned, and this turns off DuckDB's signature check for the session, so load only the file you verified. In Python, open the connection with `duckdb.connect(config={'allow_unsigned_extensions': 'true'})`.
4. Run `LOAD` with the full path to the file. Some DuckDB builds refuse a relative path, the 2.0.0 command for macOS among them.

DuckDB's community extension repository does not carry ThinkThen, so `INSTALL thinkthen FROM community` finds nothing.

```sql
LOAD '/absolute/path/to/thinkthen.duckdb_extension';
```

## Writing the call

The examples on this page share one `messages` table:

```sql
CREATE TABLE messages (msg_id INTEGER PRIMARY KEY, body TEXT);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.'),
  (4, 'I was charged twice and my parcel is late again.'),
  (5, 'Morgan works for Example Group.
Please refund my order.');
```

Here is one call, with everything a call can hold:

```sql
SELECT msg_id, body, answer AS is_refund, status
FROM thinkthen_decide((SELECT msg_id, body FROM messages),
                      'Does the customer request a refund?',
                      record := 'body', low := 0.2, high := 0.8);
```

The records:

- The records are a query in parentheses, in the first argument.
- `record := 'body'` names the column to judge. Without it, the column named `record` is judged.
- Every column of your query comes back beside the answer, under its own name.

The options:

- The question is text, in the second argument. recognize, relate and annotate take none.
- An option goes inside the call, by name, as `low := 0.2`.
- A list is written in brackets: `['billing', 'shipping', 'other']`.

The results:

- A yes is `true` and a no is `false`.
- An answer with parts is a value with fields, read as `answer.text`. A list is a list, read with `unnest`.
- `status` and `error_code` are enumerated types. They compare with plain text, as in `status = 'unsure'`.

## decide

### The simplest if

SQL's `if` is `WHERE`. A decide answer goes straight into it.

```sql
SELECT 'refund queue' AS where_it_goes
FROM thinkthen_decide((SELECT 'Please refund my order.' AS record),
                      'Does the customer request a refund?')
WHERE answer;
```

The row comes back only on a yes. If the message could not be judged, `answer` is NULL, NULL is never true, and no row comes back. It never reads as a yes.

Don't turn this into a `CASE` with a plain `ELSE`. `ELSE` runs for every row that is not a yes: no, not sure and failed alike. To act on a no, name it:

```sql
SELECT CASE
         WHEN answer THEN 'refund queue'
         WHEN NOT answer THEN 'inbox'
       END AS where_it_goes
FROM thinkthen_decide((SELECT 'Please refund my order.' AS record),
                      'Does the customer request a refund?');
```

Where there is no answer, this gives NULL. It never gives `inbox`.

### Yes, no, or something else

Set two bars and decide can also be not sure. `status` tells you whether there is a yes or no to read. This call saves its one result in a small table named `is_refund`, so the next few examples can read it without asking again.

```sql
CREATE TEMP TABLE is_refund AS
SELECT answer, probability, status, error_code, error_message
FROM thinkthen_decide((SELECT 'Where is my parcel?' AS record),
                      'Does the customer request a refund?',
                      low := 0.2, high := 0.8);
```

```sql
SELECT CASE
         WHEN status = 'answered'
           THEN CASE WHEN answer THEN 'refund queue' ELSE 'inbox' END
         ELSE 'a person reads it'
       END AS where_it_goes
FROM is_refund;
```

The inner `ELSE` is safe, because the row is already known to be answered. `status <> 'answered'` is the one test that covers both not sure and failed.

### Not sure or failed

```sql
SELECT CASE
         WHEN status = 'unsure' THEN 'a person reads it'
         WHEN status = 'failed' THEN 'try again later'
         WHEN answer THEN 'refund queue'
         ELSE 'inbox'
       END AS where_it_goes,
       probability,
       error_code
FROM is_refund;
```

Yes and no are the two answers. Not sure and failed are the two ways of having none. To name every state, switch on `status`:

```sql
SELECT CASE status
         WHEN 'answered' THEN CASE WHEN answer THEN 'refund queue' ELSE 'inbox' END
         WHEN 'unsure' THEN 'a person reads it'
         WHEN 'failed' THEN 'try again later'
       END AS where_it_goes
FROM is_refund;
```

DuckDB can't check that a `CASE` names every state. Leave the outer `ELSE` off, as here, so a state you forgot shows up as NULL and not as the wrong queue.

### Did it work

```sql
SELECT 'could not judge it: failed: ' || error_code || ': ' || error_message AS note
FROM is_refund
WHERE status = 'failed';
```

For a failed result, this query combines `could not judge it: failed:` with its actual code and reason, using the same failure format as the `thinkthen` command. Not sure counts as working: ThinkThen did its job, and the answer was "not sure".

`error_code` and `error_message` are NULL on every row that did not fail. `error_http_status` holds the service's HTTP status when it gave one.

### Raise if it failed

```sql
SELECT answer AS is_refund
FROM thinkthen_decide((SELECT 'Please refund my order.' AS record),
                      'Does the customer request a refund?',
                      raise_if_failed := true);
```

With `raise_if_failed`, the statement fails with one error when any record failed. For annotate, a failed form or any failed entry counts as a failed record; a not-sure entry does not. Count each affected record once, even when several entries failed. For a failed outer form, report its error; otherwise report the first failed entry in question-name order. If no record failed, the rows come back as usual, so the option fits on any call. Every function but filter takes it. filter always raises, as its section says.

For a fully consumed call, the error comes after every record is judged. It says how many records failed, the position of the first, and its error code. The statement fails as a whole, so a `CREATE TABLE ... AS` or an `INSERT ... SELECT` around it saves nothing. An error can't carry rows, so the answers that did come are gone with it. Leave the option off whenever you save answers: the failed rows then sit in your table with the reason beside them.

### Both bars and the probability

```sql
SELECT probability FROM is_refund;
```

- No bar option: one bar at 0.5. The answer is always yes or no.
- `threshold := 0.7`: one bar. Yes at 0.7 or above, otherwise no.
- `low := 0.2, high := 0.8`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. It is NULL only when the row failed.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `thinkthen_decide`, save each `probability` in a table, and compare with your labels.

## choose with your own choices

Give your choices as a list. The answer comes back as one of them, as text, spelled exactly as you wrote it.

```sql
CREATE TEMP TABLE topic AS
SELECT answer, probability, status
FROM thinkthen_choose((SELECT 'I was charged twice for one invoice.' AS record),
                      'What is the message about?',
                      choices := ['billing', 'shipping', 'other']);
```

```sql
SELECT CASE
         WHEN status = 'unsure' THEN 'a person reads it'
         WHEN answer = 'billing' THEN 'billing team'
         WHEN answer = 'shipping' THEN 'warehouse'
         WHEN answer = 'other' THEN 'front desk'
       END AS team_for
FROM topic;
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So test `status` before the answer. `threshold := 0.3` moves the bar. choose takes one bar only.

The choices (`billing`, `shipping`, `other`) are the words the model reads, so make them plain words or short phrases, not codes.

DuckDB has no list of choices it can check a `CASE` against. Add a choice and nothing points at this `CASE`. Name every choice with its own `WHEN` and leave `ELSE` off, so a new choice shows up as NULL and a not-sure row is never sent to the front desk.

`probability` is the best choice's. `detail.labels` holds every choice with its probability.

## Many records

Every function takes many records the same way: the query you give it can return any number of rows. You get one row back per record, in the order of your query, and each row carries your own columns beside the answer.

```sql
CREATE TABLE refund_checks AS
SELECT msg_id, body, answer AS is_refund, probability, status, error_code, error_message
FROM thinkthen_decide((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                      'Does the customer request a refund?',
                      record := 'body', low := 0.2, high := 0.8);
```

```sql
SELECT msg_id, body,
       CASE
         WHEN status = 'unsure' THEN 'a person reads it'
         WHEN status = 'failed' THEN 'try again later'
         WHEN is_refund THEN 'refund queue'
         ELSE 'inbox'
       END AS where_it_goes
FROM refund_checks
ORDER BY msg_id;
```

One failed record does not stop the statement. Each row keeps its own state, with `position` counted from 1 and the record's `id` when it has one.

A column of your query can't share a name with a result column, such as `id`, `answer` or `status`. The call is refused with a `usage.sql-selection` error. Rename the column in your query, as in `SELECT id AS ticket_id, body FROM tickets`.

Give the whole set of records to one call, so ThinkThen can send them in batches. Don't call a function once per row of another table: each call is a separate piece of work.

To save every result, put the call in `CREATE TABLE ... AS`, as above. The table keeps the state of every row, so nothing is lost. Then read the table as often as you like. Running the call again asks the service again, and costs again.

There is no separate call for one record. One record is a query that returns one row, as in the decide examples above.

## filter

filter means "give me the yeses". It hands back your own rows, in order.

```sql
SELECT refund_request.msg_id, refund_request.body
FROM thinkthen_filter((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                      'Does the customer request a refund?',
                      record := 'body') AS refund_request;
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `threshold := 0.7` moves the bar. `low` and `high` are refused with a `usage.threshold` error that names `thinkthen_decide`: when you want not sure, use `thinkthen_decide`, as in the section above.

Your own rows can't carry a failure. So if any record fails, a fully consumed filter call raises one error after every record is judged, and the statement fails as a whole. It works as `raise_if_failed` does: nothing is saved, and the error says how many records failed, the position of the first, and its code.

To see each failed record, or to keep the answers you paid for whatever happens, ask with `thinkthen_decide`, save the rows, and pick the yeses yourself:

```sql
SELECT msg_id, body FROM refund_checks WHERE is_refund;

SELECT msg_id, body, error_code FROM refund_checks WHERE status = 'failed';
```

The first query is the refund queue. The second lists what to try again later.

## tag

Labels work as choices do. The answer is the list of labels that fit, in the order you gave them, and it can be empty.

```sql
CREATE TEMP TABLE topics AS
SELECT position, id, answer, status
FROM thinkthen_tag((SELECT body FROM messages WHERE msg_id = 4),
                   'Which topics appear?',
                   labels := ['billing', 'shipping'], record := 'body');
```

```sql
SELECT 'billing team' AS team_for FROM topics WHERE 'billing' = ANY(answer);

SELECT topics.position, topics.id, topics.status, label.topic
FROM topics
LEFT JOIN LATERAL (SELECT unnest(topics.answer) AS topic) AS label ON true;
```

The left join keeps an empty or failed result as one row with a NULL topic. `position`, `id` and `status` still identify it and distinguish an empty answer from a failure.

tag is never not sure. `threshold := 0.7` sets how likely a label must be to count. An empty list is an answer: no label fit. A NULL `answer` means the row failed. `probability` is always NULL for tag, and each label's own number is in `detail.labels`.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it.

```sql
CREATE TEMP TABLE urgency AS
SELECT answer, status
FROM thinkthen_score((SELECT 'Please refund my order.' AS record),
                     'How urgent is the request?',
                     levels := ['routine', 'urgent']);
```

```sql
SELECT answer AS urgency,
       CASE WHEN answer > 0.5 THEN 'answer today' END AS note
FROM urgency;
```

The score is a position between your levels, not one of them, so it is a number and there is nothing to put in a `CASE` by name. To get exactly one level back, ask with `thinkthen_choose`. score is never not sure, and its `probability` is always NULL.

Don't put a score or a rank straight into `WHERE`. DuckDB accepts a number there and reads every number but 0 as a yes. Compare it with a number, as in `answer > 0.5`.

## rank

rank puts many records in order along your levels, highest first.

```sql
SELECT ranked.answer, ranked.body
FROM thinkthen_rank((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                    'How urgent is the request?',
                    levels := ['routine', 'urgent'], record := 'body') AS ranked
ORDER BY ranked.answer NULLS LAST;
```

`answer` is the rank, starting at 1, and the other columns are your record. `position` still says where the record was in your query. rank reads every record before it gives anything back. `top := 5` selects the five highest-ranked messages after judging the collection. A failed record comes back with a NULL rank and `status` set to `failed`. rank is never not sure.

While rank reads, DuckDB keeps your columns in its own storage, which moves to disk under `memory_limit` and `temp_directory` as any large DuckDB query does.

## find

find points to the one line that answers.

```sql
CREATE TEMP TABLE action AS
SELECT body, answer, status
FROM thinkthen_find((SELECT body FROM messages WHERE msg_id = 5),
                    'Which line states the requested action?',
                    record := 'body');
```

```sql
SELECT CASE
         WHEN status = 'unsure' THEN 'a person reads it'
         WHEN status = 'answered' AND answer IS NULL THEN 'no line answers'
         WHEN status = 'answered' THEN answer.text
       END AS line
FROM action;
```

The answer has three parts, `text`, `start_offset` and `end_offset`, or it is NULL when no line answers. find is the one function where a NULL answer can be an answer, so `status` is what tells "no line answers" from not sure and failed.

The offsets count characters from 0, and `end_offset` is one past the last character. SQL counts from 1, so `substr(body, answer.start_offset + 1, answer.end_offset - answer.start_offset)` is the line. Like choose, find can be not sure with no bar set.

## recognize

```sql
SELECT names.position, names.id, names.status, name.kind, name.text
FROM thinkthen_recognize((SELECT 'Morgan works for Example Group.' AS record),
                         kinds := ['person', 'organization']) AS names
LEFT JOIN LATERAL (SELECT unnest(names.answer) AS name) ON true;
```

`answer` is a list of names. Each name has `kind`, `text`, `start_offset`, `end_offset` and `probability`. The list can be empty. recognize is never not sure.

The left join gives one row per name. A record with no names, or one that failed, stays as one row whose name columns are NULL, so it is never dropped without a word. A plain `unnest` in the `SELECT` list, or a cross join, would drop that record.

## relate

```sql
SELECT relations.position, relations.id, relations.status,
       relation.source.text AS source, relation.relation, relation.target.text AS target
FROM thinkthen_relate((SELECT body FROM messages WHERE msg_id = 5),
                      rules := [{'source': 'person', 'target': 'organization', 'relation': 'works_for'}],
                      record := 'body') AS relations
LEFT JOIN LATERAL (SELECT unnest(relations.answer) AS relation) ON true;
```

A rule names the kind it starts from, the kind it points to, and the relation, in that order. It may also give `reads`, the wording the model sees, and `either`, for a relation with no direction. Each relation in `answer` has `source`, `target`, `relation`, `probability` and `either`. `detail.names` holds every name found, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record. The questions are written as JSON, each under a name of your choosing.

```sql
CREATE TABLE forms AS
SELECT msg_id, body, status, error_code, error_message, detail,
       answer.refund AS is_refund,
       answer.topic AS topic,
       answer.urgency AS urgency,
       detail.refund.status AS refund_status,
       detail.topic.status AS topic_status,
       detail.urgency.status AS urgency_status
FROM thinkthen_annotate((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  record := 'body',
  questions := '{
    "refund": {"function": "decide", "question": "Does the customer request a refund?",
               "threshold": {"low": 0.2, "high": 0.8}},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }'::JSON);
```

```sql
SELECT msg_id, body, status, refund_status, topic_status, urgency_status,
       CASE WHEN topic_status = 'answered' THEN topic END AS topic,
       CASE WHEN urgency_status = 'answered' THEN urgency END AS urgency,
       CASE
         WHEN status = 'failed' OR refund_status = 'failed'
              OR topic_status = 'failed' OR urgency_status = 'failed'
           THEN 'try again later'
         WHEN refund_status = 'unsure' OR topic_status = 'unsure'
              OR urgency_status = 'unsure'
           THEN 'a person reads it'
         WHEN refund_status = 'answered' AND topic_status = 'answered'
              AND urgency_status = 'answered'
           THEN CASE WHEN is_refund THEN 'refund queue' ELSE 'inbox' END
       END AS where_it_goes
FROM forms
ORDER BY msg_id;
```

`answer` holds one entry per question, under the name you gave it. Each entry is the answer of its own function: `answer.refund` is a yes or no, so it works in `WHERE`. An entry is NULL when its question was not sure or failed, and `detail` holds that entry's own `status`, `probability` and error under the same name. One entry can be not sure while the others answer. The row's own `status` is `failed` only when the record could not be judged at all.

The saved `detail` keeps every entry's state and error. The routing query checks the form and each entry it uses: a failed entry goes to retry, a not-sure entry goes to a person, and an answer is used only after its entry is answered. With `raise_if_failed`, a failed form or any failed entry raises; a not-sure entry does not. A record with several failed entries counts once in the failure total.

Question names must not collide with generated result columns such as `status`, `probability`, `position`, `id`, `answer`, `error_code` and `detail`. The call reports a usage error naming the clash before sending any paid request. SQL keeps each question's answer and details nested under `answer` and `detail`, so `refund` beside `refund_status` is allowed: neither generates a column named after the other.

Any function except filter, rank and annotate can be an entry. filter and rank are about many records, and a form is about one.

DuckDB gives `answer` one typed field per question, so it must know the questions before the statement runs. Write `questions` out in the call, as here. It can't come from a column or a subquery.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```sql
SELECT answer AS is_refund, status
FROM thinkthen_decide((SELECT 'I was charged twice for one invoice.' AS record),
                      'Does the customer request a refund?',
                      yes_means := 'the customer wants money back, including for a duplicate charge',
                      no_means := 'the customer wants anything else');

SELECT answer AS topic, status
FROM thinkthen_choose((SELECT 'I was charged twice for one invoice.' AS record),
                      'What is the message about?',
                      choices := ['billing', 'shipping', 'other'],
                      descriptions := MAP {'billing': 'charges, invoices and refunds',
                                           'shipping': 'parcels and delivery'});
```

decide and filter take `yes_means` and `no_means`. choose, tag, score and rank take `descriptions`, keyed by your choices, labels or levels. A key that is not one of them is refused.

## Records and fields

A record is one value of the column you name. The column's type decides how it is read: a `VARCHAR` column is judged as text, even when it looks like JSON, and a `JSON` column is judged as JSON. `field := '/text'` picks one field of a JSON record. Without it, ThinkThen judges the whole record.

```sql
SELECT id->>'$' AS id, answer AS is_refund, status
FROM thinkthen_decide((SELECT '{"id": "a1", "text": "Please refund my order."}'::JSON AS record),
                      'Does the customer request a refund?',
                      field := '/text');
```

Give JSON records an `id`. Each row carries it back in the `id` column. `id` is NULL when the record has none. The column's type is `JSON`, because an id can be text or a number, and `id->>'$'` reads it as text, ready to join to your own key. A key column of your own needs none of this: it comes back beside the answer under its own name.

A record that can't be judged always comes back as a failed row, and nothing about it is sent to the service:

| The record | The failed row's `error_code` |
| --- | --- |
| Blank: an empty string, only spaces, or an empty JSON object | `record.empty` |
| NULL, or a `field` that points at nothing | `record.missing` |

A record column of any other type is a mistake in the call: DuckDB knows the type before the statement runs, so the call is refused with `usage.sql-selection`. Cast the column in your query, as in `CAST(note AS VARCHAR) AS record`.

A query that returns no rows gives no rows back and sends nothing.

## Tables

In SQL every result is already a table, so there is nothing to switch on: the answer column and the columns that explain it always come together.

| Column | Type | Holds |
| --- | --- | --- |
| your own columns | their own | The record, as your query gave it |
| `position` | `BIGINT` | The record's place in your query, from 1 |
| `id` | `JSON` | A JSON record's own `id`, or NULL |
| `answer` | by function | The answer. NULL when not sure or failed |
| `probability` | `DOUBLE` | The number compared with your bars. NULL when failed, and for tag, score, rank, recognize, relate and annotate |
| `status` | `thinkthen_status` | `answered`, `unsure` or `failed` |
| `error_code` | `thinkthen_error_code` | Why a failed row failed. NULL on every other row |
| `error_message` | `VARCHAR` | The same, in words |
| `error_http_status` | `BIGINT` | The service's HTTP status, when it gave one |
| `source` | `STRUCT` | Which question, service and model produced the row |
| `detail` | by function | The numbers behind the answer |

| Function | The type of `answer` |
| --- | --- |
| decide | `BOOLEAN` |
| choose | `VARCHAR` |
| tag | `VARCHAR[]` |
| score | `DOUBLE` |
| rank | `BIGINT` |
| find | `STRUCT(text VARCHAR, start_offset BIGINT, end_offset BIGINT)` |
| recognize | A list of `STRUCT`, one per name |
| relate | A list of `STRUCT`, one per relation |
| annotate | A `STRUCT` with one field per question, typed as that question's answer |

`thinkthen_status` and `thinkthen_error_code` are enum types the extension adds. They compare with plain text, as in `status = 'unsure'`.

Tables treat missing values their own way. `WHERE is_refund` drops the NULL rows, for example, and so does a join on the answer. `count(*)` counts every row and `count(is_refund)` counts only the answered ones, so the difference is how many had no answer. To see why an answer is missing, read `status`:

```sql
SELECT status, count(*) AS records FROM refund_checks GROUP BY status;

SELECT msg_id, body FROM refund_checks WHERE status = 'unsure';
```

Save answers in a table before you use them, as `refund_checks` is saved above. A saved table can be read, joined and filtered for free. A call can't: every run asks the service again.

Choose the records before the call, not after. Put `WHERE` and `LIMIT` inside the query you give the function. An outer `LIMIT` limits the rows you see; it is not a reliable limit on work or cost because the database may read ahead. Do not treat an early-stopped statement as proof that every input record succeeded. Save the complete results before inspecting a subset when failure reporting matters.

One statement is all or nothing. If it is cancelled or fails, nothing it would have saved is kept, even when the answers were already paid for. For a large job, save a few thousand records per statement and commit each one before starting the next.

## Settings

The extension reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment of the DuckDB process. To choose settings yourself, set them for the session.

```sql
SET thinkthen_model = '...';
SET thinkthen_batch = 16;
```

| Setting | Default | Sets |
| --- | --- | --- |
| `thinkthen_backend` | `systemone` | The service: `systemone` or `openai` |
| `thinkthen_model` | the service's default | The model |
| `thinkthen_batch` | the engine's default | The most questions in one request |
| `thinkthen_max_retries` | 3 | Retries after a failed request, from 0 to 3 |
| `thinkthen_initial_backoff_ms` | 500 | The first wait before a retry, in milliseconds |
| `thinkthen_attempt_timeout_ms` | 120000 | The time limit for one attempt, in milliseconds |

A setting lasts for the session and never changes a statement that is already running. `RESET thinkthen_model` returns a setting to its default. `SET GLOBAL` is refused.

SQL can't set a key or a service address. Both come only from the environment of the DuckDB process, so no query can read a key or send your records somewhere else. The `openai` service reads its key from `OPENAI_API_KEY`.

## Cancel

Stop a statement the way DuckDB stops any statement: Ctrl-C in its shell, or your client's interrupt. The statement fails with DuckDB's own interrupt error, and the connection is ready for the next one. Requests already sent are still charged, and a cancelled statement saves nothing.

## Errors

| What happened | What you get |
| --- | --- |
| A mistake in the call, such as a bad threshold, a missing list, `low` and `high` on filter, or a column your query does not have | The statement fails before anything is sent. The error carries a code such as `usage.threshold` |
| The engine could not start, such as no API key | The statement fails before anything is sent, with a code such as `usage.key-missing` |
| The statement was cancelled | The statement fails with DuckDB's own interrupt error |
| A record could not be judged, or the service failed | A row with `status` of `failed`, a NULL `answer`, and the reason in `error_code` and `error_message`. The statement goes on |
| The same, in `thinkthen_filter` or with `raise_if_failed` | On full consumption, one error after every record is judged. The statement fails and saves nothing; see Tables for early stopping |
| Not sure | A row with `status` of `unsure`, a NULL `answer` and its `probability`. Never an error |
| You read `status`, `probability` or the error columns | Never an error |

Two tests let you sort rows by meaning:

- `status <> 'answered'` finds every row with no answer. That covers not sure and failed.
- `status = 'failed'` finds every row where something went wrong. Not sure is a result and not an error.

A failed row's `error_code` is one of eight: `record.empty`, `record.missing`, `record.invalid`, `record.json`, `record.too-large`, `backend.transport`, `backend.status` and `backend.reply`. Every other code belongs to a failed statement. Branch on the code, never on the message, which is written for people and can change.

A raised error is DuckDB's `Invalid Input Error`, and its message starts with the code, as in `Invalid Input Error: usage.threshold: ...`. For failed records, that prefix is followed by the first failed record's code, how many failed out of those judged, the first failed position and its actual reason.

When an error is raised for failed records, treat any rows your program had already read from that statement as not returned. A client that collects the whole result first sees only the error. A client that reads the result a piece at a time can receive rows before the error reaches it. Either way the statement has failed and nothing is saved.

## Upgrading from 0.1

The baseline is the database implementation in the [published 0.1.2 source](https://proxy.golang.org/github.com/botassembly/thinkthen/@v/v0.1.2.zip).

| 0.1.2 | Now |
| --- | --- |
| Scalar calls return a bare answer or NULL | Table calls return an answer with `status`, probability and errors |
| Keyed `_many` calls take JSON objects and return `key`/`value` | The query/table input and position/id/result columns shown above |
| Separate details/probability functions | State and detail columns on each result |
| find selects among supplied units | find returns a span within each record |

Update stored queries and dependent views using this database’s calling convention. The public SQL driver ticket proves the shared contract; each database ticket proves these statements against its built extension.

---

The [complete guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/duckdb.md) is tested as written.

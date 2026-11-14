# SQL functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

In a database, each ThinkThen function is a table function. You give it a query of records and one question, and it gives back one row per record, with the answer in a column beside your own columns. This page covers what DuckDB, SQLite and PostgreSQL share. Your database's own page is a complete guide in that database's spelling, and says how to install the extension. Set `THINKTHEN_API_KEY` in the environment of the database process first.

Every row is in exactly one of three states, named by its `status` column:

- **`answered`**: there is an answer, and the `answer` column holds it.
- **`unsure`**: the model's number fell between the two bars you set, or no choice was likely enough.
- **`failed`**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Where there is no answer, `answer` is NULL, and NULL is never a yes or a no.** `WHERE answer` keeps only the yeses. `WHERE NOT answer` keeps only the noes. A missing answer passes neither test.
2. **Checking the state never fails.** `status`, `probability`, `error_code` and `error_message` are on every row, beside the answer, in every state.
3. **A failed record is a row, not a failed statement.** One bad record never costs you the others. A statement fails for a record only where you ask it to: in `thinkthen_filter`, and in any call with `raise_if_failed`.

Rule 1 has one trap, and it is SQL's own: `ELSE`, `coalesce` and `NOT IN` all treat NULL as "everything else". `CASE WHEN answer THEN 'refund queue' ELSE 'inbox' END` sends a not-sure message and a failed message to the inbox. Test `status` first, or name the no with `WHEN NOT answer`.

## Spelling the call

The examples on this page share one `messages` table. This runs in all three databases:

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

The ten functions, their options, their result columns and every rule on this page are the same in all three databases. The SQL that reads the results is the same too. Only the call itself is spelled differently, because the three databases pass arguments to a table function in three ways. Here is one call in each.

DuckDB:

```sql
SELECT msg_id, body, answer AS is_refund, status
FROM thinkthen_decide((SELECT msg_id, body FROM messages),
                      'Does the customer request a refund?',
                      record := 'body', low := 0.2, high := 0.8);
```

SQLite:

```sql
SELECT id AS msg_id, record AS body, answer AS is_refund, status
FROM thinkthen_decide('SELECT msg_id AS id, body AS record FROM messages',
                      'Does the customer request a refund?')
WHERE low = 0.2 AND high = 0.8;
```

PostgreSQL:

```sql
SELECT id AS msg_id, record AS body, answer AS is_refund, status
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages',
                      'Does the customer request a refund?',
                      low => 0.2, high => 0.8);
```

| | DuckDB | SQLite | PostgreSQL |
| --- | --- | --- | --- |
| The function's name | `thinkthen_decide` | `thinkthen_decide` | `thinkthen.decide` |
| The records | A query in parentheses. `record := 'body'` names the column to judge | A query as text. The column to judge is named `record`, and your key is named `id` | As SQLite |
| Your own columns | Every column of your query comes back beside the answer | `id` and `record` come back beside the answer | As SQLite |
| An option | `low := 0.2`, inside the call | `WHERE low = 0.2`, after the call | `low => 0.2`, inside the call |
| A list | `['billing', 'shipping']` | `'["billing", "shipping"]'`, as JSON text | `ARRAY['billing', 'shipping']` |
| A yes and a no | `true`, `false` | `1`, `0` | `true`, `false` |

From here on, each call is written once, the DuckDB way. A block of SQL with no comment on its first line runs unchanged in all three databases. A block that can't starts with a comment that names the databases that write it differently, and which part changes.

## decide

### The simplest if

SQL's `if` is `WHERE`. A decide answer goes straight into it.

```sql
-- Differs in SQLite and PostgreSQL: the call.
SELECT 'refund queue' AS where_it_goes
FROM thinkthen_decide((SELECT 'Please refund my order.' AS record),
                      'Does the customer request a refund?')
WHERE answer;
```

The row comes back only on a yes. If the message could not be judged, `answer` is NULL, NULL is never true, and no row comes back. It never reads as a yes.

Don't turn this into a `CASE` with a plain `ELSE`. `ELSE` runs for every row that is not a yes: no, not sure and failed alike. To act on a no, name it:

```sql
-- Differs in SQLite and PostgreSQL: the call.
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
-- Differs in SQLite and PostgreSQL: the call.
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

SQL can't check that a `CASE` names every state. Leave the outer `ELSE` off, as here, so a state you forgot shows up as NULL and not as the wrong queue.

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
-- Differs in SQLite and PostgreSQL: the call.
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
-- Differs in SQLite and PostgreSQL: the call and the list.
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

SQL has no list of choices it can check a `CASE` against. Add a choice and nothing points at this `CASE`. Name every choice with its own `WHEN` and leave `ELSE` off, so a new choice shows up as NULL and a not-sure row is never sent to the front desk.

`probability` is the best choice's. `detail` holds every choice with its probability.

## Many records

Every function takes many records the same way: the query you give it can return any number of rows. You get one row back per record, in the order of your query, and each row carries your own columns beside the answer.

```sql
-- Differs in SQLite and PostgreSQL: the call.
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

Give the whole set of records to one call, so ThinkThen can send them in batches. Don't call a function once per row of another table: each call is a separate piece of work.

To save every result, put the call in `CREATE TABLE ... AS`, as above. The table keeps the state of every row, so nothing is lost. Then read the table as often as you like. Running the call again asks the service again, and costs again.

There is no separate call for one record. One record is a query that returns one row, as in the decide examples above.

## filter

filter means "give me the yeses". It hands back your own rows, in order.

```sql
-- Differs in SQLite and PostgreSQL: the call.
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
-- Differs in SQLite and PostgreSQL: the call and the list.
CREATE TEMP TABLE topics AS
SELECT position, id, answer, status
FROM thinkthen_tag((SELECT body FROM messages WHERE msg_id = 4),
                   'Which topics appear?',
                   labels := ['billing', 'shipping'], record := 'body');
```

```sql
-- Differs in SQLite: reading a list.
SELECT 'billing team' AS team_for FROM topics WHERE 'billing' = ANY(answer);

SELECT topics.position, topics.id, topics.status, label.topic
FROM topics
LEFT JOIN LATERAL (SELECT unnest(topics.answer) AS topic) AS label ON true;
```

The left join keeps an empty or failed result as one row with a NULL topic. `position`, `id` and `status` still identify it and distinguish an empty answer from a failure.

tag is never not sure. `threshold := 0.7` sets how likely a label must be to count. An empty list is an answer: no label fit. A NULL `answer` means the row failed. `probability` is always NULL for tag, and each label's own number is in `detail`.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it.

```sql
-- Differs in SQLite and PostgreSQL: the call and the list.
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

Don't put a score or a rank straight into `WHERE`. DuckDB and SQLite accept a number there and read every number but 0 as a yes. PostgreSQL refuses it. Compare it with a number, as in `answer > 0.5`.

## rank

rank puts many records in order along your levels, highest first.

```sql
-- Differs in SQLite and PostgreSQL: the call and the list.
SELECT ranked.answer, ranked.body
FROM thinkthen_rank((SELECT msg_id, body FROM messages WHERE msg_id <= 3),
                    'How urgent is the request?',
                    levels := ['routine', 'urgent'], record := 'body') AS ranked
ORDER BY ranked.answer NULLS LAST;
```

`answer` is the rank, starting at 1, and the other columns are your record. `position` still says where the record was in your query. rank reads every record before it gives anything back. `top := 5` selects the five highest-ranked messages after judging the collection. A failed record comes back with a NULL rank and `status` set to `failed`. rank is never not sure.

## find

find points to the one line that answers.

```sql
-- Differs in SQLite and PostgreSQL: the call.
CREATE TEMP TABLE action AS
SELECT body, answer, status
FROM thinkthen_find((SELECT body FROM messages WHERE msg_id = 5),
                    'Which line states the requested action?',
                    record := 'body');
```

```sql
-- Differs in SQLite and PostgreSQL: reading the parts of an answer.
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
-- Differs in SQLite and PostgreSQL: the call, the list, and reading a list and the parts of an answer.
SELECT names.position, names.id, names.status, name.kind, name.text
FROM thinkthen_recognize((SELECT 'Morgan works for Example Group.' AS record),
                         kinds := ['person', 'organization']) AS names
LEFT JOIN LATERAL (SELECT unnest(names.answer) AS name) ON true;
```

`answer` is a list of names. Each name has `kind`, `text`, `start_offset`, `end_offset` and `probability`. The list can be empty. recognize is never not sure.

The left join gives one row per name. A record with no names, or one that failed, stays as one row whose name columns are NULL, so it is never dropped without a word. A plain `unnest` in the `SELECT` list, or a cross join, would drop that record.

## relate

```sql
-- Differs in SQLite and PostgreSQL: the call, the list, and reading a list and the parts of an answer.
SELECT relations.position, relations.id, relations.status,
       relation.source.text AS source, relation.relation, relation.target.text AS target
FROM thinkthen_relate((SELECT body FROM messages WHERE msg_id = 5),
                      rules := [{'source': 'person', 'target': 'organization', 'relation': 'works_for'}],
                      record := 'body') AS relations
LEFT JOIN LATERAL (SELECT unnest(relations.answer) AS relation) ON true;
```

A rule names the kind it starts from, the kind it points to, and the relation, in that order. It may also give `reads`, the wording the model sees, and `either`, for a relation with no direction. Each relation in `answer` has `source`, `target`, `relation`, `probability` and `either`. `detail` holds every name found, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record. The questions are written as JSON, each under a name of your choosing.

```sql
-- Differs in SQLite and PostgreSQL: the call, and reading the parts of an answer.
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

## Say what each answer means

When a short question is not enough, say what the answers mean.

```sql
-- Differs in SQLite and PostgreSQL: the call and the list.
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

A record is one value of the column you name: text, or JSON. Text is judged as it stands, even when it looks like JSON. `field := '/text'` picks one field of a JSON record. Without it, ThinkThen judges the whole record.

```sql
-- Differs in SQLite and PostgreSQL: the call. Differs in SQLite: how a record is marked as JSON.
SELECT id, answer AS is_refund, status
FROM thinkthen_decide((SELECT '{"id": "a1", "text": "Please refund my order."}'::JSON AS record),
                      'Does the customer request a refund?',
                      field := '/text');
```

Give JSON records an `id`. Each row carries it back in the `id` column. `id` is NULL when the record has none.

A record that can't be judged always comes back as a failed row, and nothing about it is sent to the service:

| The record | The failed row's `error_code` |
| --- | --- |
| Blank: an empty string, only spaces, or `{}` | `record.empty` |
| NULL, or a `field` that points at nothing | `record.missing` |
| Not text and not JSON | `record.invalid` |

A query that returns no rows gives no rows back and sends nothing.

## Tables

In SQL every result is already a table, so there is nothing to switch on: the answer column and the columns that explain it always come together.

| Column | Holds |
| --- | --- |
| your own columns | The record, as your query gave it |
| `position` | The record's place in your query, from 1 |
| `id` | A JSON record's own `id`, or NULL |
| `answer` | The answer, in the type of the function. NULL when not sure or failed |
| `probability` | The number compared with your bars. NULL when failed, and for tag, score, rank, recognize, relate and annotate |
| `status` | `answered`, `unsure` or `failed` |
| `error_code`, `error_message`, `error_http_status` | Why a failed row failed. NULL on every other row |
| `source` | Which question, service and model produced the row |
| `detail` | The numbers behind the answer, by function |

Tables treat missing values their own way. `WHERE is_refund` drops the NULL rows, for example, and so does a join on the answer. `count(*)` counts every row and `count(is_refund)` counts only the answered ones, so the difference is how many had no answer. To see why an answer is missing, read `status`:

```sql
SELECT status, count(*) AS records FROM refund_checks GROUP BY status;

SELECT msg_id, body FROM refund_checks WHERE status = 'unsure';
```

Save answers in a table before you use them, as `refund_checks` is saved above. A saved table can be read, joined and filtered for free. A call can't: every run asks the service again.

Choose the records before the call, not after. Put `WHERE` and `LIMIT` inside the query you give the function. An outer `LIMIT` limits the rows you see; it is not a reliable limit on work or cost because the database may read ahead. Do not treat an early-stopped statement as proof that every input record succeeded. Save the complete results before inspecting a subset when failure reporting matters.

One statement is all or nothing. If it is cancelled or fails, nothing it would have saved is kept, even when the answers were already paid for. For a large job, save a few thousand records per statement and commit each one before starting the next.

## Settings

The extension reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment of the database process. To choose settings yourself, set them for the session.

```sql
-- Differs in SQLite and PostgreSQL: how a setting is set.
SET thinkthen_model = '...';
SET thinkthen_batch = 16;
```

The settings are `backend`, `model`, `batch`, `max_retries`, the first wait before a retry and the time limit for one attempt. A setting lasts for the session and never changes a statement that is already running. `max_retries` goes from 0 to 3.

SQL can't set a key or a service address. Both come only from the environment of the database process, so no query can read a key or send your records somewhere else.

## Cancel

Stop a statement the way your database stops any statement: Ctrl-C in its shell, or your client's interrupt. The statement fails with the database's own "cancelled" error, and the connection is ready for the next one. Requests already sent are still charged, and a cancelled statement saves nothing.

## Errors

| What happened | What you get |
| --- | --- |
| A mistake in the call, such as a bad threshold, a missing list, `low` and `high` on filter, or a column your query does not have | The statement fails before anything is sent. The error carries a code such as `usage.threshold` |
| The engine could not start, such as no API key | The statement fails before anything is sent, with a code such as `usage.key-missing` |
| The statement was cancelled | The statement fails with the database's own "cancelled" error |
| A record could not be judged, or the service failed | A row with `status` of `failed`, a NULL `answer`, and the reason in `error_code` and `error_message`. The statement goes on |
| The same, in `thinkthen_filter` or with `raise_if_failed` | On full consumption, one error after every record is judged. The statement fails and saves nothing; see Tables for early stopping |
| Not sure | A row with `status` of `unsure`, a NULL `answer` and its `probability`. Never an error |
| You read `status`, `probability` or the error columns | Never an error |

Two tests let you sort rows by meaning:

- `status <> 'answered'` finds every row with no answer. That covers not sure and failed.
- `status = 'failed'` finds every row where something went wrong. Not sure is a result and not an error.

A failed row's `error_code` is one of eight: `record.empty`, `record.missing`, `record.invalid`, `record.json`, `record.too-large`, `backend.transport`, `backend.status` and `backend.reply`. Every other code belongs to a failed statement. Branch on the code, never on the message, which is written for people and can change.

The error for failed records reports how many failed out of those judged, followed by the first failed position and that record's actual code and reason. Your database's page shows where its errors put the code.

When an error is raised for failed records, treat any rows your program had already read from that statement as not returned. PostgreSQL never hands any over. SQLite and DuckDB can hand over rows before the error reaches a program that reads them one at a time. In all three, the statement has failed and nothing is saved.

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

# PostgreSQL functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

In PostgreSQL, each ThinkThen function is a table function in the `thinkthen` schema. You give it a query of records and one question, and it gives back one row per record, with the answer in a column beside your record. This page is the whole guide for PostgreSQL, so you need no other page open to run it. [Install](#install) comes first.

Every row is in exactly one of three states, named by its `status` column:

- **`answered`**: there is an answer, and the `answer` column holds it.
- **`unsure`**: the model's number fell between the two bars you set, or no choice was likely enough.
- **`failed`**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Where there is no answer, `answer` is NULL, and NULL is never a yes or a no.** `WHERE answer` keeps only the yeses. `WHERE NOT answer` keeps only the noes. A missing answer passes neither test.
2. **Checking the state never fails.** `status`, `probability`, `error_code` and `error_message` are on every row, beside the answer, in every state.
3. **A failed record is a row, not a failed statement.** One bad record never costs you the others. A statement fails for a record only where you ask it to: in `thinkthen.filter`, and in any call with `raise_if_failed`.

Rule 1 has one trap, and it is SQL's own: `ELSE`, `coalesce` and `NOT IN` all treat NULL as "everything else". `CASE WHEN answer THEN 'refund queue' ELSE 'inbox' END` sends a not-sure message and a failed message to the inbox. Test `status` first, or name the no with `WHEN NOT answer`.

## Install

The extension runs in PostgreSQL 18, on Linux x86-64 or arm64 with glibc 2.28 or later. It contains the engine, so there is nothing else to install.

1. Download the PostgreSQL 18 archive for your server from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0), check it against the release's checksum, and extract it.
2. Copy `lib/thinkthen_postgres.so` into the folder `pg_config --pkglibdir` prints, and `share/extension/*` into the `extension` folder under `pg_config --sharedir`.
3. Set `THINKTHEN_API_KEY` in the server's environment, then restart the server.
4. In each database that needs it, run this as an administrator:

```sql
CREATE SCHEMA thinkthen;
CREATE EXTENSION thinkthen SCHEMA thinkthen;
```

The examples on this page are run by an administrator. [Who may call the functions](#who-may-call-the-functions) shows how to let other roles in.

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
SELECT id AS msg_id, record AS body, answer AS is_refund, status
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages',
                      'Does the customer request a refund?',
                      low => 0.2, high => 0.8);
```

The records:

- The records are a query, given as text in the first argument. It must be one read-only statement.
- The column to judge is named `record`, and your key is a column named `id`. `record_column` and `id_column` name other columns.
- Your other columns do not come back. Each row carries `id` and `record`. To get other columns, join on `id`.
- Bind values into the query with `params`, a JSON array, and never paste them into its text.
- Add `ORDER BY` inside the query so `position` follows your order.

The options:

- The functions are in the `thinkthen` schema: `thinkthen.decide`, `thinkthen.choose`, `thinkthen.filter` and so on.
- An option goes inside the call, by name, as `low => 0.2`.
- A list is an array: `ARRAY['billing', 'shipping', 'other']`. `descriptions` and annotate's `questions` are JSON.
- When the query holds quotes of its own, write it in dollar quotes: `$$SELECT 'Please refund my order.' AS record$$`.

The results:

- A yes is `true` and a no is `false`. psql shows them as `t` and `f`.
- `id` is always `jsonb`, because a function's column types are fixed and a key can be a number or text. Cast it to join: `(id)::integer`, or `id #>> '{}'` for text.
- `record` is always text, whatever the type of your column.
- An answer with parts is a value with fields, read with parentheses around the column, as `(answer).text`. A list is an array.
- `status` and `error_code` are enumerated types. They compare with plain text, as in `status = 'unsure'`.

## decide

### The simplest if

SQL's `if` is `WHERE`. A decide answer goes straight into it.

```sql
SELECT 'refund queue' AS where_it_goes
FROM thinkthen.decide($$SELECT 'Please refund my order.' AS record$$,
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
FROM thinkthen.decide($$SELECT 'Please refund my order.' AS record$$,
                      'Does the customer request a refund?');
```

Where there is no answer, this gives NULL. It never gives `inbox`.

### Yes, no, or something else

Set two bars and decide can also be not sure. `status` tells you whether there is a yes or no to read. This call saves its one result in a small table named `is_refund`, so the next few examples can read it without asking again.

```sql
CREATE TEMP TABLE is_refund AS
SELECT answer, probability, status, error_code, error_message
FROM thinkthen.decide($$SELECT 'Where is my parcel?' AS record$$,
                      'Does the customer request a refund?',
                      low => 0.2, high => 0.8);
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
SELECT answer AS is_refund
FROM thinkthen.decide($$SELECT 'Please refund my order.' AS record$$,
                      'Does the customer request a refund?',
                      raise_if_failed => true);
```

With `raise_if_failed`, the statement fails with one error when any record failed. For annotate, a failed form or any failed entry counts as a failed record; a not-sure entry does not. Count each affected record once, even when several entries failed. For a failed outer form, report its error; otherwise report the first failed entry in question-name order. If no record failed, the rows come back as usual, so the option fits on any call. Every function but filter takes it. filter always raises, as its section says.

For a fully consumed call, the error comes after every record is judged. It says how many records failed, the position of the first, and its error code. The statement fails as a whole, so a `CREATE TABLE ... AS` or an `INSERT ... SELECT` around it saves nothing. An error can't carry rows, so the answers that did come are gone with it. Leave the option off whenever you save answers: the failed rows then sit in your table with the reason beside them.

### Both bars and the probability

```sql
SELECT probability FROM is_refund;
```

- No bar option: one bar at 0.5. The answer is always yes or no.
- `threshold => 0.7`: one bar. Yes at 0.7 or above, otherwise no.
- `low => 0.2, high => 0.8`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. It is NULL only when the row failed.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `thinkthen.decide`, save each `probability` in a table, and compare with your labels.

## choose with your own choices

Give your choices as a list. The answer comes back as one of them, as text, spelled exactly as you wrote it.

```sql
CREATE TEMP TABLE topic AS
SELECT answer, probability, status
FROM thinkthen.choose($$SELECT 'I was charged twice for one invoice.' AS record$$,
                      'What is the message about?',
                      choices => ARRAY['billing', 'shipping', 'other']);
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

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So test `status` before the answer. `threshold => 0.3` moves the bar. choose takes one bar only.

The choices (`billing`, `shipping`, `other`) are the words the model reads, so make them plain words or short phrases, not codes.

SQL has no list of choices it can check a `CASE` against. Add a choice and nothing points at this `CASE`. Name every choice with its own `WHEN` and leave `ELSE` off, so a new choice shows up as NULL and a not-sure row is never sent to the front desk.

`probability` is the best choice's. `detail` holds every choice with its probability.

## Many records

Every function takes many records the same way: the query you give it can return any number of rows. You get one row back per record, in the order of your query, and each row carries your `id` and `record` beside the answer.

```sql
CREATE TABLE refund_checks AS
SELECT (id)::integer AS msg_id, record AS body, answer AS is_refund,
       probability, status, error_code, error_message
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                      'Does the customer request a refund?',
                      low => 0.2, high => 0.8);
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

filter means "give me the yeses". It hands back your own records, in order.

```sql
SELECT (refund_request.id)::integer AS msg_id, refund_request.record AS body
FROM thinkthen.filter('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                      'Does the customer request a refund?') AS refund_request;
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `threshold => 0.7` moves the bar. `low` and `high` are refused with a `usage.threshold` error that names `thinkthen.decide`: when you want not sure, use `thinkthen.decide`, as in the section above.

Your own records can't carry a failure. So if any record fails, a fully consumed filter call raises one error after every record is judged, and the statement fails as a whole. It works as `raise_if_failed` does: nothing is saved, and the error says how many records failed, the position of the first, and its code.

To see each failed record, or to keep the answers you paid for whatever happens, ask with `thinkthen.decide`, save the rows, and pick the yeses yourself:

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
FROM thinkthen.tag('SELECT body AS record FROM messages WHERE msg_id = 4',
                   'Which topics appear?',
                   labels => ARRAY['billing', 'shipping']);
```

```sql
SELECT 'billing team' AS team_for FROM topics WHERE 'billing' = ANY(answer);

SELECT topics.position, topics.id, topics.status, label.topic
FROM topics
LEFT JOIN LATERAL unnest(topics.answer) AS label(topic) ON true;
```

The left join keeps an empty or failed result as one row with a NULL topic. `position`, `id` and `status` still identify it and distinguish an empty answer from a failure.

tag is never not sure. `threshold => 0.7` sets how likely a label must be to count. An empty list is an answer: no label fit. A NULL `answer` means the row failed. `probability` is always NULL for tag, and each label's own number is in `detail`.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it.

```sql
CREATE TEMP TABLE urgency AS
SELECT answer, status
FROM thinkthen.score($$SELECT 'Please refund my order.' AS record$$,
                     'How urgent is the request?',
                     levels => ARRAY['routine', 'urgent']);
```

```sql
SELECT answer AS urgency,
       CASE WHEN answer > 0.5 THEN 'answer today' END AS note
FROM urgency;
```

The score is a position between your levels, not one of them, so it is a number and there is nothing to put in a `CASE` by name. To get exactly one level back, ask with `thinkthen.choose`. score is never not sure, and its `probability` is always NULL.

## rank

rank puts many records in order along your levels, highest first.

```sql
SELECT ranked.answer, ranked.record AS body
FROM thinkthen.rank('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
                    'How urgent is the request?',
                    levels => ARRAY['routine', 'urgent']) AS ranked
ORDER BY ranked.answer NULLS LAST;
```

`answer` is the rank, starting at 1, and `id` and `record` are your record. `position` still says where the record was in your query. rank reads every record before it gives anything back. `top => 5` selects the five highest-ranked messages after judging the collection. A failed record comes back with a NULL rank and `status` set to `failed`. rank is never not sure.

## find

find points to the one line that answers.

```sql
CREATE TEMP TABLE action AS
SELECT record AS body, answer, status
FROM thinkthen.find('SELECT body AS record FROM messages WHERE msg_id = 5',
                    'Which line states the requested action?');
```

```sql
SELECT CASE
         WHEN status = 'unsure' THEN 'a person reads it'
         WHEN status = 'answered' AND answer IS NULL THEN 'no line answers'
         WHEN status = 'answered' THEN (answer).text
       END AS line
FROM action;
```

The answer has three parts, `text`, `start_offset` and `end_offset`, or it is NULL when no line answers. Read a part with parentheses around the column, as `(answer).text`. find is the one function where a NULL answer can be an answer, so `status` is what tells "no line answers" from not sure and failed.

The offsets count characters from 0, and `end_offset` is one past the last character. SQL counts from 1, so `substr(body, (answer).start_offset + 1, (answer).end_offset - (answer).start_offset)` is the line. Like choose, find can be not sure with no bar set.

## recognize

```sql
SELECT names.position, names.id, names.status, name.kind, name.text
FROM thinkthen.recognize($$SELECT 'Morgan works for Example Group.' AS record$$,
                         kinds => ARRAY['person', 'organization']) AS names
LEFT JOIN LATERAL unnest(names.answer) AS name ON true;
```

`answer` is a list of names. Each name has `kind`, `text`, `start_offset`, `end_offset` and `probability`. The list can be empty. recognize is never not sure.

`LEFT JOIN LATERAL unnest(...)` gives one row per name. A record with no names, or one that failed, stays as one row whose name columns are NULL, so it is never dropped without a word.

## relate

```sql
SELECT relations.position, relations.id, relations.status,
       (relation.source).text AS source, relation.relation, (relation.target).text AS target
FROM thinkthen.relate('SELECT body AS record FROM messages WHERE msg_id = 5',
                      rules => ARRAY[ROW('person', 'organization', 'works_for', NULL, false)::thinkthen.rule]) AS relations
LEFT JOIN LATERAL unnest(relations.answer) AS relation ON true;
```

A rule names the kind it starts from, the kind it points to, and the relation, in that order. It is a `thinkthen.rule`, written by position with all five parts: `ROW(source, target, relation, reads, either)`. `reads` is the wording the model sees, and NULL leaves it to the relation's name. `either` is true for a relation with no direction. Each relation in `answer` has `source`, `target`, `relation`, `probability` and `either`. `source` and `target` are names without a probability, read as `(relation.source).text`. `detail` holds every name found, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record. The questions are written as JSON, each under a name of your choosing.

```sql
CREATE TABLE forms AS
SELECT (id)::integer AS msg_id, record AS body, status, error_code, error_message, detail,
       (answer ->> 'refund')::boolean AS is_refund,
       answer ->> 'topic' AS topic,
       (answer ->> 'urgency')::float8 AS urgency,
       detail -> 'refund' ->> 'status' AS refund_status,
       detail -> 'topic' ->> 'status' AS topic_status,
       detail -> 'urgency' ->> 'status' AS urgency_status
FROM thinkthen.annotate('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id <= 3 ORDER BY msg_id',
  questions => '{
    "refund": {"function": "decide", "question": "Does the customer request a refund?",
               "threshold": {"low": 0.2, "high": 0.8}},
    "topic": {"function": "choose", "question": "What is the message about?",
              "choices": ["billing", "shipping", "other"]},
    "urgency": {"function": "score", "question": "How urgent is the request?",
                "levels": ["routine", "urgent"]}
  }');
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

`answer` holds one entry per question, under the name you gave it. Each entry is the answer of its own function. `answer` is `jsonb`, so `answer ->> 'refund'` reads an entry as text and a cast gives it its type: `(answer ->> 'refund')::boolean` is a yes or no, so it works in `WHERE`. An entry is NULL when its question was not sure or failed, and `detail` holds that entry's own `status`, `probability` and error under the same name. One entry can be not sure while the others answer. The row's own `status` is `failed` only when the record could not be judged at all.

The saved `detail` keeps every entry's state and error. The routing query checks the form and each entry it uses: a failed entry goes to retry, a not-sure entry goes to a person, and an answer is used only after its entry is answered. With `raise_if_failed`, a failed form or any failed entry raises; a not-sure entry does not. A record with several failed entries counts once in the failure total.

Question names must not collide with generated result columns such as `status`, `probability`, `position`, `id`, `answer`, `error_code` and `detail`. The call reports a usage error naming the clash before sending any paid request. SQL keeps each question's answer and details nested under `answer` and `detail`, so `refund` beside `refund_status` is allowed: neither generates a column named after the other.

Any function except filter, rank and annotate can be an entry. filter and rank are about many records, and a form is about one.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```sql
SELECT answer AS is_refund, status
FROM thinkthen.decide($$SELECT 'I was charged twice for one invoice.' AS record$$,
                      'Does the customer request a refund?',
                      yes_means => 'the customer wants money back, including for a duplicate charge',
                      no_means => 'the customer wants anything else');

SELECT answer AS topic, status
FROM thinkthen.choose($$SELECT 'I was charged twice for one invoice.' AS record$$,
                      'What is the message about?',
                      choices => ARRAY['billing', 'shipping', 'other'],
                      descriptions => '{"billing": "charges, invoices and refunds",
                                        "shipping": "parcels and delivery"}');
```

decide and filter take `yes_means` and `no_means`. choose, tag, score and rank take `descriptions`, keyed by your choices, labels or levels. A key that is not one of them is refused.

## Records and fields

A record is one value of the `record` column: `text`, `json` or `jsonb`. Text is judged as it stands, even when it looks like JSON. `field => '/text'` picks one field of a JSON record. Without it, ThinkThen judges the whole record.

```sql
SELECT id #>> '{}' AS id, answer AS is_refund, status
FROM thinkthen.decide($$SELECT '{"id": "a1", "text": "Please refund my order."}'::jsonb AS record$$,
                      'Does the customer request a refund?',
                      field => '/text');
```

Give JSON records an `id`. Each row carries it back in the `id` column. `id` is NULL when the record has none.

`id` holds your query's `id` column when it has one, and otherwise the JSON record's own `id`. Either way it is `jsonb`. A SQL NULL in the query's `id` column falls back to the JSON record's own `id`. A JSON `null` is a supplied id and overrides it.

A record that can't be judged always comes back as a failed row, and nothing about it is sent to the service:

| The record | The failed row's `error_code` |
| --- | --- |
| Blank: an empty string, only spaces, or `{}` | `record.empty` |
| NULL, JSON `null`, or a `field` that points at nothing | `record.missing` |

A `record` column of any other type is a mistake in the call, not a failed row: PostgreSQL knows the column's type before it reads a record, so the statement fails before anything is sent. A JSON array in one cell is one record, not several.

A query that returns no rows gives no rows back and sends nothing.

Bind values into the query with `params`. Each `$1`, `$2` in the query text takes the value at that place in the array. Text binds as `text`, a number as `numeric` and a yes or no as `boolean`.

```sql
SELECT (id)::integer AS msg_id, answer AS is_refund, status
FROM thinkthen.decide('SELECT msg_id AS id, body AS record FROM messages WHERE msg_id = $1',
                      'Does the customer request a refund?',
                      params => '[1]');
```

To read another column of your table beside an answer, join on `id`:

```sql
SELECT messages.msg_id, messages.body, refund_checks.is_refund, refund_checks.status
FROM refund_checks
JOIN messages ON messages.msg_id = refund_checks.msg_id
ORDER BY messages.msg_id;
```

When your columns have other names, say so in the call and leave the query as it is: `record_column => 'body'` names the column to judge and `id_column => 'msg_id'` names the key.

## Tables

In SQL every result is already a table, so there is nothing to switch on: the answer column and the columns that explain it always come together.

| Column | Type | Holds |
| --- | --- | --- |
| `position` | `bigint` | The record's place in your query, from 1 |
| `id` | `jsonb` | Your `id` column, or a JSON record's own `id`, or NULL |
| `record` | `text` | The record, as your query gave it |
| `answer` | by function | The answer. NULL when not sure or failed |
| `probability` | `float8` | The number compared with your bars. NULL when failed, and for tag, score, rank, recognize, relate and annotate |
| `status` | `thinkthen.status` | `answered`, `unsure` or `failed` |
| `error_code`, `error_message`, `error_http_status` | `thinkthen.error_code`, `text`, `integer` | Why a failed row failed. NULL on every other row |
| `source` | `thinkthen.source` | Which question, service and model produced the row |
| `detail` | `jsonb` | The numbers behind the answer, by function |

`answer` is `boolean` for decide and filter, `text` for choose, `text[]` for tag, `float8` for score, `bigint` for rank, `thinkthen.span` for find, `thinkthen.recognized_name[]` for recognize, `thinkthen.relation[]` for relate and `jsonb` for annotate.

Tables treat missing values their own way. `WHERE is_refund` drops the NULL rows, for example, and so does a join on the answer. `count(*)` counts every row and `count(is_refund)` counts only the answered ones, so the difference is how many had no answer. To see why an answer is missing, read `status`:

```sql
SELECT status, count(*) AS records FROM refund_checks GROUP BY status;

SELECT msg_id, body FROM refund_checks WHERE status = 'unsure';
```

Save answers in a table before you use them, as `refund_checks` is saved above. A saved table can be read, joined and filtered for free. A call can't: every run asks the service again.

Choose the records before the call, not after. Put `WHERE` and `LIMIT` inside the query you give the function. A `LIMIT` on the outer statement limits the rows you see, not the records you pay for: PostgreSQL runs the whole function, and collects every row, before it applies an outer `LIMIT`.

One statement is all or nothing. If it is cancelled or fails, nothing it would have saved is kept, even when the answers were already paid for. For a large job, save a few thousand records per statement and commit each one before starting the next.

## Settings

The extension reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment of the database process. To choose settings yourself, set them for the session.

```sql
SET thinkthen.batch = 16;
SET thinkthen.attempt_timeout = '30s';
```

| Setting | Default | Sets |
| --- | --- | --- |
| `thinkthen.backend` | `systemone` | The service: `systemone` or `openai` |
| `thinkthen.model` | the service's default | The model |
| `thinkthen.batch` | `0`, which leaves it to ThinkThen | Records per request |
| `thinkthen.max_retries` | `3` | Retries after a failed request |
| `thinkthen.initial_backoff` | `500ms` | The first wait before a retry |
| `thinkthen.attempt_timeout` | `2min` | The time limit for one attempt |

A setting lasts for the session and never changes a statement that is already running. `max_retries` goes from 0 to 3. The two times take PostgreSQL's units, as `'30s'` or `'2min'`, and a bare number is milliseconds. `ALTER ROLE ... SET` and `ALTER DATABASE ... SET` give a role or a database its own defaults, `SHOW thinkthen.batch` reads a setting, and `RESET` returns it to its default.

SQL can't set a key or a service address. Both come only from the environment of the database process, so no query can read a key or send your records somewhere else. The `openai` service reads `OPENAI_API_KEY` from the same place.

### Who may call the functions

`CREATE EXTENSION` takes the ten functions away from `PUBLIC`, so at first only an administrator can call them. An administrator grants a role the schema, each function it calls and the tables its queries read:

```sql
CREATE ROLE app LOGIN;
GRANT USAGE ON SCHEMA thinkthen TO app;
GRANT EXECUTE ON FUNCTION thinkthen.decide TO app;
GRANT SELECT ON messages TO app;
ALTER ROLE app SET thinkthen.allowed_backends = 'systemone:*';
```

The query you give a function runs as the role that calls it, so it can read only what that role can read.

`thinkthen.allowed_backends` says which services and models a session may choose. It lists `backend:model` pairs separated by commas, such as `'systemone:*,openai:*'`, where `*` allows any model. Only an administrator can set it. Without it, a session may use only `systemone` with its default model, and that holds for a service chosen through `THINKTHEN_BACKEND` too. A session that chooses anything else gets a `usage.sql-selection` error at its next call.

## Cancel

Stop a statement the way PostgreSQL stops any statement: Ctrl-C in psql, your client's interrupt, or `pg_cancel_backend` from another session. `statement_timeout` stops it the same way. The statement fails with PostgreSQL's own "cancelled" error, SQLSTATE `57014`, and the connection is ready for the next one. Requests already sent are still charged, and a cancelled statement saves nothing.

## Errors

| What happened | What you get |
| --- | --- |
| A mistake in the call, such as a bad threshold, a missing list, `low` and `high` on filter, or a column your query does not have | The statement fails before anything is sent. The error carries a code such as `usage.threshold` |
| The engine could not start, such as no API key | The statement fails before anything is sent, with a code such as `usage.key-missing` |
| The statement was cancelled | The statement fails with PostgreSQL's own "cancelled" error |
| A record could not be judged, or the service failed | A row with `status` of `failed`, a NULL `answer`, and the reason in `error_code` and `error_message`. The statement goes on |
| The same, in `thinkthen.filter` or with `raise_if_failed` | On full consumption, one error after every record is judged. The statement fails and saves nothing; see Tables for early stopping |
| Not sure | A row with `status` of `unsure`, a NULL `answer` and its `probability`. Never an error |
| You read `status`, `probability` or the error columns | Never an error |

Two tests let you sort rows by meaning:

- `status <> 'answered'` finds every row with no answer. That covers not sure and failed.
- `status = 'failed'` finds every row where something went wrong. Not sure is a result and not an error.

HTTP 401 or 403 stops the call with `backend.status`. The call also stops when every request in its first batch fails and at least one failure has an HTTP status from 400 through 499. These are statement errors even without `raise_if_failed`; other service failures follow the failed-row rule above.

A failed row's `error_code` is one of eight: `record.empty`, `record.missing`, `record.invalid`, `record.json`, `record.too-large`, `backend.transport`, `backend.status` and `backend.reply`. Every other code belongs to a failed statement. Branch on the code, never on the message, which is written for people and can change.

A raised error carries its code in `DETAIL`, not at the start of the message. psql prints it on the `DETAIL:` line, and a client library gives it as the error's detail. Many errors also carry a `HINT` with the fix. For a failed-record report, the main message carries how many failed out of those judged, then the first failed position and that record's actual code and reason. In psql, the main message follows `ERROR:` and the code appears separately after `DETAIL:`.

The error's SQLSTATE says what kind of fix it needs, so a program that retries can tell a service that is down from a mistake in the call. A "records failed" error takes the SQLSTATE of its first failure.

| SQLSTATE | Codes | Means |
| --- | --- | --- |
| `22023` | every `usage.` code but the next | Fix the call or a setting |
| `42501` | `usage.sql-selection` | Ask an administrator to allow the service or model |
| `22000` | every `record.` code | A record failed, in `thinkthen.filter` or with `raise_if_failed` |
| `08001` | `backend.transport` | The server can't reach the service |
| `58000` | `backend.status`, `backend.reply` and every `local.` code but the next | The service or the server refused or failed |
| `57014` | `local.cancelled` | The statement was cancelled |
| `XX000` | `internal.bug` | A bug in ThinkThen |

Four mistakes in the call are caught by PostgreSQL's own checks and carry PostgreSQL's codes: `42703` for a column your query does not have, `42804` for a `record` column that is not `text`, `json` or `jsonb`, `22004` for a required argument that is NULL, and `22023` for a query that is not one read-only `SELECT` or a `params` that is not a JSON array of text, numbers, yes-or-no values and nulls.

When an error is raised for failed records, no rows have been handed over. PostgreSQL collects a function's rows before it returns any, so a program never holds part of a failed statement's result.

## Upgrading from 0.1

The [published 0.1.2 source archive](https://proxy.golang.org/github.com/botassembly/thinkthen/@v/v0.1.2.zip) includes the real PostgreSQL extension under `databases/postgresql`, with scalar and keyed `_many` functions in `public`. Use that source as the migration baseline. The current test-build fixture alone does not prove migration from those definitions; the PostgreSQL ticket must run the migration against the actual old definitions.

The 0.2.0 package has no `ALTER EXTENSION` update path from the published 0.1.2 source's definitions. Back up the database, copy the 0.2.0 files over the old ones, and make the extension again in each database:

```sql
DROP EXTENSION thinkthen;
CREATE EXTENSION thinkthen;
```

The new extension lives in the `thinkthen` schema. Your own tables and their rows are untouched, and dumps of the database restore.

A view or a column that uses the old functions or types blocks the `DROP`. Drop it yourself, and make it again afterward on the new functions. Never add `CASCADE`, which drops them without asking.

Then rewrite each call. The 0.1 functions took one text and gave back one value. Each of the ten now takes a query and gives back rows:

| 0.1 | 0.2 |
| --- | --- |
| `thinkthen_decide(question, text)` and the other `thinkthen_` functions in `public` | `thinkthen.decide(query, question)` and the other nine in `thinkthen`, with the records first and the question second |
| The `_many` functions, such as `thinkthen_decide_many` | The same function, over a query of many rows |
| `thinkthen_details` and `thinkthen_try_details` | The `probability`, `status`, error and `detail` columns on every row |
| A model, a batch size or other settings passed in the call | Session settings, as in [Settings](#settings) |
| `thinkthen_usage()` and `thinkthen_plan` | Gone, with nothing in their place |

The extension in the published 0.1.2 source already refuses `thinkthen_warm` and `thinkthen_probability` with migration advice; they are not working old calls to preserve.

---

# DuckDB

The DuckDB extension adds the ten functions to DuckDB as table functions. `thinkthen_decide` answers a yes-or-no question about each text in a query: `true` for yes, `false` for no, or NULL with the status `unsure` when it is not sure.

## Install

Download the DuckDB archive for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0), [verify it](https://thinkthen.dev/install/#download-a-release-file), and extract it into a folder of its own. Start DuckDB 2.0.0 in that folder:

```sh
mkdir thinkthen-duckdb
tar -xzf thinkthen-duckdb-*.tar.gz -C thinkthen-duckdb
cd thinkthen-duckdb
duckdb -unsigned
```

The extension is not signed, so `-unsigned` lets DuckDB load it; load only a file you checked. DuckDB loads an extension only into the version it was built for. The extension reads your key from `THINKTHEN_API_KEY` in DuckDB's environment. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Load the extension from the folder you downloaded it to, make a table of support messages, and ask whether the first three request a refund:

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
FROM thinkthen_decide(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'Does the customer request a refund?',
  record := 'body');
```

Output:

```text
┌────────┬───────────┬──────────────────┐
│ msg_id │ is_refund │      status      │
│ int32  │  boolean  │ thinkthen_status │
├────────┼───────────┼──────────────────┤
│      1 │ true      │ answered         │
│      2 │ false     │ answered         │
│      3 │ true      │ answered         │
└────────┴───────────┴──────────────────┘
```

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. A not-sure answer is NULL with the status `unsure`, and a message ThinkThen could not judge is NULL with the status `failed` and an `error_code`, so neither one passes for `false`. The query keeps going either way. Send both to a person. In the same session:

```sql
SELECT msg_id, answer AS is_refund, status
FROM thinkthen_decide(
  (SELECT msg_id, body FROM messages WHERE msg_id <= 3),
  'Does the customer request a refund?',
  record := 'body', low := 0.2, high := 0.8);
```

Output:

```text
┌────────┬───────────┬──────────────────┐
│ msg_id │ is_refund │      status      │
│ int32  │  boolean  │ thinkthen_status │
├────────┼───────────┼──────────────────┤
│      1 │ true      │ answered         │
│      2 │ false     │ answered         │
│      3 │ NULL      │ unsure           │
└────────┴───────────┴──────────────────┘
```

Test the answer with `IS TRUE` and `IS FALSE`, never with `coalesce(is_refund, false)`, which turns not sure and failures into no.

## Next

The [DuckDB page](https://thinkthen.dev/sdks/duckdb/) shows all ten functions. The [DuckDB guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/duckdb.md) covers every option, column and setting.

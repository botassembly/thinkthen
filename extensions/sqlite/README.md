# SQLite

The SQLite extension adds the ten functions to SQLite as table-valued functions. `thinkthen_decide` answers a yes-or-no question about each text in a query: 1 for yes, 0 for no, or NULL with the status `unsure` when it is not sure.

## Install

Download the SQLite archive for your platform from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0), [verify it](https://thinkthen.dev/install/#verify-a-download), and extract it into a folder of its own. Start `sqlite3` in that folder:

```sh
mkdir thinkthen-sqlite
tar -xzf thinkthen-sqlite-*.tar.gz -C thinkthen-sqlite
cd thinkthen-sqlite
sqlite3
```

The extension needs SQLite 3.41.0 or later with extension loading on, which the `sqlite3` shell has. It reads your key from `THINKTHEN_API_KEY` in SQLite's environment. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Load the extension, make a table of support messages, and ask whether the first three request a refund:

```sql
.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
.mode box
CREATE TABLE messages (
  msg_id INTEGER PRIMARY KEY, body TEXT);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.'),
  (4, 'I was charged twice and my parcel is late again.'),
  (5, 'Morgan works for Example Group.
Please refund my order.'),
  (6, 'Morgan works for Example Group.');

SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'Does the customer request a refund?');
```

Output:

The source query names the text `record` and the key `id`, and `id` comes back with your key's type, so it joins straight back to `messages`.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. A not-sure answer is NULL with the status `unsure`, and a message ThinkThen could not judge is NULL with the status `failed` and an `error_code`, so neither one passes for 0. The query keeps going either way. Send both to a person. In the same session:

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen_decide(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'Does the customer request a refund?')
WHERE low = 0.2 AND high = 0.8;
```

Output:

Test the answer with `IS TRUE` and `IS FALSE`, never with `coalesce(is_refund, 0)`, which turns not sure and failures into no.

## Next

The [SQLite page](https://thinkthen.dev/sdks/sqlite/) shows all ten functions. The [SQLite guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/sqlite.md) covers every option, column and setting.

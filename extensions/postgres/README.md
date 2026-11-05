# PostgreSQL

The PostgreSQL extension adds the ten functions to PostgreSQL 18, in the `thinkthen` schema. `thinkthen.decide` answers a yes-or-no question about each text in a query: `t` for yes, `f` for no, or NULL with the status `unsure` when it is not sure.

## Install

The extension runs on PostgreSQL 18, on Linux x86-64 or arm64 with glibc 2.28 or later. Download the PostgreSQL 18 archive for your server from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0), [verify it](https://thinkthen.dev/install/#download-a-release-file), and copy its files into the server's folders:

```sh
mkdir thinkthen-postgres
tar -xzf thinkthen-postgres-*.tar.gz -C thinkthen-postgres
cd thinkthen-postgres
sudo cp lib/thinkthen_postgres.so \
  "$(pg_config --pkglibdir)/"
sudo cp share/extension/* \
  "$(pg_config --sharedir)/extension/"
```

Set `THINKTHEN_API_KEY` in the server's environment and restart the server; SQL can never set the key. [Configuration](https://thinkthen.dev/configuration/) shows how to get one. `CREATE EXTENSION` revokes the functions from `PUBLIC`, so grant each application role `USAGE` on schema `thinkthen` and `EXECUTE` on the functions it calls.

## Ask a question

Connect with `psql` as an administrator. Create the extension, make a table of support messages, and ask whether the first three request a refund:

```sql
CREATE SCHEMA thinkthen;
CREATE EXTENSION thinkthen SCHEMA thinkthen;
CREATE TABLE messages (
  msg_id integer PRIMARY KEY, body text);
INSERT INTO messages VALUES
  (1, 'Please refund my order.'),
  (2, 'Where is my parcel?'),
  (3, 'I was charged twice for one invoice.'),
  (4, 'I was charged twice and my parcel is late again.'),
  (5, 'Morgan works for Example Group.
Please refund my order.'),
  (6, 'Morgan works for Example Group.');

SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen.decide(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'Does the customer request a refund?');
```

Output:

```text
CREATE SCHEMA
CREATE EXTENSION
CREATE TABLE
INSERT 0 6
 msg_id | is_refund |  status  
--------+-----------+----------
 1      | t         | answered
 2      | f         | answered
 3      | t         | answered
(3 rows)
```

`id` comes back as `jsonb`, so `(id)::integer` joins it back to `messages`.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. A not-sure answer is NULL with the status `unsure`, and a message ThinkThen could not judge is NULL with the status `failed` and an `error_code`, so neither one passes for `f`. The query keeps going either way. Send both to a person. In the same database:

```sql
SELECT id AS msg_id, answer AS is_refund, status
FROM thinkthen.decide(
  'SELECT msg_id AS id, body AS record FROM messages
   WHERE msg_id <= 3',
  'Does the customer request a refund?',
  low => 0.2, high => 0.8);
```

Output:

```text
 msg_id | is_refund |  status  
--------+-----------+----------
 1      | t         | answered
 2      | f         | answered
 3      |           | unsure
(3 rows)
```

Test the answer with `IS TRUE` and `IS FALSE`, never with `coalesce(is_refund, false)`, which turns not sure and failures into no.

## Next

The [PostgreSQL page](https://thinkthen.dev/sdks/postgresql/) shows all ten functions. The [PostgreSQL guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/postgresql.md) covers every option, column, setting and error, and moving from a 0.1 test build.

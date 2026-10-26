# ThinkThen for SQLite

Download the matching SQLite archive from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) and extract it. Use SQLite 3.41.0 or later with extension loading enabled. Run this example in the SQLite shell from the extracted directory.

The example uses the stub backend and needs no key or service.

```sql
.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init
SELECT thinkthen_set('backend', 'stub');
SELECT thinkthen_set('batch', 16);

CREATE TABLE documents(id INTEGER PRIMARY KEY, tenant INTEGER, body TEXT NOT NULL);
INSERT INTO documents VALUES
  (1, 7, 'Please refund the extra charge.'),
  (2, 7, 'Thanks for the quick help.');

SELECT position, status, answer AS refund, probability
FROM thinkthen_decide(
  'SELECT body AS record FROM documents ORDER BY id',
  'Does the message request a refund?');
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/extensions/sqlite/src/extension.c).

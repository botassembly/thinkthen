# ThinkThen for DuckDB

Download the matching DuckDB archive from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) and extract it. Use exactly DuckDB 1.4.3. Start the shell with `duckdb -unsigned`, then load the extension by its absolute path.

The example uses the stub backend and needs no key or service.

```sql
LOAD '/absolute/path/to/thinkthen.duckdb_extension';
SET thinkthen_backend = 'stub';
SET thinkthen_batch = 16;

CREATE TABLE documents(id INTEGER, body VARCHAR);
INSERT INTO documents VALUES
    (1, 'Please refund this invoice.'),
    (2, 'The replacement arrived today.');

SELECT position, status, answer, probability
FROM thinkthen_decide(
    (SELECT body AS record FROM documents ORDER BY id),
    'Does the message request a refund?');
```

SQL cannot set `max_retries` above the engine default.

DuckDB can buffer one native input vector while a backend request is held. This host buffer is separate from the engine iterator’s read-ahead bound.

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/databases/duckdb/src/extension.cpp).

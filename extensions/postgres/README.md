# ThinkThen for PostgreSQL

Download the PostgreSQL 18 archive for your server architecture from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0) and extract it. Copy `lib/thinkthen_postgres.so` into `pg_config --pkglibdir` and `share/extension/*` into the `extension` directory under `pg_config --sharedir`. Run the example as an administrator. Grant application roles schema USAGE and EXECUTE on the functions they need.

The example uses the stub backend and needs no key or service.

```sql
CREATE SCHEMA thinkthen;
CREATE EXTENSION thinkthen SCHEMA thinkthen;
SET thinkthen.allowed_backends = 'stub:*';
SET thinkthen.backend = 'stub';

CREATE TABLE documents(id bigint PRIMARY KEY, body text NOT NULL);
INSERT INTO documents VALUES
  (1, 'Please refund the extra charge.'),
  (2, 'The meeting is on Monday.');

SELECT * FROM thinkthen.decide(
  'SELECT body AS record FROM documents ORDER BY id',
  'Does the message request a refund?');
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/extensions/postgres/thinkthen--0.2.0.sql).

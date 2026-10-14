# ThinkThen for DuckDB

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Use DuckDB 1.5.6 and 2.0 with the matching source host and archive from the [release downloads](https://github.com/botassembly/thinkthen/releases). Unpack `thinkthen-duckdb-0.2.0-x86_64-unknown-linux-gnu.tar.gz` and start `duckdb -unsigned`. Load the member for your host and platform; Linux x86_64 on 1.5.6 uses:

```sql
LOAD 'v1.5.6/linux_amd64/thinkthen.duckdb_extension';
```

For 2.0 on Linux x86_64, load `d4355d01c5/linux_amd64/thinkthen.duckdb_extension` with its matching [source host](https://github.com/duckdb/duckdb/tree/d4355d01c59362d57036ad377735dd5340276a5b). The [1.5.6 source host](https://github.com/duckdb/duckdb/tree/069cc9f9b5be802405797faecc284961b07c70ef) matches the first member.

Set your hosted backend key before starting DuckDB: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```sql
CREATE TABLE tickets AS FROM (VALUES
    (1, 'My parcel is a week late. Not okay.'),
    (2, 'Thanks for the quick help yesterday!'),
    (3, 'We are locked out of our account.'),
    (4, 'I was billed twice for one order.')
) t(id, body);
SELECT d.ordinal + 1 AS id, d.value AS is_complaint,
       d.probability
FROM (SELECT unnest(thinkthen_decide_details(
    'Is this a complaint?',
    (SELECT list(thinkthen_record(text := body)
                 ORDER BY id) FROM tickets))) AS d);
SELECT d.ordinal + 1 AS id, d.value AS team, d.probability
FROM (SELECT unnest(thinkthen_choose_details(
    {'choose': 'Which team owns this?',
     'options': ['billing', 'shipping', 'account']},
    (SELECT list(thinkthen_record(text := body)
                 ORDER BY id) FROM tickets))) AS d);
```

The extension is free and MIT licensed. Hosted models charge per token. Local models are free to use on your own machine.

[Backend setup](https://thinkthen.dev/models/) · [SQL reference](https://thinkthen.dev/databases/duckdb/)

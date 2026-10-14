# ThinkThen for DuckDB

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Use the matching source-built DuckDB host: [1.5.6](https://github.com/duckdb/duckdb/tree/069cc9f9b5be802405797faecc284961b07c70ef) or [2.0-cyanoptera preview](https://github.com/duckdb/duckdb/tree/d4355d01c59362d57036ad377735dd5340276a5b). Community installation is pending.
Unpack `thinkthen-duckdb-0.2.0-x86_64-unknown-linux-gnu.tar.gz` and start the matching host with `duckdb -unsigned`. For the Linux x86_64 1.5.6 host:

```sql
LOAD 'v1.5.6/linux_amd64/thinkthen.duckdb_extension';
```

For the pinned preview host, load `d4355d01c5/linux_amd64/thinkthen.duckdb_extension`. Other platforms need their matching archive and member.

Set your hosted backend key before starting DuckDB: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```sql
WITH refunds(body) AS (VALUES
    ('Please refund my order. It arrived broken.'),
    ('Thanks for the quick help yesterday!'), ('I want to send this back.'))
SELECT body, thinkthen_decide('Does the customer ask for a refund?', body) AS refund
FROM refunds;
WITH messages(body) AS (VALUES
    ('Please refund the extra fee on my invoice.'),
    ('My parcel went to the wrong address.'), ('I cannot reset my password.'))
SELECT body, thinkthen_choose(
    '{"choose":"Which team owns this?","options":{"billing":"Invoices, fees, and refunds.","shipping":"Parcels and delivery.","account":"Logins and passwords."}}',
    body) AS team FROM messages;
```

The extension is free and MIT licensed. Hosted models charge per token. Local models are free to use on your own machine.

[Backend setup](https://thinkthen.dev/install/backends/) · [SQL reference](https://thinkthen.dev/install/duckdb/)

# ThinkThen for DuckDB

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Install from the DuckDB community repository for DuckDB 1.5.6 and 2.0:

```sql
INSTALL thinkthen FROM community;
LOAD thinkthen;
```

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

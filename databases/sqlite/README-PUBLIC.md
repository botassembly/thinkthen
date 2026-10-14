# ThinkThen for SQLite

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Before starting SQLite, set your hosted backend key in the shell: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

Download and unpack the release archive for your platform. Enable extension loading in your SQLite host and load the shared library:

```sql
.load ./libthinkthen0
```

Read each typed value and its probability from the same row:

```sql
WITH refunds(body) AS (VALUES
    ('Please refund my order. It arrived broken.'),
    ('Thanks for the quick help yesterday!'),
    ('I want to send this back.'))
SELECT body, refund.value AS is_refund, refund.probability AS p
FROM refunds JOIN thinkthen_decide_details(
    'Does the customer ask for a refund?', body) AS refund;
WITH messages(body) AS (VALUES
    ('Please refund the extra fee on my invoice.'),
    ('My parcel went to the wrong address.'),
    ('I cannot reset my password.'))
SELECT body, owner.value AS team, owner.probability AS p
FROM messages JOIN thinkthen_choose_details(
    '{"choose":"Which team owns this?","options":{"billing":"Invoices, fees, and refunds.","shipping":"Parcels and delivery.","account":"Logins and passwords."}}',
    body) AS owner;
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/databases/sqlite/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

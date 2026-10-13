# ThinkThen for PostgreSQL

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

Download and unpack the release archive for your platform. Install the extension files into your PostgreSQL 16 server, then enable the extension as an administrator:

```sql
CREATE EXTENSION thinkthen;
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. An administrator must grant the application role access to the ThinkThen functions.

```sql
SELECT thinkthen_decide('Does this ask for a refund?', 'Please refund my order.');
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

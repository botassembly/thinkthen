# ThinkThen for DuckDB

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

Install from the DuckDB community repository for DuckDB 1.5.6 and 2.0:

```sql
INSTALL thinkthen FROM community;
LOAD thinkthen;
```

Set your hosted backend key before starting DuckDB: `export THINKTHEN_API_KEY=...`.

```sql
SELECT thinkthen_decide('Does the writer ask for a refund?', 'Please refund my order.');
```

The extension is free and MIT licensed. Hosted models charge per token. Local models are free to use on your own machine.

[Backend setup](https://thinkthen.dev/install/backends/) · [SQL reference](https://thinkthen.dev/install/duckdb/)

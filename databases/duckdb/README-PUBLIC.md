# ThinkThen for DuckDB

ThinkThen answers questions about text in SQL. Use it to find messages that ask for a refund, label support requests or score answers against a rubric. Functions return SQL values such as booleans, labels and numbers.

ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

Install from the DuckDB community repository:

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

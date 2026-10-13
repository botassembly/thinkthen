# Draft DuckDB README for approval

# ThinkThen

ThinkThen answers questions about text in SQL. Use it to find messages that ask for a refund, label support requests or score answers against a rubric. Functions return SQL values such as booleans, labels and numbers.

The extension is free and MIT licensed. Hosted models charge per token. Local models are free to use on your own machine.

This example previews one request. It needs no key and makes no model call:

```sql
SELECT (thinkthen_plan(
  'Does the writer ask for a refund?',
  '{"7":"Please refund my order."}',
  '{}'
)).records;
-- 1
```

To judge text, select a backend and model first. A hosted call sends the question and text to that backend. See the [backend setup guide](https://thinkthen.dev/install/backends/) and [SQL reference](https://github.com/botassembly/thinkthen/tree/main/databases/duckdb) for settings and functions.

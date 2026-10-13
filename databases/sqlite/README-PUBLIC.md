# ThinkThen for SQLite

ThinkThen answers typed questions about text inside SQL. Ask whether a message requests a refund, label records, or score answers. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

Download and unpack the release archive for your platform. Enable extension loading in your SQLite host and load the shared library:

```sql
.load ./libthinkthen0
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```sql
SELECT thinkthen_decide('Does this ask for a refund?', 'Please refund my order.');
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

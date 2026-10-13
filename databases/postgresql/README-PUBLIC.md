# ThinkThen for PostgreSQL

ThinkThen answers typed questions about text inside SQL. Ask whether a message requests a refund, label records, or score answers. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

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

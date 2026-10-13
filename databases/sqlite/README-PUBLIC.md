# ThinkThen for sqlite

ThinkThen answers typed questions about text inside SQL. Ask whether a message requests a refund, label records, or score answers. Failed calls remain separate from answers.

## Install

Download and unpack the release archive for your platform. Enable extension loading in your SQLite host and load the shared library:

```sql
.load ./libthinkthen0
```

## Example

Configure a model backend before running this query.

```sql
SELECT thinkthen_decide('Does this ask for a refund?', 'Please refund my order.');
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

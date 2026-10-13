# ThinkThen for postgresql

ThinkThen answers typed questions about text inside SQL. Ask whether a message requests a refund, label records, or score answers. Failed calls remain separate from answers.

## Install

Download and unpack the release archive for your platform. Install the extension files into your PostgreSQL 16 server, then enable the extension as an administrator:

```sql
CREATE EXTENSION thinkthen;
```

## Example

Configure a model backend before running this query. An administrator must grant the application role access to the ThinkThen functions.

```sql
SELECT thinkthen_decide('Does this ask for a refund?', 'Please refund my order.');
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

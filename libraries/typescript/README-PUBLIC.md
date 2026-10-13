# ThinkThen for typescript

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```sh
npm install thinkthen
```

## Example

Configure a model backend before running this example.

```typescript
import {Client} from 'thinkthen';

const client = new Client({cache: false});
try {
  const call = await client.decide('Does this ask for a refund?', 'Please refund my order.');
  console.log(call.results[0].value);
} finally { client.close(); }
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

# ThinkThen for TypeScript

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

```sh
npm install thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```typescript
import {Client} from 'thinkthen';

const client = new Client();
try {
  const call = await client.decide('Does this ask for a refund?', 'Please refund my order.');
  for (const answer of call.results) console.log(answer.value);
} finally { client.close(); }
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

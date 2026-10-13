# ThinkThen for TypeScript

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

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

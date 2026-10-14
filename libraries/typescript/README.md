# ThinkThen for TypeScript

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
npm install thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```typescript
import {Client} from 'thinkthen';
const refunds = ['Please refund my order. It arrived broken.', 'Thanks for the quick help yesterday!', 'I want to send this back.'];
const messages = ['Please refund the extra fee on my invoice.', 'My parcel went to the wrong address.', 'I cannot reset my password.'];
const teams = {billing: 'Invoices, fees, and refunds.', shipping: 'Parcels and delivery.', account: 'Logins and passwords.'};
const client = new Client();
try {
  for (const text of refunds) {
    const refund = await client.decideAnswer('Does the customer ask for a refund?', text);
    console.log(refund);
  }
  for (const text of messages) {
    const team = await client.chooseAnswer({choose: 'Which team owns this?', options: teams}, text);
    console.log(team);
  }
} finally { client.close(); }
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

# TypeScript SDK

The TypeScript SDK asks questions about text from JavaScript and TypeScript. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Install it in a Node.js 22.12 or later project:

```sh
npm install thinkthen
```

It runs on Linux (x64 or Arm64, with glibc 2.28 or later), macOS on Apple silicon and Windows x64, and carries its TypeScript types. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund. Save it as `first.mjs` and run `node first.mjs`:

```js
import { ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  // await throws UnsureError when not sure, RecordError on failure.
  const isRefund = await tt.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
  );
  console.log(isRefund);
} finally {
  await tt.close();
}
```

Output:

```text
true
```

`true` means yes: the message asks for a refund.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. A not-sure answer throws `UnsureError`, and a message ThinkThen could not judge throws `RecordError`, so neither one passes for no. Send both to a person.

```js
import { ThinkThen, UnsureError } from "thinkthen";

const tt = new ThinkThen();
try {
  const isRefund = await tt.decide(
    "Does the customer request a refund?",
    "I was charged twice for one invoice.",
    { threshold: { low: 0.2, high: 0.8 } },
  );
  console.log(isRefund ? "Send to refunds." : "Leave in the inbox.");
} catch (error) {
  if (!(error instanceof UnsureError)) throw error;
  console.log("Not sure. Ask a person.");
} finally {
  await tt.close();
}
```

Output:

```text
Not sure. Ask a person.
```

## Next

The [TypeScript SDK page](https://thinkthen.dev/sdks/typescript/) shows all ten functions. The [TypeScript guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/typescript.md) and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/typescript/index.d.ts) cover every option.

# ThinkThen for JavaScript and TypeScript

Run `npm install thinkthen` in a Node.js ESM project. The package includes the native engine and TypeScript declarations.

The example uses the stub backend and needs no key or service.

```ts
import { ThinkThen } from "thinkthen";

const client = new ThinkThen({ backend: "stub" });

const row = await client.decide(
  "Does the customer ask for money back?",
  "Please refund my order.",
  { threshold: 0.8 },
);

if (row.status === "answered") {
  console.log(row.answer); // boolean
} else if (row.status === "unsure") {
  console.log("Not sure");
} else {
  console.error(row.error.code, row.error.message);
}
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/typescript/index.d.ts).

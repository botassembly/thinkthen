# The TypeScript surface

Lands in Phase B. The acceptance sample, drawn in
the product deck's surfaces page, runs
as drawn against the stand-in when this folder holds the shim:

```ts
import * as tt from "thinkthen";

await tt.decide("Does the customer ask for a refund?", text);  // true
const refund = tt.question({ decide: "...", threshold: [0.2, 0.8] });
await tt.decide(refund, "I was charged twice. Can you fix this?");  // null
const complaints = await tt.filter("Is this a complaint?", reviews);
const rows = await tt.annotate("form.json", tickets, { signal });
```

`null` is "not sure". An `AbortSignal` cancels a batch: the binding's brief
item 7 proves it against the stand-in. The bulk spelling is `decide_many`.
The width — the engine value's `width`, or `ENGINE_WIDTH` — is the number
of requests in flight, and each in-flight request holds its own connection:
1,000 records at width 32 measured 33 pooled connections.

Each call holds one worker of Node's libuv thread pool while it runs, and
file reads, `dns.lookup`, and `crypto` share that pool. The pool holds
`UV_THREADPOOL_SIZE` workers, 4 by default. A program that runs many
calls at once raises it before Node starts (`DIVERGENCES.md`, item 9).

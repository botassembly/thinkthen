# The TypeScript surface

Lands in Phase B. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
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

# ThinkThen for JavaScript and TypeScript

ThinkThen judges evidence by meaning. Its ten functions return typed result rows through promises and async iteration. Import the ESM package in Node.js. Type declarations ship with the package.

```sh
npm install thinkthen
```

Set `THINKTHEN_API_KEY` in your shell, then call a function:

```ts
import { decide } from "thinkthen";

const row = await decide(
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

The same code runs in JavaScript. TypeScript narrows the readonly row by `status`. An unsure row has a null answer. A failed row has an error with the engine's stable `code` and message. The optional `detail` field is plain JSON. Probabilities use the engine's output precision; scores retain their precision.

## Functions

`decide`, `choose`, `tag`, `score`, `find`, `annotate`, `recognize` and `relate` accept one record and return a promise for one row. Their stream forms append `Stream` to the name and accept an iterable or async iterable of records. `filterStream` and `rankStream` operate on record sequences. Stream forms return async iterables; `filterStream` yields original records, and the others yield rows. Question settings go in the final options object.

```ts
import {
  decide, choose, tag, score, filterStream, rankStream, find,
  annotate, recognize, relate,
} from "thinkthen";

const text = "Ada works for Acme. Please refund my order.";

await decide("Is a refund requested?", text);
await choose("What is the topic?", text, { options: ["billing", "shipping"] });
await tag("Which topics apply?", text, { labels: ["refund", "delivery"] });
await score("How urgent is it?", text, { levels: ["routine", "urgent"] });
await find("Who works for Acme?", text);
await recognize(["person", "company"], text);
await relate([{ name: "works_for", source: "person", target: "company" }], text);
await annotate({
  questions: {
    refund: { decide: "Is a refund requested?" },
    topic: { choose: "What is the topic?", options: ["billing", "shipping"] },
  },
}, text);

for await (const record of filterStream("Is a refund requested?", [text])) {
  console.log(record); // the original string
}

for await (const row of rankStream("How urgent is it?", ["Routine update", "Urgent help needed"])) {
  if (row.status === "answered") console.log(row.id, row.answer); // position
}
```

`filterStream` yields the original records the engine keeps, unchanged and in input order. It preserves each record's TypeScript type, yields all kept records, and raises the first engine failure after exhausting the input. `rankStream` emits rows in rank order, preserves input order for ties, and raises the first engine failure after exhausting the input and yielding its rows. Other row streams emit rows in input order. A `top` option limits retained ranked records. `find` returns a text span or null; an empty result can be an answered row. Answer span offsets count UTF-16 code units, with an exclusive end, so `text.slice(start, end)` extracts the span. Rust selects UTF-16 offsets for answers and detail before handing rows to JavaScript.

`recognize([], text)` recognizes names without a kind list. Relation rules may also set `reads` and `either`. Annotate accepts a plain toolbox object. Question strings stay literal, including strings ending in `.json`. The library reads no question files. To select a named in-memory question, pass `{ type: "toolbox", value: toolbox, name: "refund" }` in place of its text, kinds or rules. Model and batch settings belong to the client; call options set thresholds and evidence selection.

## Record streams

Strings are text records. All other JSON values are JSON records, including arrays, numbers, booleans and null. Select evidence with the `on` JSON Pointer. An object's `id` passes through unchanged; the engine supplies an ID when one is absent. Single-record functions always treat an array as one JSON record; stream functions treat their input as a sequence of records. Pass a sequence directly to a stream function without collecting an async source into an array.

```ts
import { ThinkThen } from "thinkthen";

const client = new ThinkThen({ batch: 32 });

async function* messages() {
  yield { id: "a1", body: "Please refund my order." };
  yield { id: "a2", body: "Thank you for the delivery." };
}

for await (const row of client.decideStream("Is a refund requested?", messages(), {
  on: "/body",
})) {
  console.log(row.id, row.status);
}
```

One stream uses one engine operation. The engine owns batching, retries and ordering. Backend work runs off the JavaScript thread. Streams pull input with bounded read-ahead rather than collecting it before sending. Ranking reads the input before emitting ranked rows; `top` limits the retained results. Row streams deliver every row, including failed rows, then raise the first engine failure. Filters deliver every kept record before that terminal error. An invalid call rejects its promise or throws during iteration as a `ThinkThenError` subclass. A source iterator's own exception propagates unchanged and cancels the operation. Leaving a loop early cancels the operation and calls the input iterator's `return()` when present. Engine errors and aborts also close the input iterator. Cleanup never replaces the error that caused it.

## Errors and cancellation

```ts
import { decide, ThinkThenError } from "thinkthen";

const controller = new AbortController();
const pending = decide("Is a refund requested?", "Please refund my order.", {
  signal: controller.signal,
});
controller.abort();

try {
  await pending;
} catch (error) {
  if (controller.signal.aborted && error === controller.signal.reason) {
    console.log("Cancelled");
  } else if (error instanceof ThinkThenError) {
    console.error(error.code, error.message);
  } else {
    throw error;
  }
}
```

Engine call failures use `UsageError`, `BackendError` or `LocalError`, all extending `ThinkThenError`, with the engine's code and message. Aborting rejects the outstanding promise or fails iteration with `signal.reason`. The default reason is a `DOMException` named `AbortError`; a reason passed to `controller.abort(reason)` is preserved unchanged. Cancellation stops new sends and retry waits while open requests finish at their reply or attempt timeout. Rows already delivered remain valid. An already aborted signal rejects the promise or fails the first iteration without sending any requests.

## Settings

Options use ordinary JavaScript values: `threshold` is a number or a band string such as `"0.2:0.8"`, `on` is a JSON Pointer string. Function-specific options are `options`, `labels`, `levels` and rank's `top`. `new ThinkThen({ mode, adapter, url, keySource, model, batch })` selects the backend and batch settings once and reuses its connections. The client exposes the same functions and stream methods as the module. Module functions share one lazily created default client; final call options contain only question settings and `signal`. Omitted settings use the engine's defaults and environment rules. The engine parses and validates every setting.

The default System One adapter reads `THINKTHEN_API_KEY`; the OpenAI adapter reads `OPENAI_API_KEY`. Keys stay in the environment and never appear in a result row.

The complete public declarations are in [index.d.ts](index.d.ts).

Rows decode `detail` when the caller first reads it. Detail remains an ordinary JSON value. String offsets in answers and detail use the language’s own string indexing. Single-record calls raise the library error when the record fails; streams deliver failed rows before raising their terminal error. Filters yield their passing records before raising the first failure.

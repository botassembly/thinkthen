# ThinkThen in JavaScript and TypeScript

ThinkThen judges evidence with ten functions. Use Node.js 22.12 or later and an ESM project. Install the supplied 0.2.0 package archive; replace its path with the supplied filename:

Run `npm init -y`, then `npm pkg set type=module`, then `npm install /path/to/thinkthen-0.2.0.tgz`.

The package includes the native engine and TypeScript declarations. Once 0.2.0 is published, `npm install thinkthen@0.2.0` installs it from the package index. The examples below also run as JavaScript in a `.mjs` file unless they use `as const` or are labeled TypeScript. Run JavaScript with `node example.mjs`. Compile TypeScript with your project's usual compiler and NodeNext module resolution.

Set `THINKTHEN_API_KEY` through your environment or secret manager before creating a normal client. Keep keys out of scripts, output and reports. `new ThinkThen()` selects the default backend. `new ThinkThen({backend: "stub"})` needs no key or network and produces deterministic synthetic answers for installation checks.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```js
import { readFile } from "node:fs/promises";
import { ThinkThen } from "thinkthen";

const records = (await readFile("records.jsonl", "utf8"))
  .split(/\r?\n/).filter(line => line.trim()).map(line => JSON.parse(line));
const client = new ThinkThen({ backend: "stub" });
try {
  const array = await client.decide("Does the customer request a refund?", ["first", "second"]);
  console.log("array", array.status);
  for await (const row of client.decideStream("Does the customer request a refund?", records, { evidence: "/text" })) {
    const message = row.status === "failed" ? row.error.message : "";
    console.log(row.position, row.status, row.status === "answered" ? row.answer : null, message);
  }
} finally {
  await client.close();
}
```

`decide`, `choose`, `tag`, `score`, `find`, `recognize`, `relate` and `annotate` return promises for one row. Their `Stream` counterparts, `filterStream` and `rankStream` return async iterators for many records. Await single-record calls. Rows need no closing. `for await` closes a stream after completion or an early break. Close manual iteration with `await stream.return()` in `finally`.

Use a normal client for meaningful judgments. Reuse a client and close it in `finally`. Client settings include `apiKey`, `batch`, and `retry` with `maxRetries`, `initialBackoffMs` and `attemptTimeoutMs`. Prefer environment keys to literal `apiKey` values.

## Records and rows

A record is a string or JSON-compatible value. An array passed to a single-record call is one JSON-array record. Object records can carry a string, safe integer or bigint `id`. Use bigint for IDs outside JavaScript's safe integer range. Other fields must be JSON-compatible. The SDK preserves bigint IDs, so ordinary `JSON.stringify` needs your own bigint conversion if you export such rows.

Use `evidence: "/text"` or `evidence: ["text"]` to select a field. The string form is a JSON Pointer: `/` separates keys, `~1` means a literal slash, and `~0` means a literal tilde. A key path can mix strings and array indexes. Without a selector, ThinkThen uses the whole record.

```js
const row = await client.decide(
  "Does the customer request a refund?",
  { id: "a1", text: "Please refund my order." },
  { evidence: ["text"], threshold: { low: 0.2, high: 0.8 } },
);
if (row.status === "answered") {
  console.log(row.id, row.answer, row.probability);
} else if (row.status === "unsure") {
  console.log("unsure", row.probability);
} else {
  throw new Error(`${row.error.code}: ${row.error.message}`);
}
```

An evidence key path selects one nested value; `["subject", "body"]` means a body inside subject, not two sibling fields. To judge several fields together, use `{id: "a1", evidence: {subject: "Invoice refund", body: "Please refund my order."}}` and `evidence: ["evidence"]`. The selected object contains both fields, and the outer ID remains on the result.

Rows carry `function`, `position`, optional `id`, `source` and a discriminated `status`. Positions start at 1. An answered row has `answer`; an unsure row has no answer; a failed row has `error`. Narrow on status before accessing these fields. `probability` and `detail` appear where the function supplies them. `source` identifies the exact question execution. Types are readonly.

A rank row's `answer` is its rank and `position` is its input order.

A numeric decide or filter threshold yields true at or above the cut and false below it. `{low, high}` yields false below the lower bound, unsure inside the band and true at or above the upper bound. Choose accepts the same forms but yields unsure unless the winning choice reaches the upper cut. Tag, find, recognize and relate accept only numeric cuts.

Preserve the original ID when judging one text. Pass `{id: originalId, text}` with `evidence: ["text"]`, just as you do in a batch. Passing only the text string loses its ID. Keep IDs as strings when exporting evidence for another program; JSON has no bigint type. Positions identify input order within one call and cannot replace source IDs across calls or ranks.

The following application helper stops on a failed row or a failed annotation entry. It returns unsure outcomes for the caller to report. Define it once before the later examples:

```js
function stopOnFailure(row) {
  if (row.status === "failed") {
    throw new Error(`${row.error.code}: ${row.error.message}`);
  }
  if (row.function === "annotate" && row.status === "answered") {
    for (const outcome of Object.values(row.answer)) {
      if (outcome.status === "failed") {
        throw new Error(`${outcome.error.code}: ${outcome.error.message}`);
      }
    }
  }
  return row;
}
```

## The ten functions

Each call below invokes a separate function. List options belong to the function call. TypeScript preserves literal choices when you use `as const`.

```js
import { ThinkThen, defineToolbox } from "thinkthen";

const text = "Morgan works for Example Group.\nPlease refund my order.";
const record = { id: "a1", text };
const evidence = { evidence: ["text"] };
const records = [record, { id: "a2", text: "Where is my parcel?" }];
const rules = [{ name: "works_for", source: "person", target: "organization" }];
const questions = defineToolbox({
  refund: { function: "decide", question: "Does the customer request a refund?" },
  topic: { function: "choose", question: "What is the message about?", choices: ["billing", "shipping", "other"] },
  urgency: { function: "score", question: "How urgent is the request?", levels: ["routine", "urgent"] },
});
const client = new ThinkThen();
try {
  const decision = stopOnFailure(await client.decide("Does the customer request a refund?", record, evidence));
  const choice = stopOnFailure(await client.choose("What is the message about?", record, { ...evidence, choices: ["billing", "shipping", "other"] }));
  const tags = stopOnFailure(await client.tag("Which topics appear?", record, { ...evidence, labels: ["billing", "shipping"] }));
  const score = stopOnFailure(await client.score("How urgent is the request?", record, { ...evidence, levels: ["routine", "urgent"] }));
  const found = stopOnFailure(await client.find("Which line states the requested action?", record, evidence));
  const names = stopOnFailure(await client.recognize(["person", "organization"], record, evidence));
  const relations = stopOnFailure(await client.relate(rules, record, evidence));
  const annotation = stopOnFailure(await client.annotate(questions, record, evidence));
  for await (const result of client.filterStream("Does the customer request a refund?", records, { evidence: ["text"] })) {
    stopOnFailure(result.row);
    console.log(result.item.id, result.kept, result.row.status);
  }
  for await (const row of client.rankStream("How urgent is the request?", records, { levels: ["routine", "urgent"], evidence: ["text"], top: 5 })) {
    stopOnFailure(row);
    if (row.status === "answered") console.log(row.id, row.answer);
  }
} finally {
  await client.close();
}
```

| Function | Answer when answered |
| --- | --- |
| decide | Boolean |
| filterStream | Each item has the original `item`, a boolean `kept`, and its outcome `row` |
| choose | One choice string |
| tag | Array of matching labels, possibly empty |
| score | Number from 0 to 1 on the ordered scale |
| rankStream | Rank starting at 1, highest score first |
| find | `{text, start, end}`, or `null` |
| recognize | Array of names with `text`, `start`, `end`, `kind` and `probability` |
| relate | Array of relations with `source`, `target`, `relation`, `probability` and optional `symmetric` |
| annotate | Object mapping question names to typed outcomes |

Choose requires distinct choices. Score and rank require distinct levels ordered from lowest to highest. Score is the probability-weighted scale position. Rank reads the whole collection before emitting ranked answers; `top` limits answered ranks and retains failed records. Find chooses one whole nonblank line, so preserve line breaks in evidence. Spans use JavaScript UTF-16 offsets with an inclusive start and exclusive end: `text.slice(span.start, span.end)`. Relation source and target are named spans. A rule can add `phrase` for wording and `symmetric: true` for an undirected relation.

Toolbox entries can contain every per-record function, including filter, find, recognize and relate. Rank cannot be an annotation entry. Inspect each nested outcome: an outer annotation row can be answered while an entry is failed or unsure. Entries can select their own evidence; otherwise they use the selector passed to annotate. `defineToolbox` takes a snapshot and preserves TypeScript inference. It does not execute shell commands.

You can pass a typed question object instead of a question string. For example, `client.decide(questions.refund, text)` uses a saved decide definition. Recognize objects use `{function: "recognize", kinds: [...]}` and relate objects use `{function: "relate", rules: [...]}`. Annotation takes the named questions object directly.

A saved filter definition has type `FilterQuestion`, with `function: "filter"`; a decide definition is a different function. This TypeScript example preserves the original item and live kept flag beside its row:

```typescript
import type { FilterQuestion } from "thinkthen";
const refundFilter: FilterQuestion = {
  function: "filter", question: "Does the customer request a refund?", evidence: ["text"],
};
const filterOutcomes = await client.filterStream(refundFilter, records).toArray();
for (const item of filterOutcomes) {
  console.log(item.row.id, item.kept, item.row.status);
}
```

Export the complete `FilterItem` if you need its `kept` flag and original `item`; exporting only `item.row` omits those wrapper fields. Keep false, unsure and failed outcomes in saved results and inspect statuses before starting another call.

Write an unambiguous question before defining a toolbox entry. Name the evidence field, allowed values and any date rule you intend; phrases such as “currently available” leave those choices implicit. Reuse that definition across single-record and stream calls.

## Streams, files and batching

Streams accept either an iterable or an async iterable. `.toArray()` collects the stream in memory.

```js
import { readFile, writeFile } from "node:fs/promises";

const records = (await readFile("messages.jsonl", "utf8"))
  .split(/\r?\n/).filter(line => line.trim()).map(line => JSON.parse(line));
const outcomes = [];
try {
  for await (const row of client.annotateStream(questions, records, { evidence: ["text"] })) {
    outcomes.push(row);
  }
} finally {
  const json = JSON.stringify(outcomes, (_key, value) =>
    typeof value === "bigint" ? value.toString() : value);
  await writeFile("outcomes.json", json + "\n");
}
console.log("rows:", outcomes.length, "IDs:", outcomes.map(row => row.id));
for (const row of outcomes) stopOnFailure(row);
```

Rows and nested outcomes are plain JSON-compatible objects, apart from bigint IDs. `JSON.stringify` preserves their statuses, IDs, answers, errors and full-precision probabilities. The replacer above exports bigint IDs as decimal strings. `JSON.parse` reads the exported evidence back; it does not reconstruct bigint values automatically. The `finally` block saves every consumed row, including its ID, even if a call-level error interrupts traversal. After saving, the example checks all outer and nested failures before later work starts. Do not replace IDs with enumeration indexes or save only positive answers. Do not replace structured answers with display strings or drop failed and unsure statuses from evidence.

Plan the work before making calls when your task has a call or request-body budget. Count input records and named questions, select only the evidence fields each question needs, and estimate how much text will repeat across questions. Use one `annotateStream` for questions over the same collection. Keep full text for find, recognize and relate when it is needed to answer the question. Save completed evidence and stop after a failure; do not spend the remaining budget retrying the same request without understanding its error. Task budgets come from the task and are separate from engine batching settings.

Use one stream over the whole collection to let the engine batch questions. Avoid a loop that awaits one call per record. Arrays supply finite collections; iterables supply records incrementally. `batch` controls request packing, and the engine also applies request size and backend limits. A row can require several backend questions, so batch size does not equal network request count.

The SDK does not parse CSV or XLSX itself. Read the supplied table with your application's table library, convert each data row to an object, and pass the objects to `annotateStream`. Keep a stable `id` to join results back to the original rows. Reuse the same toolbox and batch the collection. Export results with your table library. There is no SDK equivalent of a shell toolbox run.

Top-level exports such as `decide` and `annotateStream` use shared default settings. An explicit `ThinkThen` client gives your application clear resource ownership. Projects configured for explicit resource management can also use `await using` for clients and streams.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```js
import { readFile, writeFile } from "node:fs/promises";
import { ThinkThen, parseToolbox, serializeToolbox } from "thinkthen";

const box = parseToolbox(await readFile("toolbox.json", "utf8"));
await writeFile("toolbox-copy.json", serializeToolbox(box), "utf8");
const ready = box.ready;
const topic = box.topic;
if (ready?.function !== "filter" || topic?.function !== "choose") {
  throw new Error("toolbox needs ready/filter and topic/choose questions");
}
const client = new ThinkThen({ backend: "stub" });
try {
  const filtered = await client.filterStream(ready, records).toArray();
  for (const item of filtered) stopOnFailure(item.row);
  const kept = filtered.filter(item => item.kept).map(item => item.item);
  const rows = await client.annotateStream({ topic }, kept).toArray();
  for (const row of rows) {
    stopOnFailure(row);
    console.log(row.id, row.answer);
  }
} finally {
  await client.close();
}
```

`parseToolbox` returns a read-only named question map. Narrow a loaded entry's `function` discriminator before passing it to the matching method. Filter results carry `item`, `kept` and `row`; selecting kept items is ordinary JavaScript collection code. Annotation takes a named questions object and `annotateStream` processes its collection.

Call `relate` or `relateStream` alone when you need names and relations from the same evidence. After narrowing to a row with details, `row.detail.names` contains all admitted recognized names, including names with no returned relation. `row.detail.pairs` contains scored relation pairs and `row.answer` contains accepted relations. There is no need for a preceding recognize call.

## Failures and cancellation

A bad record or backend answer returns a failed row. Even a single-record call resolves to that row. Save returned rows for diagnosis, then use `stopOnFailure` before acting on answers or starting another call. A thrown error exits the loop, closes its stream and skips subsequent calls unless your application catches it. Preserve the error in your report and stop the workload when a task requires a stop after failure. Call-level problems reject promises or stream reads with `ThinkThenError` subclasses, including `UsageError`, `BackendError` and `CancelledError`. These expose a stable `code`, the library message and optional `httpStatus`. Your input iterator can also throw its own error.

Pass an `AbortSignal` to any call. `abort()` stops the operation and reading it rejects with `CancelledError`, whose code is `local.cancelled`. The abort reason becomes the error's `cause`.

```js
import { CancelledError } from "thinkthen";

const controller = new AbortController();
const onInterrupt = () => controller.abort();
process.once("SIGINT", onInterrupt);
try {
  for await (const row of client.decideStream("Does the customer request a refund?", records, {
    evidence: ["text"], signal: controller.signal,
  })) {
    stopOnFailure(row);
    console.log(row.id, row.status);
  }
} catch (error) {
  if (error instanceof CancelledError) console.error("cancelled", error.code);
  else throw error;
} finally {
  process.removeListener("SIGINT", onInterrupt);
}
```

Closing the client closes its outstanding streams. Results already consumed before cancellation remain in your application.

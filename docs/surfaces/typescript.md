# JavaScript and TypeScript functions that answer questions about text

ThinkThen is a classifier you call like a function: it reads text, answers one bounded question, and returns a typed value. `await` gives you the answer, ready for an `if`.

Run `npm install thinkthen@0.2.0` in a Node.js 22.12 or later project. The package runs on Linux (x64 or Arm64, with glibc 2.28 or later), macOS on Apple silicon and Windows x64, and carries its native engine and TypeScript types. It is an ES module: `import` it, or `require` it on Node.js 22.12 and later. Every example below runs as written in a `.mjs` file with `node example.mjs`. In TypeScript, each answer gets its type from the call.

## Ask a question

```js
import { ThinkThen, UnsureError } from "thinkthen";

const tt = new ThinkThen();
try {
  const isRefund = await tt.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
    { threshold: { low: 0.2, high: 0.8 } },
  );
  console.log(isRefund ? "refund queue" : "inbox");
} catch (error) {
  if (!(error instanceof UnsureError)) throw error;
  console.log("a person reads it", error.probability);
} finally {
  await tt.close();
}
```

<!-- output: capture -->

`isRefund` is a `boolean`. Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there. [Where record text goes](service.md) covers keys, backends and connection settings.

## Handle not sure and failures

Awaiting a call gives its answer or throws:

| Thrown | When | What it carries |
| --- | --- | --- |
| `UnsureError` | The probability falls inside the not-sure band | `probability`, `row` |
| `RecordError` | ThinkThen could not judge the record | `code`, `message`, `row`, `rows` |
| `UsageError` | The call itself is wrong, such as an array passed to a one-record call | `code`, `message` |
| `BackendError` | The backend service failed | `code`, `message`, `httpStatus` when the service sent one |
| `CancelledError` | Your `AbortSignal` fired | `code` `local.cancelled`, `cause` |

Send not sure to a person. Let the others stop your program, or catch `ThinkThenError`, the parent of every class except `UnsureError`. Not sure is not a failure.

To read the probability and details without a throw, call `.row()` on the same call. It makes no second request.

```js
import { ThinkThen } from "thinkthen";

const tt = new ThinkThen();
try {
  const refundCheck = await tt.decide(
    "Does the customer request a refund?",
    { id: "a2", text: "Where is my parcel?" },
    { field: "/text", threshold: { low: 0.3, high: 0.7 } },
  ).row();
  if (refundCheck.status === "answered") {
    console.log(refundCheck.id, refundCheck.answer ? "refund queue" : "inbox", refundCheck.probability);
  } else if (refundCheck.status === "unsure") {
    console.log(refundCheck.id, "a person reads it", refundCheck.probability);
  } else {
    console.error(refundCheck.id, refundCheck.error.code, refundCheck.error.message);
  }
} finally {
  await tt.close();
}
```

<!-- output: capture -->

A row has `status` (`answered`, `unsure` or `failed`), `function`, `position`, `id` when the record has one, and `source`. Only an answered row has `answer`; only a failed row has `error`. TypeScript makes you check `status` before reading either.

`threshold` sets when an answer counts. A number is a cut: yes at or above it, no below. `{ low, high }` is a band: no below `low`, not sure from `low` up to `high`, yes at or above `high`. The default is a cut at 0.5, so decide is never not sure until you set a band. Choose takes only a number and is not sure unless its best choice reaches it. Tag, find, recognize and relate take only a number.

## Judge many messages at once

Each function has a `Stream` form that takes many records and yields one row per record, with its `position` from 1. Pass the whole collection to one stream so the engine can batch it.

```js
import { ThinkThen } from "thinkthen";

const messages = [
  { id: "a1", text: "Please refund my order." },
  { id: "a2", text: "Where is my parcel?" },
  { id: "a3", text: "I was charged twice for one invoice." },
];
const tt = new ThinkThen();
try {
  const refundChecks = tt.decideStream("Does the customer request a refund?", messages, {
    field: "/text",
    threshold: { low: 0.3, high: 0.7 },
  });
  for await (const refundCheck of refundChecks) {
    if (refundCheck.status === "answered") {
      console.log(refundCheck.id, refundCheck.answer ? "refund queue" : "inbox", refundCheck.probability);
    } else if (refundCheck.status === "unsure") {
      console.log(refundCheck.id, "a person reads it", refundCheck.probability);
    } else {
      console.error(refundCheck.id, refundCheck.error.code, refundCheck.error.message);
    }
  }
} finally {
  await tt.close();
}
```

<!-- output: capture -->

A stream keeps going past a not-sure or failed record, so check each row's `status`. To stop at the first one instead, read each row with `answer(row)`, exported from `thinkthen`: it returns the answer or throws `UnsureError` or `RecordError`. `for await` closes the stream when the loop ends or breaks; `.toArray()` collects every row. Streams take arrays, iterables and async iterables.

## Judge a file of records

Create `records.jsonl` with one JSON value per line:

```text
{"id":"a1","text":"Please refund my order."}
{"id":"a2","text":"Where is my parcel?"}
null
{"id":"a3"}
{"id":"a4","text":null}
```

```js
import { readFile } from "node:fs/promises";
import { ThinkThen } from "thinkthen";

const records = (await readFile("records.jsonl", "utf8"))
  .split(/\r?\n/).filter(line => line.trim()).map(line => JSON.parse(line));
const tt = new ThinkThen();
try {
  for await (const refundCheck of tt.decideStream("Does the customer request a refund?", records, { field: "/text" })) {
    if (refundCheck.status === "answered") {
      console.log(refundCheck.position, "answered", refundCheck.answer);
    } else if (refundCheck.status === "unsure") {
      console.log(refundCheck.position, "unsure", refundCheck.probability);
    } else {
      console.log(refundCheck.position, "failed", refundCheck.error.code, refundCheck.error.message);
    }
  }
} finally {
  await tt.close();
}
```

<!-- output: capture -->

Look for one line per record, numbered from 1, with the failures in place. A `null` record, a record without `text` and a `text` that is `null` all fail with `record.missing` and send nothing to the backend.

## Records

A record is a string or a JSON value. `field` picks the part to judge: a JSON Pointer such as `"/text"` (`~1` is a literal slash, `~0` a literal tilde), or a key path such as `["text"]`. Without it, ThinkThen judges the whole record. A pointer picks one value; to judge two fields together, nest them, as in `{ id: "a1", message: { subject: "Invoice", body: "Please refund my order." } }` with `field: "/message"`.

Give object records an `id`: a string, a safe integer or a `bigint`. Rows carry it back, so keep it to join answers to your data; `position` only counts within one call. `JSON.stringify` cannot write a `bigint`, so convert those ids before you save rows.

A one-record call rejects an array or any other iterable, and a stream rejects a string, with `UsageError` code `usage.record-kind`. `null` and `undefined` records, and a selected value that is `null`, fail with `record.missing`.

## The ten functions

Each function below uses the support inbox. Any call that is not sure throws `UnsureError`, which ends this script at the `catch`.

```js
import { ThinkThen, UnsureError, answer } from "thinkthen";

const refund = "Please refund my order.";
const note = "Morgan works for Example Group.\nPlease refund my order.\n";
const messages = [
  { id: "a1", text: "Please refund my order." },
  { id: "a2", text: "Where is my parcel?" },
  { id: "a3", text: "I was charged twice for one invoice." },
];
const asksRefund = "Does the customer request a refund?";
const topics = ["billing", "shipping", "other"];
const urgency = ["routine", "urgent"];

const tt = new ThinkThen();
try {
  const isRefund = await tt.decide(asksRefund, refund);
  console.log("decide:", isRefund);

  const topic = await tt.choose("What is the message about?", refund, { choices: topics });
  console.log("choose:", topic);

  const mentioned = await tt.tag("Which topics appear?",
    "I was charged twice and my parcel is late again.", { labels: ["billing", "shipping"] });
  console.log("tag:", mentioned.join(", "));

  const howUrgent = await tt.score("How urgent is the request?", refund, { levels: urgency });
  console.log("score:", howUrgent);

  const refundRequests = await tt.filter(asksRefund, messages, { field: "/text" }).toArray();
  console.log("filter:", refundRequests.map(message => message.id).join(", "));

  for await (const urgencyRank of tt.rankStream("How urgent is the request?", messages,
    { levels: urgency, field: "/text" })) {
    console.log("rank:", answer(urgencyRank), urgencyRank.id);
  }

  const actionLine = await tt.find("Which line states the requested action?", note);
  console.log("find:", actionLine ? actionLine.text : "no line answers it");

  const triage = await tt.annotate({
    refund: { function: "decide", question: asksRefund },
    topic: { function: "choose", question: "What is the message about?", choices: topics },
    urgency: { function: "score", question: "How urgent is the request?", levels: urgency },
  }, refund);
  for (const [question, outcome] of Object.entries(triage)) {
    console.log("annotate:", question, answer(outcome));
  }

  const names = await tt.recognize(["person", "organization"], "Morgan works for Example Group.");
  for (const name of names) console.log("recognize:", name.kind, name.text);

  const employers = await tt.relate(
    [{ name: "works_for", source: "person", target: "organization" }], note);
  for (const link of employers) console.log("relate:", link.source.text, link.relation, link.target.text);
} catch (error) {
  if (!(error instanceof UnsureError)) throw error;
  console.log("not sure:", error.row.function, error.probability);
} finally {
  await tt.close();
}
```

<!-- output: capture -->

| Function | `await` gives |
| --- | --- |
| decide | `true` or `false` |
| choose | one of your choices, typed as their union |
| tag | an array of the labels that fit, possibly empty |
| score | a number from 0 to 1 along your levels, lowest first |
| filter | the records that pass, unchanged |
| rank | `rankStream` only: each row's answer is its rank, 1 for highest |
| find | `{ text, start, end }` for the line that answers, or `null` |
| annotate | each named question's outcome; read each with `answer(outcome)` |
| recognize | an array of `{ text, kind, start, end, probability }` |
| relate | an array of `{ source, target, relation, probability }`, with `symmetric` on undirected rules |

- **filter** yields only the records answered yes, as the objects you passed in. Not-sure records are left out. If any record fails, the kept records still arrive first, then the loop throws one `RecordError` whose `rows` lists every failed record. Use `filterStream` for every row as `{ item, row, kept }`.
- **rank** reads the whole collection before it yields. `top` limits how many ranks it returns; failed records still come back.
- **choose** needs distinct choices; **score** and **rank** need distinct levels ordered from lowest to highest.
- **find** picks one whole line, so keep the line breaks. Spans use JavaScript string offsets: `text.slice(start, end)`.
- **annotate** answers when the record is judged, but each question inside it can still be not sure or fail. `answer(outcome)` gives that question's answer or throws `UnsureError` or `RecordError`, as a one-record call does. Rank cannot be one of its questions.
- **relate** finds the names itself. On its row from `.row()`, `detail.names` lists every name it found, including names with no relation. A rule can add `phrase` for its wording and `symmetric: true` for a relation that reads both ways.

## Say what the answers mean

`yesMeans` and `noMeans` tell decide and filter what each answer stands for. `descriptions` does the same for the choices of choose, the labels of tag and the levels of score and rank; a key that is not one of them is refused.

```js
import { ThinkThen, UnsureError } from "thinkthen";

const tt = new ThinkThen();
try {
  const topic = await tt.choose("What is the message about?", "I was charged twice for one invoice.", {
    choices: ["billing", "shipping", "other"],
    descriptions: { billing: "charges, invoices and refunds", shipping: "parcels and delivery" },
  });
  const isRefund = await tt.decide("Does the customer request a refund?", "I was charged twice for one invoice.", {
    yesMeans: "the customer wants money back",
    noMeans: "the customer wants something else",
  });
  console.log(topic, isRefund ? "refund queue" : "inbox");
} catch (error) {
  if (!(error instanceof UnsureError)) throw error;
  console.log("a person reads it", error.row.function, error.probability);
} finally {
  await tt.close();
}
```

<!-- output: capture -->

## Save and load questions

A toolbox saves named questions for reuse here, in another language or in the command. Save this as `toolbox.json`:

```json
{"entries":{"refund":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"]}}}}
```

```js
import { ThinkThen, UnsureError, loadToolbox } from "thinkthen";

const box = await loadToolbox("toolbox.json");
if (box.refund?.function !== "filter" || box.topic?.function !== "choose") {
  throw new Error("toolbox.json needs refund as a filter and topic as a choose question");
}
const messages = [
  { id: "a1", text: "Please refund my order." },
  { id: "a2", text: "Where is my parcel?" },
  { id: "a3", text: "I was charged twice for one invoice." },
];
const tt = new ThinkThen();
try {
  for await (const refundRequest of tt.filter(box.refund, messages)) {
    try {
      const topic = await tt.choose(box.topic, refundRequest.text);
      console.log(refundRequest.id, topic);
    } catch (error) {
      if (!(error instanceof UnsureError)) throw error;
      console.log(refundRequest.id, "a person reads it");
    }
  }
} finally {
  await tt.close();
}
```

<!-- output: capture -->

Look for one line per kept record: its id and the topic choose picked.

Check each entry's `function` before you pass it, as the `if` does; TypeScript then knows which call it fits. A saved question carries its own options, such as `field` and `choices`, so the call takes only the record. `defineToolbox({...})` builds the same named questions in code, and `serializeToolbox(box)` turns them back into JSON text.

## Call without a client

The package also exports each function, such as `decide` and `filter`. With no default client set, each call opens a client for the default backend and closes it afterwards. `setDefaultClient(tt)` sends them through your client, which they never close; `setDefaultClient(undefined)` resets it. A client you create and close yourself makes ownership plain, so prefer it in applications.

## Cancel a call

Pass an `AbortSignal` as `signal` to any call. Aborting stops the call, and the pending `await` throws `CancelledError`, named `AbortError`, with code `local.cancelled` and the abort reason as its `cause`. Rows you already read stay yours.

```js
import { ThinkThen, CancelledError } from "thinkthen";

const messages = [
  { id: "a1", text: "Please refund my order." },
  { id: "a2", text: "Where is my parcel?" },
  { id: "a3", text: "I was charged twice for one invoice." },
];
const controller = new AbortController();
const stop = () => controller.abort();
process.once("SIGINT", stop);
const tt = new ThinkThen();
try {
  const refundRequests = tt.filter("Does the customer request a refund?", messages, {
    field: "/text", signal: controller.signal,
  });
  for await (const refundRequest of refundRequests) console.log("refund request", refundRequest.id);
} catch (error) {
  if (!(error instanceof CancelledError)) throw error;
  console.error("cancelled", error.code);
} finally {
  process.removeListener("SIGINT", stop);
  await tt.close();
}
```

<!-- output: capture -->

Closing a client also closes its open streams. Client options include `batch` for records per request, and `retry` with `maxRetries`, `initialBackoffMs` and `attemptTimeoutMs`.

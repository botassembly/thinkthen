# TypeScript and JavaScript functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

Use Node.js 22.12 or later. Run `npm install thinkthen`. The package includes the engine and its TypeScript types. It is an ES module: `import` it, or `require` it. Set `THINKTHEN_API_KEY` in the environment first.

The examples are TypeScript. In plain JavaScript, leave out the type annotations, the `type` lines and `as const`. Everything else is the same.

A one-record call rejects with `tt.FailedError` when its record could not be judged. Successfully awaiting it returns an answered or unsure result, with `.failed` false and its own `.error` set to `undefined`. The shared result type also represents bulk results, so it retains all three states:

- **answered**: there is an answer, and `result.answer` is it.
- **not sure**: the model's number fell between the two bars you set, or no choice was likely enough.
- **failed**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Reading the answer throws when there is none.** On a successfully returned scalar result, `result.answer` can throw `tt.UnsureError`. On a bulk result, form entry or saved row, it can also throw `tt.FailedError`. A missing answer never reads as "no" or as an empty value.
2. **Checking the state never throws.** `result.answered`, `result.unsure`, `result.failed`, `result.probability` and `result.error` are always safe to read.
3. **Printing never throws.** `console.log(result)`, `String(result)` and `` `${result}` `` show the answer, `unsure`, or `failed:` with the reason. A yes prints as `true` and a no as `false`, the same words the `thinkthen` command prints.

## decide

### The simplest if

Read a decide result's `.answer` in an `if`.

```ts
import * as tt from "thinkthen";

const isRefund = await tt.decide("Does the customer request a refund?", "Please refund my order.");

if (isRefund.answer) {
  console.log("refund queue");
} else {
  console.log("inbox");
}
```

If the message could not be judged, awaiting `tt.decide` throws `tt.FailedError` at the call. The `if` never receives a failed result and never falls through to `inbox` for a failure. With one bar a returned decide result is yes or no; two bars can make its answer read throw `tt.UnsureError`.

Always write `.answer`. A result is an object, and JavaScript counts every object as true, so `if (isRefund)` takes the yes branch every time. The language gives a library no way to make that throw. The typescript-eslint rule `no-unnecessary-condition` reports it.

### Yes, no, or not sure

Set two bars and decide can also be not sure. `answered` tells you whether there is a yes or no to read.

```ts
import * as tt from "thinkthen";

const isRefund = await tt.decide(
  "Does the customer request a refund?",
  "Where is my parcel?",
  { threshold: { low: 0.2, high: 0.8 } },
);

if (isRefund.answered) {
  console.log(isRefund.answer ? "refund queue" : "inbox");
} else {
  console.log("a person reads it");
}
```

You can also let the read throw and catch `tt.UnsureError`. The call above has already succeeded, so this read cannot report a failed result.

```ts
try {
  console.log(isRefund.answer ? "refund queue" : "inbox");
} catch (error) {
  if (!(error instanceof tt.UnsureError)) throw error;
  console.log("a person reads it");
}
```

### Did the call work

```ts
import * as tt from "thinkthen";

try {
  const isRefund = await tt.decide(
    "Does the customer request a refund?", "Please refund my order.",
  );
  console.log(isRefund);
} catch (error) {
  if (error instanceof tt.FailedError) {
    console.log("could not judge it:", error.message);
  } else if (error instanceof tt.ThinkThenError) {
    console.log("the call did not complete:", error.message);
  } else {
    throw error;
  }
}
```

`tt.FailedError` reports a failed judgement, including a service failure on that record. Other usage, setup, authentication and cancellation failures retain their classifications; not every call error becomes `FailedError`. JavaScript catch blocks must rethrow errors they do not handle. There is no answer read inside either handler.

Not sure counts as a successful call. The returned result's answer read can throw `tt.UnsureError`, which is not a `tt.ThinkThenError`.

### Both bars and the probability

```ts
const isRefund = await tt.decide(
  "Does the customer request a refund?",
  "Where is my parcel?",
  { threshold: { low: 0.2, high: 0.8 } },
);
console.log(isRefund.probability);
```

- No `threshold`: one bar at 0.5. The answer is always yes or no.
- `threshold: 0.7`: one bar. Yes at 0.7 or above, otherwise no.
- `threshold: { low: 0.2, high: 0.8 }`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. A returned scalar decide result has it even when unsure; a failed bulk row has `undefined`.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `tt.decideStream`, save each `probability`, and compare with your labels.

## choose with your own choices

Give your choices as an `as const` array. The answer comes back as one of them, typed as their union.

```ts
import * as tt from "thinkthen";

const Topic = ["billing", "shipping", "other"] as const;
type Topic = (typeof Topic)[number];

function teamFor(topic: Topic): string {
  switch (topic) {
    case "billing":
      return "billing team";
    case "shipping":
      return "warehouse";
    case "other":
      return "front desk";
  }
}

const topic = await tt.choose(
  "What is the message about?",
  "I was charged twice for one invoice.",
  { choices: Topic },
);

if (topic.unsure) {
  console.log("a person reads it");
} else {
  console.log(teamFor(topic.answer));
}
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So check `topic.unsure` before reading the answer. `threshold: 0.3` moves the bar.

The strings in the array (`"billing"`, `"shipping"`, `"other"`) are the words the model reads, so make them plain words or short phrases, not codes. The same strings are what your code switches on.

Add a string to `Topic` and TypeScript points at `teamFor` until you handle it. This check needs `strict` on in your `tsconfig.json`.

A choose result is not equal to its answer. `topic === "billing"` is a compile error in TypeScript. In plain JavaScript it is quietly false every time, because `===` can't be made to throw, so always compare `topic.answer`. `topic == "billing"` throws `TypeError` and tells you to read `topic.answer`.

An array without `as const` also works, `{ choices: someStrings }`, and the answer is then typed `string`.

## Many records

Each one-record function has a `Stream` form that takes many records and gives one result per record, in order. Each result carries your own record as `.item`.

```ts
import * as tt from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];

const refundChecks = tt.decideStream("Does the customer request a refund?", messages, {
  threshold: { low: 0.2, high: 0.8 },
});

for await (const isRefund of refundChecks) {
  if (isRefund.unsure) {
    console.log("a person reads it:", isRefund.item);
  } else if (isRefund.failed) {
    console.log("try again later:", isRefund.item);
  } else if (isRefund.answer) {
    console.log("refund queue:", isRefund.item);
  } else {
    console.log("inbox:", isRefund.item);
  }
}
```

One failed record does not stop the stream. Each result keeps its own state, with `position` counted from 1 and the record's `id` when it has one.

Pass the whole collection to one stream so ThinkThen can send the records in batches. A stream takes an array, an iterable or an async iterable. `for await` closes the stream when the loop ends, even after an early `break`. `await stream.toArray()` collects every result and closes the stream.

To save every result, write `JSON.stringify(result)` as a line of JSON. It includes the state, so nothing is lost.

## Many-result not sure or failed

Yes and no are the two answers. Not sure and failed are the two ways a bulk result has none. A stream retains a failed row even when its input collection has only one element. To name every state, use a switch in a function with a declared return type:

```ts
import * as tt from "thinkthen";

function whereItGoes(isRefund: tt.DecideRow): string {
  switch (isRefund.status) {
    case "answered":
      return isRefund.answer ? "refund queue" : "inbox";
    case "unsure":
      return "a person reads it";
    case "failed":
      return "try again later";
  }
}

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];
for await (const isRefund of tt.decideStream(
  "Does the customer request a refund?", messages,
  { threshold: { low: 0.2, high: 0.8 } },
)) {
  console.log(whereItGoes(isRefund), isRefund.item);
}
```

With `strict` enabled, omitting a state makes TypeScript report that the function lacks an ending return statement. The same shared `DecideRow` type is used for scalar and bulk calls: its static failed arm remains even though a successfully awaited scalar call never returns that arm at runtime. There is no second scalar-only result type.

To reject a failed bulk result explicitly, call `isRefund.throwIfFailed()` before reading it:

```ts
for await (const isRefund of tt.decideStream(
  "Does the customer request a refund?", ["Please refund my order."],
  { threshold: { low: 0.2, high: 0.8 } },
)) {
  isRefund.throwIfFailed();
  console.log(isRefund);
}
```

It returns the same result for answered or unsure rows; unsure still throws only when its answer is read. The one-item stream keeps bulk behavior: a failed row reaches the explicit check rather than rejecting a scalar call. This helper remains available on the shared type but is redundant after an ordinary successful scalar call.

## filter

filter means "give me the yeses". It hands back your own records, in order.

```ts
import * as tt from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];

for await (const refundRequest of tt.filter("Does the customer request a refund?", messages)) {
  console.log("refund queue:", refundRequest);
}
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `threshold: 0.7` moves the bar. Two bars are refused: TypeScript does not compile them, and plain JavaScript throws `tt.UsageError`. When you want not sure, use `tt.decideStream`, as in the section above.

If a record fails, you still get every kept record first. Then filter throws one `tt.FailedError`, and its `rows` holds every failed result with its `.item`.

```ts
try {
  for await (const refundRequest of tt.filter("Does the customer request a refund?", messages)) {
    console.log("refund queue:", refundRequest);
  }
} catch (failure) {
  if (!(failure instanceof tt.FailedError)) throw failure;
  for (const failed of failure.rows) {
    console.log("try again later:", failed.item, failed.error.code);
  }
}
```

## tag

Labels work as choices do. The answer is an array of the labels that fit, and it can be empty.

```ts
import * as tt from "thinkthen";

const Topic = ["billing", "shipping"] as const;

const topics = await tt.tag(
  "Which topics appear?",
  "I was charged twice and my parcel is late again.",
  { labels: Topic },
);

if (topics.answer.includes("billing")) {
  console.log("billing team");
}
for (const topic of topics.answer) {
  console.log(topic);
}
```

tag is never not sure. `threshold` sets how likely a label must be to count.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it. The order of the supplied `as const` array defines the scale.

```ts
import * as tt from "thinkthen";

const Urgency = ["routine", "urgent"] as const;

const urgency = await tt.score(
  "How urgent is the request?",
  "Please refund my order.",
  { levels: Urgency },
);

if (urgency.answer > 0.5) {
  console.log("answer today");
}
console.log(urgency.answer.toFixed(2));
```

The score is a position between your levels, not one of them, so it is a `number` and there is nothing to `switch` on. To get exactly one level back, ask with `tt.choose`. score is never not sure.

## rank

rank puts many records in order along your levels, highest first.

```ts
import * as tt from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];

for await (const ranked of tt.rank("How urgent is the request?", messages, { levels: Urgency })) {
  console.log(ranked.answer, ranked.item);
}
```

`ranked.answer` is the rank, starting at 1, and `ranked.item` is your record. rank reads the whole collection before it gives anything back. `top: 5` retains the five highest-ranked records after judging the collection. A failed record comes back after the ranked ones, with `failed` set.

## find

find points to the one line that answers.

```ts
import * as tt from "thinkthen";

const message = "Morgan works for Example Group.\nPlease refund my order.";
const action = await tt.find("Which line states the requested action?", message);

if (action.unsure) {
  console.log("a person reads it");
} else if (action.answer === null) {
  console.log("no line answers");
} else {
  console.log(action.answer.text);
}
```

The answer is a `tt.Span` with `text`, `start` and `end`, or `null` when no line answers. `message.slice(span.start, span.end)` is the line. Like choose, find can be not sure with no bar set.

## recognize

```ts
import * as tt from "thinkthen";

const names = await tt.recognize(
  "Morgan works for Example Group.",
  { kinds: ["person", "organization"] },
);

for (const name of names.answer) {
  console.log(name.kind, name.text);
}
```

Each name also has `start`, `end` and `probability`. The answer can be empty. recognize is never not sure.

## relate

```ts
import * as tt from "thinkthen";

const message = "Morgan works for Example Group.\nPlease refund my order.";
const rules = [{ source: "person", target: "organization", relation: "works_for" }] as const;

const relations = await tt.relate(message, { rules });

for (const relation of relations.answer) {
  console.log(relation.source.text, relation.relation, relation.target.text);
}
```

Each relation has `source`, `target`, `relation`, `probability` and `either`. `relations.detail.names` holds every name found, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record.

```ts
import * as tt from "thinkthen";

const messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
];

const questions = {
  refund: { function: "decide", question: "Does the customer request a refund?", threshold: { low: 0.2, high: 0.8 } },
  topic: { function: "choose", question: "What is the message about?", choices: Topic },
  urgency: { function: "score", question: "How urgent is the request?", levels: Urgency },
} as const;

const forms = tt.annotateStream(questions, messages);

for await (const form of forms) {
  if (form.failed) {
    console.log("try again later:", form.item);
    continue;
  }
  const entries = form.answer;
  const { refund: isRefund, topic, urgency } = entries;
  if (isRefund.answered && topic.answered && urgency.answered) {
    if (isRefund.answer) {
      console.log("refund queue:", topic.answer, urgency.answer.toFixed(2));
    }
  } else if (isRefund.failed || topic.failed || urgency.failed) {
    console.log("try again later:", form.item);
  } else {
    console.log("a person reads it:", form.item);
  }
}
```

`form.answer` holds one entry per question, under the name you gave it. Each entry is a result of its own function, with its own state. `form.answer.refund` is a decide result, so its `.answer` is the yes or no. One entry can be not sure or failed while the others answer. Check every entry used by a branch before reading its answer; an answered form alone does not mean its entries all answered. Here `Topic` is the three-choice array from the choose section, and `Urgency` is the array from the score section.

A scalar `await tt.annotate(questions, message)` throws `tt.FailedError` only if the parent form failed. An answered parent is returned even if one, several or all entries failed. Promotion at the scalar call checks the parent's own state, not the recursive helper.

```ts
const form = await tt.annotate(questions, "Please refund my order.");
form.throwIfFailed();
```

The optional second line explicitly checks the form and all entries. It throws `tt.FailedError` for a failed form or any failed entry, never just for an unsure entry; otherwise it returns the same form unchanged. It also works on bulk forms. Checking only the parent's status never authorizes reading an unchecked child's answer.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```ts
const isRefund = await tt.decide(
  "Does the customer request a refund?",
  "I was charged twice for one invoice.",
  {
    yesMeans: "the customer wants money back, including for a duplicate charge",
    noMeans: "the customer wants anything else",
  },
);

const topic = await tt.choose(
  "What is the message about?",
  "I was charged twice for one invoice.",
  {
    choices: Topic,
    descriptions: {
      billing: "charges, invoices and refunds",
      shipping: "parcels and delivery",
    },
  },
);
```

decide and filter take `yesMeans` and `noMeans`. choose, tag, score and rank take `descriptions`, keyed by your choices, labels or levels. TypeScript refuses a key that is not one of them.

## Records and fields

A record is a string, or an object or other JSON-like value. `field: "/text"` picks one field of an object. Without it, ThinkThen judges the whole record.

Give object records an `id`: a string, a whole number or a `bigint`. Each result carries it back as `.id`.

A one-record call given an array throws `TypeError` and names the `Stream` form. A blank record (an empty string, only spaces, or `{}`) rejects the scalar call with `tt.FailedError` and code `record.empty` before any service request. The corresponding stream, even with one input, retains a failed row instead.

## Async

Every call is asynchronous. A one-record call gives a promise of its result, and a many-record call gives an async iterable for `for await`. There is no second, blocking set of functions. A failed scalar parent rejects its promise at `await`; an unsure result fulfills it normally. Stream startup or cancellation errors can reject iteration, while per-record failures remain stream rows.

```ts
import * as tt from "thinkthen";

const isRefund = await tt.decide("Does the customer request a refund?", "Please refund my order.");
console.log(isRefund.answer ? "refund queue" : "inbox");

const messages = ["Please refund my order.", "Where is my parcel?"];
for await (const isRefund of tt.decideStream("Does the customer request a refund?", messages)) {
  console.log(`${isRefund}`, isRefund.item);
}
```

If you forget the `await`, you hold a promise and not a result. TypeScript reports it. In plain JavaScript, reading `.answer` or any state from the promise throws `TypeError` and tells you to `await` the call.

## Saved questions

A question object holds a question and its options, so many calls can share one definition. Pass it in place of the question text.

```ts
const asksRefund: tt.DecideQuestion = {
  function: "decide",
  question: "Does the customer request a refund?",
  threshold: { low: 0.2, high: 0.8 },
  field: "/text",
};
const isRefund = await tt.decide(asksRefund, { id: "a1", text: "Please refund my order." });
```

There is one type per function, such as `tt.ChooseQuestion` and `tt.ScoreQuestion`. A `tt.Toolbox` keeps named questions in a JSON file:

```ts
const toolbox = await tt.Toolbox.load("toolbox.json");
const topic = await tt.choose(toolbox.choose("topic"), { id: "a1", text: "Where is my parcel?" });
```

## Settings

The functions on `tt` use a shared client that reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. To choose settings yourself, make a client and call the same functions on it.

```ts
const client = new tt.Client({ model: "...", batch: 16, retry: { maxRetries: 5 } });
try {
  const isRefund = await client.decide("Does the customer request a refund?", "Please refund my order.");
} finally {
  await client.close();
}
```

The options are `backend`, `url`, `model`, `apiKey`, `batch` and `retry`. Times in `retry` are in milliseconds and say so in their names: `initialBackoffMs` and `attemptTimeoutMs`. Keep the key in the environment, not in your code. Close a client you made. The shared client needs no closing.

## Usage

```ts
const client = new tt.Client();
try {
  await client.decide("Does the customer request a refund?", "Please refund my order.");
  const usage = client.usage;
  console.log(usage.attempts, usage.inputTokens.reported, usage.outputTokens.reported);
} finally {
  await client.close();
}
```

`client.usage` counts everything the client has sent so far: `attempts`, `responses`, retries, bytes sent, and tokens in and out as the service reported them. The counts are `bigint` values.

## Cancel

```ts
const stop = new AbortController();
const isRefund = await tt.decide(
  "Does the customer request a refund?",
  "Please refund my order.",
  { signal: stop.signal },
);
```

Calling `stop.abort()` stops the call, which throws `tt.CancelledError`. Any `AbortSignal` works, such as `AbortSignal.timeout(5000)`.

## Errors

| What happened | When it throws | What is thrown |
| --- | --- | --- |
| A mistake in the call, such as a bad threshold or an array given to a one-record call | When you `await` the call, or at the first step of a `for await` | `tt.UsageError`, `TypeError` |
| The client could not start, such as no API key | When you `await` the call, or at `new tt.Client()` | `tt.SetupError` |
| The call was cancelled | When you `await` the call, or at the next step of a `for await` | `tt.CancelledError` |
| A scalar judgement failed, including blank input or service failure on its row | When you await the call | `tt.FailedError` |
| A bulk result or form entry failed | At its answer read or `throwIfFailed()` | `tt.FailedError` |
| A scalar annotate parent failed | When you await the call; bulk keeps a failed form | `tt.FailedError` at the scalar call or a later bulk read/check |
| An answered annotate parent contains failed entries | Call returns it; a failed-entry read or explicit form check throws | `tt.FailedError` |
| Not sure | When you read the answer | `tt.UnsureError` |
| You print a result, store it, or ask about its state | Never | |

JavaScript's `catch` takes everything, so test what you caught with `instanceof` and throw the rest again. Two parents let you test by meaning:

- `error instanceof tt.NoAnswerError` is true for `tt.UnsureError` and `tt.FailedError`: there was no answer to read.
- `error instanceof tt.ThinkThenError` is true for everything that went wrong, including `tt.FailedError`. It is false for `tt.UnsureError`, because not sure is a result and not an error.

`tt.FailedError` is the renamed concrete `RecordError`; wire codes such as `record.empty` stay unchanged. Other call errors keep their existing classifications and payloads. It carries `code`, `httpStatus`, `row` and `rows`, the failed results, each affected record included once. A form contributes its first failed entry in question-name order, carrying the form's record, position, id and source. `tt.UnsureError` carries `probability` and `row`.

Three things JavaScript itself decides, which no library can change:

- `if (result)` is always true. Read `result.answer`.
- `result === "billing"` is always false. Compare `result.answer`. TypeScript reports the comparison; the lint rule above reports the always-true condition.
- Joining a result to a string with `+`, comparing it with `==`, and using it as a number all throw `TypeError`, in every state, so the first run shows the slip. To print a result, use a template literal, `String(result)` or `console.log(result)`.

## Upgrading from 0.1

The baseline is the [published 0.1.2 package/source](https://registry.npmjs.org/thinkthen/-/thinkthen-0.1.2.tgz). Replace its call envelopes with the results shown above.

| 0.1.2 | Now |
| --- | --- |
| `Call<T>.value` and nullable answers | Await the result, then read `.answer`; scalar failures reject at the call and unsure throws at the answer read. Bulk results retain all three states |
| `Engine` and `question(...)` | `Client` and typed saved questions |
| Per-call `.facts` and `.details` | Client usage and result detail |

Code using the pre-release `RecordError` name must use `FailedError`. Filter aggregation, one-element streams and failed form entries retain their existing result behavior; no wire error code is renamed.

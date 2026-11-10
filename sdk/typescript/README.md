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

## Usage

Call `client.usage()` for an immutable cumulative snapshot. Counters and reported token sums are `bigint`; `null` means an overflowed subtotal, and `unknownAttempts` counts finished sends without a reported token count. Active calls may perform more work even when `activeAttempts` is zero. The snapshot is not an invoice. Await `client.close()` to cancel and join owned calls; the final snapshot remains available afterward.

```ts
const usage = client.usage();
console.log(usage.attempts, usage.inputTokens.reported);
```

## Request allowance

```typescript
const client = new ThinkThen({maxRequests: 100});
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.

`maxRequests` accepts exact JavaScript safe integers; unsupported precision is rejected rather than rounded.

### Local response storage

`new ThinkThen({responseStore: {path, mode: "record", namespace, modelGeneration}})` explicitly enables local recording. Modes `"cache"` and `"replay"` select eligible reuse or strict offline playback without credentials. Optional `maxBytes`, `maxEntries`, `maxAge` (seconds), and `busyTimeoutMs` use engine defaults when omitted. `client.usage().cacheHits` and `replayHits` are exact `bigint` counters.

Storage is disabled unless configured. Local recordings retain question and record text; use a private local path and a nonsecret namespace/model-generation label. The shared engine owns limits, identity and concurrency policy. Replay misses fail locally rather than making a model call.

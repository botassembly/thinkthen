# The TypeScript surface

`thinkthen` for Node 22 and later. A napi-rs addon calls the public Rust API in `crates/thinkthen`, and `index.js` gives each verb an async JavaScript form.

```ts
import * as tt from "thinkthen";

const answered = await tt.decide("Does the customer ask for a refund?", text);
answered.value;  // true, false, or null
answered.facts.requests_sent;  // final requests for this call
const refund = tt.question({ decide: "Does the customer ask for a refund?", threshold: [0.2, 0.8] });
(await tt.decide(refund, "I was charged twice. Can you fix this?")).value;
const complaints = (await tt.filter("Is this a complaint?", reviews, { batch: 2, context: "Shared note." })).value;
const rows = (await tt.annotate("form.json", tickets, { signal, batch: 2 })).value;
```

`null` means unsure. The verbs are `decide`, `decide_many`, `choose`, `choose_many`, `score`, `score_many`, `tag`, `tag_many`, `filter`, `rank`, `find`, `annotate`, `details`, `recognize`, and `relate`. `usage()` returns the counters, and `plan()` previews a call. Each verb is also a method of `new tt.Engine(options)`.

`tt.questionFile(path)` synchronously reads one named UTF-8 question file of at most 1 MiB and returns a frozen question value for the same verbs as `tt.question(spec)`. Its validated source JSON preserves described-label order. An unreadable or invalid named file throws a non-retryable `ThinkThenError` of kind `local` before any request. `tt.question(spec)` and ordinary strings, including strings beginning with `@`, keep their existing typed and literal meanings. `rank` and `find` still accept only plain decide text.

## Settings

The module-level verbs use the engine the environment describes: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, and the rest. `new tt.Engine({ backend, baseUrl, model, throttle, maxRequests, maxRequestsTotal, maxRequestBytes, cache, timeoutSeconds, maxRetries, profile, record, replay, batch })` starts from the same environment, and each given key overrides one setting. `cache: false` keeps no cache. A refused setting throws `ThinkThenError` of kind `usage`. An engine counts its own usage. `maxRequestsTotal` caps the live sends every engine of the process makes together; `0` refuses the first with `usage`.

A question spec's `profile` names saved calibration. The engine's separate `profile` option selects a runtime limits file. A mismatch appears as optional `details.meta.profile_warning`; the type declarations cover both names.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `new tt.Engine({ cache: false })`.

The throttle is the most requests in flight at once. It holds per loaded copy of the engine, and the first explicit throttle holds for the process. A later engine that asks for another throttle is refused.

## Calls

Each call runs on its own worker thread, off the JavaScript thread and off Node's libuv pool. A file read or a timer keeps running while calls wait. Each call in flight holds one OS thread, so ten thousand single calls start ten thousand threads. For volume, pass a list to `decide_many`, `choose_many`, `score_many`, `tag_many`, `filter`, `rank`, or `annotate`. A list crosses into the engine once.

The last object of each verb takes `signal` and `deadlineMs`. Eligible many-record calls also take `batch: "max"` or a positive integer. `decide_many`, `choose_many`, `score_many`, `tag_many`, `filter`, and `rank` take nonblank literal `context` text; `annotate` takes `batch` without `context`. Bare-text `choose_many`, `score_many`, and `tag_many` put their ordered `options`, `levels`, or `labels` in that same object. A score list sends bare levels, while a score map entry of `null` sends an explicit no-description level. An `AbortSignal` cancels a call at once: the promise rejects with `cancelled`, no new request starts, and a request already sent finishes. `deadlineMs` bounds the whole call in milliseconds, and `0` is spent. No deadline is spelled null, left out, or -1.

A failure rejects with one `ThinkThenError` class. Its `kind` is `usage`, `backend`, `local`, `cancelled`, `deadline`, or `defect`, `code` is the C door's number for that kind from 1 (`usage`) to 6 (`defect`), and `retryable` says whether the same call may pass later. `tt.YES`, `tt.NO`, and `tt.UNSURE` are the C outcome codes 1, 0, and 2, and `tt.outcome(answer)` maps `true`, `false`, and `null` to them.

`recognize` returns each name as `text`, `start`, `end`, `length`, `kind`, and `strength`. `start`, `end`, and `length` count UTF-16 units, so `text.slice(start, end)` is the name. With no kinds, every name has the kind `ENTITY`.

## Run facts

Each asking method returns a `Call<T>` with its former answer in `.value`, final `.facts`, and ordered immutable `.details` for completed questions. A started failure retains final facts and details on `ThinkThenError`; a pre-account refusal has neither. An early signal rejection offers `.completion.wait()` for the same worker's final `{ok}` or `{err}` report. Waiting keeps Node alive; an unobserved receipt lets a child exit while a held reply finishes. `details(question, text).value` returns the command's `--details` line for one text, typed as `Details`, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. Facts include elapsed seconds but do not report cost or priced tokens.

Facts, details, and annotate rows are the engine's JSON, so a member this package does not name still reads. In an annotate row, `null` is unresolved and `{ failed: { kind, cause } }` is a failed question. `tt.failed(answer)` returns that failure object, or `null` for any value.

`plan(question, records, { batch, context })` previews a `decide`, `choose`, `score`, or `tag` call without asking it and returns at once. Its value is the result schema's `plan` object: `records`, `requests`, `estimated_bytes`, `estimated_input_tokens` with `lower` and `upper`, `upper_bound`, and `first_body_utf8`. It reads no key and no cache and sends nothing, so it takes no `signal` or `deadlineMs`.

`usage()` returns this engine's running totals of requests sent, retries, cache answers and tokens.

## Build and check

`setup-toolchain.sh` places the pinned Node under `~/.cache/thinkthen-toolchains/` once, on a networked machine. `build-addon.sh` builds `thinkthen-<platform>-<arch>.node` offline, the name `loader.js` picks. `check.sh` is this surface's entry in the surface rung, `sdlc/scripts/surfaces`. It runs offline against loopback backends only and reports "not run" when the toolchain is missing.

`new tt.Engine({backend: "liquid"})` selects a named backend in code. The Rust builder captures its key from that backend's environment variable. Explicit `backend` outranks environment and configuration selection. With `baseUrl` too, the selected key, posting path, description form and setup prices/profile apply at that address. An explicit model or profile overrides its setup value. No TypeScript key option exists. Omission preserves the environment-driven default engine.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

## Complete native calls

`engine.complete` and the default `complete` facade expose all ten named
functions: decide, choose, tag, score, filter, rank, find, annotate, recognize and
relate. Each resolves to a `Completed<ResultType>` with typed `results`, native
final `facts`, original `inputs` and zero-based `ordinals`. `CompleteTypes`
exports the result, probability, author declaration, identity and source types.

```typescript
import { Engine } from 'thinkthen';

const engine = new Engine({ cache: false });
const call = await engine.complete.decide(
  { role: 'atomic', body: { decide: 'Is this a complaint?' } },
  { kind: 'records', records: [{ content: { kind: 'text', value: 'Please refund.' } }] },
  { attempts: true },
);
const probability: number = call.results[0].answer.probability;
```

Question sources select exactly one of `body`, `raw`, `path`, `name` or
`reference`, with an explicit `role`: atomic, dynamic, rank, set, find, recognize
or relate. Rank sets use set. Native loaders retain structured descriptions,
author names, wording versions, declarations and annotation members. Complete
rank accepts the native rich grammar and question sets.

A file source is `{ kind: 'files', paths: ['report.txt'], options: { reading:
{ unit: 'file' }, media: 'text' } }`; line and window readings retain physical
locations. `jsonl: true` reads records; `media: 'image'` reads explicit images.
Records may include ordered `{ media: 'image/png', bytes: [...] }` attachments,
explicit text/JSON context and described options. Images are admitted for
decide, choose and score; the other seven functions refuse before sending.

`decideBatch`, `chooseBatch`, `tagBatch`, `scoreBatch`, `filterBatch` and
`annotateBatch` are native lazy async iterators. Rows have typed `result`,
`ordinal` and `input`; `facts` is final at exhaustion or terminal failure.
Breaking `for await` closes the batch; explicit `return()` closes it and
`cancel()` stops native work. Source files and requests wait for the first pull.
The ordinary `signal`, `deadlineMs`, `context` and `attempts` controls apply.

Failures are ordinary `ThinkThenError` values with typed `complete`, including
available final native facts and stopped position. Cache, record and replay
remain native, including changed-reading identity and zero-send strict replay.
JavaScript uses the same runtime; its tests execute separately from the compiled
TypeScript consumer. Existing bare calls and their declarations stay compatible.

Dynamic choose batches accept a `dynamic` question source with `choose`, optional input/context/candidate pointers, and a whole ordered candidate list on every record. The native lazy dynamic choose API admits each record and supplies its own probabilities and identity. Missing later candidates yield the completed prefix, then a usage error with joined final facts.

Rank set results retain ordered `members`, each with its saved `name` and typed `RankMemberResult`. The child carries its own positive member position, answer ID, authored question, probability, source and metadata; the parent carries the final turns position and winner. Usage dimensions remain independently optional. Parent and member usage overlap; use the final call facts for invocation totals. Ordinary ranks retain absent members.

The `Client` API calls Rust-owned Request sessions. Create `new Client(settings)`, call `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` or `relate`, then close the client. Strings, JSON values and arrays are ordinary inputs. `Client.item(value, fields)` adds per-record context or attachments. `Client.files(paths, reading)` reads physical source units. `Client.questionFile(path)` selects a question file explicitly. Request options retain the names in the [native schema](../../specification/request.schema.json), including `deadline_ms`. Add `signal` for an AbortSignal. Rust resolves admission, settings, cache identity and results.

Each call returns a Promise of `Completed`, with generated typed `results`, a `terminal` packet and optional observed `facts`. `client.start(function, question, input, controls)` returns an async iterator over generated native packets, and also provides `result()`, `cancel()` and `close()`. Iterables and async iterables feed bounded native sessions. Cancellation closes the feed, removes the signal listener and drops the session while provider replies remain held. A cancelled host call has no final facts unless Rust has already delivered a terminal packet.

Results retain their values after the client closes. An omitted member is `undefined`; `result.has(name)` distinguishes it from an explicit null. Unknown output members remain present. Failures raise `ClientError`, a subclass of `ThinkThenError`, with native `kind`, `retryable`, `complete`, `terminal`, completed `results` and any observed `facts`. The existing `Engine` and `complete` APIs remain available during migration. Replace `engine.complete.decide(questionSource, input)` with `client.decide(question, input)` when migrating a caller; installed full parity will govern compatibility retirement.

JavaScript numbers use the host's binary floating point representation. Native integer tokens outside JavaScript's safe integer range become generated `Results.NativeNumber` values with exact `raw` text; JSON serialization preserves those tokens. IDs retain their native strings. Ordinary fractional numbers retain JavaScript's numeric precision. Input values must be representable as JSON; undefined, nonfinite numbers, lone surrogates and cycles refuse during conversion.

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

`null` means unsure. The verbs are `decide`, `decide_many`, `choose`, `choose_many`, `score`, `score_many`, `tag`, `tag_many`, `filter`, `rank`, `find`, `annotate`, `details`, `recognize`, and `relate`. `usage()` returns the counters. Each verb is also a method of `new tt.Engine(options)`.

`tt.questionFile(path)` synchronously reads one named UTF-8 question file of at most 1 MiB and returns a frozen question value for the same verbs as `tt.question(spec)`. Its validated source JSON preserves described-label order. An unreadable or invalid named file throws a non-retryable `ThinkThenError` of kind `local` before any request. `tt.question(spec)` and ordinary strings, including strings beginning with `@`, keep their existing typed and literal meanings. `rank` and `find` still accept only plain decide text.

## Settings

The module-level verbs use the engine the environment describes: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, and the rest. `new tt.Engine({ baseUrl, model, throttle, maxRequests, maxRequestBytes, cache, timeoutSeconds, maxRetries, profile, record, replay, batch })` starts from the same environment, and each given key overrides one setting. `cache: false` keeps no cache. A refused setting throws `ThinkThenError` of kind `usage`. An engine counts its own usage.

A question spec's `profile` names saved calibration. The engine's separate `profile` option selects a runtime limits file. A mismatch appears as optional `details.meta.profile_warning`; the type declarations cover both names.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `new tt.Engine({ cache: false })`.

The throttle is the most requests in flight at once. It holds per loaded copy of the engine, and the first explicit throttle holds for the process. A later engine that asks for another throttle is refused.

## Calls

Each call runs on its own worker thread, off the JavaScript thread and off Node's libuv pool. A file read or a timer keeps running while calls wait. Each call in flight holds one OS thread, so ten thousand single calls start ten thousand threads. For volume, pass a list to `decide_many`, `choose_many`, `score_many`, `tag_many`, `filter`, `rank`, or `annotate`. A list crosses into the engine once.

The last object of each verb takes `signal` and `deadlineMs`. Eligible many-record calls also take `batch: "max"` or a positive integer. `decide_many`, `choose_many`, `score_many`, `tag_many`, `filter`, and `rank` take nonblank literal `context` text; `annotate` takes `batch` without `context`. Bare-text `choose_many`, `score_many`, and `tag_many` put their ordered `options`, `levels`, or `labels` in that same object. A score list sends bare levels, while a score map entry of `null` sends an explicit no-description level. An `AbortSignal` cancels a call at once: the promise rejects with `cancelled`, no new request starts, and a request already sent finishes. `deadlineMs` bounds the whole call in milliseconds, and `0` is spent. No deadline is spelled null, left out, or -1.

A failure rejects with one `ThinkThenError` class. Its `kind` is `usage`, `backend`, `local`, `cancelled`, `deadline`, or `defect`, and `retryable` says whether the same call may pass later.

`recognize` returns each name as `text`, `start`, `end`, `length`, `kind`, and `strength`. `start`, `end`, and `length` count UTF-16 units, so `text.slice(start, end)` is the name. With no kinds, every name has the kind `ENTITY`.

## Run facts

Each asking method returns a `Call<T>` with its former answer in `.value`, final `.facts`, and ordered immutable `.details` for completed questions. A started failure retains final facts and details on `ThinkThenError`; a pre-account refusal has neither. An early signal rejection offers `.completion.wait()` for the same worker's final `{ok}` or `{err}` report. Waiting keeps Node alive; an unobserved receipt lets a child exit while a held reply finishes. `details(question, text).value` returns the command's `--details` line for one text, typed as `Details`, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. Facts include elapsed seconds but do not report cost or priced tokens.

`usage()` returns this engine's running totals of requests sent, retries, cache answers and tokens.

## Build and check

`setup-toolchain.sh` places the pinned Node under `~/.cache/thinkthen-toolchains/` once, on a networked machine. `build-addon.sh` builds `thinkthen-<platform>-<arch>.node` offline, the name `loader.js` picks. `check.sh` is this surface's entry in the surface rung, `sdlc/scripts/surfaces`. It runs offline against loopback backends only and reports "not run" when the toolchain is missing.

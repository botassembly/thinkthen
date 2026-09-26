# The TypeScript surface

`thinkthen` for Node 22 and later. A napi-rs addon calls the public Rust API in `crates/thinkthen`, and `index.js` gives each verb an async JavaScript form.

```ts
import * as tt from "thinkthen";

await tt.decide("Does the customer ask for a refund?", text);  // true, false, or null
const refund = tt.question({ decide: "Does the customer ask for a refund?", threshold: [0.2, 0.8] });
await tt.decide(refund, "I was charged twice. Can you fix this?");
const complaints = await tt.filter("Is this a complaint?", reviews);
const rows = await tt.annotate("form.json", tickets, { signal });
```

`null` means unsure. The verbs are `decide`, `decide_many`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`, `details`, `recognize`, and `relate`. `usage()` returns the counters. Each verb is also a method of `new tt.Engine(options)`.

## Settings

The module-level verbs use the engine the environment describes: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, and the rest. `new tt.Engine({ baseUrl, model, throttle, maxRequests, cache, cacheBytes })` starts from the same environment, and each given key overrides one setting. `cache: false` keeps no cache. A refused setting throws `ThinkThenError` of kind `usage`. An engine counts its own usage.

The throttle is the most requests in flight at once. It holds per loaded copy of the engine, and the first explicit throttle holds for the process. A later engine that asks for another throttle is refused.

## Calls

Each call runs on its own worker thread, off the JavaScript thread and off Node's libuv pool. A file read or a timer keeps running while calls wait. Each call in flight holds one OS thread, so ten thousand single calls start ten thousand threads. For volume, pass a list to `decide_many`, `filter`, `rank`, or `annotate`. A list crosses into the engine once.

The last object of each verb takes `signal` and `deadlineMs`. An `AbortSignal` cancels a call at once: the promise rejects with `cancelled`, no new request starts, and a request already sent finishes. `deadlineMs` bounds the whole call in milliseconds, and `0` is spent. No deadline is spelled null, left out, or -1.

A failure rejects with one `ThinkThenError` class. Its `kind` is `usage`, `backend`, `local`, `cancelled`, `deadline`, or `defect`, and `retryable` says whether the same call may pass later.

`recognize` returns `start` and `end` in UTF-16 units, so `text.slice(start, end)` is the name.

## Build and check

`setup-toolchain.sh` places the pinned Node under `~/.cache/thinkthen-toolchains/` once, on a networked machine. `build-addon.sh` builds `thinkthen-<platform>-<arch>.node` offline, the name `loader.js` picks. `check.sh` is this surface's entry in the surface rung, `sdlc/scripts/surfaces`. It runs offline against loopback backends only and reports "not run" when the toolchain is missing.

# The JavaScript and TypeScript surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to JavaScript and TypeScript.

## What really good looks like

A JavaScript user expects one import, no build step, no `await init()`, and types that know the answer's shape. The best packages install a prebuilt binary, work under `import` and `require` alike, take an `AbortSignal`, and add nothing more to `node_modules`. This one earns it when a wrong branch is a compile error and a thousand records stream back in order with the event loop free.

```ts
import * as tt from "thinkthen";

if (await tt.decide("The command only reads files.", command)) runIt();

const dated = await Array.fromAsync(
  tt.filter("Names a delivery date.", notes, { field: "/text", threshold: 0.9 }),
);
```

## Goals

- No verb does its work on the main thread. A timer probe reads the same event loop delay during a run as at idle.
- Rust holds one pool per process. The stub backend counts one handshake per hundred calls.
- Record verbs return async iterables in input order, one chunk per crossing. The bulk spelling is `decide_many` beside `filter` (ADR 0017 pick 8). For large batches and large records, `filterBytes` takes one UTF-8 buffer plus an offsets array and crosses as a flat pointer handoff, 0.001–0.009 ms at any size, where string conversion alone costs about 0.1 µs per 37-byte record and at 4 KB records more than the whole 10k crossing (205, round two). Strings stay the default for ergonomics.
- A number gives `Promise<boolean>`, a `[low, high]` tuple gives `Promise<boolean | null>` with `null` for unsure (ADR 0017 pick 3), and `as const` labels narrow `choose` to that union plus `null`.
- One tarball serves ESM and CommonJS and installs with no compiler and no script.

## Anti-goals

- No WebAssembly build. It cannot open a socket, so sending, retries, concurrency, and recording get written twice.
- No browser bundle in the first release. An API key in a browser is a disclosure.
- No install script, no compiler, and no network beyond the registry fetch of one optional dependency. Measured: 452 ms in `node:22-slim` with `--ignore-scripts` (205).
- No runtime dependency. An HTTP client, limiter, or validator duplicates Rust.
- No synchronous variant. It freezes the event loop for a whole round trip.

## Where this language wastes time

- **The event loop is one thread.** Work in the synchronous entry stalls every timer and socket. The entry returns a promise, and a Node-API async task over a Rust thread pool resolves it.
- **Every crossing copies, and a string copies twice over.** Field-by-field conversion pays a crossing per field, and JSON text parses the same bytes twice. One chunk goes down and one comes up, built as JavaScript values in Rust. Node-API reads one string per call and copies it, and no bulk string call exists, so the fix cuts the crossings and not the copies. napi-rs takes a whole `Array` in one call, and `AsyncTask` runs the work on a libuv thread and resolves the promise on the main one. A `Buffer` or a typed array is the one container that arrives with no copy, because Node-API hands back a pointer into the engine's backing store. A caller holding UTF-8 bytes may send one buffer and one offsets array.
- **An `await` inside a `for` loop sends one request at a time.** Record verbs hand the array, the async iterable, or the stream to Rust at width `jobs`.
- **An endless stream with no backpressure.** A reader that pushes faster than the engine answers grows the heap. `write` returns false once the queued bytes pass `highWaterMark`, and the writer waits for `drain`. The default is 64 KiB since Node 22 and 16 objects in object mode. `for await` pulls one chunk at a time in practice, and the Node documents do not say so, so it is unchecked. The verbs hold `jobs` records in flight, so memory stays flat at any input length. napi-rs has `ReadableStream` and `WritableStream` behind its `web_stream` feature, and its generator classes are experimental.
- **Work the shim must not do.** The engine asks an equal pair of question and evidence once inside a batch, and a cached answer costs nothing. No `Set` of seen records in JavaScript. `jobs` is one number for the process, and two callers, a worker thread, and a stream feed the one scheduler in Rust. A called question is serial, and `tt.filter(q, records)` and `tt.decide_many(q, records)` are the bulk forms.
- **A forgotten `await` is truthy.** `if (tt.decide(...))` always takes the yes branch. The docs turn on `@typescript-eslint/no-floating-promises` and `@typescript-eslint/no-misused-promises`.

## How little code

**The binding tool is `napi-rs` over Node-API.** Node-API is ABI stable, so one binary per platform serves every Node version that targets it. Rust keeps the socket, the retries, the pool, the scheduler, and the recordings.

**Runtimes.** Node is covered. Bun and Deno claim Node-API support, and running there is unchecked. No native addon reaches a browser or an edge worker. A browser gets a package over a server the user runs.

**The shim contains** argument conversion, the promise from an async task, the async iterable over chunks, `AbortSignal` wired to cancellation, errors, declarations, and two entry wrappers.

**It never contains** an HTTP call, a retry, a rate limit wait, threshold math, JSON building, or a recording read or write.

**Build and ship.** `napi build` produces one `.node` per platform. A user installs `thinkthen`, and each platform binary publishes under the `@thinkthen` scope with `os` and `cpu` fields, listed under `optionalDependencies`; the one-name ruling survives as "one landing zone, one scope", and the scope claim is a hard prerequisite on Ian's list (205). `exports` names an `import` and a `require` condition, and a `createRequire` ESM door is the required bridge for Deno, which needs `env`, `read`, and `sys` permissions for the loader (205).

## Tests only this surface needs

- Event loop delay during a thousand-record run matches the idle reading.
- An aborted signal rejects the promise and ends the iterable.
- One tarball answers a replayed case in ESM and CommonJS projects, with `--ignore-scripts` and no Rust toolchain.
- Type tests assert both overloads, and a lint fixture fails on a missing `await`.
- A stream of ten million records holds heap flat, and a slow consumer stops the reader through backpressure.
- A bench counts rows a second through the array form and the stream form against the stub, beside the engine's own number from pure Rust. A gap is a defect in the shim.

## Open questions for the ADR

1. Answered by 205: a configured threshold typed `number | [number, number]` is a compile error under the overloads, which is the wanted behavior. The page documents the narrowing step (`typeof t === "number"`).
2. Do Bun and Deno load the addon, and may the first release claim them? The string form passed on both in round one; the byte form is not runtime-specific but was not run there.
3. Does breaking out of a `for await` early cancel the Rust work? Measured cancellation semantics (205): abort stops mid-crossing and closes sockets; break stops at the chunk boundary. The engine's cancel promise (ADR 0017 section 2) adds: no new request starts, and sent requests finish.
4. Answered: the platform binaries publish under the `@thinkthen` scope once claimed, beside `thinkthen`.
5. Answered by 205, round two: the byte form ships as `filterBytes` beside the string form.

## The call shape as built (ticket 0107)

Ticket 0107 ported the surface onto the public Rust API in `crates/thinkthen`. The shape follows the ruling of 2026-09-21.

- Each verb takes its question first, the text or the record array second, and one last object. That object holds the question's inputs (`options`, `levels`, `labels`, `top`, and the `recognize` and `relate` rules) beside `signal` and `deadlineMs`.
- A list crosses into the engine once, as one JSON string, and the answer comes back as one JSON envelope.
- Each call runs on its own worker thread, off the JavaScript thread and off the libuv pool. `decide_many` is the lever for volume, because each single call in flight holds one OS thread.
- `new tt.Engine(options)` holds the engine settings of ADR 0017 section 5, and the module-level verbs use the engine the environment describes.
- The async iterables, the stream form, and `filterBytes` above are not built. Ticket 0107 excludes them.

## Differences from Python

The Python surface is the reference. Each difference below is decided, and its reason is its line.

1. **The error classes.** Python raises six classes under a `ThinkThenError` base. TypeScript raises one `ThinkThenError` with `kind` and `retryable`, the JavaScript habit. The six kind words are the same.
2. **The cancel gesture.** Python takes a `CancelToken`, and TypeScript takes an `AbortSignal`. Both follow the engine's rule: no new request starts, and a sent request finishes. The TypeScript promise rejects at once and does not wait for the sent request.
3. **The deadline unit.** Python takes seconds, and TypeScript takes `deadlineMs` in milliseconds, the unit of `Date.now()`. `-1`, `null`, and a missing key mean no deadline. `0` is spent. Any other value is a whole number up to 4294967295000, and anything else is a `usage` error that names the number.
4. **The question value.** Python returns a Question object. TypeScript returns a frozen function that carries the spec, so no spread or `JSON.stringify` turns it back into text by accident. Calling it throws `usage`.
5. **The `recognize` offsets.** Python counts code points. TypeScript converts them to UTF-16 units, so `text.slice(start, end)` is the name.
6. **Blocking against async.** Python's verbs block the calling thread. TypeScript's verbs return promises, and each call runs on a worker thread of its own.
7. **The column forms.** Python's verbs take Polars columns and frames. TypeScript has no column form, and a Node column form needs its own design.
8. **The record text.** Both typed hosts refuse a record that is not a string. TypeScript also refuses a string with a lone surrogate and names its index, because Node-API would replace it silently.
9. **Usage counters.** `usage()` returns the four counters synchronously. An `Engine` value counts its own calls, apart from the module-level engine.
10. **The `find` none case.** Both return null when nothing is selected. Both take a `none` switch since ticket 0150, so the shared cases that name one run on both surfaces.

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
- Record verbs return async iterables in input order, one chunk per crossing.
- A number gives `Promise<boolean>`, a `[low, high]` tuple gives `Promise<Outcome>`, and `as const` labels narrow `choose` to that union plus `null`.
- One tarball serves ESM and CommonJS and installs with no compiler and no script.

## Anti-goals

- No WebAssembly build. It cannot open a socket, so sending, retries, concurrency, and recording get written twice.
- No browser bundle in the first release. An API key in a browser is a disclosure.
- No install script and no `node-gyp`. Compiling on install breaks offline machines and private registries.
- No runtime dependency. An HTTP client, limiter, or validator duplicates Rust.
- No synchronous variant. It freezes the event loop for a whole round trip.

## Where this language wastes time

- **The event loop is one thread.** Work in the synchronous entry stalls every timer and socket. The entry returns a promise, and a Node-API async task over a Rust thread pool resolves it.
- **Every crossing copies, and a string copies twice over.** Field-by-field conversion pays a crossing per field, and JSON text parses the same bytes twice. One chunk goes down and one comes up, built as JavaScript values in Rust. Node-API reads one string per call and copies it, and no bulk string call exists, so the fix cuts the crossings and not the copies. napi-rs takes a whole `Array` in one call, and `AsyncTask` runs the work on a libuv thread and resolves the promise on the main one. A `Buffer` or a typed array is the one container that arrives with no copy, because Node-API hands back a pointer into the engine's backing store. A caller holding UTF-8 bytes may send one buffer and one offsets array.
- **An `await` inside a `for` loop sends one request at a time.** Record verbs hand the array, the async iterable, or the stream to Rust at width `jobs`.
- **An endless stream with no backpressure.** A reader that pushes faster than the engine answers grows the heap. `write` returns false once the queued bytes pass `highWaterMark`, and the writer waits for `drain`. The default is 64 KiB since Node 22 and 16 objects in object mode. `for await` pulls one chunk at a time in practice, and the Node documents do not say so, so it is unchecked. The verbs hold `jobs` records in flight, so memory stays flat at any input length. napi-rs has `ReadableStream` and `WritableStream` behind its `web_stream` feature, and its generator classes are experimental.
- **Work the shim must not do.** The engine asks an equal pair of question and evidence once inside a batch, and a cached answer costs nothing. No `Set` of seen records in JavaScript. `jobs` is one number for the process, and two callers, a worker thread, and a stream feed the one scheduler in Rust. A called question is serial, and `tt.filter(q, records)` is the bulk form.
- **A forgotten `await` is truthy.** `if (tt.decide(...))` always takes the yes branch. The docs turn on `@typescript-eslint/no-floating-promises` and `@typescript-eslint/no-misused-promises`.

## How little code

**The binding tool is `napi-rs` over Node-API.** Node-API is ABI stable, so one binary per platform serves every Node version that targets it. Rust keeps the socket, the retries, the pool, the scheduler, and the recordings.

**Runtimes.** Node is covered. Bun and Deno claim Node-API support, and running there is unchecked. No native addon reaches a browser or an edge worker. A browser gets a package over a server the user runs.

**The shim contains** argument conversion, the promise from an async task, the async iterable over chunks, `AbortSignal` wired to cancellation, errors, declarations, and two entry wrappers.

**It never contains** an HTTP call, a retry, a rate limit wait, threshold math, JSON building, or a recording read or write.

**Build and ship.** `napi build` produces one `.node` per platform. A user installs `thinkthen`, and each platform binary publishes under its own scoped name with `os` and `cpu` fields, listed under `optionalDependencies`. Prebuilt: darwin, linux gnu and musl, and win32 on x64 and arm64 where each exists. `exports` names an `import` and a `require` condition.

## Tests only this surface needs

- Event loop delay during a thousand-record run matches the idle reading.
- An aborted signal rejects the promise and ends the iterable.
- One tarball answers a replayed case in ESM and CommonJS projects, with `--ignore-scripts` and no Rust toolchain.
- Type tests assert both overloads, and a lint fixture fails on a missing `await`.
- A stream of ten million records holds heap flat, and a slow consumer stops the reader through backpressure.
- A bench counts rows a second through the array form and the stream form against the stub, beside the engine's own number from pure Rust. A gap is a defect in the shim.

## Open questions for the ADR

1. Does the tuple overload hold up when a configured threshold is typed `number | [number, number]`?
2. Do Bun and Deno load the addon, and may the first release claim them?
3. Does breaking out of a `for await` early cancel the Rust work?
4. May a platform binary publish under a scoped name beside `thinkthen`?
5. Does a record verb accept a `Uint8Array` of UTF-8 bytes with an offsets array, or only arrays of strings? The byte form is the only one that skips a copy per record.

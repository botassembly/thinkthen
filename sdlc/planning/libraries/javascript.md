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
- **Every crossing copies.** Field-by-field conversion pays a crossing per field, and JSON text parses the same bytes twice. One chunk goes down and one comes up, built as JavaScript values in Rust.
- **An `await` inside a `for` loop sends one request at a time.** Record verbs hand the iterable to Rust at width `jobs`.
- **A forgotten `await` is truthy.** `if (tt.decide(...))` always takes the yes branch. The docs turn on `@typescript-eslint/no-floating-promises` and `@typescript-eslint/no-misused-promises`.

## How little code

**The binding tool is `napi-rs` over Node-API.** Node-API is ABI stable, so one binary per platform serves every Node version that targets it. Rust keeps the socket, the retries, the pool, the scheduler, and the recordings.

**Runtimes.** Node is covered. Bun and Deno claim Node-API support, and running there is unchecked. No native addon reaches a browser or an edge worker. A browser gets a package over a server the user runs.

**The shim contains** argument conversion, the promise from an async task, the async iterable over chunks, `AbortSignal` wired to cancellation, errors, declarations, and two entry wrappers.

**It never contains** an HTTP call, a retry, a rate limit wait, threshold math, JSON building, or a recording read or write.

**Build and ship.** `napi build` produces one `.node` per platform. A user installs `thinkthen`, and each platform binary publishes under its own scoped name with `os` and `cpu` fields, listed under `optionalDependencies`. Prebuilt: darwin-arm64, darwin-x64, linux-x64-gnu, linux-arm64-gnu, linux-x64-musl, win32-x64-msvc. `exports` names an `import` and a `require` condition.

## Tests only this surface needs

- Event loop delay during a thousand-record run matches the idle reading.
- An aborted signal rejects the promise and ends the iterable.
- One tarball answers a replayed case in ESM and CommonJS projects, with `--ignore-scripts` and no Rust toolchain.
- Type tests assert both overloads, and a lint fixture fails on a missing `await`.

## Open questions for the ADR

1. Does the tuple overload hold up when a configured threshold is typed `number | [number, number]`?
2. Do Bun and Deno load the addon, and may the first release claim them?
3. Does breaking out of a `for await` early cancel the Rust work?
4. May a platform binary publish under a scoped name beside `thinkthen`?

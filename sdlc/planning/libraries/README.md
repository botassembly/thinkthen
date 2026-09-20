# The seven surfaces: shared goals and anti-goals

Status: a planning study for the rewrite of ADR 0017. Ian asked for it on 2026-09-20. His words: "The product is the code, it's the library, it's the API." Each surface has its own page beside this one. This page holds what all seven share.

## The list

R keeps getting left out. This is the one list. Any page that names the surfaces links here.

| No. | Surface | Page | How it reaches the engine |
| --- | --- | --- | --- |
| 1 | The command. A shell runs it, and any program can start it as a child process. Both are one binary | [command.md](command.md) | It is the engine with an argument parser and a printer |
| 2 | Rust | [rust.md](rust.md) | The crate itself |
| 3 | Python | [python.md](python.md) | A native extension |
| 4 | JavaScript and TypeScript | [javascript.md](javascript.md) | A native addon |
| 5 | Ruby | [ruby.md](ruby.md) | A native extension |
| 6 | R | [r.md](r.md) | A native extension |
| 7 | C | [c.md](c.md) | The engine as a shared library with one generated header |

Ian approved Ruby, R, and C on 2026-09-20. `annotate` is the launch, and no library is part of it.

## Ian's two rules

1. **Fast.** "These models are so fast that any millisecond that we're wasting in a slow language is not done. If we can get microseconds of performance on some of the code, we should try to get that."
2. **The least code to maintain.** "The more we can do in Rust, pushing it down below that barrier, the better." One consistent set of tests works across every surface.

## What the two rules overturn

ADR 0017 is still Proposed. Its item 3 says the host language owns everything that touches the world: sending, waiting on a rate limit, several requests at once, and recording files. That writes the same machinery once for every host language. It is the most code and the slowest path, so Ian's two rules reverse it. ADR 0017 also picks WebAssembly for JavaScript. WebAssembly cannot open a socket, so that choice forces the sending into JavaScript. A native addon lets Rust own it. The rewrite of ADR 0017 decides both. Ian can overturn either.

## Three layers

- **The core.** The rules: the question-file grammar, the request and the response shapes, the threshold and the band, the recording names. It stays pure, as it is today.
- **The engine.** Rust that touches the world: one pooled connection to the backend, retries and the wait a rate limit asks for, the scheduler that runs many requests at once, record, replay, and cache, and the reading of the two environment variables. This layer is what gets bound. ADR 0017 bound the core alone, and this is the change.
- **A shim per surface.** It converts arguments, calls one engine function, converts the result, and maps a failure to the language's own error. The command is the first shim.

## Where the time goes

One measured judgment took over 300 ms, and nearly all of it is the network. A shim's own cost is microseconds. The order of work follows the size of the win.

1. **Keep the connection open.** A new secure connection costs a large share of a judgment. The engine holds one pool for the life of the process. This is the largest gain a library has over the command. The command pays for a new process and a new connection on every call.
2. **Run many requests at once inside Rust.** A record verb hands its records to the engine, and the engine runs them at the width `--jobs` names. The host language's lock is released while Rust waits.
3. **Send many questions in one request.** The vendor measured twelve times cheaper and ten times faster. `annotate` and `tag` ride on it.
4. **Cross the barrier once per batch.** One call carries a chunk of records down and one carries the answers up. A call per record pays the conversion a thousand times.
5. **Start fast.** The command has no work before its first request beyond parsing arguments. A library import loads one compiled file and nothing else.
6. **Then count microseconds.** The shim builds no JSON, copies no evidence twice, and allocates nothing the engine already holds.

## Budgets, each a failing build

These numbers are proposals until a bench measures them. A bench runs against a stub backend on the loopback address, so the network is out of the number.

- A shim adds under 50 microseconds to one engine call, at the 99th percentile.
- An import or a `require` takes under 20 ms.
- The command reaches its first request byte in under 10 ms from a cold start.
- Each library has no runtime dependency in its host language.
- Each shim has a line ceiling in `sdlc/ratchet.json`, as the source has today.

## One set of tests

- **The cases are data.** A `conformance/` folder holds one case per file: the verb, the arguments, the evidence, the recorded exchange, the expected bare answer, the expected details, and the expected failure kind. The recording format already exists and already replays with no network and no key.
- **Every surface runs every case.** The command runs them too, so it is held to the same suite as the libraries. A runner is about a hundred lines per language.
- **A new verb adds its cases once.** No language writes a test for a rule. A rule lives in the core, and its cases live in `conformance/`.
- **A language tests only its shim**: the conversion of types, the mapping of errors, an interrupt from the keyboard while Rust waits, the release of the language's lock, and safety across threads and forks.
- **One matrix runs all seven** on every change to the core or the engine.

## Shared goals

- Every surface gives the same answer for the same case, to the byte.
- The names are the command's names. The options are the command's options.
- A bare answer comes back by default, and details come on request.
- No, not sure, and broken never read alike.
- A user installs a prebuilt package. Nobody needs a Rust toolchain.
- A recording made on one surface replays on every other.

## Shared anti-goals

- No rule is written in a host language. No threshold math, no JSON building, no retry, no file format.
- No feature exists on one surface only.
- No second implementation stands by as a fallback in the host language.
- No client object is required for the common case.
- No telemetry, no log of the evidence, and no configuration file read on the hot path.
- No claim about speed without the bench that measured it.

## The seventh is C, and C is the door to the rest

Ian asked whether a seventh language made sense, and he approved C on 2026-09-20. His words: "Add C as the seventh language. I like that."

Nobody writes C. Rust exports the functions, and a tool writes the header. The engine ships as a shared library and a static library with that one header. A small surface is enough: one call that takes a request as JSON text and returns the answer as JSON text, and one call that frees it. [c.md](c.md) holds the goals.

C is ruled in because every other language can call it. No library is promised for any language below. The header makes each one possible for whoever wants it.

- **Zig.** It reads a C header directly.
- **Java, Kotlin, and Scala.** Java 22 and later call a C interface with no glue code. Older Java needs JNI. Java matters in hospital and enterprise systems.
- **Go.** Go can call C, and Go programmers avoid it because it breaks their easy cross-builds. The command as a child process serves Go well. Go is the awkward one.
- **Swift, C#, C++, Julia, Lua, PHP, and Dart** all call a C interface.

An eighth library is Ian's call. The recommendation is to ship these seven and let the header carry the rest.

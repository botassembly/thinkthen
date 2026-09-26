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

Nearly all of a judgment's time is the network. A shim's own cost is microseconds. The order of work follows the size of the win.

1. **Keep the connection open.** A new secure connection costs a large share of a judgment. The engine holds one pool for the life of the process. This is the largest gain a library has over the command. The command pays for a new process and a new connection on every call.
2. **Run many requests at once inside Rust.** A record verb hands its records to the engine, and the engine runs them at the width `--jobs` names. The host language's lock is released while Rust waits.
3. **Send many questions in one request.** The vendor measured twelve times cheaper and ten times faster. `annotate` and `tag` ride on it.
4. **Cross the barrier once per batch.** One call carries a chunk of records down and one carries the answers up. A call per record pays the conversion a thousand times.
5. **Start fast.** The command has no work before its first request beyond parsing arguments. A library import loads one compiled file and nothing else.
6. **Then count microseconds.** The shim builds no JSON, copies no evidence twice, and allocates nothing the engine already holds.

## Use every bulk form the host offers

Ian ruled on 2026-09-20: "Make sure we're maximizing performance using whatever the database engine supports, whether it's vectorization, batching, or whatever schemes we need... That's true for all the libraries." The rule binds all ten surfaces.

- **Each surface finds the widest native container its host has and takes it whole.** A character vector in R. A list, a NumPy array, a pandas or Polars column, or an Arrow array in Python. An array or an async iterable in JavaScript. An `Enumerable` in Ruby. A slice or a stream in Rust. An array of pointers and lengths in C. A chunk of rows in DuckDB. The per-surface page names the container and says how it crosses into Rust once, with no copy where the host allows it.
- **Work the engine can skip is skipped before any request.** Equal pairs of question and evidence inside one batch are asked once, and the answer is shared. A constant column is one judgment. A dictionary-encoded column is one judgment per distinct value. A value already in the cache costs nothing.
- **Every question about one piece of evidence rides in one request.** Measured on 2026-09-20: forty questions in one request billed 20.8 times fewer tokens than forty requests, and no answer changed.
- **The width is one number for the whole process.** Host threads, database worker threads, and async tasks all pass one attempt gate in the engine. The first engine given a width sets it, an engine given none follows it, and a later engine given a different width fails before it sends. Two callers never double the width by accident.
- **A scalar, one-at-a-time form may exist for convenience. Its page says it is serial, and points at the bulk form in the same breath.**
- **Each experiment measures it.** A surface reports rows a second through its bulk form against the stub, beside the engine's own number from pure Rust. A gap between the two is a defect in the shim.
- Packing many records into one request is a separate question. It trades accuracy for speed, and a measurement decides it. `../databases/README.md` has the state of that.

## Budgets, each a failing build

These numbers are proposals until a bench measures them. A bench runs against a stub backend on the loopback address, so the network is out of the number.

- A shim adds under 50 microseconds to one engine call, at the 99th percentile.
- An import or a `require` takes under 20 ms.
- The command reaches its first request byte in under 10 ms from a cold start.
- Each library has no runtime dependency in its host language.
- Each shim has a line ceiling in `sdlc/ratchet.json`, as the source has today. The experiments measured the honest floors: about 400 code lines for Python (330 was the floor without Arrow), 700–800 for the Rust user layer across all eight verbs set per module, 173 for the C runner, about 240 for the SQLite surface, 299 for PostgreSQL, and about 400 for DuckDB's provable functions.

## One set of tests

- **The cases are data.** A `conformance/` folder holds one data file that every surface reads, so a pick that changes in review is one edit (ADR 0017 section 7, Ian's condition of 2026-09-21). Each case carries the verb, the arguments, the evidence, the recorded exchange, the expected bare answer, the expected details, and the expected failure kind. The recording format already exists and already replays with no network and no key. The one file exists today at `experiments/207-thinkthen-db/engine/cases2/conformance.json`, twenty cases, validated offline.
- **Every surface runs every case.** The command runs them too, so it is held to the same suite as the libraries. A runner is about a hundred lines per language.
- **A new verb adds its cases once.** No language writes a test for a rule. A rule lives in the core, and its cases live in `conformance/`.
- **A language tests only its shim**: the conversion of types, the mapping of errors, an interrupt from the keyboard while Rust waits, the release of the language's lock, and safety across threads and forks.
- **One matrix runs all seven** on every change to the core or the engine.

## Shared goals

- Every surface gives the same answer for the same case, to the byte.
- The names are the command's names. The options are the command's options.
- A bare answer comes back by default, and details come on request.
- No, not sure, and broken never read alike. "Not sure" is the host's own empty value, and the public word is "unsure" (ADR 0017 picks 3 and 4): `None` in Python, `nil` in Ruby, `null` in JavaScript, `NA` in R. A band's three answers read through that empty value, and every page's band example leads with the empty-value check, the way the command's help shows `case $?`.
- `score` returns a number on every surface, the specification's probability-weighted position from 0 to K−1, and the nearest level's name rides in `details` (ADR 0017 pick 6).
- A user installs a prebuilt package. Nobody needs a Rust toolchain.
- A recording made on one surface replays on every other.

## Shared anti-goals

- No rule is written in a host language. No threshold math, no JSON building, no retry, no file format.
- No feature exists on one surface only.
- No second implementation stands by as a fallback in the host language.
- No client object is required for the common case.
- No telemetry, no log of the evidence, and no configuration file read on the hot path.
- No claim about speed without the bench that measured it.

## What every library shows

Ian's ruling, 2026-09-20: "I just want there to be a clean layer of eight functions, and it's kept as simple as possible from all the libraries. The libraries should be the eight functions plus the question setup thing."

- The public surface of every library is the eight verbs (`decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`), the question setup, and the few types an answer needs: the three-valued outcome, the details of a result, and the error.
- The engine is private plumbing. No user imports it, no document shows it, and no version promise covers it. The binding crates sit in this workspace and are never published, so they may call it freely.
- A check fails the build when a library's public surface grows past this list without an ADR.

## One crate, one way to install the command

Ian's ruling, 2026-09-20: one crate named `thinkthen`. No `thinkthen-core` and no `thinkthen-cli` is ever published. "I just don't really want to have multiple landing zones."

- The command installs through a `curl` installer that downloads a release from GitHub. That path has to work.
- `cargo install thinkthen` should give the command too, if the one crate can carry the library and the binary. It is welcome and it is second.
- The C surface ships from the same GitHub releases: the shared library, the static library, and the header.

## A saved question is a value

Ian asked on 2026-09-20 what the library equivalent of the command's saved question is, whether the name is right, and whether the libraries should bake in currying so nobody needs a class.

**The name stays.** The repository calls it a question file, and a file of several named questions is a question set. No page says "argument file". The name says what the file holds, and the line "a question is a file" sells it.

**In a library the same thing is a question value.** It is built with the keys the file uses, so the file and the code share one grammar.

    asks_for_refund = tt.question(decide="The customer asks for a refund.", threshold=0.9)
    route = tt.question(choose="Which team owns this?", options=["billing", "shipping", "other"])

- The keywords are the file's keys: `decide`, `choose`, `score`, `tag`, `options`, `levels`, `labels`, `true`, `false`, `threshold`, `on`, `model`. Loading a file and building from keywords give the same value. In JavaScript the object literal is the file's JSON. Rust has no keywords and gets a builder. In C the question is the JSON text.
- A question is frozen data. Nobody extends a class and nobody builds a client.
- Every verb takes a question value wherever it takes question text: the first slot.
- Calling a question with evidence asks it once: `asks_for_refund(message)`.
- One `decide` question serves three verbs, as it does on the command line: `decide`, `filter`, and `rank`.
- A question set is a plain mapping from a name to a question, and `annotate` takes it.

| Verb | Takes | Configure once | Then run |
| --- | --- | --- | --- |
| `decide` | a `decide` question | `q = tt.question(decide=...)` | `q(text)` or `tt.decide(q, text)` |
| `choose` | a `choose` question | `tt.question(choose=..., options=...)` | `q(text)` or `tt.choose(q, text)` |
| `score` | a `score` question | `tt.question(score=..., levels=...)` | `q(text)` or `tt.score(q, text)` |
| `tag` | a `tag` question | `tt.question(tag=..., labels=...)` | `q(text)` or `tt.tag(q, text)` |
| `filter` | a `decide` question | the same value `decide` takes | `tt.filter(q, records)` |
| `rank` | a `decide` question | the same value | `tt.rank(q, records, top=10)` |
| `find` | question text. `find` reads no question file today | none | `tt.find("Which line names the date?", lines)` |
| `annotate` | a question set | `{"spam": q1, "folder": q2}` or a loaded file | `tt.annotate(questions, records)` |

**No automatic currying.** A verb called with no evidence is an error. If it returned a function instead, a forgotten argument would hand `if` a function, and a function is true in JavaScript and in Ruby. That is the forgotten `await` bug again. Partial application stays free for anyone who wants it, because the question comes first: Python's `functools.partial(tt.decide, question)` already works.

**The bulk spelling follows the host (ADR 0017 pick 8).** `filter`, `rank`, and `annotate` take the host's container and cross once. Python, TypeScript, and Ruby also spell `decide`'s bulk form `decide_many`, because a string is also a sequence there and guessing is a trap. R and SQL keep a vectorized `decide`, which is their habit. The C door carries `thinkthen_decide_many`. `decide_many` is `decide`'s bulk spelling, not a ninth verb, and the surface check admits it by name on those surfaces.

**One speed trap, and the pages must name it.** A called question inside the host language's own `filter` or loop makes one judgment at a time. `tt.filter(q, records)` hands the whole list to the engine, and the engine runs it at full width. The documents show the record verb first.

## The seventh is C, and C is the door to the rest

Ian asked whether a seventh language made sense, and he approved C on 2026-09-20. His words: "Add C as the seventh language. I like that."

Nobody writes C. Rust exports the functions, and a tool writes the header. The engine ships as a shared library and a static library with that one header. A small surface is enough: one call that takes a request as JSON text and returns the answer as JSON text, and one call that frees it. [c.md](c.md) holds the goals.

C is ruled in because every other language can call it. No library is promised for any language below. The header makes each one possible for whoever wants it.

- **Zig.** It reads a C header directly.
- **Java, Kotlin, and Scala.** Java 22 and later call a C interface with no glue code. Older Java needs JNI. Java matters in hospital and enterprise systems.
- **Go.** Go can call C, and Go programmers avoid it because it breaks their easy cross-builds. The command as a child process serves Go well. Go is the awkward one.
- **Swift, C#, C++, Julia, Lua, PHP, and Dart** all call a C interface.

An eighth library is Ian's call. The recommendation is to ship these seven and let the header carry the rest.

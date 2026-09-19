# ADR 0017: Libraries for other languages over one bound core

- Status: Proposed. It becomes Accepted on Ian's word. No library is built until then
- Date: 2026-09-19

Four of the ten circulating use cases live inside a program: picking an agent's next action, checking whether an agent is stuck, picking a skill, and screening every turn. A program can start the shell tool as a child process for a handful of decisions, and that is the wrong shape for a loop. Ian asked for the same grammar as a library in Python, in JavaScript and TypeScript, and in Rust. He also ruled that other backends will come, so nothing may tie a library to one vendor.

## The design

1. **One core, bound into every language.** `thinkthen-core` is compiled into each library: a native extension for Python, WebAssembly for JavaScript and TypeScript, and the crate itself for Rust. Every rule exists once. The core holds the question-file grammar with every refusal and its sentence, precedence with the source of each setting, the threshold rule, the score arithmetic, each adapter's encoder and decoder, the result row, the encoding behind every digest, and the recording key.
2. **The core's public surface is a handful of plain functions over text and bytes.** Parse a question file. Resolve a question from a file, overrides, and defaults. Encode a request. Decode a response into a row. Name a recording. The core touches no socket, no file, no clock, and no environment, as it does today. The shell tool is the first caller of this surface, and each library is another.
3. **The host language owns everything that touches the world.** Each library writes, in its own language, the sending of a request, the wait a rate limit asks for, several requests at once, the reading and writing of recording files, the reading of the key from the environment, and the types that make a row feel native. Nothing else is written per language.
4. **The grammar is the shell tool's grammar.** The six commands keep their names as functions. The question comes first and the evidence second, and each option of the command line is a keyword argument under the same word. Rust uses a builder, because it has no keyword arguments.

   ```python
   d = tt.decide("The command only reads files.", cmd, threshold="0.1:0.9")
   match d.outcome:
       case Outcome.YES: allow()
       case Outcome.NO: refuse()
       case Outcome.UNRESOLVED: ask_a_person()
   ```

5. **Unresolved is a value, and a failure is an error.** A failure is an exception in Python, a rejected promise in TypeScript, and an `Err` in Rust. No error carries an outcome, and no outcome names an error. A Python decision refuses to be read as a boolean, so `if tt.decide(...)` fails loudly and an unsure answer can never pass for a no.
6. **The same files work everywhere.** `Question.load("refund.json")` reads the file the shell tool reads as `@refund.json`, under the same precedence: the call site, then the file, then the default. `d.row()` returns the row that `--details` prints, so every transform works on library output. `Client(replay="tests/recordings")` replays the recordings the shell tool makes, so a test needs no key and no network. Because one encoder writes the request bytes, a recording made in one language replays in every other.
7. **One backend seam, and no vendor inside it.** An adapter is two pure functions: a plan goes in and bytes come out, and bytes go in and answers come out. The host language sends the bytes. A built-in adapter lives in the core. A user's adapter is plain code in the host language beside it, and its `name` enters the recording key. No vendor's client is a dependency of any library, not even an optional one, because a second encoder would change the bytes that name a recording. No public type, option, error, or page in any language carries a vendor's word.
8. **What does not carry over.** Exit codes, `--quiet`, and `--raw` stay in the shell, because a library returns values. `filter` returns the very objects it was handed. The transforms stay `jq` over rows, and version one of a library ships no metrics of its own.
9. **Packages.** `thinkthen` on PyPI and on npm. On crates.io, `thinkthen-core` stays the pure crate and `thinkthen` becomes a library with the binary as its first caller.

## The order of work

The libraries start after version one of the shell tool is whole, which means after `annotate` and `find` land.

1. The core's surface becomes the plain functions of item 2, with the shell tool as their caller. No behavior changes.
2. A spike binds `decide` alone for Python. It builds the packages in the push check, installs one on a clean machine with no Rust, and reports the lines of wrapper code and what broke. The spike decides whether this design holds.
3. Python in full: `decide`, `choose`, `score`, question files, replay, and rows first. Then `filter`, `rank`, `annotate`, and several requests at once.
4. JavaScript and TypeScript over WebAssembly, for Node first. The browser waits for someone who needs it.
5. A small set of shared cases checks each binding end to end: the same question file and the same recorded response give the same row and the same digests in every language.

## Why one core and not a port per language

`sdlc/planning/sdk-design-study.md` recommends a plain port per language, held to a large shared set of cases. This ADR departs from it for three reasons.

- **The study's objection to a bound core rests on a wrong picture.** It says an adapter written in Python would have to call back through the binding. That holds only if the core drives the request, and the core drives nothing. Item 7 is the answer.
- **The rules are not small once they are counted.** Item 1 lists them. Ports would write all of them three times, and every refusal sentence would have to match in three places.
- **The hardest rule to port is the one that must never drift.** A recording is found by a digest of the exact request bytes. Python's JSON writer escapes every character outside ASCII by default, and JavaScript's and Rust's do not. One accented letter in the evidence would give a Python port a different digest, and a recording made by the shell tool would silently fail to replay.

Speed is no part of the argument. The network call takes hundreds of milliseconds, and everything the core does takes microseconds.

## What it costs

- The release workflow builds a Python package per platform. A machine with no matching package needs a Rust toolchain to install from source.
- Bundlers load WebAssembly in different ways, so the JavaScript package needs care for each target it claims to support.
- A debugger stops at the edge of the core. The core is small and pure, and it answers every mistake with a sentence.
- Each language still carries one small HTTP client with retries.
- If the spike fails, the fallback is the study's design: plain ports held to shared cases.

## What this asks of the tickets being built now

Each line is decided by the agent, and Ian can overturn it.

- **Ticket 0017.** The question-file parser takes text and never a path. Precedence is one pure function that also returns the source of each setting. The encoding that `question_sha256` digests is written into `question-file.md` as a rule.
- **Tickets 0014 and 0015.** The cut, the sort, the tiebreak, the `--top` slice, and the grouping of questions are pure functions in the core. `meta.questions_sha256` digests the question set that results and never the file's bytes.
- **Ticket 0020.** The default address, the default model, and the adapter's name move into the adapter's module. A `Backend` trait in Rust waits for step 1 above or for a second adapter, because a trait with one implementation is a guess.
- **`meta` gains no `adapter` field yet.** ADR 0010 dropped it, and `meta.url` names what answered. The field returns with the second adapter.
- **An agent skill for the shell tool is written in the release pass.** The study gives its outline and its five rules: word the question so that yes permits the action, use a band for anything risky, never treat a failure as a no, keep the details rows, and test with replay.

## For Ian

1. Whether libraries are wanted after version one of the shell tool, and whether Python comes first.
2. Whether the bound core is the design, with ports as the fallback if the spike fails.
3. The names `thinkthen` on PyPI, npm, and crates.io and `thinkthen-core` on crates.io were unclaimed on 2026-09-19. Claiming a name is an outward act and is his alone.

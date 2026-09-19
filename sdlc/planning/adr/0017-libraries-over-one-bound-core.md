# ADR 0017: Libraries for other languages over one bound core

- Status: Proposed. It becomes Accepted on Ian's word. Nothing is built from it until then, except the small guards named under "Decided now"
- Date: 2026-09-19

Ian asked whether the grammar of the shell tool can also exist as a library for Python, for JavaScript and TypeScript, and for Rust, perhaps with the Rust core compiled to WebAssembly. He also ruled that other backends will come, so nothing may tie the project to one vendor. A second agent read the core, the vendor's two thin clients, two community Rust crates, and one real third-party caller, and it wrote `sdlc/planning/sdk-design-study.md`. This ADR judges that study, and it departs from the study on the main question.

## Why a library at all

`ten-use-cases.md` found four uses that live inside a program: picking an agent's next action, checking whether an agent is stuck, picking a skill, and screening every turn. A program written in Python can start the shell tool as a child process, and that works for a handful of decisions. It is the wrong shape for a loop. The study also read one real caller of the vendor's client. That caller wrote by hand a limit on concurrent calls, a digest, a cache of results, and fakes for tests, and it still loses a whole batch when one row fails. The shell tool already owns every one of those jobs.

## What the grammar looks like

The six commands keep their names. Each takes the question first and the evidence second, and the options of the command line become keyword arguments under the same words.

```python
d = tt.decide("The command only reads files.", cmd, threshold="0.1:0.9")
match d.outcome:
    case Outcome.YES: allow()
    case Outcome.NO: refuse()
    case Outcome.UNRESOLVED: ask_a_person()
```

- Unresolved is a value, and a failure is an exception. Neither can be mistaken for the other. A Python decision refuses to be read as a boolean, so `if tt.decide(...)` fails loudly.
- `Question.load("refund.json")` reads the same question file the shell tool reads as `@refund.json`. Precedence is the same: the call site, then the file, then the default.
- `d.row()` returns the same result row that `--details` prints, so every transform works on library output unchanged.
- `Client(replay="tests/recordings")` replays the same recordings the shell tool makes, so a test needs no key and no network.
- Exit codes, `--quiet`, and `--raw` do not carry over, because a library returns values.

## Options

- **A. Bind the Rust core into every language.** A native extension for Python and WebAssembly for JavaScript. The rules can never drift, because one copy exists. The cost is a build per platform for Python, a different loader per JavaScript target, and a debugger that stops at the edge of the extension. An adapter written in Python would have to call back through the binding.
- **B. A plain port per language, held to a shared set of cases.** Each library is ordinary code in its own language with no native part. A folder of JSON cases in this repository states every rule as input and expected output: every edge of a threshold, every refusal of a question file, precedence, the exact bytes of a request, and the digests. Every library must pass every case, and a ticket that changes a rule adds its case first. The cost is that each rule is written once per language and a rule the cases miss can drift.
- **C. No libraries. Programs start the shell tool.** This costs nothing and leaves the four in-program uses badly served.

## Recommendation

**A, bind the Rust core, in stages, and Python first.** The first draft of this ADR followed the study and recommended B. Ian challenged it on 2026-09-19: performance aside, one copy of the rules is the whole point of having a pure core. The agent read the study's argument again and reversed. Three of its points do not hold.

- **The study's seam objection rests on a wrong picture.** It says an adapter written in Python would have to call back through the binding. That is true only if the core drives the request. It does not. The core touches no socket, no file, and no clock, and an adapter is two pure functions: plan in and bytes out, bytes in and answers out. The host language sends the request. A Python adapter is then plain Python beside the built-in one, and nothing calls back through anything.
- **The rules are not that small once they are counted.** The core holds the question-file grammar with every refusal and its sentence, precedence with the source of each setting, the threshold rule, the score arithmetic, the adapter's encoder and decoder, the row, the encoding behind every digest, and the recording key. Under B all of that is written three times, and every refusal sentence must match in three places. Under A each language writes only what is truly its own: sending the request, waiting and retrying, running several at once, reading and writing recording files, and wrapping the row in types that feel native.
- **The hardest rule to port is the one that must never drift.** A recording is found by a digest of the exact request bytes. Python's JSON writer escapes every character outside ASCII by default, and JavaScript's and Rust's do not. One accented letter in the evidence would give a Python port a different digest, and a recording made by the shell tool would silently fail to replay. The study itself called this the hard part. Under A the bytes come from one encoder, so recordings are shared across languages by construction.

What A really costs. Python wheels are built per platform in the release workflow, and a machine with no matching wheel needs a Rust toolchain. This is a solved problem with standard tools, and it is paid once. JavaScript gets the core as WebAssembly, and bundlers differ in how they load it, so Node comes first and the browser waits for someone who needs it. A debugger stops at the edge of the core, which matters little because the core is small, pure, and answers every mistake with a sentence. Speed is not part of the argument either way, because the network call takes hundreds of milliseconds and everything else takes microseconds.

The stages:

1. The core's public surface becomes a handful of plain functions over text and bytes: parse a question file, resolve a question from a file and overrides, encode a request, decode a response into a row, and name a recording. The shell tool is their first caller and the Rust library is their second.
2. A spike binds `decide` alone for Python, builds the wheels in the push check, and installs one on a clean machine with no Rust. It reports the lines of wrapper code and what broke. The spike decides whether A holds, and B remains the fallback if it does not.
3. Python in full: `decide`, `choose`, `score`, question files, replay, and rows first, then `filter`, `rank`, `annotate`, and several requests at once.
4. JavaScript and TypeScript over WebAssembly, Node first.

A small set of shared cases still exists, and it is far smaller than under B. It checks each binding end to end: the same question file, the same recorded response, the same row, and the same digest in every language.

The libraries start after version one of the shell tool is whole, which means after `annotate` and `find` land. The cost of this recommendation is a release workflow that builds native packages, and one small HTTP client with retries in each language.

## Decided now by the agent, and Ian can overturn each

- **No vendor's client is ever a dependency of a library,** not even an optional one. A library that imports one vendor's client is tied to that vendor. A second encoder would also break the digest that names a recording. The default adapter needs one POST with a key header, the wait a rate limit asks for, and the bytes back.
- **No public name in any language carries a vendor's word.** Each library has one small backend interface with a `name`, and the first adapter behind it speaks the shape the shell tool speaks today.
- **The core stays ready.** Ticket 0017's parser takes text and never a path, its precedence is one pure function that also returns the source of each setting, and the encoding that `question_sha256` digests is written into `question-file.md` as a rule. Tickets 0014 and 0015 keep the cut, the sort, the tiebreak, the `--top` slice, and the grouping of questions as pure functions in the core. Ticket 0015 makes `meta.questions_sha256` digest the resolved question set and never the file's bytes.
- **Ticket 0020 moves three constants.** The study found the default address, the default model, and the adapter's name used as constants outside the adapter's module. The move is small and changes no behavior. It waits for tickets 0017 and 0019, which touch the same files. A `Backend` trait in Rust waits for the library split or for a second adapter, whichever comes first, because a trait with one implementation is a guess.
- **`meta` gains no `adapter` field yet.** ADR 0010 dropped it, and `meta.url` names what answered. The field returns with the second adapter.
- **An agent skill for the shell tool is written in the release pass.** The study gives its outline and its five rules: word the question so that yes permits the action, use a band for anything risky, never treat a failure as a no, keep the details rows, and test with replay.

## For Ian

1. Whether libraries are wanted after version one of the shell tool, and whether Python comes first.
2. The bound Rust core, as he first pictured and as the agent now recommends, or ports held by shared cases, as the study recommends.
3. The names `thinkthen` on PyPI, npm, and crates.io and `thinkthen-core` on crates.io were all unclaimed on 2026-09-19. Claiming a name is an outward act and is his alone.

# The surface samples against the branch API

Status: Open

Owner: the library team.

A read-only review of `origin/surfaces` (at `ce177c1`, tip `4dc276a` when this was checked) compared the drawn surface samples with the branch's headers, READMEs, and NOTES. The deck's `surfaces.md` and `recognize-surfaces.md` are fixed in mktg `6394261`, and the site pulled them in `741816f`. This page records what did not match, and what still blocks a real run on any surface.

## The mismatches, fixed on the deck side

- Rust: `Question::decide(asks).band(0.2, 0.8)?` did not compile. `decide` returns a `Result`, so the chain is `Question::decide(asks)?.band(0.2, 0.8)?`. The Rust NOTES filed this as finding 1.
- Rust: `Recognize::kinds([...])` does not exist. The contract ships `Recognize::new().kinds([...])`.
- Rust: the comment "reads THINKTHEN_API_KEY" was false. The stand-in reads no key.
- Rust: the sample used `ticket`, `refunds`, and `review` without defining them. It now loops over two inline texts.
- PostgreSQL: `thinkthen_annotate('form.json', body)` is refused. Every file name on that surface needs the `@` spelling, `'@form.json'`.
- C: the recognize sample freed with `thinkthen_string_free`. The header names `thinkthen_free_string`. The slide overwrote `rc` without checking it and printed nothing. It now checks `rc` and prints the outcome and the probability.
- DuckDB: `thinkthen_relate((SELECT id, body FROM bugs), ...)` fails in the binder. The C API cannot register a function that takes a subquery, so the query goes in as a string.
- DuckDB: `INSTALL thinkthen FROM community` does not work today. The branch loads the extension file with `duckdb -unsigned`, and the path needs a `./` or DuckDB searches the system library path.
- Ruby: the sample scored an undefined `outage`, and its comment pinned 2.0. The Ruby NOTES recorded 1.7 from the stand-in. The sample now scores an inline text and says "near the top".
- Python and TypeScript: the samples judged `text`, `message`, `messages`, and `inbox` without showing them. Each text is now inline. TypeScript logged the whole `urgent[0]` object, and it now logs `urgent[0].record`.
- Relate's both-ways rule: SQLite spells it `'either:same_as'`. DuckDB takes it only as inline JSON, `'{"either": ["same_as"]}'`. PostgreSQL reads every entry as a bare relation name, so `'either:same_as'` there would name a relation called `either:same_as`. PostgreSQL has no both-ways spelling yet.

## What still blocks a real run

- The stand-in engine (`standin/src/lib.rs`) sends no key. Its module doc says so: "No key is read and none is sent." Every library and database surface binds the stand-in, so no surface can reach the paid backend until the real engine binds. Only the command reaches it today.
- `choose`, `score`, `tag`, and `annotate` have no live capture on any surface. The wire suites run against the loopback stub, and the stub answers one probability per request. A choice or a score needs a distribution, so the DuckDB NOTES record the stub refusing the shape.
- The branch's slide tests for Python (`libraries/python/tests/slide_sample.py`), TypeScript (`libraries/typescript/tests/slide.test.mjs`), and Ruby (`libraries/ruby/tests/slide_sample.rb`) are older than the deck text. Each copies an earlier drawn sample and pins the stand-in's keyword answers. None of them runs the sample the deck shows now.
- The PostgreSQL sample reads a `tickets` table the slide never draws, and its `team` and `urgency` fields come from the extension's own fixture form. The deck's `form.json` asks `steps`, `area`, and `impact`.

## What would close it

The real engine binds under every surface and sends the key only to the named address. Each surface's slide test then extracts the drawn sample from `surfaces.md` at run time. The DuckDB `tools/run_slide.sh` runs a saved copy in `tools/slide.sql`, so it goes stale the same way. A recording of each sample replays in the gate with no key.

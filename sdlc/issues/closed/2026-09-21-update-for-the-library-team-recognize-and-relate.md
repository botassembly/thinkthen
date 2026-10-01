# Update for the library team: two new functions, `recognize` and `relate`

Status: Closed on 2026-09-22. The library team landed both functions on all nine surfaces and closed this update.

Ian told the library team about `recognize` on 2026-09-21. He has since ruled that `relate` is a function of its own and is built with it. The count is ten functions and one special form, the question file. This page is what the library and database extension work needs to get ready. Nothing here is built in the core yet, and the core comes first.

## Read these, in this order

1. `sdlc/planning/recognize-design.md`: the command, the relation rule, the output object, and the call on all nine surfaces.
2. `sdlc/planning/relate-design.md`: the same for `relate`.
3. The marketing repository's `decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md`: every call written out in each language and each database. This page is the acceptance test. Each call must run as written.
4. The real outputs to test against: local experiment 222's `output.json`, local experiment 225's `edges-0.5.jsonl`, and local experiment 226's `arm/run.jsonl`.

## What is the same as the other eight

The names, one crossing for a list or a column, the six error kinds, cancel, the cache, and the question file going where the question goes. ADR 0017 holds unchanged.

## What is new for a binding

1. **The result has no fixed size.** `recognize` returns a list of names and a list of relations. `relate` returns a list of edges. Rust returns typed structs. C returns a JSON string and one free function, and every other language parses that string into its own records. Plan the C surface around that one pattern.
2. **Offsets.** `start` and `end` must slice the name out of the original text in the host's own string indexing. Python counts code points, JavaScript counts UTF-16 units, Rust and C count bytes. Each binding converts once. The test text holds an accented letter and an emoji before the name.
3. **The relation rule.** A name, a `from` end, and a `to` end. Each end is a kind or `"*"`. `either` marks a relation that reads the same both ways. One parser in the core checks it. No binding checks it again.
4. **`from` is a reserved word in Python.** The JSON says `from` and `to`. The Python record says `source` and `target`. The library team says whether any other host needs the same care, and whether every host should then say `source` and `target`. One answer for all nine is worth more than the nicer word.
5. **Data frames.** `recognize` over a column gives a long table, one row per name, with the source row's index. `relate` over a column gives a table of edges, ready for a graph library.
6. **Databases.** `recognize` gives rows: a list of structs in DuckDB, a table-valued function in SQLite, a set-returning function in PostgreSQL. `relate` needs every record at once, so it takes a table or a query and no single row. It is the first function with that form, and each engine's page needs a section on it.
7. **The join rule.** The database pages teach "recognize once, then join on the names", and they carry the measured cost of a join by meaning. The numbers come from the surfaces experiment.

## One thing the core team decides and the bindings then follow

`recognize` calls the number on a relation `confidence`. `relate` calls it `probability`. The suggestion is `probability` for relations in both. Bind neither name until that is ruled.

## What the recognize experiments closed on, for context

The method is final: three passes, each a pick-one question. The confirmed numbers and the list of claims nobody may make are in local experiment 222's `MARKETING-SCORECARD.md`. Relations are beta on every page.

## Added later the same day: Polars

Python's data frame is Polars, by Ian's ruling. What goes in decides what comes out: a Series returns a Series, an expression returns an expression, and a DataFrame with `on=` returns a DataFrame. The full table is `sdlc/issues/2026-09-21-the-polars-shape-as-the-deck-shows-it.md`. The `recognize` and `relate` frame forms in `recognize-surfaces.md` now take and return Polars frames.

## Added 2026-09-21, after the surfaces experiment closed: the brief for both functions

The surfaces team is cleared to start. The job is **two functions, `recognize` and `relate`**, on all nine surfaces and the Polars door, on branch `surfaces`, against the stand-in engine. No paid call.

Read in this order:

1. `sdlc/planning/recognize-design.md` and `sdlc/planning/relate-design.md`. They rule what a user types and what comes back. Each now ends with a rulings section dated 2026-09-21.
2. The marketing repository's `decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md`. Every call written out for both functions. It is the acceptance test.
3. Local experiment 225's `cases/`. Forty recorded cases in the ruled shape, and the `relate` set beside them. The stand-in answers from these. They become conformance cases for both functions.
4. `sdlc/planning/build-team-response-to-handoff-2026-09-21.md`, section on public shapes. Follow its result shapes: the `requests` list, the failed marker, and the record that comes back with its answer.

What to prove, beyond "the calls run as written":

- Character offsets are right in each host's own string indexing, on a text with an accent and an emoji.
- A result of no fixed size crosses the C door as one returned string with one free function.
- DuckDB returns a list of structs that `unnest` turns into rows. SQLite and PostgreSQL return rows. The "Names become rows" slide runs as drawn.
- Python spells a relation's ends `source` and `target`. A Polars frame in gives a long Polars frame out, one row per name and one row per edge.
- `relate` takes every record at once. A database gives it a table or a query. It refuses more than 255 records with a usage error on every surface.
- The any-kind end is the one-character string `"*"` everywhere except Rust.

What to leave alone: the method itself, how a long text is cut, and the question count per request. Those are the engine's, and the build team owns them. The number on a relation is `probability`. The number on a recognized name is `strength`, settled after the recognize team's comparison. The harvest cases say `confidence`, and the rename in the conformance file is mechanical. The rule is `sdlc/issues/closed/2026-09-21-one-rule-for-every-number-the-tool-prints.md`.

This work does not touch the build team. It stays on the `surfaces` branch against the stand-in. The merge of `surfaces` into main remains the build team's gate.

## Closed by the library team, 2026-09-21

Both functions landed on all nine surfaces and the Polars door, from `1af5290` through the DuckDB page text on branch `surfaces`. The full report is `FINDINGS.md` at the branch root, "The recognize and relate addition". Where the design pages were wrong or unclear, four findings, all filed: the deck's `located_in` rule has no recording; the deck's DuckDB `relate` subquery cannot bind on the stable C API and crosses as a string; case 72 lost its `form` field in translation and is restored; the brief's `confidence` spelling for a name's number was overturned by the one-rule page mid-flight and the rename to `strength` is mechanical history. The manual's page-section text for `relate`'s whole-table form is written in each database lane's notes for the build team to lift. Nothing waited on the build team; the merge of `surfaces` remains their gate.

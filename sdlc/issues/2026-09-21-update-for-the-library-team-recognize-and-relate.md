# Update for the library team: two new functions, `recognize` and `relate`

Status: Open

Ian told the library team about `recognize` on 2026-09-21. He has since ruled that `relate` is a function of its own and is built with it. The count is ten functions and one special form, the question file. This page is what the library and database extension work needs to get ready. Nothing here is built in the core yet, and the core comes first.

## Read these, in this order

1. `sdlc/planning/recognize-design.md`: the command, the relation rule, the output object, and the call on all nine surfaces.
2. `sdlc/planning/relate-design.md`: the same for `relate`.
3. `repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md`: every call written out in each language and each database. This page is the acceptance test. Each call must run as written.
4. The real outputs to test against: `experiments/222-recognize-demo/output.json`, `experiments/225-relate-demo/edges-0.5.jsonl`, and `experiments/226-graph-demo/arm/run.jsonl`.

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

The method is final: three passes, each a pick-one question. The confirmed numbers and the list of claims nobody may make are in `experiments/222-recognize-demo/MARKETING-SCORECARD.md`. Relations are beta on every page.

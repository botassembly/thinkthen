# Database extensions: DuckDB, SQLite, and PostgreSQL

Status: a study. Ian ruled the three extensions in on 2026-09-20. No code is authorized by this page.

Ian's words: "I want to make extensions for all three using Rust. That'll be the next set of experiments after the libraries are done. I won't hold up the launch for the database extensions, but I definitely want to do them as a fast follow."

They are surfaces eight, nine, and ten over the one Rust engine. `../libraries/README.md` holds the two rules every surface obeys: as fast as possible, and the least code to maintain, with everything possible pushed down into Rust. Both rules bind here. One page per database sits beside this one: [duckdb.md](duckdb.md), [sqlite.md](sqlite.md), and [postgres.md](postgres.md).

## Why this, and why soon

A table is where most text already sits, and SQL users will not leave it to call a command. The wish is proven. Jev opened on 2026-09-15, and five days later these were public, read by survey on 2026-09-20:

| Project | What it is | What to know |
| --- | --- | --- |
| `realZachi/pg-jev` | PostgreSQL functions in PL/Python, 243 stars | The design the others copy. It sends whole rows, packs many rows into one request, and measured that accuracy falls past 20 to 25 rows in a request |
| `colliber/duckdb-jev` | A DuckDB extension in C++, 14 stars | Takes an explicit text argument. Checks the question when the query is planned. Keeps the key in a DuckDB secret. Warns that the same call in `WHERE` and in `SELECT` is billed twice |
| `recodelabs/duckdb-jev` | A DuckDB extension in C++, a port of `pg-jev` | Packs 40 rows a request by default, past the cliff that `pg-jev` measured. Installs unsigned |
| A DuckDB extension by a well-known DuckDB developer | Announced on 2026-09-16, "about 10sec for 1k rows" | No public source found |
| `jev-pandas` | A data frame wrapper in Python | No stars yet |

Nobody has shipped one for SQLite. Nobody has shipped one for PostgreSQL in Rust. None of them records and replays, none keeps answers on disk between sessions, and none reports which model and which request produced an answer.

## The SQL surface, the same in all three

The question comes first and the text second, as in every other surface. The text is always a value the caller names. The extension never sends a whole row.

| Function | Returns | Notes |
| --- | --- | --- |
| `thinkthen_decide(question, text)` | A boolean, and `NULL` for "not sure" | SQL already has three-valued logic. `WHERE thinkthen_decide(...)` keeps the yes rows, and `IS NULL` finds the rows for a person |
| `thinkthen_probability(question, text)` | A double | `ORDER BY` it and the query is `rank`. Add `LIMIT 1` and it is `find` over rows |
| `thinkthen_choose(question, text, options)` | Text, and `NULL` under the threshold | |
| `thinkthen_score(question, text, levels)` | A double | |
| `thinkthen_tag(question, text, labels)` | A list of text. JSON text in SQLite | One request for all the labels |
| `thinkthen_annotate(question_set, text)` | A struct, `jsonb`, or JSON text, one member per question | One request for every question about one row. This is the 20.8 times saving measured on 2026-09-20 |
| `thinkthen_details(question, text)` | The `--details` object | The probability, the model version, and the request digest. This is the audit trail the others lack |
| `thinkthen_usage()` | A table | Requests, answers from the cache, and tokens for this session |

`filter`, `rank`, and `find` get no functions. `WHERE`, `ORDER BY`, and `LIMIT` are those verbs, and SQL users already know them. Eight functions are the whole surface.

A question is either the bare sentence or JSON text with the keys a question file uses, such as `{"decide": "...", "threshold": 0.9}`. That form works in all three databases and adds no new grammar. The per-database pages may offer a native struct beside it. They may not drop it.

## Rules every extension keeps

1. **No rule lives in the extension.** Thresholds, bands, scoring, request building, retries, and the cache are the engine's. The extension maps SQL types in and out.
2. **Every bulk form the database offers is used.** Ian ruled on 2026-09-20 that each surface maximizes performance with whatever its engine supports: vectors, batches, or any other scheme. `../libraries/README.md` has the rule for all ten surfaces. DuckDB hands a function about 2,048 rows at a time, and the whole chunk crosses into the engine once and runs at full width there. A constant vector is one judgment, and a dictionary vector is one judgment per distinct value. DuckDB's worker threads share one scheduler, so the width is one number for the process. SQLite and PostgreSQL call a scalar function one row at a time, so each page gives a bulk form that feeds the engine many rows, and says plainly that the scalar form is serial. Equal pairs of question and text are asked once. Each page lists every such mechanism its database has, and each experiment measures rows a second beside the engine's own number.
3. **The key never appears in SQL text.** No function takes it as an argument. Statement logs, `pg_stat_statements`, and shell history would hold it. DuckDB uses a secret. PostgreSQL uses a setting that only a superuser sets and that no view shows. SQLite reads the environment variable.
4. **The question is checked when the query is planned** wherever the database allows it, so a bad threshold fails before the first paid request.
5. **Answers are kept on disk.** The engine's immutable cache is keyed by the whole request. The same call in `WHERE` and in `SELECT` is asked once. Tomorrow's run of the same query costs nothing. A recording folder makes a SQL test run with no network and no key.
6. **A failure is an error, never a `NULL`.** `NULL` means "not sure". A setting may turn a failed row into `NULL` for a long query, and it is off by default.
7. **A query can be cancelled.** A wait on the network checks for the database's interrupt, and a statement timeout is honored.
8. **The function is marked volatile** unless a page proves a weaker marking safe. The cache stops a second bill. The planner cannot be trusted to.

## Rows in one request: measured, and one record per request stays the rule

Every project above packs many rows into one request, and that is why they report thousands of rows in a few seconds. ThinkThen sends one request per record, and `specification/annotate.md` says records never share a request. Inside the vendor's documented rate limit that is about 980 short rows a minute and about 1.2 US cents per thousand. `pg-jev` reports 2,000 rows in 3.5 seconds for 1.2 US cents.

Our own run on 2026-09-20 used the largest project's layout on the same 1,000 SMS messages as the accuracy round. `sdlc/issues/2026-09-20-packing-rows-into-one-request-measured.md` has the tables.

- At 10 rows per request the accuracy matched one row per request, 0.968, for 3.3 times fewer tokens and ten times fewer requests.
- At 20 rows the accuracy fell to 0.941. At 40 rows the recall fell from 0.94 to 0.43, and the later rows in a request did worst. One public DuckDB extension defaults to 40.
- At 10 rows, dealing the same messages into different groups changed the answer for 34 rows of 1,000. A row's answer depends on its neighbors.

So packing keeps the totals and loses the single answers. It also breaks replay, because a recording is keyed by the whole request. One record per request stays the rule and the default on every surface. If packing is ever offered it is an engine option for an ADR, off by default, capped at ten, for `filter` and `rank` over short rows only. No page here assumes it.

## Shared goals

- One `INSTALL` or one file, with no Rust toolchain on the user's machine.
- The shared conformance cases run as SQL under replay, with the same expected answers as every library.
- The extension's own code stays a thin shim with a line ceiling in `sdlc/ratchet.json`.
- The name is `thinkthen` in every registry, with no suffix.

## Shared anti-goals

- No whole-row sending, and no function that reads a table the caller did not name.
- No text generation, no embeddings, no vector search. Other extensions do those.
- No function that writes to a table or a file.
- No background worker, queue table, or daemon in the first version.
- No difference in names or meanings between the three databases beyond what a type system forces.

## The order

DuckDB first. Its vectors fit the engine's width, its community repository signs and ships the binary, and it is where the public interest is. SQLite second, because nobody has done it and it is the smallest. PostgreSQL third. It is the hardest: each connection is its own forked process, managed services allow only approved extensions, and the realistic first user runs their own server or a Docker image.

The experiments run as `experiments/207-thinkthen-db/` in the workspace, after experiment 205 on the libraries, and they bind the same stand-in engine and stub.

## What Ian can overturn

Everything but his own ruling that the three are built in Rust and do not hold the launch. The function list, the order, and the packing question are open.
